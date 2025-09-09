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
        println!("  capabilities: {}", dev.query_capabilities().is_ok());
        println!("  status params: {}", dev.get_status_parameters().is_ok());
        println!("  control params: {}", dev.get_control_parameters().is_ok());

        // Topology counts
        if let Ok((gpus, displays)) = dev.get_topology() {
            println!("  topology: {} gpus, {} displays", gpus.len(), displays.len());
        }
    }

    Ok(())
}
