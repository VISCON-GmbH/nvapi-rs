//! G-SYNC device helpers.
//!
//! This module provides a thin, safe wrapper around NVAPI's G-SYNC device
//! handle to enumerate G-SYNC modules present in the system and to query
//! their synchronization status for a given GPU.

use crate::sys::gsync::{self};
use crate::PhysicalGpu;
use std::vec::IntoIter as VecIntoIter;
use log::trace;
use nvapi_sys::{handles, status_result, NVAPI_MAX_GSYNC_DEVICES};

/// A handle to an NVIDIA G-SYNC device.
///
/// This is a light wrapper around `nvapi_sys::handles::NvGSyncDeviceHandle` that
/// exposes convenient methods to enumerate devices and query their status.
///
/// See also: [`GSyncDevice::get_sync_devices`].
#[derive(Debug)]
pub struct GSyncDevice {
    handle: handles::NvGSyncDeviceHandle,
}

impl GSyncDevice {
    /// Creates a new wrapper from a raw NVAPI G-SYNC device handle.
    ///
    /// Most callers should prefer [`GSyncDevice::get_sync_devices`] to obtain
    /// valid handles from NVAPI directly.
    pub fn new(handle: handles::NvGSyncDeviceHandle) -> Self {
        Self { handle }
    }

    /// Returns a reference to the underlying NVAPI G-SYNC device handle.
    pub fn handle(&self) -> &handles::NvGSyncDeviceHandle {
        &self.handle
    }

    /// Enumerates all G-SYNC devices known to NVAPI.
    ///
    /// Returns an empty vector if no devices are present.
    ///
    /// Errors are forwarded from `NvAPI_GSync_EnumSyncDevices` when NVAPI is not
    /// available or the call fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::GSyncDevice;
    ///
    /// let devices = GSyncDevice::get_sync_devices()?;
    /// println!("found {} G-SYNC device(s)", devices.len());
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_sync_devices() -> crate::Result<Vec<GSyncDevice>> {
        trace!("gsync.enumerate()");
        let mut handles = [Default::default(); NVAPI_MAX_GSYNC_DEVICES];
        let mut len = 0;
        match unsafe { gsync::NvAPI_GSync_EnumSyncDevices(&mut handles, &mut len) } {
            status => status_result(status).map(move |_| {
                handles[..len as usize]
                    .iter()
                    .cloned()
                    .map(|x| GSyncDevice::new(x))
                    .collect()
            }),
        }
    }

    /// Queries the G-SYNC synchronization status for this device and a given GPU.
    ///
    /// The returned type is the raw NVAPI struct [`crate::sys::gsync::NV_GSYNC_STATUS`].
    ///
    /// # Errors
    /// Returns any error produced by `NvAPI_GSync_GetSyncStatus`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::{GSyncDevice, PhysicalGpu};
    ///
    /// let gpu = PhysicalGpu::enumerate()?.into_iter().next().expect("no GPU found");
    /// let dev = GSyncDevice::get_sync_devices()?.into_iter().next().expect("no G-SYNC device found");
    /// let status = dev.get_sync_status(gpu)?;
    /// println!("status: {:?}", status);
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_sync_status(&self, gpu: PhysicalGpu) -> crate::Result<gsync::NV_GSYNC_STATUS> {
        let mut status = gsync::NV_GSYNC_STATUS::default();
        status.version = gsync::NV_GSYNC_STATUS_VER;
        match unsafe {
            gsync::NvAPI_GSync_GetSyncStatus(*self.handle(), *gpu.handle(), &mut status)
        } {
            ret => status_result(ret).map(|_| status),
        }
    }

    /// Queries static capabilities of this G-SYNC device.
    ///
    /// Wraps `NvAPI_GSync_QueryCapabilities`.
    pub fn query_capabilities(&self) -> crate::Result<gsync::NV_GSYNC_CAPABILITIES> {
        trace!("gsync.query_capabilities()");
        let mut caps = gsync::NV_GSYNC_CAPABILITIES::zeroed();
        caps.version = gsync::NV_GSYNC_CAPABILITIES_VER;
        match unsafe { gsync::NvAPI_GSync_QueryCapabilities(*self.handle(), &mut caps) } {
            ret => status_result(ret).map(|_| caps),
        }
    }

    /// Queries extended status parameters of this G-SYNC device.
    ///
    /// Wraps `NvAPI_GSync_GetStatusParameters`.
    pub fn get_status_parameters(&self) -> crate::Result<gsync::NV_GSYNC_STATUS_PARAMS> {
        trace!("gsync.get_status_parameters()");
        let mut params = gsync::NV_GSYNC_STATUS_PARAMS::zeroed();
        params.version = gsync::NV_GSYNC_STATUS_PARAMS_VER;
        match unsafe { gsync::NvAPI_GSync_GetStatusParameters(*self.handle(), &mut params) } {
            ret => status_result(ret).map(|_| params),
        }
    }

