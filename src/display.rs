//! Display configuration helpers.
//!
//! Safe wrapper around `NvAPI_DISP_GetDisplayConfig` / `NvAPI_DISP_SetDisplayConfig`, which
//! read and write the *complete* desktop configuration: which displays are active, at which
//! resolution, refresh rate, position and rotation, and which one is the GDI primary.
//!
//! The raw NVAPI structures form a pointer graph that has to be allocated by the caller. This
//! module converts between that graph and an owned, pointer-free Rust representation.

use crate::sys::dispcontrol as raw;
use crate::sys::{self, status_result};
use log::trace;
use std::ptr;

pub use crate::sys::dispcontrol::*;

/// Optional per-target (per-display) settings of a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetDetails {
    pub rotation: Rotate,
    pub scaling: Scaling,
    /// Refresh rate in mHz (Hz × 1000). `0` lets the driver pick.
    pub refresh_rate_1k: u32,
    pub interlaced: bool,
    /// Primary target *within a clone group* — not the GDI primary.
    pub clone_primary: bool,
    pub preferred_unscaled: bool,
}

impl Default for TargetDetails {
    fn default() -> Self {
        Self {
            rotation: Rotate::R0,
            scaling: Scaling::Default,
            refresh_rate_1k: 0,
            interlaced: false,
            clone_primary: false,
            preferred_unscaled: false,
        }
    }
}

/// A display driven by a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TargetInfo {
    pub display_id: u32,
    /// Windows CCD target id; only meaningful for non-NVIDIA adapters.
    pub target_id: u32,
    /// `None` leaves rotation, scaling and refresh rate to the driver. Displays that are being
    /// activated should generally use `None`.
    pub details: Option<TargetDetails>,
}

impl TargetInfo {
    pub fn new(display_id: u32) -> Self {
        Self {
            display_id,
            ..Default::default()
        }
    }
}

/// The desktop surface backing a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceMode {
    pub width: u32,
    pub height: u32,
    pub color_depth: u32,
    pub x: i32,
    pub y: i32,
    pub gdi_primary: bool,
    pub sli_focus: bool,
}

/// One source (desktop surface) plus the displays it drives.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PathInfo {
    /// Windows CCD source id. Leave every path at `0` to let NVAPI assign them.
    pub source_id: u32,
    pub targets: Vec<TargetInfo>,
    pub source_mode: Option<SourceMode>,
    pub non_nvidia_adapter: bool,
}

/// The complete desktop configuration.
///
/// A display that is not referenced by any path is inactive. Applying a configuration therefore
/// deactivates every display that was left out of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DisplayConfig {
    pub paths: Vec<PathInfo>,
}

impl DisplayConfig {
    /// Reads the current global display configuration.
    ///
    /// Returns [`Status::DeviceBusy`](sys::Status::DeviceBusy) while a modeset is still in flight;
    /// callers should retry.
    pub fn get() -> sys::Result<Self> {
        trace!("display.get()");
        unsafe { get_display_config() }
    }

    /// Applies this configuration. `flags` is a bitwise OR of the `NV_DISPLAYCONFIG_*` constants.
    pub fn apply(&self, flags: u32) -> sys::Result<()> {
        trace!("display.apply({} paths, flags = {:#x})", self.paths.len(), flags);
        let mut raw = RawConfig::build(self);
        unsafe {
            status_result(raw::NvAPI_DISP_SetDisplayConfig(
                raw.paths.len() as u32,
                raw.paths.as_mut_ptr(),
                flags,
            ))
        }
    }

    /// Asks the driver whether this configuration could be applied, without applying it.
    pub fn validate(&self) -> sys::Result<()> {
        self.apply(NV_DISPLAYCONFIG_VALIDATE_ONLY)
    }

    /// Display IDs of every target in the configuration, i.e. every active display.
    pub fn display_ids(&self) -> Vec<u32> {
        self.paths
            .iter()
            .flat_map(|p| p.targets.iter().map(|t| t.display_id))
            .collect()
    }

    /// Finds the path index and target index of `display_id`.
    pub fn find(&self, display_id: u32) -> Option<(usize, usize)> {
        self.paths.iter().enumerate().find_map(|(pi, path)| {
            path.targets
                .iter()
                .position(|t| t.display_id == display_id)
                .map(|ti| (pi, ti))
        })
    }

    /// Display ID of the path marked as GDI primary, if any.
    pub fn gdi_primary(&self) -> Option<u32> {
        self.paths
            .iter()
            .find(|p| p.source_mode.is_some_and(|m| m.gdi_primary))
            .and_then(|p| p.targets.first())
            .map(|t| t.display_id)
    }
}

/// Returns the display ID of the current GDI primary display.
pub fn gdi_primary_display_id() -> sys::Result<u32> {
    trace!("display.gdi_primary_display_id()");
    let mut display_id = 0u32;
    unsafe {
        status_result(raw::NvAPI_DISP_GetGDIPrimaryDisplayId(&mut display_id)).map(|_| display_id)
    }
}

