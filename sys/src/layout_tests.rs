//! Struct layout tests against the C headers in `c_nvapi/`.
//!
//! NVAPI encodes `sizeof(struct)` in the low 16 bits of every version constant, so a struct
//! whose Rust layout differs from the C layout is rejected by the driver (or worse, accepted and
//! misread). The `nvversion!` tests only compare a version constant with `size_of` of the Rust
//! struct, which proves nothing when the constant itself is derived from `size_of`. The expected
//! sizes below are computed by hand from `nvapi.h` (MSVC layout, C bitfields packed into their
//! `NvU32` storage unit) and must not be derived from the Rust types.

use crate::types::{GET_NVAPI_SIZE, GET_NVAPI_VERSION};
use std::mem::size_of;

/// Picks the expected size for the current target: `(64-bit, 32-bit)`.
const fn ptr(sz64: usize, sz32: usize) -> usize {
    if cfg!(target_pointer_width = "64") {
        sz64
    } else {
        sz32
    }
}

#[track_caller]
fn check_ver(ver: u32, size: usize, version: u16) {
    assert_eq!(GET_NVAPI_SIZE(ver), size, "size encoded in version constant 0x{:08x}", ver);
    assert_eq!(GET_NVAPI_VERSION(ver), version, "version number in 0x{:08x}", ver);
}

#[test]
fn gsync() {
    use crate::gsync::*;

    check_ver(NV_GSYNC_CAPABILITIES_VER_1, 16, 1);
    check_ver(NV_GSYNC_CAPABILITIES_VER_2, 20, 2);
    check_ver(NV_GSYNC_DISPLAY_VER, 16, 1);
    check_ver(NV_GSYNC_GPU_VER, ptr(40, 20), 1);
    check_ver(NV_GSYNC_DELAY_VER, 20, 1);
    check_ver(NV_GSYNC_CONTROL_PARAMS_VER1, 64, 1);
    check_ver(NV_GSYNC_CONTROL_PARAMS_VER2, 72, 2);
    check_ver(NV_GSYNC_CONTROL_PARAMS_VER, 72, 2);
    check_ver(NV_GSYNC_STATUS_VER, 16, 1);
    check_ver(NV_GSYNC_STATUS_PARAMS_VER_1, 32, 1);
    check_ver(NV_GSYNC_STATUS_PARAMS_VER_2, 36, 2);
}

#[test]
fn gsync_bitfields() {
    use crate::gsync::*;

    let mut params = NV_GSYNC_CONTROL_PARAMS::zeroed();
    params.set_syncSourceIsOutput(true);
    assert_eq!(params.flags, 0b10);
    assert!(!params.interlaceMode());
    params.set_interlaceMode(true);
    params.set_syncSourceIsOutput(false);
    assert_eq!(params.flags, 0b01);
    assert!(params.interlaceMode());
}

#[test]
fn dispcontrol() {
    use crate::dispcontrol::*;

    check_ver(NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_VER1, 128, 1);
    check_ver(NV_DISPLAYCONFIG_PATH_INFO_VER2, ptr(48, 28), 2);
    assert_eq!(size_of::<NV_DISPLAYCONFIG_PATH_TARGET_INFO_V2>(), ptr(24, 12));
    assert_eq!(size_of::<NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1>(), 32);
    assert_eq!(size_of::<NV_TIMING>(), 96);
    assert_eq!(size_of::<NV_TIMINGEXT>(), 64);
}

