// Comprehensive tests for NVAPI GSync sys-level functions
// All tests return Result<(), String> with NVAPI error codes when failures occur
#![allow(unused_must_use)]

extern crate nvapi;

use nvapi::sys::{self, status_result};
use nvapi::sys::gsync as g;
use nvapi::sys::handles::NvGSyncDeviceHandle;
use nvapi::sys::types::NVAPI_MAX_GSYNC_DEVICES;

fn init() -> bool {
    matches!(nvapi::initialize(), Ok(()))
}

#[test]
fn test_gsync_driver_version_info() -> Result<(), String> {
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
    
    println!("=== GSync Test Environment ===");
    Ok(())
}

#[test]
fn test_gsync_enum_sync_devices() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        let mut handles: [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES] = 
            [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut count: u32 = 0;
        
        let status = g::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut count);
        match status_result(status) {
            Ok(()) => {
                println!("EnumSyncDevices: Found {} GSync devices", count);
                for i in 0..count as usize {
                    println!("  Device[{}]: handle valid={}", i, !handles[i].is_null());
                }
                Ok(())
            }
            Err(e) => {
                // GSync devices may not be available, which is normal
                println!("EnumSyncDevices: {:?} (this is normal if no GSync hardware is present)", e);
                match e {
                    sys::Status::NvidiaDeviceNotFound => Ok(()),  // Normal case
                    _ => Err(format!("{:?}", e))
                }
            }
        }
    }
}

#[test]
fn test_gsync_query_capabilities() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First enumerate sync devices
        let mut handles: [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES] = 
            [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut count: u32 = 0;
        
        let status = g::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut count);
        match status_result(status) {
            Ok(()) => {
                if count == 0 {
                    println!("QueryCapabilities: No GSync devices to query");
                    return Ok(());
                }
                
                // Query capabilities for first device
                let mut caps = g::NV_GSYNC_CAPABILITIES::zeroed();
                caps.version = g::NV_GSYNC_CAPABILITIES_VER;
                
                let status = g::NvAPI_GSync_QueryCapabilities(handles[0], &mut caps);
                match status_result(status) {
                    Ok(()) => {
                        println!("QueryCapabilities: version=0x{:08x} (capabilities retrieved successfully)", caps.version);
                        Ok(())
                    }
                    Err(e) => Err(format!("QueryCapabilities failed: {:?}", e))
                }
            }
            Err(e) => {
                println!("QueryCapabilities: No GSync devices available ({:?})", e);
                match e {
                    sys::Status::NvidiaDeviceNotFound => Ok(()),
                    _ => Err(format!("{:?}", e))
                }
            }
        }
    }
}

