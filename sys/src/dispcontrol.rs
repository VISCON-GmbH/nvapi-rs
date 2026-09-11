use crate::handles;
use crate::status::NvAPI_Status;
use std::os::raw::{c_char, c_int, c_void};

// Display control enums
nvenum! {
    /// NV_ROTATE: Rotate modes used in NvAPI_SetViewEx() and Mosaic grid displays
    pub enum NV_ROTATE / Rotate {
        NV_ROTATE_0 / R0 = 0,
        NV_ROTATE_90 / R90 = 1,
        NV_ROTATE_180 / R180 = 2,
        NV_ROTATE_270 / R270 = 3,
        NV_ROTATE_IGNORED / Ignored = 4,
    }
}

nvapi_fn! {
    pub type EnumNvidiaDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns NVAPI_END_ENUMERATION.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaDisplayHandle;
}

nvapi_fn! {
    pub type EnumNvidiaUnAttachedDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvUnAttachedDispHandle: *mut handles::NvUnAttachedDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA unattached display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns error.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaUnAttachedDisplayHandle;
}

nvapi_fn! {
    pub type GetAssociatedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display that is associated
    /// with the given display "name" (such as "\\.\DISPLAY1").
    pub unsafe fn NvAPI_GetAssociatedNvidiaDisplayHandle;
}

nvapi_fn! {
    pub type DISP_GetAssociatedUnAttachedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvUnAttachedDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of an unattached NVIDIA display that is
    /// associated with the given display name (such as "\\DISPLAY1").
    pub unsafe fn NvAPI_DISP_GetAssociatedUnAttachedNvidiaDisplayHandle;
}

nvapi_fn! {
    pub type DISP_GetDisplayIdByDisplayNameFn = extern "C" fn(displayName: *const c_char, displayId: *mut u32) -> NvAPI_Status;

    /// Retrieves the displayId of a given, currently active display by its GDI
    /// device name (such as "\\.\DISPLAY1"). In clone/Surround configurations
    /// the primary or top-left display is returned.
    ///
    /// This is the reverse of enumerating a Windows display name to its
    /// NVAPI displayId; pairing it with `EnumDisplayDevices` lets you map a
    /// displayId back to the OS display index it corresponds to.
    pub unsafe fn NvAPI_DISP_GetDisplayIdByDisplayName;
}

nvapi_fn! {
    pub type SYS_GetDisplayIdFromGpuAndOutputIdFn = extern "C" fn(hPhysicalGpu: handles::NvPhysicalGpuHandle, outputId: u32, displayId: *mut u32) -> NvAPI_Status;

    /// Converts a physical GPU handle and output ID (a single-bit mask) to a displayId.
    pub unsafe fn NvAPI_SYS_GetDisplayIdFromGpuAndOutputId;
}

nvapi_fn! {
    pub type SYS_GetGpuAndOutputIdFromDisplayIdFn = extern "C" fn(displayId: u32, hPhysicalGpu: *mut handles::NvPhysicalGpuHandle, outputId: *mut u32) -> NvAPI_Status;

    /// Converts a displayId to a physical GPU handle and output ID (a single-bit mask).
    pub unsafe fn NvAPI_SYS_GetGpuAndOutputIdFromDisplayId;
}

// ============================================================================
// Display configuration — `NvAPI_DISP_GetDisplayConfig` / `NvAPI_DISP_SetDisplayConfig`.
//
// These are the modern replacements for the deprecated `NvAPI_SetView` / `NvAPI_SetViewEx`
// entry points. They describe the *complete* desktop as an array of paths: one path per
// source (desktop surface), each with one or more targets (displays). A display that is not
// present in the array passed to `NvAPI_DISP_SetDisplayConfig` is deactivated.
// ============================================================================

// ---- Display configuration enums ----

nvenum! {
    /// NV_SCALING: scaling mode applied to a display target.
    pub enum NV_SCALING / Scaling {
        /// No change.
        NV_SCALING_DEFAULT / Default = 0,
        /// Balanced — full screen.
        NV_SCALING_GPU_SCALING_TO_CLOSEST / GpuScalingToClosest = 1,
        /// Force GPU — full screen.
        NV_SCALING_GPU_SCALING_TO_NATIVE / GpuScalingToNative = 2,
        /// Force GPU — centered, no scaling.
        NV_SCALING_GPU_SCANOUT_TO_NATIVE / GpuScanoutToNative = 3,
        /// Force GPU — aspect ratio.
        NV_SCALING_GPU_SCALING_TO_ASPECT_SCANOUT_TO_NATIVE / GpuScalingToAspectScanoutToNative = 5,
        /// Balanced — aspect ratio.
        NV_SCALING_GPU_SCALING_TO_ASPECT_SCANOUT_TO_CLOSEST / GpuScalingToAspectScanoutToClosest = 6,
        /// Balanced — centered, no scaling.
        NV_SCALING_GPU_SCANOUT_TO_CLOSEST / GpuScanoutToClosest = 7,
        /// Force GPU — integer scaling.
        NV_SCALING_GPU_INTEGER_ASPECT_SCALING / GpuIntegerAspectScaling = 8,
        /// For future use.
        NV_SCALING_CUSTOMIZED / Customized = 255,
    }
}

