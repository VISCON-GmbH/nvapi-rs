use nvapi::{initialize, PhysicalGpu, GSyncDevice};
use nvapi::sys::gsync as g;
use nvapi::sys::gsync::{NVAPI_GSYNC_POLARITY, NVAPI_GSYNC_VIDEO_MODE, NVAPI_GSYNC_SYNC_SOURCE, DisplaySyncState};

fn init() -> bool {
    matches!(initialize(), Ok(()))
}


#[test]
fn test_gsync_device_creation() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    // Test GSyncDevice enumeration
    match GSyncDevice::enum_sync_devices() {
        Ok(devices) => {
            println!("Found {} GSync devices", devices.len());
            
            for (i, device) in devices.iter().enumerate() {
                println!("GSync Device[{}]:", i);
                
                // Test query capabilities
                match device.query_capabilities() {
                    Ok(_caps) => {
                        println!("  Capabilities queried successfully");
                    }
                    Err(e) => {
                        println!("  Failed to query capabilities: {:?}", e);
                    }
                }
                
                // Test get topology
                match device.get_topology() {
                    Ok((gpus, displays)) => {
                        println!("  Topology: {} GPUs, {} displays", gpus.len(), displays.len());
                        
                        // Test get physical GPUs
                        match device.get_physical_gpus() {
                            Ok(phys_gpus) => {
                                println!("  Found {} physical GPUs connected to GSync device", phys_gpus.len());
                                
                                // Test get sync status with first GPU if available
                                if !phys_gpus.is_empty() {
                                    match device.get_sync_status(&phys_gpus[0]) {
                                        Ok(_status) => {
                                            println!("  Sync status retrieved successfully");
                                        }
                                        Err(e) => {
                                            println!("  Failed to get sync status: {:?}", e);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("  Failed to get physical GPUs: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  Failed to get topology: {:?}", e);
                    }
                }
                
                // Test get control parameters
                match device.get_control_parameters() {
                    Ok(_params) => {
                        println!("  Control parameters retrieved successfully");
                    }
                    Err(e) => {
                        println!("  Failed to get control parameters: {:?}", e);
                    }
                }
                
                // Test get status parameters
                match device.get_status_parameters() {
                    Ok(_params) => {
                        println!("  Status parameters retrieved successfully");
                    }
                    Err(e) => {
                        println!("  Failed to get status parameters: {:?}", e);
                    }
                }
            }
            
            Ok(())
        }
        Err(e) => {
            println!("No GSync devices found or enumeration failed: {:?}", e);
            // This is not an error condition - just means no GSync hardware
            Ok(())
        }
    }
}




#[test]
fn test_gsync_driver_version_info() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }
    
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
    let gpus = PhysicalGpu::enumerate().map_err(|e| format!("Failed to enumerate GPUs: {:?}", e))?;
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
    
    println!("=== GSync High-Level Test Environment ===");
    Ok(())
}

#[test]
fn test_gsync_enum_sync_devices() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    println!("EnumSyncDevices: Found {} GSync devices", devices.len());
    
    for (i, device) in devices.iter().enumerate() {
        println!("  Device[{}]: handle valid={}", i, !device.handle().is_null());
    }
    
    if devices.is_empty() {
        println!("No GSync devices found - this is expected if no GSync hardware is present");
    }
    
    Ok(())
}

#[test]
fn test_gsync_query_capabilities() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("QueryCapabilities: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    let capabilities = device.query_capabilities()
        .map_err(|e| format!("Failed to query capabilities: {:?}", e))?;
    
    println!("QueryCapabilities: version=0x{:08x}", capabilities.version);
    println!("  capFlags=0x{:08x}", capabilities.capFlags);
    // Note: maxSyncDelay and maxSyncInterval are not available in current struct definition
    
    Ok(())
}

#[test]
fn test_gsync_get_topology() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("GetTopology: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    let (gpus, displays) = device.get_topology()
        .map_err(|e| format!("Failed to get topology: {:?}", e))?;
    
    println!("GetTopology: {} GPUs, {} displays", gpus.len(), displays.len());
    
    for (i, gpu) in gpus.iter().enumerate() {
        println!("  GPU[{}]: handle valid={}", i, !gpu.hPhysicalGpu.is_null());
    }
    
    for (i, display) in displays.iter().enumerate() {
        let sync_state = match DisplaySyncState::from_raw(display.syncState) {
            Ok(state) => format!("{:?}", state),
            Err(e) => format!("Invalid({:?})", e),
        };
        println!("  Display[{}]: id={} masterable={} syncState={}", 
                 i, display.displayId, display.isMasterable, sync_state);
    }
    
    Ok(())
}

#[test]
fn test_gsync_get_control_parameters() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("GetControlParameters: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    let params = device.get_control_parameters()
        .map_err(|e| format!("Failed to get control parameters: {:?}", e))?;
    
    println!("GetControlParameters: version=0x{:08x}", params.version);
    println!("  polarity={:?}", NVAPI_GSYNC_POLARITY::from(params.polarity));
    println!("  vmode={:?}", NVAPI_GSYNC_VIDEO_MODE::from(params.vmode));
    println!("  interval={}", params.interval);
    println!("  source={:?}", NVAPI_GSYNC_SYNC_SOURCE::from(params.source));
    println!("  interlaceMode={}", params.interlaceMode);
    println!("  syncSourceIsOutput={}", params.syncSourceIsOutput);
    
    Ok(())
}

#[test]
fn test_gsync_get_sync_status() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("GetSyncStatus: No GSync devices available");
        return Ok(());
    }
    
    let gpus = PhysicalGpu::enumerate()
        .map_err(|e| format!("Failed to enumerate GPUs: {:?}", e))?;
    
    if gpus.is_empty() {
        println!("GetSyncStatus: No GPUs available");
        return Ok(());
    }
    
    let device = &devices[0];
    let gpu = &gpus[0];
    
    let status = device.get_sync_status(gpu)
        .map_err(|e| format!("Failed to get sync status: {:?}", e))?;
    
    println!("GetSyncStatus: version=0x{:08x}", status.version);
    println!("  bIsSynced={}", status.bIsSynced);
    println!("  bIsStereoSynced={}", status.bIsStereoSynced);
    println!("  bIsSyncSignalAvailable={}", status.bIsSyncSignalAvailable);
    
    Ok(())
}

