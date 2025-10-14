use nvapi::PhysicalGpu;

pub fn main() {
    PhysicalGpu::enumerate().unwrap().iter().for_each(|gpu| {
        println!("GPU: {:?}", gpu.full_name());
        println!("  Arch: {:?}", gpu.architecture_info());
        println!();
    });
}
