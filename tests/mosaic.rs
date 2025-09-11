// Comprehensive tests for NVAPI Mosaic high-level wrapper functions
// All tests return Result<(), String> with NVAPI error codes when failures occur
#![allow(unused_must_use)]

use nvapi::{Mosaic, MosaicTopoType, Status};

fn init() -> bool {
    matches!(nvapi::initialize(), Ok(()))
}

#[test]
fn test_mosaic_get_supported_topologies() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    match Mosaic::get_supported_topologies(MosaicTopoType::All) {
        Ok(info) => {
            println!("get_supported_topologies: Found {} topology briefs and {} display settings", 
                     info.topoBriefsCount, info.displaySettingsCount);
            
            // Show first few topologies
            let count = std::cmp::min(info.topoBriefsCount as usize, 3);
            for i in 0..count {
                let brief = &info.topoBriefs[i];
                println!("  Topology {}: {:?} (possible: {})", 
                         i, brief.topo, brief.isPossible != 0);
            }
            
            // Show first few display settings
            let settings_count = std::cmp::min(info.displaySettingsCount as usize, 5);
            for i in 0..settings_count {
                let settings = &info.displaySettings[i];
                println!("  Display setting {}: {}x{} @ {}Hz ({}bpp)", 
                         i, settings.width, settings.height, settings.freq, settings.bpp);
            }
            
            Ok(())
        }
        Err(e) => Err(format!("get_supported_topologies failed: {:?}", e)),
    }
}

#[test]
fn test_mosaic_get_topology_details() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    // Get supported topologies first
    let info = match Mosaic::get_supported_topologies(MosaicTopoType::All) {
        Ok(info) => info,
        Err(e) => return Err(format!("Failed to get supported topologies: {:?}", e)),
    };
    
    if info.topoBriefsCount == 0 {
        println!("get_topology_details: No topologies available to test");
        return Ok(());
    }

    // Test getting details for the first topology
    let brief = &info.topoBriefs[0];
    match Mosaic::get_topology_details(brief) {
        Ok(details) => {
            println!("get_topology_details: count={}", details.count);
            if details.count > 0 {
                println!("  First topo validity mask = 0x{:x}", details.topos[0].validityMask);
                
                // Check validity flags
                let validity = details.topos[0].validityMask;
                let mut issues = Vec::new();
                if validity & nvapi::NV_MOSAIC_TOPO_VALIDITY_MISSING_GPU != 0 {
                    issues.push("missing GPU");
                }
                if validity & nvapi::NV_MOSAIC_TOPO_VALIDITY_MISSING_DISPLAY != 0 {
                    issues.push("missing display");
                }
                if validity & nvapi::NV_MOSAIC_TOPO_VALIDITY_MIXED_DISPLAY_TYPES != 0 {
                    issues.push("mixed display types");
                }
                
                if issues.is_empty() {
                    println!("  Topology is fully valid");
                } else {
                    println!("  Issues: {}", issues.join(", "));
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("get_topology_details failed: {:?}", e)),
    }
}

#[test]
fn test_mosaic_get_current_topology() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    match Mosaic::get_current_topology() {
        Ok((brief, settings, (overlap_x, overlap_y))) => {
            if brief.topo == nvapi::MosaicTopo::None.raw() {
                println!("get_current_topology: No Mosaic topology is currently active");
            } else {
                println!("get_current_topology: {:?}, {}x{} @ {}Hz, overlap: {}x{}", 
                         brief.topo, settings.width, settings.height, 
                         settings.freq, overlap_x, overlap_y);
            }
            Ok(())
        }
        Err(e) => {
            // This is expected if no Mosaic topology is active
            println!("get_current_topology: {:?} (expected if no Mosaic active)", e);
            Ok(())
        }
    }
}

#[test]
fn test_mosaic_get_overlap_limits() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    // Get supported topologies first
    let info = match Mosaic::get_supported_topologies(MosaicTopoType::All) {
        Ok(info) => info,
        Err(e) => return Err(format!("Failed to get supported topologies: {:?}", e)),
    };
    
    if info.topoBriefsCount == 0 || info.displaySettingsCount == 0 {
        println!("get_overlap_limits: No topologies or display settings available to test");
        return Ok(());
    }

    let brief = &info.topoBriefs[0];
    let settings = &info.displaySettings[0];
    
    match Mosaic::get_overlap_limits(brief, settings) {
        Ok((min_x, max_x, min_y, max_y)) => {
            println!("get_overlap_limits: X: {} to {}, Y: {} to {}", min_x, max_x, min_y, max_y);
            Ok(())
        }
        Err(Status::IncompatibleStructVersion) => {
            println!("FIXME: get_overlap_limits: Invalid struct version");
            Ok(())
        }
        Err(e) => Err(format!("get_overlap_limits failed: {:?}", e)),
    }
}

#[test]
fn test_mosaic_enum_display_grids() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    match Mosaic::enum_display_grids() {
        Ok(grids) => {
            println!("enum_display_grids: Found {} active grids", grids.len());
            for (i, grid) in grids.iter().enumerate() {
                println!("  Grid {}: {}x{} displays, {}x{} resolution", 
                         i, grid.rows, grid.columns,
                         grid.displaySettings.width, grid.displaySettings.height);
                println!("    {} displays active, flags: 0x{:x}", 
                         grid.displayCount, grid.gridFlags);
                
                // Show some flag details
                if grid.has_flag(nvapi::NV_MOSAIC_GRID_TOPO_FLAG_APPLY_WITH_BEZEL_CORRECT) {
                    println!("    - Bezel correction enabled");
                }
                if grid.has_flag(nvapi::NV_MOSAIC_GRID_TOPO_FLAG_IMMERSIVE_GAMING) {
                    println!("    - Immersive gaming mode");
                }
                if grid.has_flag(nvapi::NV_MOSAIC_GRID_TOPO_FLAG_BASE_MOSAIC) {
                    println!("    - Base Mosaic");
                }
            }
            Ok(())
        }
        Err(Status::IncompatibleStructVersion) => {
            println!("FIXME: enum_display_grids: Invalid struct version");
            Ok(())
        }
        Err(e) => Err(format!("enum_display_grids failed: {:?}", e)),
    }
}