nvenum_display! {
    Scaling => _
}

nvenum! {
    /// NV_FORMAT: color format of a source mode. Only `Unknown` (driver picks) is currently accepted.
    pub enum NV_FORMAT / ColorFormat {
        /// Unknown — the driver will choose.
        NV_FORMAT_UNKNOWN / Unknown = 0,
        /// 32bpp.
        NV_FORMAT_A8R8G8B8 / A8R8G8B8 = 21,
        /// 16bpp.
        NV_FORMAT_R5G6B5 / R5G6B5 = 23,
        /// 8bpp.
        NV_FORMAT_P8 / P8 = 41,
        /// 64bpp floating point.
        NV_FORMAT_A16B16G16R16F / A16B16G16R16F = 113,
    }
}

nvenum_display! {
    ColorFormat => _
}

nvenum! {
    /// NV_TIMING_OVERRIDE: which timing standard the driver should scan out with.
    pub enum NV_TIMING_OVERRIDE / TimingOverride {
        /// Keep the current timing.
        NV_TIMING_OVERRIDE_CURRENT / Current = 0,
        /// The timing the driver picks based on the current policy.
        NV_TIMING_OVERRIDE_AUTO / Auto = 1,
        NV_TIMING_OVERRIDE_EDID / Edid = 2,
        NV_TIMING_OVERRIDE_DMT / Dmt = 3,
        NV_TIMING_OVERRIDE_DMT_RB / DmtRb = 4,
        NV_TIMING_OVERRIDE_CVT / Cvt = 5,
        NV_TIMING_OVERRIDE_CVT_RB / CvtRb = 6,
        NV_TIMING_OVERRIDE_GTF / Gtf = 7,
        NV_TIMING_OVERRIDE_EIA861 / Eia861 = 8,
        NV_TIMING_OVERRIDE_ANALOG_TV / AnalogTv = 9,
        /// Custom timings; only then is `NV_TIMING` read.
        NV_TIMING_OVERRIDE_CUST / Cust = 10,
        NV_TIMING_OVERRIDE_NV_PREDEFINED / NvPredefined = 11,
        NV_TIMING_OVERRIDE_NV_ASPR / NvAspr = 12,
        NV_TIMING_OVERRIDE_SDI / Sdi = 13,
    }
}

nvenum_display! {
    TimingOverride => _
}

nvenum! {
    /// NV_DISPLAYCONFIG_SPANNING_ORIENTATION: legacy spanning mode (Windows XP only).
    pub enum NV_DISPLAYCONFIG_SPANNING_ORIENTATION / SpanningOrientation {
        NV_DISPLAYCONFIG_SPAN_NONE / None = 0,
        NV_DISPLAYCONFIG_SPAN_HORIZONTAL / Horizontal = 1,
        NV_DISPLAYCONFIG_SPAN_VERTICAL / Vertical = 2,
    }
}

nvenum_display! {
    SpanningOrientation => _
}

/// NV_GPU_CONNECTOR_TYPE. Only carried through for TV targets; kept as a raw `int`.
pub type NV_GPU_CONNECTOR_TYPE = c_int;
pub const NV_GPU_CONNECTOR_UNKNOWN: NV_GPU_CONNECTOR_TYPE = -1;

/// NV_DISPLAY_TV_FORMAT. Only relevant for TV targets; kept as a raw bit mask.
pub type NV_DISPLAY_TV_FORMAT = u32;
pub const NV_DISPLAY_TV_FORMAT_NONE: NV_DISPLAY_TV_FORMAT = 0;

// ---- Display configuration flags ----

/// Flags accepted by `NvAPI_DISP_SetDisplayConfig`.
pub const NV_DISPLAYCONFIG_VALIDATE_ONLY: u32 = 0x0000_0001;
/// Persist the configuration in the driver so it survives a reboot.
pub const NV_DISPLAYCONFIG_SAVE_TO_PERSISTENCE: u32 = 0x0000_0002;
/// Permit a driver reload if the modeset requires one.
pub const NV_DISPLAYCONFIG_DRIVER_RELOAD_ALLOWED: u32 = 0x0000_0004;
/// Refresh the OS mode list.
pub const NV_DISPLAYCONFIG_FORCE_MODE_ENUMERATION: u32 = 0x0000_0008;
/// Tell the OS to avoid optimizing the CommitVidPn call during a modeset.
pub const NV_FORCE_COMMIT_VIDPN: u32 = 0x0000_0010;

