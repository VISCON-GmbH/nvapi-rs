//! Mosaic display helpers.
//!
//! This module provides a thin, safe wrapper around NVAPI's Mosaic topology
//! functions to query, validate, and configure multi-display setups.
//!
//! NVIDIA Mosaic allows combining multiple displays into a single desktop surface
//! or creating passive stereo configurations with identical layouts for each eye.

use crate::sys::mosaic::{self};
use log::trace;
use nvapi_sys::{status_result};

pub use crate::sys::mosaic::*;

/// High-level Mosaic topology management.
///
/// This provides safe wrappers around NVAPI's Mosaic functions for querying
/// supported topologies, getting current configuration, and setting new layouts.
///
/// # Examples
///
/// ```no_run
/// use nvapi::mosaic::{Mosaic, MosaicTopoType};
///
/// // Get all supported basic mosaic topologies
/// let topologies = Mosaic::get_supported_topologies(MosaicTopoType::Basic)?;
/// println!("Found {} supported topologies", topologies.topoBriefsCount);
///
/// // Get current mosaic configuration
/// if let Ok(current) = Mosaic::get_current_topology() {
///     println!("Current topology: {:?}", current.0.topo);
/// }
/// # Ok::<_, nvapi::Status>(())
/// ```
pub struct Mosaic;

