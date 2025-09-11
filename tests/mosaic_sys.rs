// Comprehensive tests for NVAPI Mosaic query/getter functions
// All tests return Result<(), String> with NVAPI error codes when failures occur
#![allow(unused_must_use)]

extern crate nvapi;

use nvapi::sys::{self, status_result};
use nvapi::sys::mosaic as m;
use nvapi::sys::types::NV_RECT;
use nvapi::Status;

fn init() -> bool {
    matches!(nvapi::initialize(), Ok(()))
}

#[test]
fn test_mosaic_struct_sizes() {
    println!("NV_MOSAIC_GRID_TOPO_V1 size: {}", std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_V1>());
    println!("NV_MOSAIC_GRID_TOPO_V2 size: {}", std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_V2>());
    println!("NV_MOSAIC_GRID_TOPO size: {}", std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO>());
    
    println!("NV_MOSAIC_GRID_TOPO_DISPLAY_V1 size: {}", std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_DISPLAY_V1>());
    println!("NV_MOSAIC_GRID_TOPO_DISPLAY_V2 size: {}", std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_DISPLAY_V2>());
    
    println!("NV_MOSAIC_DISPLAY_SETTING_V1 size: {}", std::mem::size_of::<m::NV_MOSAIC_DISPLAY_SETTING_V1>());
    println!("NV_MOSAIC_DISPLAY_SETTING_V2 size: {}", std::mem::size_of::<m::NV_MOSAIC_DISPLAY_SETTING_V2>());
    
    println!("NV_MOSAIC_GRID_TOPO_VER1: 0x{:08x}", m::NV_MOSAIC_GRID_TOPO_VER1);
    println!("NV_MOSAIC_GRID_TOPO_VER2: 0x{:08x}", m::NV_MOSAIC_GRID_TOPO_VER2);
}