    /// Queries current control parameters of this G-SYNC device.
    ///
    /// Wraps `NvAPI_GSync_GetControlParameters`.
    pub fn get_control_parameters(&self) -> crate::Result<gsync::NV_GSYNC_CONTROL_PARAMS> {
        trace!("gsync.get_control_parameters()");
        let mut params = gsync::NV_GSYNC_CONTROL_PARAMS::zeroed();
        params.version = gsync::NV_GSYNC_CONTROL_PARAMS_VER;
        match unsafe { gsync::NvAPI_GSync_GetControlParameters(*self.handle(), &mut params) } {
            ret => status_result(ret).map(|_| params),
        }
    }

    /// Returns the topology for this G-SYNC device: connected GPUs and active displays.
    ///
    /// This performs the two-step NVAPI pattern: first query counts, then allocate and fill.
    /// See: https://docs.nvidia.com/gameworks/content/gameworkslibrary/coresdk/nvapi/group__gsyncapi.html#ga4fef69c9edcd58f2c47e2721f5c67528
    pub fn get_topology(
        &self,
    ) -> crate::Result<(Vec<gsync::NV_GSYNC_GPU>, Vec<gsync::NV_GSYNC_DISPLAY>)> {
        trace!("gsync.get_topology() [count]");
        let mut gpu_count: u32 = 0;
        let mut disp_count: u32 = 0;
        // Count-only call (both buffers null)
        unsafe {
            status_result(gsync::NvAPI_GSync_GetTopology(
                *self.handle(),
                &mut gpu_count,
                std::ptr::null_mut(),
                &mut disp_count,
                std::ptr::null_mut(),
            ))?;
        }

        let mut gpus = Vec::with_capacity(gpu_count as usize);
        let mut displays = Vec::with_capacity(disp_count as usize);
        // Pre-initialize entries with correct version tags per NVAPI convention
        for _ in 0..gpu_count {
            let mut e = gsync::NV_GSYNC_GPU::zeroed();
            e.version = gsync::NV_GSYNC_GPU_VER;
            gpus.push(e);
        }
        for _ in 0..disp_count {
            let mut e = gsync::NV_GSYNC_DISPLAY::zeroed();
            e.version = gsync::NV_GSYNC_DISPLAY_VER;
            displays.push(e);
        }

        trace!(
            "gsync.get_topology() [fill] {} gpus, {} displays",
            gpu_count,
            disp_count
        );
        unsafe {
            status_result(gsync::NvAPI_GSync_GetTopology(
                *self.handle(),
                &mut gpu_count,
                gpus.as_mut_ptr(),
                &mut disp_count,
                displays.as_mut_ptr(),
            ))?;
            // NVAPI may return fewer than allocated; adjust lengths
            gpus.set_len(gpu_count as usize);
            displays.set_len(disp_count as usize);
        }

        Ok((gpus, displays))
    }

    /// Converts an NV_GSYNC_GPU entry from topology into a PhysicalGpu wrapper.
    ///
    /// Prefer the `hPhysicalGpu` handle; falls back to `hProxyPhysicalGpu` if the
    /// physical one is null.
    pub fn to_physical_gpu(entry: &gsync::NV_GSYNC_GPU) -> Option<crate::PhysicalGpu> {
        let primary = entry.hPhysicalGpu;
        let handle = match primary.is_null() {
            false => primary,
            true => entry.hProxyPhysicalGpu,
        };
        crate::PhysicalGpu::from_raw_handle(handle)
    }

    /// Returns an iterator over PhysicalGpu devices connected to this G-SYNC device.
    ///
    /// Internally fetches topology once and then yields valid GPUs only.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nvapi::GSyncDevice;
    ///
    /// let dev = GSyncDevice::get_sync_devices()?.into_iter().next()
    ///     .expect("no G-SYNC device found");
    ///
    /// for gpu in dev.iter_physical_gpus()? {
    ///     // Query and print GPU name (optional)
    ///     println!("GPU: {}", gpu.full_name()?);
    /// }
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn iter_physical_gpus(&self) -> crate::Result<GSyncGpuIter> {
        let (gpus, _displays) = self.get_topology()?;
        Ok(GSyncGpuIter { inner: gpus.into_iter() })
    }
}

/// Iterator over GPUs connected to a G-SYNC device.
pub struct GSyncGpuIter {
    inner: VecIntoIter<gsync::NV_GSYNC_GPU>,
}

impl Iterator for GSyncGpuIter {
    type Item = PhysicalGpu;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            if let Some(pg) = GSyncDevice::to_physical_gpu(&entry) {
                return Some(pg);
            }
        }
        None
    }
}