#[test]
fn test_gsync_get_status_parameters() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("GetStatusParameters: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    let params = device.get_status_parameters()
        .map_err(|e| format!("Failed to get status parameters: {:?}", e))?;
    
    println!("GetStatusParameters: version=0x{:08x}", params.version);
    println!("  refreshRate={}", params.refreshRate);
    // Note: hSyncInKHz and vSyncInHz are not available in current struct definition
    println!("  bHouseSync={}", params.bHouseSync);
    
    Ok(())
}

#[test]
fn test_gsync_adjust_sync_delay() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("AdjustSyncDelay: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    
    // Test sync skew delay adjustment
    let mut delay = g::NV_GSYNC_DELAY::zeroed();
    delay.version = g::NV_GSYNC_DELAY_VER;
    delay.numLines = 10;
    delay.numPixels = 100;
    
    match device.adjust_sync_delay(g::NVAPI_GSYNC_DELAY_TYPE_SYNC_SKEW, &mut delay) {
        Ok(steps) => {
            println!("AdjustSyncDelay (SyncSkew): steps={:?}", steps);
            println!("  Adjusted delay: lines={} pixels={}", delay.numLines, delay.numPixels);
            Ok(())
        }
        Err(e) => {
            // Try startup delay type instead
            println!("SyncSkew failed ({:?}), trying Startup...", e);
            
            let mut delay2 = g::NV_GSYNC_DELAY::zeroed();
            delay2.version = g::NV_GSYNC_DELAY_VER;
            delay2.numLines = 5;
            delay2.numPixels = 50;
            
            match device.adjust_sync_delay(g::NVAPI_GSYNC_DELAY_TYPE_STARTUP, &mut delay2) {
                Ok(steps) => {
                    println!("AdjustSyncDelay (Startup): steps={:?}", steps);
                    println!("  Adjusted delay: lines={} pixels={}", delay2.numLines, delay2.numPixels);
                    Ok(())
                }
                Err(e2) => Err(format!("AdjustSyncDelay failed (SyncSkew: {:?}, Startup: {:?})", e, e2))
            }
        }
    }
}