impl Mosaic {
    /// Queries all supported Mosaic topologies for the given type.
    ///
    /// Returns a structure containing topology briefs and compatible display settings.
    /// Each topology brief includes an `isPossible` flag indicating whether it can
    /// be enabled immediately with the current hardware configuration.
    ///
    /// For topologies that are not possible, use [`get_topology_details`] to inspect
    /// the validity mask and determine what's missing (GPUs, displays, etc.).
    ///
    /// # Arguments
    /// * `topo_type` - The type of topologies to query (All, Basic, PassiveStereo, etc.)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::{Mosaic, MosaicTopoType};
    ///
    /// let info = Mosaic::get_supported_topologies(MosaicTopoType::Basic)?;
    /// for i in 0..info.topoBriefsCount as usize {
    ///     let brief = &info.topoBriefs[i];
    ///     println!("Topology {:?}: possible={}", brief.topo, brief.isPossible != 0);
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_supported_topologies(
        topo_type: MosaicTopoType,
    ) -> crate::Result<mosaic::NV_MOSAIC_SUPPORTED_TOPO_INFO> {
        trace!("mosaic.get_supported_topologies({:?})", topo_type);
        let mut info = mosaic::NV_MOSAIC_SUPPORTED_TOPO_INFO::zeroed();
        info.version = mosaic::NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER;

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_GetSupportedTopoInfo(
                &mut info,
                topo_type.raw(),
            ))
            .map(|_| info)
        }
    }

    /// Gets detailed information about a specific topology.
    ///
    /// Returns detailed layout information including GPU assignments, validity status,
    /// and grid configuration for the specified topology.
    ///
    /// The validity mask in the returned details can be checked against
    /// `NV_MOSAIC_TOPO_VALIDITY_*` constants to determine why a topology
    /// might not be possible (missing GPU, missing display, mixed display types).
    ///
    /// # Arguments
    /// * `brief` - A topology brief obtained from [`get_supported_topologies`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::{Mosaic, MosaicTopoType, NV_MOSAIC_TOPO_VALIDITY_VALID};
    ///
    /// let topologies = Mosaic::get_supported_topologies(MosaicTopoType::Basic)?;
    /// if topologies.topoBriefsCount > 0 {
    ///     let brief = &topologies.topoBriefs[0];
    ///     let details = Mosaic::get_topology_details(brief)?;
    ///     
    ///     if details.validityMask == NV_MOSAIC_TOPO_VALIDITY_VALID {
    ///         println!("Topology is valid and ready to use");
    ///     } else {
    ///         println!("Topology has issues: validity mask = 0x{:x}", details.validityMask);
    ///     }
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_topology_details(
        brief: &mosaic::NV_MOSAIC_TOPO_BRIEF,
    ) -> crate::Result<mosaic::NV_MOSAIC_TOPO_GROUP> {
        trace!("mosaic.get_topology_details({:?})", brief.topo);
        let mut group = mosaic::NV_MOSAIC_TOPO_GROUP::zeroed();
        group.version = mosaic::NVAPI_MOSAIC_TOPO_GROUP_VER;

        // Need to create a mutable copy since the API expects a mutable pointer
        let mut brief_copy = *brief;

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_GetTopoGroup(&mut brief_copy, &mut group))
                .map(|_| group)
        }
    }

    /// Gets the current active Mosaic topology and settings.
    ///
    /// Returns the currently active topology brief, display settings, and overlap values.
    /// If no Mosaic topology is currently active, the topology will be `MosaicTopo::None`.
    ///
    /// # Returns
    /// A tuple containing:
    /// * `NV_MOSAIC_TOPO_BRIEF` - Brief description of the current topology
    /// * `NV_MOSAIC_DISPLAY_SETTING` - Per-display settings  
    /// * `(i32, i32)` - X and Y overlap values in pixels
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::{Mosaic, MosaicTopo};
    ///
    /// match Mosaic::get_current_topology() {
    ///     Ok((brief, settings, (overlap_x, overlap_y))) => {
    ///         if brief.topo != MosaicTopo::None {
    ///             println!("Active topology: {:?}", brief.topo);
    ///             println!("Overlap: {}x{} pixels", overlap_x, overlap_y);
    ///             println!("Resolution: {}x{}", settings.width, settings.height);
    ///         } else {
    ///             println!("No Mosaic topology is currently active");
    ///         }
    ///     }
    ///     Err(e) => println!("Failed to get current topology: {:?}", e),
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_current_topology() -> crate::Result<(
        mosaic::NV_MOSAIC_TOPO_BRIEF,
        mosaic::NV_MOSAIC_DISPLAY_SETTING,
        (i32, i32),
    )> {
        trace!("mosaic.get_current_topology()");
        let mut brief = mosaic::NV_MOSAIC_TOPO_BRIEF::zeroed();
        brief.version = mosaic::NVAPI_MOSAIC_TOPO_BRIEF_VER;
        
        let mut settings = mosaic::NV_MOSAIC_DISPLAY_SETTING::zeroed();
        settings.version = mosaic::NVAPI_MOSAIC_DISPLAY_SETTING_VER;
        
        let mut overlap_x: i32 = 0;
        let mut overlap_y: i32 = 0;

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_GetCurrentTopo(
                &mut brief,
                &mut settings,
                &mut overlap_x,
                &mut overlap_y,
            ))
            .map(|_| (brief, settings, (overlap_x, overlap_y)))
        }
    }

    /// Sets and optionally enables a new Mosaic topology.
    ///
    /// Configures the system to use the specified topology with the given display
    /// settings and overlap values. The topology can be enabled immediately or
    /// left configured but disabled.
    ///
    /// # Arguments
    /// * `brief` - Topology brief specifying which layout to use
    /// * `settings` - Per-display settings (resolution, refresh rate, etc.)
    /// * `overlap_x` - Horizontal overlap between displays in pixels
    /// * `overlap_y` - Vertical overlap between displays in pixels  
    /// * `enable` - Whether to enable the topology immediately (true) or just configure it (false)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::Mosaic;
    ///
    /// // Get a supported topology
    /// let topologies = Mosaic::get_supported_topologies(nvapi::mosaic::MosaicTopoType::Basic)?;
    /// if topologies.topoBriefsCount > 0 && topologies.displaySettingsCount > 0 {
    ///     let brief = &topologies.topoBriefs[0];
    ///     let settings = &topologies.displaySettings[0];
    ///     
    ///     // Set topology with no overlap and enable it
    ///     Mosaic::set_current_topology(brief, settings, 0, 0, true)?;
    ///     println!("Mosaic topology enabled");
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn set_current_topology(
        brief: &mosaic::NV_MOSAIC_TOPO_BRIEF,
        settings: &mosaic::NV_MOSAIC_DISPLAY_SETTING,
        overlap_x: i32,
        overlap_y: i32,
        enable: bool,
    ) -> crate::Result<()> {
        trace!(
            "mosaic.set_current_topology({:?}, overlap={}x{}, enable={})",
            brief.topo,
            overlap_x,
            overlap_y,
            enable
        );

        // Create mutable copies since the API expects mutable pointers
        let mut brief_copy = *brief;
        let mut settings_copy = *settings;

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_SetCurrentTopo(
                &mut brief_copy,
                &mut settings_copy,
                overlap_x,
                overlap_y,
                if enable { 1 } else { 0 },
            ))
        }
    }

    /// Enables or disables the current Mosaic topology.
    ///
    /// This function can enable a previously configured topology or disable
    /// the currently active one. The topology configuration is preserved
    /// when disabling, so it can be re-enabled later without reconfiguration.
    ///
    /// # Arguments
    /// * `enable` - true to enable the current topology, false to disable it
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::Mosaic;
    ///
    /// // Disable current mosaic topology
    /// Mosaic::enable_current_topology(false)?;
    /// println!("Mosaic disabled");
    ///
    /// // Re-enable it later
    /// Mosaic::enable_current_topology(true)?;
    /// println!("Mosaic re-enabled");
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn enable_current_topology(enable: bool) -> crate::Result<()> {
        trace!("mosaic.enable_current_topology({})", enable);
        unsafe {
            status_result(mosaic::NvAPI_Mosaic_EnableCurrentTopo(if enable {
                1
            } else {
                0
            }))
        }
    }

    /// Gets the valid overlap limits for a given topology and display settings.
    ///
    /// Returns the minimum and maximum allowed overlap values in pixels for both
    /// X and Y directions. These limits depend on the specific topology and the
    /// resolution/refresh rate of the display settings.
    ///
    ///
    /// # Arguments
    /// * `brief` - Topology brief to check limits for
    /// * `settings` - Display settings to check limits with
    ///
    /// # Returns
    /// A tuple containing `(min_x, max_x, min_y, max_y)` overlap limits in pixels.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::Mosaic;
    ///
    /// let topologies = Mosaic::get_supported_topologies(nvapi::mosaic::MosaicTopoType::Basic)?;
    /// if topologies.topoBriefsCount > 0 && topologies.displaySettingsCount > 0 {
    ///     let brief = &topologies.topoBriefs[0];
    ///     let settings = &topologies.displaySettings[0];
    ///     
    ///     match Mosaic::get_overlap_limits(brief, settings) {
    ///         Ok((min_x, max_x, min_y, max_y)) => {
    ///             println!("X overlap: {} to {} pixels", min_x, max_x);
    ///             println!("Y overlap: {} to {} pixels", min_y, max_y);
    ///         }
    ///         Err(nvapi::Status::IncompatibleStructVersion) => {
    ///             println!("FIXME: get_overlap_limits: Invalid struct version");
    ///         }
    ///         Err(e) => return Err(e),
    ///     }
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_overlap_limits(
        brief: &mosaic::NV_MOSAIC_TOPO_BRIEF,
        settings: &mosaic::NV_MOSAIC_DISPLAY_SETTING,
    ) -> crate::Result<(i32, i32, i32, i32)> {
        trace!("mosaic.get_overlap_limits({:?})", brief.topo);
        
        let mut brief_copy = *brief;
        let mut settings_copy = *settings;
        let mut min_x: i32 = 0;
        let mut max_x: i32 = 0;
        let mut min_y: i32 = 0;
        let mut max_y: i32 = 0;

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_GetOverlapLimits(
                &mut brief_copy,
                &mut settings_copy,
                &mut min_x,
                &mut max_x,
                &mut min_y,
                &mut max_y,
            ))
            .map(|_| (min_x, max_x, min_y, max_y))
        }
    }

    /// Enumerates currently active display grid topologies.
    ///
    /// Returns all active grid configurations including Mosaic, SLI Multi-Monitor (IG),
    /// Panoramic, and single display configurations. This is useful for getting
    /// the current state of all display arrangements.
    ///
    /// Note: This function tries V2 first, then falls back to V1 if there's a struct 
    /// version incompatibility. Some driver/hardware combinations may not support
    /// grid enumeration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::Mosaic;
    ///
    /// match Mosaic::enum_display_grids() {
    ///     Ok(grids) => {
    ///         println!("Found {} active display grids", grids.len());
    ///         for (i, grid) in grids.iter().enumerate() {
    ///             println!("Grid {}: {}x{} displays", i, grid.rows, grid.columns);
    ///             println!("  Resolution: {}x{}", grid.displaySettings.width, grid.displaySettings.height);
    ///         }
    ///     }
    ///     Err(e) => return Err(e),
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn enum_display_grids() -> crate::Result<Vec<mosaic::NV_MOSAIC_GRID_TOPO>> {
        // Try V2 first
        match Self::enum_display_grids_v2() {
            Ok(grids) => Ok(grids),
            Err(crate::Status::IncompatibleStructVersion) => {
                trace!("V2 failed with IncompatibleStructVersion, trying V1");
                Self::enum_display_grids_v1()
            }
            Err(e) => Err(e),
        }
    }

    /// Internal helper for V2 grid enumeration.
    fn enum_display_grids_v2() -> crate::Result<Vec<mosaic::NV_MOSAIC_GRID_TOPO>> {
        trace!("mosaic.enum_display_grids_v2() [count]");
        
        // First call to get count
        let mut count: u32 = 0;
        unsafe {
            status_result(mosaic::NvAPI_Mosaic_EnumDisplayGrids(
                std::ptr::null_mut(),
                &mut count,
            ))?;
        }

        if count == 0 {
            return Ok(Vec::new());
        }

        trace!("mosaic.enum_display_grids_v2() [fill] count={}", count);
        
        // Second call to fill the array
        let mut grids = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut grid = mosaic::NV_MOSAIC_GRID_TOPO::zeroed();
            grid.version = mosaic::NV_MOSAIC_GRID_TOPO_VER;
            grids.push(grid);
        }

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_EnumDisplayGrids(
                grids.as_mut_ptr(),
                &mut count,
            ))?;
            
            // NVAPI may return fewer grids than initially reported
            grids.set_len(count as usize);
        }

        Ok(grids)
    }

    /// Internal helper for V1 grid enumeration.
    fn enum_display_grids_v1() -> crate::Result<Vec<mosaic::NV_MOSAIC_GRID_TOPO>> {
        trace!("mosaic.enum_display_grids_v1() [count]");
        
        // First call to get count
        let mut count: u32 = 0;
        unsafe {
            status_result(mosaic::NvAPI_Mosaic_EnumDisplayGrids(
                std::ptr::null_mut(),
                &mut count,
            ))?;
        }

        if count == 0 {
            return Ok(Vec::new());
        }

        trace!("mosaic.enum_display_grids_v1() [fill] count={}", count);
        
        // Second call to fill the array - use V1 structs
        let mut grids_v1 = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut grid = mosaic::NV_MOSAIC_GRID_TOPO_V1::zeroed();
            grid.version = mosaic::NV_MOSAIC_GRID_TOPO_VER1;
            grids_v1.push(grid);
        }

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_EnumDisplayGrids(
                grids_v1.as_mut_ptr() as *mut mosaic::NV_MOSAIC_GRID_TOPO,
                &mut count,
            ))?;
            
            // NVAPI may return fewer grids than initially reported
            grids_v1.set_len(count as usize);
        }

        // Convert V1 grids to V2 format for consistent return type
        let mut grids_v2 = Vec::with_capacity(grids_v1.len());
        for grid_v1 in grids_v1 {
            let mut grid_v2 = mosaic::NV_MOSAIC_GRID_TOPO_V2::zeroed();
            grid_v2.version = mosaic::NV_MOSAIC_GRID_TOPO_VER2;
            grid_v2.rows = grid_v1.rows;
            grid_v2.columns = grid_v1.columns;
            grid_v2.displayCount = grid_v1.displayCount;
            grid_v2.gridFlags = grid_v1.gridFlags;
            grid_v2.displaySettings = grid_v1.displaySettings;
            
            // Convert V1 displays to V2 displays
            for i in 0..mosaic::NV_MOSAIC_MAX_DISPLAYS {
                let mut disp_v2 = mosaic::NV_MOSAIC_GRID_TOPO_DISPLAY_V2::zeroed();
                disp_v2.version = 2; // V2 version
                disp_v2.displayId = grid_v1.displays[i].displayId;
                disp_v2.overlapX = grid_v1.displays[i].overlapX;
                disp_v2.overlapY = grid_v1.displays[i].overlapY;
                disp_v2.rotation = grid_v1.displays[i].rotation;
                disp_v2.cloneGroup = grid_v1.displays[i].cloneGroup;
                // pixelShiftType is new in V2, default to None
                disp_v2.pixelShiftType = mosaic::PixelShiftType::NoPixelShift.raw();
                
                grid_v2.displays[i] = disp_v2;
            }
            
            grids_v2.push(grid_v2);
        }

        Ok(grids_v2)
    }

    /// Sets one or more display grid topologies.
    ///
    /// Applies the specified grid configurations to the system. This can configure
    /// multiple independent grids simultaneously. Use the set_topo_flags to control
    /// behavior like driver reloading and performance optimization.
    ///
    /// # Arguments
    /// * `grids` - Array of grid topologies to apply
    /// * `set_topo_flags` - Control flags (see `NV_MOSAIC_SETDISPLAYTOPO_FLAG_*`)
    ///
    /// Common flags:
    /// * `NV_MOSAIC_SETDISPLAYTOPO_FLAG_CURRENT_GPU_TOPOLOGY` - Use current GPU topology
    /// * `NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD` - Avoid driver reload if possible
    /// * `NV_MOSAIC_SETDISPLAYTOPO_FLAG_MAXIMIZE_PERFORMANCE` - Optimize for performance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::{Mosaic, NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD};
    ///
    /// // Get current grids and modify them
    /// let mut grids = Mosaic::enum_display_grids()?;
    /// if !grids.is_empty() {
    ///     // Modify first grid settings...
    ///     grids[0].displaySettings.width = 1920;
    ///     grids[0].displaySettings.height = 1080;
    ///     
    ///     // Apply the changes without driver reload
    ///     Mosaic::set_display_grids(&mut grids, NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD)?;
    ///     println!("Display grids updated");
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn set_display_grids(
        grids: &mut [mosaic::NV_MOSAIC_GRID_TOPO],
        set_topo_flags: u32,
    ) -> crate::Result<()> {
        trace!(
            "mosaic.set_display_grids(count={}, flags=0x{:x})",
            grids.len(),
            set_topo_flags
        );

        // Ensure all grids have proper version
        for grid in grids.iter_mut() {
            if grid.version == 0 {
                grid.version = mosaic::NV_MOSAIC_GRID_TOPO_VER;
            }
        }

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_SetDisplayGrids(
                grids.as_mut_ptr(),
                grids.len() as u32,
                set_topo_flags,
            ))
        }
    }

    /// Validates one or more display grid topologies without applying them.
    ///
    /// This function checks if the specified grid topologies are valid and can
    /// be applied. It returns detailed status information including any warnings
    /// or errors for each display in the configuration.
    ///
    /// Use this to preview changes before calling [`set_display_grids`].
    ///
    /// # Arguments
    /// * `grids` - Array of grid topologies to validate
    /// * `set_topo_flags` - Control flags (same as [`set_display_grids`])
    ///
    /// # Returns
    /// A vector of status structures, one per grid, containing per-display
    /// validation results and warning/error flags.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::{Mosaic, NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD};
    ///
    /// let mut grids = Mosaic::enum_display_grids()?;
    /// if !grids.is_empty() {
    ///     // Validate current configuration
    ///     let status = Mosaic::validate_display_grids(&mut grids, NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD)?;
    ///     
    ///     for (i, grid_status) in status.iter().enumerate() {
    ///         println!("Grid {}: {} displays", i, grid_status.displayCount);
    ///         if grid_status.displayCount > 0 {
    ///             for j in 0..(grid_status.displayCount as usize) {
    ///                 let disp_status = &grid_status.displays[j];
    ///                 println!("  Display {}: warnings=0x{:x}", j, disp_status.warningFlags);
    ///             }
    ///         }
    ///     }
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn validate_display_grids(
        grids: &mut [mosaic::NV_MOSAIC_GRID_TOPO],
        set_topo_flags: u32,
    ) -> crate::Result<Vec<mosaic::NV_MOSAIC_DISPLAY_TOPO_STATUS>> {
        trace!(
            "mosaic.validate_display_grids(count={}, flags=0x{:x})",
            grids.len(),
            set_topo_flags
        );

        // Ensure all grids have proper version
        for grid in grids.iter_mut() {
            if grid.version == 0 {
                grid.version = mosaic::NV_MOSAIC_GRID_TOPO_VER;
            }
        }

        // Prepare status array
        let mut status_vec = Vec::with_capacity(grids.len());
        for _ in 0..grids.len() {
            let mut status = mosaic::NV_MOSAIC_DISPLAY_TOPO_STATUS::zeroed();
            status.version = mosaic::NV_MOSAIC_DISPLAY_TOPO_STATUS_VER;
            status_vec.push(status);
        }

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_ValidateDisplayGrids(
                set_topo_flags,
                grids.as_mut_ptr(),
                status_vec.as_mut_ptr(),
                grids.len() as u32,
            ))
            .map(|_| status_vec)
        }
    }

    /// Gets display viewports for a given resolution on a specific display.
    ///
    /// Returns the viewport rectangles that would be applied to a display when
    /// using the specified source resolution in a Mosaic configuration. This is
    /// useful for understanding how content will be positioned and scaled.
    ///
    /// # Arguments
    /// * `display_id` - The display ID to query viewports for
    /// * `src_width` - Source width in pixels (0 = use current resolution)
    /// * `src_height` - Source height in pixels (0 = use current resolution)
    ///
    /// # Returns
    /// A tuple containing:
    /// * `crate::sys::types::NV_RECT` - The viewport rectangle
    /// * `bool` - Whether the resolution is bezel corrected
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::mosaic::Mosaic;
    ///
    /// // Get viewport for 4K resolution on display 0
    /// let (viewport, bezel_corrected) = Mosaic::get_display_viewports_by_resolution(0, 3840, 2160)?;
    /// println!("Viewport: {}x{} at ({}, {})", 
    ///          viewport.right - viewport.left, 
    ///          viewport.bottom - viewport.top,
    ///          viewport.left, viewport.top);
    /// println!("Bezel corrected: {}", bezel_corrected);
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_display_viewports_by_resolution(
        display_id: u32,
        src_width: u32,
        src_height: u32,
    ) -> crate::Result<(crate::sys::types::NV_RECT, bool)> {
        trace!(
            "mosaic.get_display_viewports_by_resolution(display={}, {}x{})",
            display_id,
            src_width,
            src_height
        );

        let mut viewport = crate::sys::types::NV_RECT::zeroed();
        let mut bezel_corrected: u8 = 0;

        unsafe {
            status_result(mosaic::NvAPI_Mosaic_GetDisplayViewportsByResolution(
                display_id,
                src_width,
                src_height,
                &mut viewport,
                &mut bezel_corrected,
            ))
            .map(|_| (viewport, bezel_corrected != 0))
        }
    }
}
