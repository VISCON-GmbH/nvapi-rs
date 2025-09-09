//! Example: Iterate GPUs connected to a G-SYNC device and show sync status.
// # G-SYNC GPU Iterator Example

//! This example shows how to:
//! - Enumerate G-SYNC devices
//! - Iterate connected physical GPUs using `GSyncDevice::iter_physical_gpus()`
//! - Print each GPU's name and per-GPU sync status

//! Run:

//! ```
// cargo run --example gsync-iterator
//! ```

use nvapi::GSyncDevice;

fn main() -> Result<(), nvapi::Status> {
    let devices = GSyncDevice::get_sync_devices()?;
    if devices.is_empty() {
        println!("No G-SYNC devices found");
        return Ok(());
    }

    for (i, dev) in devices.iter().enumerate() {
        println!("G-SYNC device #{}", i);

        for (gi, gpu) in dev.iter_physical_gpus()?.enumerate() {
            let name = gpu.full_name().unwrap_or_else(|_| "<name unavailable>".to_string());
            println!("  GPU #{}: {}", gi, name);

            match dev.get_sync_status(gpu) {
                Ok(stat) => println!(
                    "    synced: {}, stereo: {}, signal: {}",
                    stat.bIsSynced != 0,
                    stat.bIsStereoSynced != 0,
                    stat.bIsSyncSignalAvailable != 0
                ),
                Err(e) => println!("    sync status error: {:?}", e),
            }
        }
    }

    Ok(())
}