#[test]
fn test_gsync_get_topology() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First enumerate sync devices
        let mut handles: [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES] = 
            [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut count: u32 = 0;
        
        let status = g::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut count);
        match status_result(status) {
            Ok(()) => {
                if count == 0 {
                    println!("GetTopology: No GSync devices to query");
                    return Ok(());
                }
                
                // First call to get required buffer sizes
                let mut gpu_count: u32 = 0;
                let mut display_count: u32 = 0;
                
                let status = g::NvAPI_GSync_GetTopology(
                    handles[0], 
                    &mut gpu_count, 
                    std::ptr::null_mut(),
                    &mut display_count,
                    std::ptr::null_mut()
                );
                
                if status == sys::status::NVAPI_INSUFFICIENT_BUFFER {
                    println!("GetTopology: Requires {} GPUs, {} displays", gpu_count, display_count);
                    
                    if gpu_count > 0 || display_count > 0 {
                        // Allocate proper buffers
                        let mut gpus: Vec<g::NV_GSYNC_GPU> = vec![g::NV_GSYNC_GPU::zeroed(); gpu_count as usize];
                        let mut displays: Vec<g::NV_GSYNC_DISPLAY> = vec![g::NV_GSYNC_DISPLAY::zeroed(); display_count as usize];
                        
                        // Initialize versions
                        for gpu in &mut gpus {
                            gpu.version = g::NV_GSYNC_GPU_VER;
                        }
                        for display in &mut displays {
                            display.version = g::NV_GSYNC_DISPLAY_VER;
                        }
                        
                        let status2 = g::NvAPI_GSync_GetTopology(
                            handles[0], 
                            &mut gpu_count, 
                            if gpu_count > 0 { gpus.as_mut_ptr() } else { std::ptr::null_mut() },
                            &mut display_count,
                            if display_count > 0 { displays.as_mut_ptr() } else { std::ptr::null_mut() }
                        );
                        
                        match status_result(status2) {
                            Ok(()) => {
                                println!("GetTopology: Successfully retrieved {} GPUs, {} displays", gpu_count, display_count);
                                
                                for (i, gpu) in gpus.iter().enumerate().take(gpu_count as usize) {
                                    println!("  GPU[{}]: handle valid={}", i, !gpu.hPhysicalGpu.is_null());
                                }
                                
                                for (i, display) in displays.iter().enumerate().take(display_count as usize) {
                                    println!("  Display[{}]: id={} masterable={} syncState={:?}", 
                                             i, display.displayId, display.isMasterable, display.syncState);
                                }
                                
                                return Ok(());
                            }
                            Err(e) => return Err(format!("GetTopology second call failed: {:?}", e)),
                        }
                    } else {
                        println!("GetTopology: No GPUs or displays found");
                        return Ok(());
                    }
                }
                
                match status_result(status) {
                    Ok(()) => {
                        println!("GetTopology: {} GPUs, {} displays (direct call)", gpu_count, display_count);
                        Ok(())
                    }
                    Err(e) => Err(format!("GetTopology failed: {:?}", e))
                }
            }
            Err(e) => {
                println!("GetTopology: No GSync devices available ({:?})", e);
                match e {
                    sys::Status::NvidiaDeviceNotFound => Ok(()),
                    _ => Err(format!("{:?}", e))
                }
            }
        }
    }
}

#[test]
fn test_gsync_get_control_parameters() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First enumerate sync devices
        let mut handles: [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES] = 
            [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut count: u32 = 0;
        
        let status = g::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut count);
        match status_result(status) {
            Ok(()) => {
                if count == 0 {
                    println!("GetControlParameters: No GSync devices to query");
                    return Ok(());
                }
                
                // Get control parameters for first device
                let mut params = g::NV_GSYNC_CONTROL_PARAMS::zeroed();
                params.version = g::NV_GSYNC_CONTROL_PARAMS_VER;
                
                let status = g::NvAPI_GSync_GetControlParameters(handles[0], &mut params);
                match status_result(status) {
                    Ok(()) => {
                        println!("GetControlParameters: version=0x{:08x} (parameters retrieved successfully)", params.version);
                        Ok(())
                    }
                    Err(e) => Err(format!("GetControlParameters failed: {:?}", e))
                }
            }
            Err(e) => {
                println!("GetControlParameters: No GSync devices available ({:?})", e);
                match e {
                    sys::Status::NvidiaDeviceNotFound => Ok(()),
                    _ => Err(format!("{:?}", e))
                }
            }
        }
    }
}

