use log::trace;
use nvapi_sys::{handles, status_result, NVAPI_MAX_GSYNC_DEVICES};
use crate::sys::gsync::{self};

use crate::PhysicalGpu;

#[derive(Debug)]
pub struct GSyncDevice {
    handle: handles::NvGSyncDeviceHandle,
}

impl GSyncDevice {
    pub fn new(handle: handles::NvGSyncDeviceHandle) -> Self {
        Self {handle}
    }

    pub fn handle(&self) -> &handles::NvGSyncDeviceHandle {
        &self.handle
    }

    pub fn get_sync_devices() -> crate::Result<Vec<GSyncDevice>> {
        trace!("gsync.enumerate()");
        let mut handles = [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut len = 0;
        match unsafe { gsync::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut len) } {
            status => status_result(status).map(move |_| handles[..len as usize].iter().cloned().map(|x| GSyncDevice::new(x)).collect()),
        }
    }

    pub fn get_sync_status(&self, gpu: PhysicalGpu) -> crate::Result<gsync::NV_GSYNC_STATUS> {
        let mut status = gsync::NV_GSYNC_STATUS::default();
        status.version  = gsync::NV_GSYNC_STATUS_VER;
        match unsafe {
            gsync::NvAPI_GSync_GetSyncStatus(*self.handle(), *gpu.handle(), &mut status)
        } {
            ret => status_result(ret).map(|_| status),
        }
    }
}