#[test]
fn test_mosaic_get_supported_topo_info() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        let mut info = m::NV_MOSAIC_SUPPORTED_TOPO_INFO::zeroed();
        info.version = m::NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER;
        
        let status = m::NvAPI_Mosaic_GetSupportedTopoInfo(&mut info, m::NV_MOSAIC_TOPO_TYPE_ALL);
        match status_result(status) {
            Ok(()) => {
                println!("GetSupportedTopoInfo: topoBriefsCount={} displaySettingsCount={}", 
                         info.topoBriefsCount, info.displaySettingsCount);
                
                // Print first few topology briefs
                let count = std::cmp::min(info.topoBriefsCount as usize, 3);
                for i in 0..count {
                    let brief = &info.topoBriefs[i];
                    println!("  topo[{}]: type={:?} enabled={} isPossible={}", 
                             i, brief.topo, brief.enabled, brief.isPossible);
                }
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

#[test]
fn test_mosaic_get_topo_group() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First get supported topologies
        let mut info = m::NV_MOSAIC_SUPPORTED_TOPO_INFO::zeroed();
        info.version = m::NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER;
        
        let status = m::NvAPI_Mosaic_GetSupportedTopoInfo(&mut info, m::NV_MOSAIC_TOPO_TYPE_ALL);
        if status_result(status).is_err() || info.topoBriefsCount == 0 {
            return Err("NoSupportedTopologies".into());
        }

        // Get group info for first topology
        let mut brief = info.topoBriefs[0];
        let mut group = m::NV_MOSAIC_TOPO_GROUP::zeroed();
        group.version = m::NVAPI_MOSAIC_TOPO_GROUP_VER;
        
        let status = m::NvAPI_Mosaic_GetTopoGroup(&mut brief, &mut group);
        match status_result(status) {
            Ok(()) => {
                println!("GetTopoGroup: count={} brief.topo={:?} brief.enabled={} brief.isPossible={}", 
                         group.count, group.brief.topo, group.brief.enabled, group.brief.isPossible);
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

#[test]
fn test_mosaic_get_current_topo() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        let mut brief = m::NV_MOSAIC_TOPO_BRIEF::zeroed();
        brief.version = m::NVAPI_MOSAIC_TOPO_BRIEF_VER;
        let mut setting = m::NV_MOSAIC_DISPLAY_SETTING::zeroed();
        setting.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER;
        let mut overlap_x: i32 = 0;
        let mut overlap_y: i32 = 0;
        
        let status = m::NvAPI_Mosaic_GetCurrentTopo(&mut brief, &mut setting, &mut overlap_x, &mut overlap_y);
        match status_result(status) {
            Ok(()) => {
                println!("GetCurrentTopo: topo={:?} enabled={} isPossible={} overlap=({},{}) res={}x{} bpp={} freq={} rrx1k={}", 
                         brief.topo, brief.enabled, brief.isPossible, overlap_x, overlap_y,
                         setting.width, setting.height, setting.bpp, setting.freq, setting.rrx1k);
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

#[test]
fn test_mosaic_get_overlap_limits() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // Get a supported topology first
        let mut info = m::NV_MOSAIC_SUPPORTED_TOPO_INFO::zeroed();
        info.version = m::NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER;
        
        let status = m::NvAPI_Mosaic_GetSupportedTopoInfo(&mut info, m::NV_MOSAIC_TOPO_TYPE_ALL);
        if status_result(status).is_err() || info.topoBriefsCount == 0 || info.displaySettingsCount == 0 {
            return Err("NoSupportedTopologies".into());
        }

        let mut brief = info.topoBriefs[0];
        // Use V1 display settings since that's what the supported topo info contains
        let mut setting_v1 = m::NV_MOSAIC_DISPLAY_SETTING_V1 {
            version: m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1,
            width: info.displaySettings[0].width,
            height: info.displaySettings[0].height,
            bpp: info.displaySettings[0].bpp,
            freq: info.displaySettings[0].freq,
        };
        let mut min_x = 0i32; let mut max_x = 0i32; let mut min_y = 0i32; let mut max_y = 0i32;
        
        let status = m::NvAPI_Mosaic_GetOverlapLimits(&mut brief, 
                                                      &mut setting_v1 as *mut _ as *mut m::NV_MOSAIC_DISPLAY_SETTING, 
                                                      &mut min_x, &mut max_x, &mut min_y, &mut max_y);
        match status_result(status) {
            Ok(()) => {
                println!("GetOverlapLimits: X=({},{}) Y=({},{})", min_x, max_x, min_y, max_y);
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

#[test]
fn test_mosaic_get_display_viewports_by_resolution() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    // Get display IDs and try multiple ones if needed
    let display_ids = match (|| -> Option<Vec<u32>> {
        let gpus = nvapi::PhysicalGpu::enumerate().ok()?;
        let mut all_displays = Vec::new();
        for gpu in gpus {
            if let Ok(displays) = gpu.display_ids_all() {
                for display in displays {
                    all_displays.push(display.display_id);
                }
            }
        }
        if all_displays.is_empty() { None } else { Some(all_displays) }
    })() {
        Some(ids) => ids,
        None => return Err("NoDisplaysAvailable".into()),
    };

    println!("Available display IDs: {:?}", display_ids);

    // Try each display ID until one works or we run out
    for &display_id in &display_ids {
        unsafe {
            let mut viewports: [NV_RECT; sys::types::NVAPI_MAX_DISPLAYS] = 
                [NV_RECT { left: 0, top: 0, right: 0, bottom: 0 }; sys::types::NVAPI_MAX_DISPLAYS];
            let mut bezel_corrected: u8 = 0;
            
            let status = m::NvAPI_Mosaic_GetDisplayViewportsByResolution(
                display_id, 2560, 1440, // use the actual Mosaic resolution
                viewports.as_mut_ptr(),
                &mut bezel_corrected,
            );
            
            match status_result(status) {
                Ok(()) => {
                    println!("GetDisplayViewportsByResolution: displayId={} bezelCorrected={}", 
                             display_id, bezel_corrected);
                    
                    // Print non-zero viewports
                    for (i, rect) in viewports.iter().enumerate() {
                        if rect.left != 0 || rect.top != 0 || rect.right != 0 || rect.bottom != 0 {
                            println!("  viewport[{}]: left={} top={} right={} bottom={}", 
                                     i, rect.left, rect.top, rect.right, rect.bottom);
                        }
                    }
                    return Ok(());
                }
                Err(e) => {
                    println!("DisplayID {} failed with {:?}", display_id, e);
                    // Continue to next display ID
                }
            }
        }
    }
    
    // If we get here, all display IDs failed
    Err("AllDisplayIDsFailed".into())
}

#[test]
fn test_mosaic_enum_display_grids() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First try: single call with a pre-allocated buffer of reasonable size
        println!("Trying single-call approach with fixed buffer size...");
        let mut grids_fixed: Vec<m::NV_MOSAIC_GRID_TOPO_V1> = vec![m::NV_MOSAIC_GRID_TOPO_V1::zeroed(); 4]; // Try with 4 grids max
        for g in &mut grids_fixed {
            g.version = m::NV_MOSAIC_GRID_TOPO_VER1;
            g.displaySettings.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
        }
        
        let mut count_fixed = grids_fixed.len() as u32;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(grids_fixed.as_mut_ptr() as *mut m::NV_MOSAIC_GRID_TOPO, &mut count_fixed);
        match status_result(status) {
            Ok(()) => {
                println!("Single-call V1 succeeded: count={}", count_fixed);
                for (i, grid) in grids_fixed.iter().take(count_fixed as usize).enumerate() {
                    println!("  grid[{}]: {}x{} displays={} flags=0x{:08x}", 
                             i, grid.rows, grid.columns, grid.displayCount, grid.gridFlags);
                }
                return Ok(());
            }
            Err(e) => {
                println!("Single-call V1 failed: {:?}", e);
            }
        }

        // Try traditional two-phase approach but with count=0 initially
        println!("Trying two-phase with count=0...");
        let mut count: u32 = 0;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(std::ptr::null_mut(), &mut count);
        match status_result(status) {
            Ok(()) => {
                println!("EnumDisplayGrids: count={}", count);
                if count == 0 { 
                    println!("No grids reported, but call succeeded");
                    return Ok(()); 
                }
                
                // Try count=1 regardless of what driver reports
                let actual_count = std::cmp::min(count, 1);
                let mut grids_v1: Vec<m::NV_MOSAIC_GRID_TOPO_V1> = vec![m::NV_MOSAIC_GRID_TOPO_V1::zeroed(); actual_count as usize];
                for g in &mut grids_v1 {
                    g.version = m::NV_MOSAIC_GRID_TOPO_VER1;
                    g.displaySettings.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
                }
                
                let mut count_actual = actual_count;
                let status = m::NvAPI_Mosaic_EnumDisplayGrids(grids_v1.as_mut_ptr() as *mut m::NV_MOSAIC_GRID_TOPO, &mut count_actual);
                match status_result(status) {
                    Ok(()) => {
                        println!("Two-phase V1 succeeded: count={}", count_actual);
                        for (i, grid) in grids_v1.iter().enumerate() {
                            println!("  grid[{}]: {}x{} displays={} flags=0x{:08x}", 
                                     i, grid.rows, grid.columns, grid.displayCount, grid.gridFlags);
                        }
                        return Ok(());
                    }
                    Err(e) => {
                        println!("Two-phase V1 failed: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Count query failed: {:?}", e);
            }
        }
        Err("FIXME: All struct versions (V1, V2) and initialization approaches return IncompatibleStructVersion. ".into())
    }
}

#[test]
fn test_mosaic_validate_display_grids() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // Try to get an actual existing grid first to validate
        let mut count: u32 = 0;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(std::ptr::null_mut(), &mut count);
        if status_result(status).is_ok() && count > 0 {
            // If we can enum grids, try to get one and validate it
            let mut grids_v1: Vec<m::NV_MOSAIC_GRID_TOPO_V1> = vec![m::NV_MOSAIC_GRID_TOPO_V1::zeroed(); count as usize];
            for g in &mut grids_v1 {
                g.version = m::NV_MOSAIC_GRID_TOPO_VER1;
                g.displaySettings.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
            }
            
            let mut count_actual = count;
            let enum_status = m::NvAPI_Mosaic_EnumDisplayGrids(grids_v1.as_mut_ptr() as *mut m::NV_MOSAIC_GRID_TOPO, &mut count_actual);
            
            if status_result(enum_status).is_ok() && count_actual > 0 {
                // Validate the first enumerated grid
                let mut status_info = m::NV_MOSAIC_DISPLAY_TOPO_STATUS::zeroed();
                status_info.version = m::NV_MOSAIC_DISPLAY_TOPO_STATUS_VER;
                
                let validate_status = m::NvAPI_Mosaic_ValidateDisplayGrids(0, 
                                                                    &mut grids_v1[0] as *mut _ as *mut m::NV_MOSAIC_GRID_TOPO, 
                                                                    &mut status_info, 1);
                match status_result(validate_status) {
                    Ok(()) => {
                        println!("ValidateDisplayGrids (from enum): errorFlags=0x{:08x} warningFlags=0x{:08x} displayCount={}", 
                                 status_info.errorFlags, status_info.warningFlags, status_info.displayCount);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("Validation of enumerated grid failed: {:?}", e);
                        // Fall through to try synthetic grid
                    }
                }
            }
        }
        
        // If enum fails or validation fails, try with a synthetic minimal grid
        println!("Trying validation with synthetic minimal grid...");
        
        // Try V2 grid first
        let mut grid_v2 = m::NV_MOSAIC_GRID_TOPO_V2::zeroed();
        grid_v2.version = m::NV_MOSAIC_GRID_TOPO_VER2;
        grid_v2.rows = 1;
        grid_v2.columns = 1;
        grid_v2.displayCount = 0; // No displays for synthetic test
        grid_v2.gridFlags = 0;
        grid_v2.displaySettings.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
        grid_v2.displaySettings.width = 1920;
        grid_v2.displaySettings.height = 1080;
        grid_v2.displaySettings.bpp = 32;
        grid_v2.displaySettings.freq = 60;
        
        let mut status_info = m::NV_MOSAIC_DISPLAY_TOPO_STATUS::zeroed();
        status_info.version = m::NV_MOSAIC_DISPLAY_TOPO_STATUS_VER;
        
        // Try validation with different flags
        let validate_flags = [
            0, // No flags
            m::NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD, // No driver reload flag
        ];
        
        for &flags in &validate_flags {
            println!("Trying validation with flags=0x{:08x}...", flags);
            let status = m::NvAPI_Mosaic_ValidateDisplayGrids(flags, 
                                                               &mut grid_v2 as *mut _ as *mut m::NV_MOSAIC_GRID_TOPO, 
                                                               &mut status_info, 1);
            match status_result(status) {
                Ok(()) => {
                    println!("ValidateDisplayGrids (V2 flags=0x{:08x}): errorFlags=0x{:08x} warningFlags=0x{:08x} displayCount={}", 
                             flags, status_info.errorFlags, status_info.warningFlags, status_info.displayCount);
                    return Ok(());
                }
                Err(e) => {
                    println!("V2 validation with flags=0x{:08x} failed: {:?}", flags, e);
                }
            }
        }
        
        // Try V1 grid as fallback
        let mut grid_v1 = m::NV_MOSAIC_GRID_TOPO_V1::zeroed();
        grid_v1.version = m::NV_MOSAIC_GRID_TOPO_VER1;
        grid_v1.rows = 1;
        grid_v1.columns = 1;
        grid_v1.displayCount = 0; // No displays for synthetic test
        grid_v1.gridFlags = 0;
        grid_v1.displaySettings.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
        grid_v1.displaySettings.width = 1920;
        grid_v1.displaySettings.height = 1080;
        grid_v1.displaySettings.bpp = 32;
        grid_v1.displaySettings.freq = 60;
        
        let mut status_info_v1 = m::NV_MOSAIC_DISPLAY_TOPO_STATUS::zeroed();
        status_info_v1.version = m::NV_MOSAIC_DISPLAY_TOPO_STATUS_VER;
        
        let status = m::NvAPI_Mosaic_ValidateDisplayGrids(0, 
                                                           &mut grid_v1 as *mut _ as *mut m::NV_MOSAIC_GRID_TOPO, 
                                                           &mut status_info_v1, 1);
        match status_result(status) {
            Ok(()) => {
                println!("ValidateDisplayGrids (V1 synthetic): errorFlags=0x{:08x} warningFlags=0x{:08x} displayCount={}", 
                         status_info_v1.errorFlags, status_info_v1.warningFlags, status_info_v1.displayCount);
                Ok(())
            }
            Err(e) if e == Status::IncompatibleStructVersion => Err(format!("FIXME: All struct versions (V1, V2) and initialization approaches return IncompatibleStructVersion")),
            Err(e) => Err(format!("Validation with synthetic V1 grid failed: {:?}", e))
        }
    }
}

// Legacy XP-era API tests (marked as deprecated)

#[test]
#[ignore = "Legacy XP-era API"]
fn test_legacy_get_supported_mosaic_topologies() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        let mut topos = m::NV_MOSAIC_SUPPORTED_TOPOLOGIES::zeroed();
        topos.version = m::NVAPI_MOSAIC_SUPPORTED_TOPOLOGIES_VER;
        
        let status = m::NvAPI_GetSupportedMosaicTopologies(&mut topos);
        match status_result(status) {
            Ok(()) => {
                println!("GetSupportedMosaicTopologies (legacy): totalCount={}", topos.totalCount);
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

#[test]
#[ignore = "Legacy XP-era API"]
fn test_legacy_get_current_mosaic_topology() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        let mut topo = m::NV_MOSAIC_TOPOLOGY::zeroed();
        topo.version = m::NVAPI_MOSAIC_TOPOLOGY_VER;
        let mut enabled: u32 = 0;
        
        let status = m::NvAPI_GetCurrentMosaicTopology(&mut topo, &mut enabled);
        match status_result(status) {
            Ok(()) => {
                println!("GetCurrentMosaicTopology (legacy): enabled={} rowCount={} colCount={}", 
                         enabled, topo.rowCount, topo.colCount);
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e))
        }
    }
}

#[test]
fn test_mosaic_enum_display_grids_c_compat() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // Get count first
        let mut count: u32 = 0;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(std::ptr::null_mut(), &mut count);
        match status_result(status) {
            Ok(()) => {
                println!("EnumDisplayGrids: count={}", count);
                if count == 0 { return Ok(()); }
            }
            Err(e) => return Err(format!("{:?}", e))
        }
        
        // Well I'm not proud of the following, but I already lost my sanity why the damn StructVersion error keeps happening, so I don't care anymore.
        // Means: Raw dogging it with byte buffers and manually setting the version fields. In Rust. Insanity.
        // This shit still doesn't work. Great.

        // Try using raw buffer with C struct size calculations
        // Based on C definitions(taken from nvapi.h release 570):
        // - V1 display: 20 bytes (5 * u32)  
        // - V2 display: 28 bytes (7 * u32)
        // - Basic grid: 20 bytes (5 * u32 for version, rows, cols, count, flags)
        // - Display settings: 20 bytes (based on V1)
        
        // Try V1 approach with exact C layout
        // V1: 20 + (128 * 20) + 20 = 2600 bytes
        println!("Trying V1 with exact C struct size (2600 bytes)...");
        let mut buffer_v1: Vec<u8> = vec![0; 2600 * count as usize];
        
        // Initialize just the version field for each grid
        for i in 0..count as usize {
            let grid_offset = i * 2600;
            let version_ptr = buffer_v1.as_mut_ptr().add(grid_offset) as *mut u32;
            *version_ptr = 0x00010a28; // Exact C V1 version
        }
        
        let mut count_v1 = count;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(buffer_v1.as_mut_ptr() as *mut m::NV_MOSAIC_GRID_TOPO, &mut count_v1);
        match status_result(status) {
            Ok(()) => {
                println!("C-compatible V1 buffer succeeded: count={}", count_v1);
                return Ok(());
            }
            Err(e) => {
                println!("C-compatible V1 failed: {:?}", e);
            }
        }

        // Try V2 approach with exact C layout  
        // V2: 20 + (128 * 28) + 20 = 3624 bytes ✓
        println!("Trying V2 with exact C struct size (3624 bytes)...");
        let mut buffer_v2: Vec<u8> = vec![0; 3624 * count as usize];
        
        // Initialize just the version field for each grid
        for i in 0..count as usize {
            let grid_offset = i * 3624;
            let version_ptr = buffer_v2.as_mut_ptr().add(grid_offset) as *mut u32;
            *version_ptr = 0x00020e28; // Exact C V2 version
        }
        
        let mut count_v2 = count;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(buffer_v2.as_mut_ptr() as *mut m::NV_MOSAIC_GRID_TOPO, &mut count_v2);
        match status_result(status) {
            Ok(()) => {
                println!("C-compatible V2 buffer succeeded: count={}", count_v2);
                return Ok(());
            }
            Err(e) => {
                println!("C-compatible V2 failed: {:?}", e);
            }
        }

        // Try minimal approach - just allocate the memory and set minimal fields
        println!("Trying minimal V1 approach...");
        let grid_size_v1 = 2600;
        let mut minimal_buffer: Vec<u8> = vec![0; grid_size_v1 * count as usize];
        
        // Only set the absolutely required fields for each grid
        for i in 0..count as usize {
            let offset = i * grid_size_v1;
            let grid_ptr = minimal_buffer.as_mut_ptr().add(offset);
            
            // Set version (offset 0)
            *(grid_ptr as *mut u32) = 0x00010a28;
            // Set rows (offset 4) - try 1
            *(grid_ptr.add(4) as *mut u32) = 1;  
            // Set columns (offset 8) - try 1
            *(grid_ptr.add(8) as *mut u32) = 1;
            // Set displayCount (offset 12) - let driver fill this
            *(grid_ptr.add(12) as *mut u32) = 0;
            // Set gridFlags (offset 16) - no flags
            *(grid_ptr.add(16) as *mut u32) = 0;
            
            // Initialize displaySettings version at the end of displays array
            // displays array: 128 * 20 = 2560 bytes at offset 20
            // displaySettings: at offset 20 + 2560 = 2580
            *(grid_ptr.add(2580) as *mut u32) = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
        }
        
        let mut count_minimal = count;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(minimal_buffer.as_mut_ptr() as *mut m::NV_MOSAIC_GRID_TOPO, &mut count_minimal);
        match status_result(status) {
            Ok(()) => {
                println!("Minimal V1 succeeded: count={}", count_minimal);
                return Ok(());
            }
            Err(e) => {
                println!("Minimal V1 failed: {:?}", e);
            }
        }

        Err("FIXME: All struct versions (V1, V2) and initialization approaches return IncompatibleStructVersion".into())
    }
}

#[test]
fn test_driver_version_info() -> Result<(), String> {
    // Initialize NVAPI
    nvapi::initialize().map_err(|e| format!("Failed to initialize NVAPI: {:?}", e))?;
    
    // Get driver version and branch string
    let (driver_version, branch_string) = nvapi::driver_version()
        .map_err(|e| format!("Failed to get driver version: {:?}", e))?;
    println!("Driver Version: {}", driver_version);
    println!("Branch String: {}", branch_string);
    
    // Get interface version string
    let interface_version = nvapi::interface_version()
        .map_err(|e| format!("Failed to get interface version: {:?}", e))?;
    println!("Interface Version: {}", interface_version);
    
    // Get physical GPUs for device info
    let gpus = nvapi::PhysicalGpu::enumerate().map_err(|e| format!("Failed to enumerate GPUs: {:?}", e))?;
    if !gpus.is_empty() {
        let gpu = &gpus[0];
        
        // Get GPU short name
        let short_name = gpu.short_name().map_err(|e| format!("Failed to get GPU short name: {:?}", e))?;
        println!("GPU Short Name: {}", short_name);
        
        // Get GPU full name
        let full_name = gpu.full_name().map_err(|e| format!("Failed to get GPU full name: {:?}", e))?;
        println!("GPU Full Name: {}", full_name);
        
        // Get VBIOS version string
        let vbios_version = gpu.vbios_version_string().map_err(|e| format!("Failed to get VBIOS version: {:?}", e))?;
        println!("VBIOS Version: {}", vbios_version);
    }
    
    println!("Driver version info retrieved successfully");
    Ok(())
}
