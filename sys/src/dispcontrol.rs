use crate::handles;
use crate::status::NvAPI_Status;
use std::os::raw::c_char;

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
