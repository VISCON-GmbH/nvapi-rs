// Tests in this file call apis an simply expect them to return without causing
// segfaults, etc.

// Not actually testing results. Just calling the api.
#![allow(unused_must_use)]

use core::panic;

extern crate nvapi;

#[test]
fn physicalgpu_display_ids_connected() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for gpu in gpus {
                // Bug: if there are zero connected displays this may crash.
                gpu.display_ids_connected(nvapi::ConnectedIdsFlags::empty());
            }
        }
    }
}

#[test]
fn physicalgpu_display_ids_all() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for gpu in gpus {
                // Bug: if there are zero connected displays this may crash.
                gpu.display_ids_all();
            }
        }
    }
}

#[test]
fn gsync_topology() {
    let Ok(_) = nvapi::initialize() else {
        panic!("nvapi::initialize failed");
    };
    
    let Ok(gsync_device) = nvapi::GSyncDevice::enum_sync_devices() else {
        panic!("GSyncDevice::enum_sync_devices failed");
    }; 

    for gsync in gsync_device {
        let Ok(topology) = gsync.get_topology() else {
            println!("get_topology error");
            continue;
        };
        println!(
            "Connected GPUs: {} ### Active Displays: {}",
            topology.0.len(),
            topology.1.len()
        );
        println!("topology: {:#?}", topology);
    }
}