#[test]
fn test_gsync_get_sync_status() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First enumerate sync devices
        let mut handles: [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES] = 
            [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut count: u32 = 0;
        
        let status = g::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut count);
        match status_result(status) {
            Ok(()) => {
                if count == 0 {
                    println!("GetSyncStatus: No GSync devices to query");
                    return Ok(());
                }
                
                // Get physical GPUs to test with
                let gpus = nvapi::PhysicalGpu::enumerate().map_err(|e| format!("Failed to enumerate GPUs: {:?}", e))?;
                if gpus.is_empty() {
                    return Err("No physical GPUs available".into());
                }
                
                // Get sync status for first device and first GPU
                let mut sync_status = g::NV_GSYNC_STATUS::default();
                sync_status.version = g::NV_GSYNC_STATUS_VER;
                
                let gpu_handle = gpus[0].handle().clone();
                let status = g::NvAPI_GSync_GetSyncStatus(handles[0], gpu_handle, &mut sync_status);
                match status_result(status) {
                    Ok(()) => {
                        println!("GetSyncStatus: synced={} stereoSynced={} signalAvailable={}", 
                                 sync_status.bIsSynced, sync_status.bIsStereoSynced, sync_status.bIsSyncSignalAvailable);
                        Ok(())
                    }
                    Err(e) => Err(format!("GetSyncStatus failed: {:?}", e))
                }
            }
            Err(e) => {
                println!("GetSyncStatus: No GSync devices available ({:?})", e);
                match e {
                    sys::Status::NvidiaDeviceNotFound => Ok(()),
                    _ => Err(format!("{:?}", e))
                }
            }
        }
    }
}

#[test]
fn test_gsync_get_status_parameters() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First enumerate sync devices
        let mut handles: [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES] = 
            [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut count: u32 = 0;
        
        let status = g::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut count);
        match status_result(status) {
            Ok(()) => {
                if count == 0 {
                    println!("GetStatusParameters: No GSync devices to query");
                    return Ok(());
                }
                
                // Try V1 first (more compatible according to comment)
                let mut params_v1 = g::NV_GSYNC_STATUS_PARAMS_V1::zeroed();
                params_v1.version = g::NV_GSYNC_STATUS_PARAMS_VER_1;
                
                let status = g::NvAPI_GSync_GetStatusParameters(handles[0], &mut params_v1 as *mut _ as *mut g::NV_GSYNC_STATUS_PARAMS);
                match status_result(status) {
                    Ok(()) => {
                        println!("GetStatusParameters (V1): version=0x{:08x} (parameters retrieved successfully)", params_v1.version);
                        Ok(())
                    }
                    Err(e) => {
                        // Try V2 if V1 fails
                        println!("V1 failed ({:?}), trying V2...", e);
                        
                        let mut params_v2 = g::NV_GSYNC_STATUS_PARAMS_V2::zeroed();
                        params_v2.v1.version = g::NV_GSYNC_STATUS_PARAMS_VER_2;
                        
                        let status = g::NvAPI_GSync_GetStatusParameters(handles[0], &mut params_v2 as *mut _ as *mut g::NV_GSYNC_STATUS_PARAMS);
                        match status_result(status) {
                            Ok(()) => {
                                println!("GetStatusParameters (V2): version=0x{:08x} (parameters retrieved successfully)", params_v2.v1.version);
                                Ok(())
                            }
                            Err(e2) => Err(format!("GetStatusParameters failed (V1: {:?}, V2: {:?})", e, e2))
                        }
                    }
                }
            }
            Err(e) => {
                println!("GetStatusParameters: No GSync devices available ({:?})", e);
                match e {
                    sys::Status::NvidiaDeviceNotFound => Ok(()),
                    _ => Err(format!("{:?}", e))
                }
            }
        }
    }
}