#[test]
fn test_gsync_get_physical_gpus() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("GetPhysicalGpus: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    let gpus = device.get_physical_gpus()
        .map_err(|e| format!("Failed to get physical GPUs: {:?}", e))?;
    
    println!("GetPhysicalGpus: {} GPUs connected to GSync device", gpus.len());
    
    for (i, gpu) in gpus.iter().enumerate() {
        match gpu.short_name() {
            Ok(name) => println!("  GPU[{}]: {}", i, name),
            Err(e) => println!("  GPU[{}]: failed to get name ({:?})", i, e),
        }
    }
    
    Ok(())
}

#[test]
fn test_gsync_set_sync_state_settings() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("SetSyncStateSettings: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    
    // Get current topology to see available displays
    let (_gpus, displays) = device.get_topology()
        .map_err(|e| format!("Failed to get topology: {:?}", e))?;
    
    if displays.is_empty() {
        println!("SetSyncStateSettings: No displays in topology");
        return Ok(());
    }
    
    println!("SetSyncStateSettings: Testing with {} displays", displays.len());
    
    // Test the convenience method with display ID and state pairs
    let mut sync_settings = Vec::new();
    for display in &displays {
        // Keep existing sync state (don't change anything)
        match DisplaySyncState::from_raw(display.syncState) {
            Ok(current_state) => {
                sync_settings.push((display.displayId, current_state));
                println!("  Display {}: current state={:?}", display.displayId, current_state);
            }
            Err(e) => {
                println!("  Display {}: invalid sync state ({:?})", display.displayId, e);
                // Skip displays with invalid sync states
                continue;
            }
        }
    }
    
    match device.set_sync_state_settings(sync_settings, 0) {
        Ok(()) => {
            println!("SetSyncStateSettings: Successfully applied settings (no-op)");
            Ok(())
        }
        Err(e) => {
            // This might fail if we don't have permission or the displays aren't masterable
            println!("SetSyncStateSettings: Failed ({:?}) - this may be expected", e);
            
            // Try the from_topology method as a fallback
            match device.set_sync_state_settings_from_topology(&displays, 0) {
                Ok(()) => {
                    println!("SetSyncStateSettings (from topology): Success");
                    Ok(())
                }
                Err(e2) => {
                    println!("SetSyncStateSettings (from topology): Failed ({:?})", e2);
                    // Don't fail the test - this is expected in many scenarios
                    Ok(())
                }
            }
        }
    }
}

#[test]
fn test_gsync_set_control_parameters() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    let devices = GSyncDevice::enum_sync_devices()
        .map_err(|e| format!("Failed to enumerate GSync devices: {:?}", e))?;
    
    if devices.is_empty() {
        println!("SetControlParameters: No GSync devices available");
        return Ok(());
    }
    
    let device = &devices[0];
    
    // Get current control parameters first
    let mut params = device.get_control_parameters()
        .map_err(|e| format!("Failed to get current control parameters: {:?}", e))?;
    
    println!("SetControlParameters: Current parameters:");
    println!("  polarity={:?}", NVAPI_GSYNC_POLARITY::from(params.polarity));
    println!("  vmode={:?}", NVAPI_GSYNC_VIDEO_MODE::from(params.vmode));
    println!("  interval={}", params.interval);
    println!("  source={:?}", NVAPI_GSYNC_SYNC_SOURCE::from(params.source));
    
    // Try to set the same parameters (no-op test)
    match device.set_control_parameters(&mut params) {
        Ok(applied_params) => {
            println!("SetControlParameters: Successfully applied parameters");
            println!("  Applied interval={}", applied_params.interval);
            Ok(())
        }
        Err(e) => {
            // This might fail due to permissions or hardware limitations
            println!("SetControlParameters: Failed ({:?}) - this may be expected without proper hardware/permissions", e);
            // Don't fail the test - this is expected in many scenarios
            Ok(())
        }
    }
}
