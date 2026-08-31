//! Display Index Lookup Example
//!
//! Lists every connected display's `(gpu_index, display_index)` pair alongside
//! its raw NVAPI `display_id`, and resolves a `display_id` back to that pair
//! (the same `GPU,DISPLAY` numbering used by NVIDIA's `configureSync` / Mosaic
//! command line tools).
//!
//! Run:
//!   cargo run --example display_index [display_id]

use nvapi::PhysicalGpu;
use std::env;

fn main() -> Result<(), nvapi::Status> {
    let mut first_display_id = None;

    for (gpu_index, gpu) in PhysicalGpu::enumerate()?.iter().enumerate() {
        println!("GPU #{}: {:?}", gpu_index, gpu.full_name());

        let displays = gpu.display_ids_connected(Default::default())?;
        for (display_index, display) in displays.iter().enumerate() {
            println!(
                "  display={},{} -> display_id={} ({:?})",
                gpu_index, display_index, display.display_id, display.connector
            );
            first_display_id.get_or_insert(display.display_id);
        }
    }

    // Use the display_id passed on the command line, or fall back to the
    // first display found above so the example works with no arguments.
    let display_id = match env::args().nth(1) {
        Some(arg) => arg.parse().expect("display_id must be a u32"),
        None => match first_display_id {
            Some(id) => id,
            None => {
                println!("No connected displays found");
                return Ok(());
            }
        },
    };

    match PhysicalGpu::display_gpu_index(display_id)? {
        Some((gpu_index, display_index)) => println!(
            "\ndisplay_id={} -> display={},{}",
            display_id, gpu_index, display_index
        ),
        None => println!("\ndisplay_id={} not found", display_id),
    }

    Ok(())
}