#[test]
fn test_gsync_adjust_sync_delay() -> Result<(), String> {
    if !init() {
        return Err("Failed to initialize NVAPI".into());
    }

    unsafe {
        // First enumerate sync devices
        let mut handles: [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES] = 
            [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut count: u32 = 0;
        
        let status = g::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut count);
        match status_result(status) {
            Ok(()) => {
                if count == 0 {
                    println!("AdjustSyncDelay: No GSync devices to query");
                    return Ok(());
                }
                
                // Test delay adjustment for sync skew
                let mut delay = g::NV_GSYNC_DELAY::zeroed();
                delay.version = g::NV_GSYNC_DELAY_VER;
                let mut sync_steps: u32 = 0;
                
                let status = g::NvAPI_GSync_AdjustSyncDelay(
                    handles[0], 
                    g::NVAPI_GSYNC_DELAY_TYPE_SYNC_SKEW,
                    &mut delay,
                    &mut sync_steps
                );
                
                match status_result(status) {
                    Ok(()) => {
                        println!("AdjustSyncDelay (SyncSkew): syncSteps={} (delay adjusted successfully)", sync_steps);
                        Ok(())
                    }
                    Err(e) => {
                        // Try startup delay type instead
                        println!("SyncSkew failed ({:?}), trying Startup...", e);
                        
                        let mut delay2 = g::NV_GSYNC_DELAY::zeroed();
                        delay2.version = g::NV_GSYNC_DELAY_VER;
                        let mut sync_steps2: u32 = 0;
                        
                        let status = g::NvAPI_GSync_AdjustSyncDelay(
                            handles[0], 
                            g::NVAPI_GSYNC_DELAY_TYPE_STARTUP,
                            &mut delay2,
                            &mut sync_steps2
                        );
                        
                        match status_result(status) {
                            Ok(()) => {
                                println!("AdjustSyncDelay (Startup): syncSteps={} (delay adjusted successfully)", sync_steps2);
                                Ok(())
                            }
                            Err(e2) => Err(format!("AdjustSyncDelay failed (SyncSkew: {:?}, Startup: {:?})", e, e2))
                        }
                    }
                }
            }
            Err(e) => {
                println!("AdjustSyncDelay: No GSync devices available ({:?})", e);
                match e {
                    sys::Status::NvidiaDeviceNotFound => Ok(()),
                    _ => Err(format!("{:?}", e))
                }
            }
        }
    }
}

#[test]
fn test_gsync_struct_sizes_and_versions() -> Result<(), String> {
    // Test struct size calculations and version constants
    println!("=== GSync Struct Size Analysis ===");
    
    // Capabilities structs
    println!("NV_GSYNC_CAPABILITIES_V1 size: {}", std::mem::size_of::<g::NV_GSYNC_CAPABILITIES_V1>());
    println!("NV_GSYNC_CAPABILITIES_V2 size: {}", std::mem::size_of::<g::NV_GSYNC_CAPABILITIES_V2>());
    println!("NV_GSYNC_CAPABILITIES_VER_1: 0x{:08x}", g::NV_GSYNC_CAPABILITIES_VER_1);
    println!("NV_GSYNC_CAPABILITIES_VER_2: 0x{:08x}", g::NV_GSYNC_CAPABILITIES_VER_2);
    
    // Display and GPU structs
    println!("NV_GSYNC_DISPLAY size: {}", std::mem::size_of::<g::NV_GSYNC_DISPLAY>());
    println!("NV_GSYNC_GPU size: {}", std::mem::size_of::<g::NV_GSYNC_GPU>());
    println!("NV_GSYNC_DISPLAY_VER: 0x{:08x}", g::NV_GSYNC_DISPLAY_VER);
    println!("NV_GSYNC_GPU_VER: 0x{:08x}", g::NV_GSYNC_GPU_VER);
    
    // Control params
    println!("NV_GSYNC_CONTROL_PARAMS size: {}", std::mem::size_of::<g::NV_GSYNC_CONTROL_PARAMS>());
    println!("NV_GSYNC_CONTROL_PARAMS_VER: 0x{:08x}", g::NV_GSYNC_CONTROL_PARAMS_VER);
    
    // Status structs
    println!("NV_GSYNC_STATUS size: {}", std::mem::size_of::<g::NV_GSYNC_STATUS>());
    println!("NV_GSYNC_STATUS_PARAMS_V1 size: {}", std::mem::size_of::<g::NV_GSYNC_STATUS_PARAMS_V1>());
    println!("NV_GSYNC_STATUS_PARAMS_V2 size: {}", std::mem::size_of::<g::NV_GSYNC_STATUS_PARAMS_V2>());
    println!("NV_GSYNC_STATUS_VER: 0x{:08x}", g::NV_GSYNC_STATUS_VER);
    println!("NV_GSYNC_STATUS_PARAMS_VER_1: 0x{:08x}", g::NV_GSYNC_STATUS_PARAMS_VER_1);
    println!("NV_GSYNC_STATUS_PARAMS_VER_2: 0x{:08x}", g::NV_GSYNC_STATUS_PARAMS_VER_2);
    
    // Delay struct
    println!("NV_GSYNC_DELAY size: {}", std::mem::size_of::<g::NV_GSYNC_DELAY>());
    println!("NV_GSYNC_DELAY_VER: 0x{:08x}", g::NV_GSYNC_DELAY_VER);
    
    // Constants
    println!("NVAPI_MAX_GSYNC_DEVICES: {}", NVAPI_MAX_GSYNC_DEVICES);
    println!("NVAPI_MAX_RJ45_PER_GSYNC: {}", g::NVAPI_MAX_RJ45_PER_GSYNC);
    
    Ok(())
}