/// Resolves a Windows GDI device name (`\\.\DISPLAY1`) to its NVAPI display ID.
///
/// Only works for displays that are currently active.
pub fn display_id_by_gdi_name(name: &str) -> sys::Result<u32> {
    trace!("display.display_id_by_gdi_name({})", name);
    let name = std::ffi::CString::new(name).map_err(|_| sys::Status::InvalidArgument)?;
    let mut display_id = 0u32;
    unsafe {
        status_result(sys::dispcontrol::NvAPI_DISP_GetDisplayIdByDisplayName(
            name.as_ptr(),
            &mut display_id,
        ))
        .map(|_| display_id)
    }
}

/// Resolves a display ID to the physical GPU driving it and its output ID bit mask.
pub fn gpu_and_output_id(display_id: u32) -> sys::Result<(sys::handles::NvPhysicalGpuHandle, u32)> {
    trace!("display.gpu_and_output_id({})", display_id);
    let mut handle = sys::handles::NvPhysicalGpuHandle::default();
    let mut output_id = 0u32;
    unsafe {
        status_result(sys::dispcontrol::NvAPI_SYS_GetGpuAndOutputIdFromDisplayId(
            display_id,
            &mut handle,
            &mut output_id,
        ))
        .map(|_| (handle, output_id))
    }
}

/// Three-pass read documented at `NvAPI_DISP_GetDisplayConfig`.
unsafe fn get_display_config() -> sys::Result<DisplayConfig> {
    let mut count = 0u32;
    status_result(unsafe { raw::NvAPI_DISP_GetDisplayConfig(&mut count, ptr::null_mut()) })?;
    if count == 0 {
        return Ok(DisplayConfig::default());
    }

    // Pass 2: learn how many targets each path has.
    let mut path = raw::NV_DISPLAYCONFIG_PATH_INFO::zeroed();
    path.version = raw::NV_DISPLAYCONFIG_PATH_INFO_VER;
    let mut paths = vec![path; count as usize];
    status_result(unsafe { raw::NvAPI_DISP_GetDisplayConfig(&mut count, paths.as_mut_ptr()) })?;
    paths.truncate(count as usize);

    // Pass 3: provide the target/source buffers the driver fills in.
    let mut targets: Vec<Vec<raw::NV_DISPLAYCONFIG_PATH_TARGET_INFO>> =
        Vec::with_capacity(paths.len());
    let mut details: Vec<Vec<raw::NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO>> =
        Vec::with_capacity(paths.len());
    let mut modes = vec![raw::NV_DISPLAYCONFIG_SOURCE_MODE_INFO::zeroed(); paths.len()];

    for p in &paths {
        let n = p.targetInfoCount as usize;
        targets.push(vec![raw::NV_DISPLAYCONFIG_PATH_TARGET_INFO::zeroed(); n]);

        let mut detail = raw::NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO::zeroed();
        detail.version = raw::NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_VER;
        details.push(vec![detail; n]);
    }

    for i in 0..paths.len() {
        for j in 0..targets[i].len() {
            targets[i][j].details = &mut details[i][j];
        }
        paths[i].targetInfo = targets[i].as_mut_ptr();
        paths[i].sourceModeInfo = &mut modes[i];
    }

    status_result(unsafe { raw::NvAPI_DISP_GetDisplayConfig(&mut count, paths.as_mut_ptr()) })?;

    let config = DisplayConfig {
        paths: paths
            .iter()
            .enumerate()
            .map(|(i, p)| PathInfo {
                source_id: p.sourceId,
                non_nvidia_adapter: p.flags & NV_DISPLAYCONFIG_PATH_IS_NON_NVIDIA_ADAPTER != 0,
                targets: targets[i]
                    .iter()
                    .enumerate()
                    .map(|(j, t)| TargetInfo {
                        display_id: t.displayId,
                        target_id: t.targetId,
                        details: Some(details_from_raw(&details[i][j])),
                    })
                    .collect(),
                source_mode: Some(source_mode_from_raw(&modes[i])),
            })
            .collect(),
    };

    Ok(config)
}

fn details_from_raw(raw: &raw::NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO) -> TargetDetails {
    TargetDetails {
        rotation: Rotate::from_raw(raw.rotation).unwrap_or(Rotate::R0),
        scaling: Scaling::from_raw(raw.scaling).unwrap_or(Scaling::Default),
        refresh_rate_1k: raw.refreshRate1K,
        interlaced: raw.flags & NV_DISPLAYCONFIG_TARGET_INTERLACED != 0,
        clone_primary: raw.flags & NV_DISPLAYCONFIG_TARGET_PRIMARY != 0,
        preferred_unscaled: raw.flags & NV_DISPLAYCONFIG_TARGET_IS_PREFERRED_UNSCALED != 0,
    }
}

fn source_mode_from_raw(raw: &raw::NV_DISPLAYCONFIG_SOURCE_MODE_INFO) -> SourceMode {
    SourceMode {
        width: raw.resolution.width,
        height: raw.resolution.height,
        color_depth: raw.resolution.colorDepth,
        x: raw.position.x,
        y: raw.position.y,
        gdi_primary: raw.flags & NV_DISPLAYCONFIG_SOURCE_GDI_PRIMARY != 0,
        sli_focus: raw.flags & NV_DISPLAYCONFIG_SOURCE_SLI_FOCUS != 0,
    }
}

