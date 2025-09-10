//! G-SYNC Overview Example
//!
//! This example demonstrates basic G-SYNC queries:
//! - Enumerate devices
//! - Query capabilities/status/control parameter availability
//! - Fetch topology counts (GPUs/displays)
//!
//! Run:
//!   cargo run --example gsync-overview

use nvapi::GSyncDevice;

fn main() -> Result<(), nvapi::Status> {
    let devices = GSyncDevice::get_sync_devices()?;
    if devices.is_empty() {
        println!("No G-SYNC devices found");
        return Ok(());
    }

    for (i, dev) in devices.iter().enumerate() {
        println!("G-SYNC device #{}", i);

        // Capabilities and status/control params
        println!("  capabilities: {:#?}", dev.query_capabilities());
        println!("  status params: {:#?}", dev.get_status_parameters());
        println!("  control params: {:#?}", dev.get_control_parameters());
        println!("  topology: {:#?}", dev.get_topology());

        // Topology counts
        if let Ok((gpus, displays)) = dev.get_topology() {
            println!("  topology: {} gpus, {} displays", gpus.len(), displays.len());

            for gpu in dev.iter_physical_gpus()?{
                dev.get_sync_status(&gpu).map(|status| {
                    println!("    gpu: {:#?}", gpu);
                    println!("    status: {:#?}", status);
                }).unwrap_or_else(|e| {
                    println!("    gpu: {:#?}", gpu);
                    println!("    status: error: {}", e);
                });
            }
        }
    }

    Ok(())
}
