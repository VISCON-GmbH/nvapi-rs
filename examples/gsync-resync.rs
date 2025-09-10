//! Safe live check for G-SYNC APIs.
//! - Enumerates devices
//! - Reads capabilities/status/control
//! - Calls SetControlParameters with the same values (no-op) to validate the call path
//! - Optionally applies current sync state back (no-op) if --apply-sync is passed

use nvapi::GSyncDevice;

fn main() -> Result<(), nvapi::Status> {
    let devices = GSyncDevice::enum_sync_devices()?;
    if devices.is_empty() {
        println!("No G-SYNC devices found");
        return Ok(());
    }

    let apply_sync = std::env::args().any(|a| a == "--apply-sync");

    for (i, dev) in devices.iter().enumerate() {
        println!("G-SYNC device #{}", i);

        // Read-only queries
        println!("  capabilities: {:#?}", dev.query_capabilities());
        println!("  status params: {:#?}", dev.get_status_parameters()); // uses V1
        let mut ctrl = dev.get_control_parameters()?;
        println!("  control params (before): {:#?}", ctrl);

        // Validate SetControlParameters by writing back the same values (no-op)
        dev.set_control_parameters(&mut ctrl)?;
        println!("  control params (applied): {:#?}", ctrl);

        // Validate SetSyncStateSettings by re-applying current topology (optional, no-op)
        if apply_sync {
            let (_gpus, displays) = dev.get_topology()?;
            println!("  re-applying current sync state to {} active displays...", displays.len());
            // Convenience wrapper that preserves current per-display state
            // (no topology change, should be a no-op if the wrapper builds from current state)
            dev.set_sync_state_settings_from_topology(&displays, 0)?;
            println!("  sync state re-applied.");
        } else {
            println!("  skipping sync state apply (pass --apply-sync to test)");
        }
    }

    Ok(())
}