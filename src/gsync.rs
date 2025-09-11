//! G-SYNC device helpers.
//!
//! This module provides a thin, safe wrapper around NVAPI's G-SYNC device
//! handle to enumerate G-SYNC modules present in the system and to query
//! their synchronization status for a given GPU.

use crate::sys::gsync::{self};
use crate::PhysicalGpu;
use log::trace;
use nvapi_sys::{handles, status_result, NVAPI_MAX_GSYNC_DEVICES};

/// A handle to an NVIDIA G-SYNC device.
///
/// This is a light wrapper around `nvapi_sys::handles::NvGSyncDeviceHandle` that
/// exposes convenient methods to enumerate devices and query their status.
///
/// See also: [`GSyncDevice::enum_sync_devices`].
#[derive(Debug)]
pub struct GSyncDevice {
    handle: handles::NvGSyncDeviceHandle,
}

impl GSyncDevice {
    /// Creates a new wrapper from a raw NVAPI G-SYNC device handle.
    ///
    /// Most callers should prefer [`GSyncDevice::enum_sync_devices`] to obtain
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
    /// let devices = GSyncDevice::enum_sync_devices()?;
    /// println!("found {} G-SYNC device(s)", devices.len());
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn enum_sync_devices() -> crate::Result<Vec<GSyncDevice>> {
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

    /// Sets the sync state for the given displays (raw form).
    ///
    /// - `displays`: slice of NV_GSYNC_DISPLAY entries to synchronize.
    ///   Any displays not present will be un-synchronized by the driver.
    /// - `flags`: reserved, pass 0.
    ///
    /// Ensures each entry has the correct version value.
    pub fn set_sync_state_settings_raw(
        &self,
        displays: &mut [gsync::NV_GSYNC_DISPLAY],
        flags: u32,
    ) -> crate::Result<()> {
        trace!(
            "gsync.set_sync_state_settings_raw(count={}, flags={})",
            displays.len(),
            flags
        );
        for d in displays.iter_mut() {
            if d.version == 0 {
                d.version = gsync::NV_GSYNC_DISPLAY_VER;
            }
        }
        unsafe {
            status_result(gsync::NvAPI_GSync_SetSyncStateSettings(
                displays.len() as u32,
                displays.as_ptr(),
                flags,
            ))
        }
    }

    /// Sets the sync state for the given display IDs with the desired state (convenience form).
    ///
    /// Example:
    ///   dev.set_sync_state_settings(&[(id1, gsync::DisplaySyncState::Master), (id2, gsync::DisplaySyncState::Slave)], 0)?;
    pub fn set_sync_state_settings<I>(&self, displays: I, flags: u32) -> crate::Result<()>
    where
        I: IntoIterator<Item = (u32, gsync::DisplaySyncState)>,
    {
        let mut buf: Vec<gsync::NV_GSYNC_DISPLAY> = Vec::new();
        for (display_id, state) in displays {
            let mut e = gsync::NV_GSYNC_DISPLAY::zeroed();
            e.version = gsync::NV_GSYNC_DISPLAY_VER;
            e.displayId = display_id;
            e.syncState = state.raw();
            // isMasterable is a bit-field exposed as u32; leave as-is (driver owned capability)
            buf.push(e);
        }
        self.set_sync_state_settings_raw(&mut buf, flags)
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

    /// Sets control parameters on this G-SYNC device.
    ///
    /// The provided buffer will be updated by NVAPI with the applied values
    /// (e.g., adjusted delays). Ensure `version` is set to `NV_GSYNC_CONTROL_PARAMS_VER`.
    pub fn set_control_parameters(
        &self,
        params: &mut gsync::NV_GSYNC_CONTROL_PARAMS,
    ) -> crate::Result<gsync::NV_GSYNC_CONTROL_PARAMS> {
        trace!("gsync.set_control_parameters()");
        if params.version == 0 {
            params.version = gsync::NV_GSYNC_CONTROL_PARAMS_VER;
        }
        match unsafe { gsync::NvAPI_GSync_SetControlParameters(*self.handle(), params) } {
            ret => status_result(ret).map(|_| *params),
        }
    }

    /// Adjusts skew or startup delay to the closest possible values.
    ///
    /// Returns the delay in unit steps if provided by the driver.
    pub fn adjust_sync_delay(
        &self,
        delay_type: gsync::NVAPI_GSYNC_DELAY_TYPE,
        delay: &mut gsync::NV_GSYNC_DELAY,
    ) -> crate::Result<Option<u32>> {
        trace!("gsync.adjust_sync_delay({:?})", delay_type);
        let mut steps: u32 = 0;
        let steps_ptr: *mut u32 = &mut steps;
        match unsafe {
            gsync::NvAPI_GSync_AdjustSyncDelay(*self.handle(), delay_type, delay, steps_ptr)
        } {
            ret => status_result(ret).map(|_| Some(steps)),
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
    /// let dev = GSyncDevice::enum_sync_devices()?.into_iter().next().expect("no G-SYNC device found");
    /// let status = dev.get_sync_status(gpu)?;
    /// println!("status: {:?}", status);
    /// # Ok::<_, nvapi::Status>(())
    /// ```
    pub fn get_sync_status(&self, gpu: &PhysicalGpu) -> crate::Result<gsync::NV_GSYNC_STATUS> {
        let mut status = gsync::NV_GSYNC_STATUS::default();
        status.version = gsync::NV_GSYNC_STATUS_VER;
        match unsafe {
            gsync::NvAPI_GSync_GetSyncStatus(*self.handle(), *gpu.handle(), &mut status)
        } {
            ret => status_result(ret).map(|_| status),
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

    // /// Queries extended status parameters (V2) of this G-SYNC device.
    // ///
    // /// This opts into the larger NV_GSYNC_STATUS_PARAMS_V2 struct. Some drivers
    // /// only support V1 and will return `Status::IncompatibleStructVersion`.
    // ///
    // /// Wraps `NvAPI_GSync_GetStatusParameters` with a V2 buffer.
    // TODO: Decide if this is needed.
    // pub fn get_status_parameters_v2(&self) -> crate::Result<gsync::NV_GSYNC_STATUS_PARAMS_V2> {
    //     trace!("gsync.get_status_parameters_v2()");
    //     let mut params2 = gsync::NV_GSYNC_STATUS_PARAMS_V2::zeroed();
    //     params2.version = gsync::NV_GSYNC_STATUS_PARAMS_VER_2;
    //     let ret = unsafe {
    //         // Call the same NVAPI entry point but pass a V2 buffer by casting to the
    //         // aliased parameter type expected by our FFI (currently V1). NVAPI uses
    //         // the version field to determine the actual layout.
    //         gsync::NvAPI_GSync_GetStatusParameters(
    //             *self.handle(),
    //             &mut params2 as *mut _ as *mut gsync::NV_GSYNC_STATUS_PARAMS,
    //         )
    //     };
    //     status_result(ret).map(|_| params2)
    // }

    /// Re-applies the current sync state using a displays slice from get_topology().
    /// Useful for a no-op validation of NvAPI_GSync_SetSyncStateSettings or resyncing after reboots, 
    /// as that sometimes clears the saved sync state.
    pub fn set_sync_state_settings_from_topology(
        &self,
        displays: &[gsync::NV_GSYNC_DISPLAY],
        flags: u32,
    ) -> crate::Result<()> {
        // Build minimal entries (id + state); driver-owned fields are left untouched.
        let mut buf: Vec<gsync::NV_GSYNC_DISPLAY> = Vec::with_capacity(displays.len());
        for d in displays {
            let mut e = gsync::NV_GSYNC_DISPLAY::zeroed();
            e.version = gsync::NV_GSYNC_DISPLAY_VER;
            e.displayId = d.displayId;
            e.syncState = d.syncState;
            buf.push(e);
        }
        self.set_sync_state_settings_raw(&mut buf, flags)
    }

    /// Retrieves the physical GPUs connected to this G-SYNC device.
    pub fn get_physical_gpus(&self) -> crate::Result<Vec<PhysicalGpu>> {
        let (gpus, _displays) = self.get_topology()?;
        let mut phys_gpus: Vec<crate::PhysicalGpu> = Vec::new();
        for gpu in gpus.iter() {
            phys_gpus.push(PhysicalGpu::from(gpu));
        }
        Ok(phys_gpus)
    }
}
