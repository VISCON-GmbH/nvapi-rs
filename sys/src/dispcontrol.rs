use std::os::raw::c_char;
use crate::status::NvAPI_Status;
use crate::handles;

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