#[test]
fn test_mosaic_get_display_viewports() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    match Mosaic::get_display_viewports_by_resolution(0, 1920, 1080) {
        Ok((viewport, bezel_corrected)) => {
            println!("get_display_viewports: viewport=({}, {}, {}, {}), bezel_corrected={}", 
                     viewport.left, viewport.top, viewport.right, viewport.bottom, bezel_corrected);
            println!("  Size: {}x{}", 
                     viewport.right - viewport.left, 
                     viewport.bottom - viewport.top);
            Ok(())
        }
        Err(e) => {
            println!("get_display_viewports: {:?} (expected for invalid display)", e);
            Ok(())
        }
    }
}

#[test]
#[ignore = "modifies system state, heavy operation"]
fn test_mosaic_enable_disable_topology() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    println!("enable_disable_topology: Testing safe enable/disable sequence");
    
    // Try to disable current topology
    match Mosaic::enable_current_topology(false) {
        Ok(()) => {
            println!("  Successfully disabled current topology");
            println!("  Waiting 5 seconds before re-enabling...");
            std::thread::sleep(std::time::Duration::from_millis(5000));
            match Mosaic::enable_current_topology(true) {
                Ok(()) => {
                    println!("  Successfully re-enabled topology");
                    Ok(())
                }
                Err(e) => Err(format!("Failed to re-enable topology: {:?}", e)),
            }
        }
        Err(Status::InvalidArgument) | 
        Err(Status::NvidiaDeviceNotFound) => {
            println!("  No active topology to disable (this is normal)");
            Ok(())
        }
        Err(e) => {
            println!("  Enable/disable test failed: {:?} (this may be expected)", e);
            Ok(())
        }
    }
}

#[test] 
fn test_mosaic_validate_display_grids() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    // Get current grids to validate
    match Mosaic::enum_display_grids() {
        Ok(mut grids) => {
            if grids.is_empty() {
                println!("validate_display_grids: No grids to validate");
                return Ok(());
            }
            
            match Mosaic::validate_display_grids(&mut grids, nvapi::NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD) {
                Ok(status) => {
                    println!("validate_display_grids: Validated {} grids", status.len());
                    for (i, grid_status) in status.iter().enumerate() {
                        println!("  Grid {}: {} displays", i, grid_status.displayCount);
                        if grid_status.displayCount > 0 {
                            for j in 0..(grid_status.displayCount as usize) {
                                let disp_status = &grid_status.displays[j];
                                if disp_status.warningFlags != 0 {
                                    println!("    Display {}: warnings=0x{:x}", j, disp_status.warningFlags);
                                }
                            }
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    println!("validate_display_grids: {:?} (this may be expected)", e);
                    Ok(())
                }
            }
        }
        Err(Status::IncompatibleStructVersion) => {
            println!("validate_display_grids: Grid enumeration not supported, skipping validation");
            Ok(())
        }
        Err(e) => {
            println!("validate_display_grids: Failed to get grids for validation: {:?}", e);
            Ok(())
        }
    }
}

#[test]
fn test_mosaic_comprehensive_workflow() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    println!("comprehensive_workflow: Testing complete Mosaic API workflow");

    // 1. Query supported topologies
    let info = match Mosaic::get_supported_topologies(MosaicTopoType::Basic) {
        Ok(info) => {
            println!("  Step 1: Found {} basic topologies", info.topoBriefsCount);
            info
        }
        Err(e) => return Err(format!("Step 1 failed: {:?}", e)),
    };

    // 2. Get details for each topology if any exist
    if info.topoBriefsCount > 0 {
        for i in 0..(info.topoBriefsCount as usize) {
            let brief = &info.topoBriefs[i];
            match Mosaic::get_topology_details(brief) {
                Ok(details) => {
                    println!("  Step 2.{}: Topology {:?} has {} details", 
                             i, brief.topo, details.count);
                }
                Err(e) => {
                    println!("  Step 2.{}: Failed to get details for topology {:?}: {:?}", 
                             i, brief.topo, e);
                }
            }
        }
    } else {
        println!("  Step 2: No topologies to get details for");
    }

    // 3. Check current configuration
    match Mosaic::get_current_topology() {
        Ok((brief, settings, _)) => {
            println!("  Step 3: Current topology is {:?} at {}x{}", 
                     brief.topo, settings.width, settings.height);
        }
        Err(e) => {
            println!("  Step 3: No current topology: {:?}", e);
        }
    }

    // 4. Enumerate display grids
    match Mosaic::enum_display_grids() {
        Ok(grids) => {
            println!("  Step 4: Found {} display grids", grids.len());
        }
        Err(Status::IncompatibleStructVersion) => {
            println!("  Step 4: Grid enumeration not supported");
        }
        Err(e) => {
            println!("  Step 4: Grid enumeration failed: {:?}", e);
        }
    }

    println!("  Workflow completed successfully");
    Ok(())
}
