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
    let devices = GSyncDevice::enum_sync_devices()?;
    if devices.is_empty() {
        println!("No G-SYNC devices found");
        return Ok(());
    }

    // Iterate all G-SYNC devices found
    for (i, dev) in devices.iter().enumerate() {
        println!("G-SYNC device #{}", i);

        //Query capabilities and status/control params
        println!("  capabilities: {:#?}", dev.query_capabilities());
        println!("  status params: {:#?}", dev.get_status_parameters());
        println!("  control params: {:#?}", dev.get_control_parameters());

        //Get topology (connected GPUs and active displays)
        println!("  topology:");
        let (gpus, displays) = dev.get_topology()?;
        println!("  connected GPUs: {}", gpus.len());
        println!("  active displays: {}", displays.len());

        //We can also get the physical GPUs connected to this G-SYNC device
        println!("  connected physical GPUs:");
        let mut index = 0;
        for gpu in dev.get_physical_gpus()? {
            println!("    GPU #{}", index);
            println!("      {}", gpu.full_name().unwrap_or_else(|_| "<unknown>".to_string()));
            println!("      Cuda cores: {}", gpu.core_count()?);
            println!("      VRAM: {:#?}", gpu.memory_info()?.dedicated);
            index += 1;
        }
    }

    Ok(())
}