#[test]
fn test_gsync_enum_creation() -> Result<(), String> {
    // Test enum value creation and display
    println!("=== GSync Enum Values ===");
    
    // Display sync states
    println!("DisplaySyncState::Unsynced: {}", g::NVAPI_GSYNC_DISPLAY_SYNC_STATE_UNSYNCED);
    println!("DisplaySyncState::Slave: {}", g::NVAPI_GSYNC_DISPLAY_SYNC_STATE_SLAVE);
    println!("DisplaySyncState::Master: {}", g::NVAPI_GSYNC_DISPLAY_SYNC_STATE_MASTER);
    
    // Topology connectors
    println!("TopologyConnector::None: {}", g::NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_NONE);
    println!("TopologyConnector::Primary: {}", g::NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_PRIMARY);
    println!("TopologyConnector::Secondary: {}", g::NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_SECONDARY);
    
    // Polarity
    println!("Polarity::RisingEdge: {}", g::NVAPI_GSYNC_POLARITY_RISING_EDGE);
    println!("Polarity::FallingEdge: {}", g::NVAPI_GSYNC_POLARITY_FALLING_EDGE);
    println!("Polarity::BothEdges: {}", g::NVAPI_GSYNC_POLARITY_BOTH_EDGES);
    
    // Video modes
    println!("VideoMode::None: {}", g::NVAPI_GSYNC_VIDEO_MODE_NONE);
    println!("VideoMode::TTL: {}", g::NVAPI_GSYNC_VIDEO_MODE_TTL);
    println!("VideoMode::NtscPalCam: {}", g::NVAPI_GSYNC_VIDEO_MODE_NTSCPALSECAM);
    
    // Sync sources
    println!("SyncSource::VSync: {}", g::NVAPI_GSYNC_SYNC_SOURCE_VSYNC);
    println!("SyncSource::HouseSync: {}", g::NVAPI_GSYNC_SYNC_SOURCE_HOUSESYNC);
    
    // Delay types
    println!("DelayType::Unknown: {}", g::NVAPI_GSYNC_DELAY_TYPE_UNKNOWN);
    println!("DelayType::SyncSkew: {}", g::NVAPI_GSYNC_DELAY_TYPE_SYNC_SKEW);
    println!("DelayType::Startup: {}", g::NVAPI_GSYNC_DELAY_TYPE_STARTUP);
    
    // RJ45 IO
    println!("RJ45_IO::Output: {}", g::NVAPI_GSYNC_RJ45_OUTPUT);
    println!("RJ45_IO::Input: {}", g::NVAPI_GSYNC_RJ45_INPUT);
    println!("RJ45_IO::Unused: {}", g::NVAPI_GSYNC_RJ45_UNUSED);
    
    Ok(())
}