/// `NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1::flags` — interlaced mode.
pub const NV_DISPLAYCONFIG_TARGET_INTERLACED: u32 = 1 << 0;
/// Primary display *within a clone group*. This is not the GDI primary.
pub const NV_DISPLAYCONFIG_TARGET_PRIMARY: u32 = 1 << 1;
pub const NV_DISPLAYCONFIG_TARGET_IS_PAN_AND_SCAN: u32 = 1 << 2;
pub const NV_DISPLAYCONFIG_TARGET_DISABLE_VIRTUAL_MODE_SUPPORT: u32 = 1 << 3;
pub const NV_DISPLAYCONFIG_TARGET_IS_PREFERRED_UNSCALED: u32 = 1 << 4;

/// `NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1::flags` — this source is the GDI primary.
pub const NV_DISPLAYCONFIG_SOURCE_GDI_PRIMARY: u32 = 1 << 0;
/// `NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1::flags` — this source has SLI focus.
pub const NV_DISPLAYCONFIG_SOURCE_SLI_FOCUS: u32 = 1 << 1;

/// `NV_DISPLAYCONFIG_PATH_INFO_V2::flags` — path belongs to a non-NVIDIA adapter.
pub const NV_DISPLAYCONFIG_PATH_IS_NON_NVIDIA_ADAPTER: u32 = 1 << 0;

// ---- Display configuration structs ----

nvstruct! {
    /// Pixel dimensions and color depth of a desktop surface.
    pub struct NV_RESOLUTION {
        pub width: u32,
        pub height: u32,
        pub colorDepth: u32,
    }
}

nvstruct! {
    /// Top-left corner of a desktop surface in virtual desktop coordinates.
    pub struct NV_POSITION {
        pub x: i32,
        pub y: i32,
    }
}

nvstruct! {
    /// NVIDIA-specific timing extras used in [`NV_TIMING`].
    pub struct NV_TIMINGEXT {
        pub flag: u32,
        pub rr: u16,
        pub rrx1k: u32,
        pub aspect: u32,
        pub rep: u16,
        pub status: u32,
        pub name: [u8; 40],
    }
}

nvstruct! {
    /// VESA scan-out timing. Only read when `timingOverride == NV_TIMING_OVERRIDE_CUST`.
    pub struct NV_TIMING {
        pub HVisible: u16,
        pub HBorder: u16,
        pub HFrontPorch: u16,
        pub HSyncWidth: u16,
        pub HTotal: u16,
        pub HSyncPol: u8,

        pub VVisible: u16,
        pub VBorder: u16,
        pub VFrontPorch: u16,
        pub VSyncWidth: u16,
        pub VTotal: u16,
        pub VSyncPol: u8,

        pub interlaced: u16,
        pub pclk: u32,

        pub etc: NV_TIMINGEXT,
    }
}

nvstruct! {
    /// Optional per-target settings: rotation, scaling and refresh rate.
    pub struct NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1 {
        pub version: u32,
        pub rotation: NV_ROTATE,
        pub scaling: NV_SCALING,
        /// Non-interlaced refresh rate in mHz (rate × 1000); 0 lets the driver choose.
        pub refreshRate1K: u32,
        /// C bit field: `interlaced:1, primary:1, isPanAndScanTarget:1,
        /// disableVirtualModeSupport:1, isPreferredUnscaledTarget:1, reserved:27`.
        pub flags: u32,
        pub connector: NV_GPU_CONNECTOR_TYPE,
        pub tvFormat: NV_DISPLAY_TV_FORMAT,
        pub timingOverride: NV_TIMING_OVERRIDE,
        pub timing: NV_TIMING,
    }
}
const NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1_SIZE: usize =
    std::mem::size_of::<NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1>();
nvversion! { NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_VER1(NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1 = NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1_SIZE, 1) }
nvversion! { NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_VER = NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_VER1 }
pub type NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO =
    NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1;

nvstruct! {
    /// A display driven by a path.
    pub struct NV_DISPLAYCONFIG_PATH_TARGET_INFO_V2 {
        pub displayId: u32,
        /// May be null when no advanced settings are required.
        pub details: *mut NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1,
        /// Windows CCD target ID; only meaningful for non-NVIDIA adapters.
        pub targetId: u32,
    }
}
pub type NV_DISPLAYCONFIG_PATH_TARGET_INFO = NV_DISPLAYCONFIG_PATH_TARGET_INFO_V2;

