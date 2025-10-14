use crate::{sys, types::RawConversion};

mod info;
mod physical;

pub use info::{
    DisplayId,
    DriverModel,
    GpuArchitectureInfo,
    MemoryInfo,
    PciIdentifiers,
};
pub use physical::PhysicalGpu;

pub use sys::gpu::arch::{ArchitectureId, ArchitectureImplementationId, ChipRevision};
pub use sys::gpu::clock::ClockFrequencyType;
pub use sys::gpu::display::{ConnectedIdsFlags, DisplayIdsFlags, MonitorConnectorType};
pub use sys::gpu::private::{Foundry, RamMaker, RamType, VendorId as Vendor};
pub use sys::gpu::{PerformanceDecreaseReason, SystemType};

pub type ClockFrequencies = <sys::gpu::clock::NV_GPU_CLOCK_FREQUENCIES as RawConversion>::Target;
pub type Utilizations = <sys::gpu::pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX as RawConversion>::Target;