#[test]
fn mosaic() {
    use crate::mosaic::*;

    assert_eq!(size_of::<NV_MOSAIC_GPU_LAYOUT_ELEM>(), ptr(24, 16));
    check_ver(NVAPI_MOSAIC_TOPO_BRIEF_VER, 16, 1);
    check_ver(NVAPI_MOSAIC_TOPO_DETAILS_VER, ptr(1568, 1044), 1);
    check_ver(NVAPI_MOSAIC_DISPLAY_SETTING_VER1, 20, 1);
    check_ver(NVAPI_MOSAIC_DISPLAY_SETTING_VER2, 24, 2);
    check_ver(NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER1, 1372, 1);
    check_ver(NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER2, 1532, 2);
    check_ver(NVAPI_MOSAIC_TOPO_GROUP_VER, ptr(3160, 2112), 1);
    assert_eq!(size_of::<NV_MOSAIC_GRID_TOPO_DISPLAY_V1>(), 20);
    assert_eq!(size_of::<NV_MOSAIC_GRID_TOPO_DISPLAY_V2>(), 28);
    check_ver(NV_MOSAIC_GRID_TOPO_VER1, 1320, 1);
    check_ver(NV_MOSAIC_GRID_TOPO_VER2, 1832, 2);
    check_ver(NV_MOSAIC_DISPLAY_TOPO_STATUS_VER, 2064, 1);
    check_ver(NVAPI_MOSAIC_TOPOLOGY_VER, ptr(1552, 1036), 1);
    check_ver(NVAPI_MOSAIC_SUPPORTED_TOPOLOGIES_VER, ptr(24840, 16584), 1);
}

#[test]
fn gpu() {
    use crate::gpu::{arch, clock, display, pstate, thermal};

    check_ver(crate::gpu::NV_BOARD_INFO_VER1, 20, 1);
    check_ver(arch::NV_GPU_ARCH_INFO_VER_1, 16, 1);
    check_ver(arch::NV_GPU_ARCH_INFO_VER_2, 16, 2);
    check_ver(clock::NV_GPU_CLOCK_FREQUENCIES_VER_1, 264, 1);
    check_ver(clock::NV_GPU_CLOCK_FREQUENCIES_VER_2, 264, 2);
    check_ver(clock::NV_GPU_CLOCK_FREQUENCIES_VER_3, 264, 3);
    check_ver(pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX_VER, 72, 1);
    check_ver(pstate::NV_GPU_PERF_PSTATES20_INFO_VER1, 7316, 1);
    check_ver(pstate::NV_GPU_PERF_PSTATES20_INFO_VER2, 7416, 2);
    check_ver(pstate::NV_GPU_PERF_PSTATES20_INFO_VER3, 7416, 3);
    check_ver(thermal::NV_GPU_THERMAL_SETTINGS_VER_1, 68, 1);
    check_ver(thermal::NV_GPU_THERMAL_SETTINGS_VER_2, 68, 2);
    check_ver(display::NV_GPU_DISPLAYIDS_VER1, 16, 1);
    check_ver(display::NV_GPU_DISPLAYIDS_VER2, 16, 3);
}

#[test]
fn i2c() {
    use crate::i2c::*;

    check_ver(NV_I2C_INFO_VER1, ptr(48, 32), 1);
    check_ver(NV_I2C_INFO_VER2, ptr(56, 36), 2);
    check_ver(NV_I2C_INFO_VER3, ptr(64, 44), 3);
}

#[test]
fn driverapi() {
    use crate::driverapi::*;

    check_ver(NV_DISPLAY_DRIVER_MEMORY_INFO_VER_1, 20, 1);
    check_ver(NV_DISPLAY_DRIVER_MEMORY_INFO_VER_2, 24, 2);
    check_ver(NV_DISPLAY_DRIVER_MEMORY_INFO_VER_3, 32, 3);
}

#[test]
fn bool_u32_set_keeps_reserved_bits() {
    let mut b = crate::types::BoolU32(0xf000_0000);
    b.set(true);
    assert_eq!(b.0, 0xf000_0001);
    b.set(false);
    assert_eq!(b.0, 0xf000_0000);
}

#[test]
fn perf_decrease_flags() {
    use crate::gpu::*;

    let reasons = PerformanceDecreaseReason::from_bits_truncate(0x8000_0002);
    assert!(reasons.contains(PerformanceDecreaseReason::UNKNOWN));
    // Iterating must terminate and skip the zero-valued NONE flag.
    let flags: Vec<_> = reasons.collect();
    assert_eq!(
        flags,
        [
            PerformanceDecreaseReason::POWER_CONTROL,
            PerformanceDecreaseReason::UNKNOWN,
        ]
    );
    assert_eq!(PerformanceDecreaseReason::NONE.count(), 0);
}