nvstruct! {
    /// The desktop surface backing a path.
    pub struct NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1 {
        pub resolution: NV_RESOLUTION,
        /// Ignored at present, must be `NV_FORMAT_UNKNOWN`.
        pub colorFormat: NV_FORMAT,
        pub position: NV_POSITION,
        pub spanningOrientation: NV_DISPLAYCONFIG_SPANNING_ORIENTATION,
        /// C bit field: `bGDIPrimary:1, bSLIFocus:1, reserved:30`.
        pub flags: u32,
    }
}
pub type NV_DISPLAYCONFIG_SOURCE_MODE_INFO = NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1;

nvstruct! {
    /// One source (desktop surface) plus the displays it drives.
    pub struct NV_DISPLAYCONFIG_PATH_INFO_V2 {
        pub version: u32,
        /// Windows CCD source id. When every path has 0, NVAPI assigns them.
        pub sourceId: u32,
        pub targetInfoCount: u32,
        pub targetInfo: *mut NV_DISPLAYCONFIG_PATH_TARGET_INFO_V2,
        /// May be null when the mode is not important.
        pub sourceModeInfo: *mut NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1,
        /// C bit field: `IsNonNVIDIAAdapter:1, reserved:31`.
        pub flags: u32,
        /// LUID of the OS adapter; only used by non-NVIDIA adapters, otherwise null.
        pub pOSAdapterID: *mut c_void,
    }
}
const NV_DISPLAYCONFIG_PATH_INFO_V2_SIZE: usize =
    std::mem::size_of::<NV_DISPLAYCONFIG_PATH_INFO_V2>();
nvversion! { NV_DISPLAYCONFIG_PATH_INFO_VER2(NV_DISPLAYCONFIG_PATH_INFO_V2 = NV_DISPLAYCONFIG_PATH_INFO_V2_SIZE, 2) }
nvversion! { NV_DISPLAYCONFIG_PATH_INFO_VER = NV_DISPLAYCONFIG_PATH_INFO_VER2 }
pub type NV_DISPLAYCONFIG_PATH_INFO = NV_DISPLAYCONFIG_PATH_INFO_V2;

// ---- Display configuration functions ----

nvapi_fn! {
    pub type DISP_GetDisplayConfigFn = extern "C" fn(pathInfoCount: *mut u32, pathInfo: *mut NV_DISPLAYCONFIG_PATH_INFO) -> NvAPI_Status;

    /// Retrieves the current global display configuration.
    ///
    /// Requires up to three passes:
    /// 1. `pathInfo` null — returns the number of paths in `pathInfoCount`.
    /// 2. `pathInfo` allocated with `version` set on each entry — fills `targetInfoCount`.
    /// 3. `targetInfo` (and optionally `sourceModeInfo`) allocated and wired up — fills everything.
    ///
    /// Returns `NVAPI_DEVICE_BUSY` while a modeset is still in flight.
    pub unsafe fn NvAPI_DISP_GetDisplayConfig;
}

nvapi_fn! {
    pub type DISP_SetDisplayConfigFn = extern "C" fn(pathInfoCount: u32, pathInfo: *mut NV_DISPLAYCONFIG_PATH_INFO, flags: u32) -> NvAPI_Status;

    /// Applies a global display configuration across all GPUs.
    ///
    /// The supplied array is the complete new desktop: any display that is not referenced by one
    /// of the paths is deactivated. `flags` is a bitwise OR of the `NV_DISPLAYCONFIG_*` flags.
    pub unsafe fn NvAPI_DISP_SetDisplayConfig;
}

nvapi_fn! {
    pub type DISP_GetGDIPrimaryDisplayIdFn = extern "C" fn(displayId: *mut u32) -> NvAPI_Status;

    /// Returns the display ID of the GDI primary display.
    pub unsafe fn NvAPI_DISP_GetGDIPrimaryDisplayId;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    /// NVAPI encodes `sizeof(struct)` in the low 16 bits of the version field, so the ported
    /// layouts have to match the C headers byte for byte.
    #[test]
    fn struct_sizes_match_headers() {
        assert_eq!(size_of::<NV_RESOLUTION>(), 12);
        assert_eq!(size_of::<NV_POSITION>(), 8);
        assert_eq!(size_of::<NV_TIMINGEXT>(), 64);
        assert_eq!(size_of::<NV_TIMING>(), 96);
        assert_eq!(
            size_of::<NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1>(),
            128
        );
        assert_eq!(size_of::<NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1>(), 32);
    }
}