/// Owns every allocation the raw pointer graph refers to for the duration of an NVAPI call.
///
/// Moving this struct only moves the `Vec` headers, never the heap buffers the wired-up pointers
/// refer to, so it is safe to build it and hand it to NVAPI afterwards.
struct RawConfig {
    paths: Vec<raw::NV_DISPLAYCONFIG_PATH_INFO>,
    #[allow(dead_code)]
    targets: Vec<Vec<raw::NV_DISPLAYCONFIG_PATH_TARGET_INFO>>,
    #[allow(dead_code)]
    details: Vec<Vec<raw::NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO>>,
    #[allow(dead_code)]
    modes: Vec<raw::NV_DISPLAYCONFIG_SOURCE_MODE_INFO>,
}

impl RawConfig {
    fn build(config: &DisplayConfig) -> Self {
        let len = config.paths.len();
        let mut paths = Vec::with_capacity(len);
        let mut targets = Vec::with_capacity(len);
        let mut details = Vec::with_capacity(len);
        let mut modes = Vec::with_capacity(len);

        for path in &config.paths {
            let mut raw_path = raw::NV_DISPLAYCONFIG_PATH_INFO::zeroed();
            raw_path.version = raw::NV_DISPLAYCONFIG_PATH_INFO_VER;
            raw_path.sourceId = path.source_id;
            raw_path.targetInfoCount = path.targets.len() as u32;
            if path.non_nvidia_adapter {
                raw_path.flags |= NV_DISPLAYCONFIG_PATH_IS_NON_NVIDIA_ADAPTER;
            }
            paths.push(raw_path);

            let mut raw_targets = Vec::with_capacity(path.targets.len());
            let mut raw_details = Vec::with_capacity(path.targets.len());
            for target in &path.targets {
                let mut raw_target = raw::NV_DISPLAYCONFIG_PATH_TARGET_INFO::zeroed();
                raw_target.displayId = target.display_id;
                raw_target.targetId = target.target_id;
                raw_targets.push(raw_target);

                let d = target.details.unwrap_or_default();
                let mut raw_detail = raw::NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO::zeroed();
                raw_detail.version = raw::NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_VER;
                raw_detail.rotation = d.rotation.raw() as NV_ROTATE;
                raw_detail.scaling = d.scaling.raw();
                raw_detail.refreshRate1K = d.refresh_rate_1k;
                raw_detail.connector = NV_GPU_CONNECTOR_UNKNOWN;
                raw_detail.tvFormat = NV_DISPLAY_TV_FORMAT_NONE;
                // The headers require AUTO here; CURRENT is a get-only value and has no meaning
                // for a display that is not driving a timing yet.
                raw_detail.timingOverride = NV_TIMING_OVERRIDE_AUTO;
                if d.interlaced {
                    raw_detail.flags |= NV_DISPLAYCONFIG_TARGET_INTERLACED;
                }
                if d.clone_primary {
                    raw_detail.flags |= NV_DISPLAYCONFIG_TARGET_PRIMARY;
                }
                if d.preferred_unscaled {
                    raw_detail.flags |= NV_DISPLAYCONFIG_TARGET_IS_PREFERRED_UNSCALED;
                }
                raw_details.push(raw_detail);
            }
            targets.push(raw_targets);
            details.push(raw_details);

            let mut raw_mode = raw::NV_DISPLAYCONFIG_SOURCE_MODE_INFO::zeroed();
            if let Some(mode) = path.source_mode {
                raw_mode.resolution.width = mode.width;
                raw_mode.resolution.height = mode.height;
                raw_mode.resolution.colorDepth = mode.color_depth;
                raw_mode.colorFormat = NV_FORMAT_UNKNOWN;
                raw_mode.position.x = mode.x;
                raw_mode.position.y = mode.y;
                raw_mode.spanningOrientation = NV_DISPLAYCONFIG_SPAN_NONE;
                if mode.gdi_primary {
                    raw_mode.flags |= NV_DISPLAYCONFIG_SOURCE_GDI_PRIMARY;
                }
                if mode.sli_focus {
                    raw_mode.flags |= NV_DISPLAYCONFIG_SOURCE_SLI_FOCUS;
                }
            }
            modes.push(raw_mode);
        }

        let mut this = RawConfig {
            paths,
            targets,
            details,
            modes,
        };

        for i in 0..this.paths.len() {
            for j in 0..this.targets[i].len() {
                // NVAPI expects a null pointer rather than a zeroed struct when a target carries
                // no advanced settings, which is the case for displays being activated.
                this.targets[i][j].details = if config.paths[i].targets[j].details.is_some() {
                    &mut this.details[i][j]
                } else {
                    ptr::null_mut()
                };
            }
            this.paths[i].targetInfo = this.targets[i].as_mut_ptr();
            this.paths[i].sourceModeInfo = if config.paths[i].source_mode.is_some() {
                &mut this.modes[i]
            } else {
                ptr::null_mut()
            };
        }

        this
    }
}
