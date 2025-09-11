// Comprehensive tests for NVAPI Mosaic query/getter functions
// All tests return Result<(), String> with NVAPI error codes when failures occur
#![allow(unused_must_use)]

extern crate nvapi;

use nvapi::sys::mosaic as m;
use nvapi::sys::types::NV_RECT;
use nvapi::sys::{self, status_result};

fn init() -> bool {
    matches!(nvapi::initialize(), Ok(()))
}

#[test]
fn test_mosaic_struct_sizes() {
    println!(
        "NV_MOSAIC_GRID_TOPO_V1 size: {}",
        std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_V1>()
    );
    println!(
        "NV_MOSAIC_GRID_TOPO_V2 size: {}",
        std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_V2>()
    );
    println!(
        "NV_MOSAIC_GRID_TOPO size: {}",
        std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO>()
    );

    println!(
        "NV_MOSAIC_GRID_TOPO_DISPLAY_V1 size: {}",
        std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_DISPLAY_V1>()
    );
    println!(
        "NV_MOSAIC_GRID_TOPO_DISPLAY_V2 size: {}",
        std::mem::size_of::<m::NV_MOSAIC_GRID_TOPO_DISPLAY_V2>()
    );

    println!(
        "NV_MOSAIC_DISPLAY_SETTING_V1 size: {}",
        std::mem::size_of::<m::NV_MOSAIC_DISPLAY_SETTING_V1>()
    );
    println!(
        "NV_MOSAIC_DISPLAY_SETTING_V2 size: {}",
        std::mem::size_of::<m::NV_MOSAIC_DISPLAY_SETTING_V2>()
    );

    println!(
        "NV_MOSAIC_GRID_TOPO_VER1: 0x{:08x}",
        m::NV_MOSAIC_GRID_TOPO_VER1
    );
    println!(
        "NV_MOSAIC_GRID_TOPO_VER2: 0x{:08x}",
        m::NV_MOSAIC_GRID_TOPO_VER2
    );
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
                println!(
                    "GetSupportedTopoInfo: topoBriefsCount={} displaySettingsCount={}",
                    info.topoBriefsCount, info.displaySettingsCount
                );

                // Print first few topology briefs
                let count = std::cmp::min(info.topoBriefsCount as usize, 3);
                for i in 0..count {
                    let brief = &info.topoBriefs[i];
                    println!(
                        "  topo[{}]: type={:?} enabled={} isPossible={}",
                        i, brief.topo, brief.enabled, brief.isPossible
                    );
                }
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e)),
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
                println!(
                    "GetTopoGroup: count={} brief.topo={:?} brief.enabled={} brief.isPossible={}",
                    group.count, group.brief.topo, group.brief.enabled, group.brief.isPossible
                );
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e)),
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

        let status = m::NvAPI_Mosaic_GetCurrentTopo(
            &mut brief,
            &mut setting,
            &mut overlap_x,
            &mut overlap_y,
        );
        match status_result(status) {
            Ok(()) => {
                println!(
                    "GetCurrentTopo: topo={:?} enabled={} isPossible={} overlap=({},{}) res={}x{} bpp={} freq={} rrx1k={}",
                    brief.topo,
                    brief.enabled,
                    brief.isPossible,
                    overlap_x,
                    overlap_y,
                    setting.width,
                    setting.height,
                    setting.bpp,
                    setting.freq,
                    setting.rrx1k
                );
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e)),
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
        if status_result(status).is_err()
            || info.topoBriefsCount == 0
            || info.displaySettingsCount == 0
        {
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
        let mut min_x = 0i32;
        let mut max_x = 0i32;
        let mut min_y = 0i32;
        let mut max_y = 0i32;

        let status = m::NvAPI_Mosaic_GetOverlapLimits(
            &mut brief,
            &mut setting_v1 as *mut _ as *mut m::NV_MOSAIC_DISPLAY_SETTING,
            &mut min_x,
            &mut max_x,
            &mut min_y,
            &mut max_y,
        );
        match status_result(status) {
            Ok(()) => {
                println!(
                    "GetOverlapLimits: X=({},{}) Y=({},{})",
                    min_x, max_x, min_y, max_y
                );
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e)),
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
        if all_displays.is_empty() {
            None
        } else {
            Some(all_displays)
        }
    })() {
        Some(ids) => ids,
        None => return Err("NoDisplaysAvailable".into()),
    };

    println!("Available display IDs: {:?}", display_ids);

    // Try each display ID until one works or we run out
    for &display_id in &display_ids {
        unsafe {
            let mut viewports: [NV_RECT; sys::types::NVAPI_MAX_DISPLAYS] = [NV_RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
                sys::types::NVAPI_MAX_DISPLAYS];
            let mut bezel_corrected: u8 = 0;

            let status = m::NvAPI_Mosaic_GetDisplayViewportsByResolution(
                display_id,
                2560,
                1440, // use the actual Mosaic resolution
                viewports.as_mut_ptr(),
                &mut bezel_corrected,
            );

            match status_result(status) {
                Ok(()) => {
                    println!(
                        "GetDisplayViewportsByResolution: displayId={} bezelCorrected={}",
                        display_id, bezel_corrected
                    );

                    // Print non-zero viewports
                    for (i, rect) in viewports.iter().enumerate() {
                        if rect.left != 0 || rect.top != 0 || rect.right != 0 || rect.bottom != 0 {
                            println!(
                                "  viewport[{}]: left={} top={} right={} bottom={}",
                                i, rect.left, rect.top, rect.right, rect.bottom
                            );
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
        // Standard two-phase enumeration
        let mut count: u32 = 0;
        let status = m::NvAPI_Mosaic_EnumDisplayGrids(std::ptr::null_mut(), &mut count);
        match status_result(status) {
            Ok(()) => {
                println!("EnumDisplayGrids: count={}", count);
                if count == 0 {
                    return Ok(());
                }

                let mut grids: Vec<m::NV_MOSAIC_GRID_TOPO> =
                    vec![m::NV_MOSAIC_GRID_TOPO::zeroed(); count as usize];
                for g in &mut grids {
                    g.version = m::NV_MOSAIC_GRID_TOPO_VER;
                    // V2 grid uses V1 display settings
                    g.displaySettings.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
                }

                let mut count_actual = count;
                let status = m::NvAPI_Mosaic_EnumDisplayGrids(
                    grids.as_mut_ptr(),
                    &mut count_actual,
                );
                match status_result(status) {
                    Ok(()) => {
                        println!("EnumDisplayGrids succeeded: count={}", count_actual);
                        for (i, grid) in grids.iter().take(count_actual as usize).enumerate() {
                            println!(
                                "  grid[{}]: {}x{} displays={} flags=0x{:08x}",
                                i, grid.rows, grid.columns, grid.displayCount, grid.gridFlags
                            );
                        }
                        Ok(())
                    }
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

#[test]
fn test_mosaic_validate_display_grids() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // Enumerate current grids and validate the first one (if any)
        let mut count: u32 = 0;
        status_result(m::NvAPI_Mosaic_EnumDisplayGrids(std::ptr::null_mut(), &mut count))
            .map_err(|e| format!("{:?}", e))?;

        if count == 0 {
            println!("No grids to validate");
            return Ok(());
        }

        let mut grids: Vec<m::NV_MOSAIC_GRID_TOPO> =
            vec![m::NV_MOSAIC_GRID_TOPO::zeroed(); count as usize];
        for g in &mut grids {
            g.version = m::NV_MOSAIC_GRID_TOPO_VER;
            g.displaySettings.version = m::NVAPI_MOSAIC_DISPLAY_SETTING_VER1;
        }

        let mut count_actual = count;
        status_result(m::NvAPI_Mosaic_EnumDisplayGrids(
            grids.as_mut_ptr(),
            &mut count_actual,
        ))
        .map_err(|e| format!("{:?}", e))?;

        if count_actual == 0 {
            println!("No grids after enumeration");
            return Ok(());
        }

        let mut status_info = m::NV_MOSAIC_DISPLAY_TOPO_STATUS::zeroed();
        status_info.version = m::NV_MOSAIC_DISPLAY_TOPO_STATUS_VER;

        status_result(m::NvAPI_Mosaic_ValidateDisplayGrids(
            0,
            &mut grids[0],
            &mut status_info,
            1,
        ))
        .map_err(|e| format!("{:?}", e))?;

        println!(
            "ValidateDisplayGrids: errorFlags=0x{:08x} warningFlags=0x{:08x} displayCount={}",
            status_info.errorFlags, status_info.warningFlags, status_info.displayCount
        );
        Ok(())
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
                println!(
                    "GetSupportedMosaicTopologies (legacy): totalCount={}",
                    topos.totalCount
                );
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e)),
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
                println!(
                    "GetCurrentMosaicTopology (legacy): enabled={} rowCount={} colCount={}",
                    enabled, topo.rowCount, topo.colCount
                );
                Ok(())
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}



#[test]
fn test_driver_version_info() -> Result<(), String> {
    // Initialize NVAPI
    nvapi::initialize().map_err(|e| format!("Failed to initialize NVAPI: {:?}", e))?;

    // Get driver version and branch string
    let (driver_version, branch_string) =
        nvapi::driver_version().map_err(|e| format!("Failed to get driver version: {:?}", e))?;
    println!("Driver Version: {}", driver_version);
    println!("Branch String: {}", branch_string);

    // Get interface version string
    let interface_version = nvapi::interface_version()
        .map_err(|e| format!("Failed to get interface version: {:?}", e))?;
    println!("Interface Version: {}", interface_version);

    // Get physical GPUs for device info
    let gpus = nvapi::PhysicalGpu::enumerate()
        .map_err(|e| format!("Failed to enumerate GPUs: {:?}", e))?;
    if !gpus.is_empty() {
        let gpu = &gpus[0];

        // Get GPU short name
        let short_name = gpu
            .short_name()
            .map_err(|e| format!("Failed to get GPU short name: {:?}", e))?;
        println!("GPU Short Name: {}", short_name);

        // Get GPU full name
        let full_name = gpu
            .full_name()
            .map_err(|e| format!("Failed to get GPU full name: {:?}", e))?;
        println!("GPU Full Name: {}", full_name);

        // Get VBIOS version string
        let vbios_version = gpu
            .vbios_version_string()
            .map_err(|e| format!("Failed to get VBIOS version: {:?}", e))?;
        println!("VBIOS Version: {}", vbios_version);
    }

    println!("Driver version info retrieved successfully");
    Ok(())
}
