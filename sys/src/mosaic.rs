// NVIDIA Mosaic: types, enums, and constants
// Based on NVAPI Reference Documentation (Release 560, Aug 5 2024)
// https://docs.nvidia.com/gameworks/content/gameworkslibrary/coresdk/nvapi/group__mosaicapi.html

use crate::handles::{NvLogicalGpuHandle, NvPhysicalGpuHandle};
use crate::dispcontrol::NV_ROTATE;
use crate::types::NVAPI_MAX_DISPLAYS;

// ---- Macros/constants (as Rust consts) ----

pub const NVAPI_MAX_MOSAIC_DISPLAY_ROWS: usize = 8;
pub const NVAPI_MAX_MOSAIC_DISPLAY_COLUMNS: usize = 8;
pub const NVAPI_MAX_MOSAIC_TOPOS: usize = 16;
pub const NV_MOSAIC_TOPO_BRIEFS_MAX: usize = 35; // max number of topo briefs (enum sentinel value)

pub const NV_MOSAIC_TOPO_VALIDITY_VALID: u32 = 0x0000_0000;
pub const NV_MOSAIC_TOPO_VALIDITY_MISSING_GPU: u32 = 0x0000_0001;
pub const NV_MOSAIC_TOPO_VALIDITY_MISSING_DISPLAY: u32 = 0x0000_0002;
pub const NV_MOSAIC_TOPO_VALIDITY_MIXED_DISPLAY_TYPES: u32 = 0x0000_0004;

pub const NV_MOSAIC_DISPLAY_SETTINGS_MAX: usize = 40;

pub const NV_MOSAIC_TOPO_IDX_DEFAULT: u32 = 0;
pub const NV_MOSAIC_TOPO_IDX_LEFT_EYE: u32 = 0;
pub const NV_MOSAIC_TOPO_IDX_RIGHT_EYE: u32 = 1;
pub const NV_MOSAIC_TOPO_NUM_EYES: u32 = 2;

pub const NV_MOSAIC_MAX_TOPO_PER_TOPO_GROUP: usize = 2;
// Alias: NV_MOSAIC_MAX_DISPLAYS is used by some grid topo structs; map to NVAPI_MAX_DISPLAYS
pub const NV_MOSAIC_MAX_DISPLAYS: usize = NVAPI_MAX_DISPLAYS;

// Grid Topology flags stored in NV_MOSAIC_GRID_TOPO_V1/V2.gridFlags (C bitfields)
pub const NV_MOSAIC_GRID_TOPO_FLAG_APPLY_WITH_BEZEL_CORRECT: u32 = 1 << 0;
pub const NV_MOSAIC_GRID_TOPO_FLAG_IMMERSIVE_GAMING: u32 = 1 << 1;
pub const NV_MOSAIC_GRID_TOPO_FLAG_BASE_MOSAIC: u32 = 1 << 2;
pub const NV_MOSAIC_GRID_TOPO_FLAG_DRIVER_RELOAD_ALLOWED: u32 = 1 << 3;
pub const NV_MOSAIC_GRID_TOPO_FLAG_ACCELERATE_PRIMARY_DISPLAY: u32 = 1 << 4;
// Present in V2 only
pub const NV_MOSAIC_GRID_TOPO_FLAG_PIXEL_SHIFT: u32 = 1 << 5;

// Display topology warnings bit flags
pub const NV_MOSAIC_DISPLAYTOPO_WARNING_DISPLAY_POSITION: u32 = 1 << 0; // NV_BIT(0)
pub const NV_MOSAIC_DISPLAYTOPO_WARNING_DRIVER_RELOAD_REQUIRED: u32 = 1 << 1; // NV_BIT(1)

// Flags for NvAPI_Mosaic_SetDisplayGrids and NvAPI_Mosaic_ValidateDisplayGrids setTopoFlags
// Source: https://github.com/NVIDIA/nvapi/blob/3d34a4faf095996663321646ebe003539a908f89/nvapi.h#L10195
pub const NV_MOSAIC_SETDISPLAYTOPO_FLAG_CURRENT_GPU_TOPOLOGY: u32 = 1 << 0;
pub const NV_MOSAIC_SETDISPLAYTOPO_FLAG_NO_DRIVER_RELOAD: u32 = 1 << 1;
pub const NV_MOSAIC_SETDISPLAYTOPO_FLAG_MAXIMIZE_PERFORMANCE: u32 = 1 << 2;
pub const NV_MOSAIC_SETDISPLAYTOPO_FLAG_ALLOW_INVALID: u32 = 1 << 3;

// ---- Enums ----

nvenum! {
	/// NV_MOSAIC_TOPO_TYPE: Types/categories of Mosaic topologies.
	pub enum NV_MOSAIC_TOPO_TYPE / MosaicTopoType {
		NV_MOSAIC_TOPO_TYPE_ALL / All = 0,
		NV_MOSAIC_TOPO_TYPE_BASIC / Basic = 1,
		NV_MOSAIC_TOPO_TYPE_PASSIVE_STEREO / PassiveStereo = 2,
		NV_MOSAIC_TOPO_TYPE_SCALED_CLONE / ScaledClone = 3,
		NV_MOSAIC_TOPO_TYPE_PASSIVE_STEREO_SCALED_CLONE / PassiveStereoScaledClone = 4,
		NV_MOSAIC_TOPO_TYPE_MAX / Max = 5,
	}
}

// The detailed numeric mapping of NV_MOSAIC_TOPO uses ranges and computed values in the docs.
// We map them explicitly in order, relying on the C enum layout.
nvenum! {
	/// NV_MOSAIC_TOPO: List of supported Mosaic topologies.
	pub enum NV_MOSAIC_TOPO / MosaicTopo {
		NV_MOSAIC_TOPO_NONE / None = 0,
		NV_MOSAIC_TOPO_1X2_BASIC / T1x2_Basic = 1,
		NV_MOSAIC_TOPO_2X1_BASIC / T2x1_Basic = 2,
		NV_MOSAIC_TOPO_1X3_BASIC / T1x3_Basic = 3,
		NV_MOSAIC_TOPO_3X1_BASIC / T3x1_Basic = 4,
		NV_MOSAIC_TOPO_1X4_BASIC / T1x4_Basic = 5,
		NV_MOSAIC_TOPO_4X1_BASIC / T4x1_Basic = 6,
		NV_MOSAIC_TOPO_2X2_BASIC / T2x2_Basic = 7,
		NV_MOSAIC_TOPO_2X3_BASIC / T2x3_Basic = 8,
		NV_MOSAIC_TOPO_2X4_BASIC / T2x4_Basic = 9,
		NV_MOSAIC_TOPO_3X2_BASIC / T3x2_Basic = 10,
		NV_MOSAIC_TOPO_4X2_BASIC / T4x2_Basic = 11,
		NV_MOSAIC_TOPO_1X5_BASIC / T1x5_Basic = 12,
		NV_MOSAIC_TOPO_1X6_BASIC / T1x6_Basic = 13,
		NV_MOSAIC_TOPO_7X1_BASIC / T7x1_Basic = 14,
		NV_MOSAIC_TOPO_1X2_PASSIVE_STEREO / T1x2_PassiveStereo = 24,
		NV_MOSAIC_TOPO_2X1_PASSIVE_STEREO / T2x1_PassiveStereo = 25,
		NV_MOSAIC_TOPO_1X3_PASSIVE_STEREO / T1x3_PassiveStereo = 26,
		NV_MOSAIC_TOPO_3X1_PASSIVE_STEREO / T3x1_PassiveStereo = 27,
		NV_MOSAIC_TOPO_1X4_PASSIVE_STEREO / T1x4_PassiveStereo = 28,
		NV_MOSAIC_TOPO_4X1_PASSIVE_STEREO / T4x1_PassiveStereo = 29,
		NV_MOSAIC_TOPO_2X2_PASSIVE_STEREO / T2x2_PassiveStereo = 30,
	NV_MOSAIC_TOPO_MAX / Max = 35,
	}
}

// ---- Structs and typed aliases ----
// NV_ROTATE is defined in the display control module

// Common item used in multiple structs: GPU layout element
nvstruct! {
	pub struct NV_MOSAIC_GPU_LAYOUT_ELEM {
		pub hPhysicalGPU: NvPhysicalGpuHandle,
		pub displayOutputId: u32,
		pub overlapX: i32,
		pub overlapY: i32,
	}
}

nvstruct! {
	/// NV_MOSAIC_TOPO_BRIEF — brief description including topo id and flags
	pub struct NV_MOSAIC_TOPO_BRIEF {
		pub version: u32,
		pub topo: NV_MOSAIC_TOPO,
		pub enabled: u32,
		pub isPossible: u32,
	}
}
const NV_MOSAIC_TOPO_BRIEF_SIZE: usize = std::mem::size_of::<NV_MOSAIC_TOPO_BRIEF>();
nvversion! { NVAPI_MOSAIC_TOPO_BRIEF_VER(NV_MOSAIC_TOPO_BRIEF = NV_MOSAIC_TOPO_BRIEF_SIZE, 1) }

nvstruct! {
	/// NV_MOSAIC_TOPO_DETAILS — detailed topology description
	pub struct NV_MOSAIC_TOPO_DETAILS {
		pub version: u32,
		pub hLogicalGPU: NvLogicalGpuHandle,
		pub validityMask: u32,
		pub rowCount: u32,
		pub colCount: u32,
		pub gpuLayout: [[NV_MOSAIC_GPU_LAYOUT_ELEM; NVAPI_MAX_MOSAIC_DISPLAY_COLUMNS]; NVAPI_MAX_MOSAIC_DISPLAY_ROWS],
	}
}
const NV_MOSAIC_TOPO_DETAILS_SIZE: usize = std::mem::size_of::<NV_MOSAIC_TOPO_DETAILS>();
nvversion! { NVAPI_MOSAIC_TOPO_DETAILS_VER(NV_MOSAIC_TOPO_DETAILS = NV_MOSAIC_TOPO_DETAILS_SIZE, 1) }

nvstruct! {
	pub struct _NV_MOSAIC_DISPLAY_SETTING_V1 {
		pub version: u32,
		pub width: u32,
		pub height: u32,
		pub bpp: u32,
		pub freq: u32,
	}
}
pub type NV_MOSAIC_DISPLAY_SETTING_V1 = _NV_MOSAIC_DISPLAY_SETTING_V1;
const NV_MOSAIC_DISPLAY_SETTING_V1_SIZE: usize = std::mem::size_of::<NV_MOSAIC_DISPLAY_SETTING_V1>();
nvversion! { NVAPI_MOSAIC_DISPLAY_SETTING_VER1(NV_MOSAIC_DISPLAY_SETTING_V1 = NV_MOSAIC_DISPLAY_SETTING_V1_SIZE, 1) }

nvstruct! {
	pub struct NV_MOSAIC_DISPLAY_SETTING_V2 {
		pub version: u32,
		pub width: u32,
		pub height: u32,
		pub bpp: u32,
		pub freq: u32,
		pub rrx1k: u32,
	}
}
const NV_MOSAIC_DISPLAY_SETTING_V2_SIZE: usize = std::mem::size_of::<NV_MOSAIC_DISPLAY_SETTING_V2>();
nvversion! { NVAPI_MOSAIC_DISPLAY_SETTING_VER2(NV_MOSAIC_DISPLAY_SETTING_V2 = NV_MOSAIC_DISPLAY_SETTING_V2_SIZE, 2) }
nvversion! { NVAPI_MOSAIC_DISPLAY_SETTING_VER = NVAPI_MOSAIC_DISPLAY_SETTING_VER2 }
pub type NV_MOSAIC_DISPLAY_SETTING = NV_MOSAIC_DISPLAY_SETTING_V2;

nvstruct! {
	pub struct _NV_MOSAIC_SUPPORTED_TOPO_INFO_V1 {
		pub version: u32,
		pub topoBriefsCount: u32,
	pub topoBriefs: [NV_MOSAIC_TOPO_BRIEF; NV_MOSAIC_TOPO_BRIEFS_MAX],
		pub displaySettingsCount: u32,
		pub displaySettings: [NV_MOSAIC_DISPLAY_SETTING_V1; NV_MOSAIC_DISPLAY_SETTINGS_MAX],
	}
}
pub type NV_MOSAIC_SUPPORTED_TOPO_INFO_V1 = _NV_MOSAIC_SUPPORTED_TOPO_INFO_V1;
const NV_MOSAIC_SUPPORTED_TOPO_INFO_V1_SIZE: usize = std::mem::size_of::<NV_MOSAIC_SUPPORTED_TOPO_INFO_V1>();
nvversion! { NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER1(NV_MOSAIC_SUPPORTED_TOPO_INFO_V1 = NV_MOSAIC_SUPPORTED_TOPO_INFO_V1_SIZE, 1) }

nvstruct! {
	pub struct _NV_MOSAIC_SUPPORTED_TOPO_INFO_V2 {
		pub version: u32,
		pub topoBriefsCount: u32,
	pub topoBriefs: [NV_MOSAIC_TOPO_BRIEF; NV_MOSAIC_TOPO_BRIEFS_MAX],
		pub displaySettingsCount: u32,
		pub displaySettings: [NV_MOSAIC_DISPLAY_SETTING_V2; NV_MOSAIC_DISPLAY_SETTINGS_MAX],
	}
}
pub type NV_MOSAIC_SUPPORTED_TOPO_INFO_V2 = _NV_MOSAIC_SUPPORTED_TOPO_INFO_V2;
const NV_MOSAIC_SUPPORTED_TOPO_INFO_V2_SIZE: usize = std::mem::size_of::<NV_MOSAIC_SUPPORTED_TOPO_INFO_V2>();
nvversion! { NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER2(NV_MOSAIC_SUPPORTED_TOPO_INFO_V2 = NV_MOSAIC_SUPPORTED_TOPO_INFO_V2_SIZE, 2) }
nvversion! { NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER = NVAPI_MOSAIC_SUPPORTED_TOPO_INFO_VER2 }
pub type NV_MOSAIC_SUPPORTED_TOPO_INFO = NV_MOSAIC_SUPPORTED_TOPO_INFO_V2;

nvstruct! {
	pub struct NV_MOSAIC_TOPO_GROUP {
		pub version: u32,
		pub brief: NV_MOSAIC_TOPO_BRIEF,
		pub count: u32,
		pub topos: [NV_MOSAIC_TOPO_DETAILS; NV_MOSAIC_MAX_TOPO_PER_TOPO_GROUP],
	}
}
const NV_MOSAIC_TOPO_GROUP_SIZE: usize = std::mem::size_of::<NV_MOSAIC_TOPO_GROUP>();
nvversion! { NVAPI_MOSAIC_TOPO_GROUP_VER(NV_MOSAIC_TOPO_GROUP = NV_MOSAIC_TOPO_GROUP_SIZE, 1) }

nvstruct! {
	pub struct _NV_MOSAIC_GRID_TOPO_DISPLAY_V1 {
		pub displayId: u32,
		pub overlapX: i32,
		pub overlapY: i32,
		pub rotation: NV_ROTATE,
		pub cloneGroup: u32,
	}
}
pub type NV_MOSAIC_GRID_TOPO_DISPLAY_V1 = _NV_MOSAIC_GRID_TOPO_DISPLAY_V1;
// Default typedef for display version
pub type NV_MOSAIC_GRID_TOPO_DISPLAY = NV_MOSAIC_GRID_TOPO_DISPLAY_V1;

// Pixel shift type used in V2 display struct
nvenum! {
	pub enum NV_PIXEL_SHIFT_TYPE / PixelShiftType {
		NV_PIXEL_SHIFT_TYPE_NO_PIXEL_SHIFT / NoPixelShift = 0,
		NV_PIXEL_SHIFT_TYPE_2X2_TOP_LEFT_PIXELS / TwoByTwoTopLeft = 1,
		NV_PIXEL_SHIFT_TYPE_2X2_BOTTOM_RIGHT_PIXELS / TwoByTwoBottomRight = 2,
		NV_PIXEL_SHIFT_TYPE_2X2_TOP_RIGHT_PIXELS / TwoByTwoTopRight = 4,
		NV_PIXEL_SHIFT_TYPE_2X2_BOTTOM_LEFT_PIXELS / TwoByTwoBottomLeft = 8,
	}
}

nvstruct! {
	pub struct NV_MOSAIC_GRID_TOPO_DISPLAY_V2 {
		pub version: u32,
		pub displayId: u32,
		pub overlapX: i32,
		pub overlapY: i32,
		pub rotation: NV_ROTATE,
		pub cloneGroup: u32,
		pub pixelShiftType: NV_PIXEL_SHIFT_TYPE,
	}
}

// Display topo status (warnings/errors bitmasks; opaque shell)
nvstruct! {
	pub struct NV_MOSAIC_DISPLAY_TOPO_STATUS {
		pub version: u32,
		pub errorFlags: u32,
		pub warningFlags: u32,
		pub displayCount: u32,
		pub displays: [NV_MOSAIC_DISPLAY_TOPO_STATUS_DISPLAY; NVAPI_MAX_DISPLAYS],
	}
}
const NV_MOSAIC_DISPLAY_TOPO_STATUS_SIZE: usize = std::mem::size_of::<NV_MOSAIC_DISPLAY_TOPO_STATUS>();
nvversion! { NV_MOSAIC_DISPLAY_TOPO_STATUS_VER(NV_MOSAIC_DISPLAY_TOPO_STATUS = NV_MOSAIC_DISPLAY_TOPO_STATUS_SIZE, 1) }

nvstruct! {
	pub struct NV_MOSAIC_DISPLAY_TOPO_STATUS_DISPLAY {
		pub displayId: u32,
		pub errorFlags: u32,
		pub warningFlags: u32,
		// C bitfield: supportsRotation:1, reserved:31 — represent as a single u32
		pub supportsRotation: u32,
	}
}

// Topology (opaque container)
nvstruct! {
	pub struct NV_MOSAIC_TOPOLOGY {
		pub version: u32,
		pub rowCount: u32,
		pub colCount: u32,
		pub gpuLayout: [[NV_MOSAIC_GPU_LAYOUT_ELEM; NVAPI_MAX_MOSAIC_DISPLAY_COLUMNS]; NVAPI_MAX_MOSAIC_DISPLAY_ROWS],
	}
}
const NV_MOSAIC_TOPOLOGY_SIZE: usize = std::mem::size_of::<NV_MOSAIC_TOPOLOGY>();
nvversion! { NVAPI_MOSAIC_TOPOLOGY_VER(NV_MOSAIC_TOPOLOGY = NV_MOSAIC_TOPOLOGY_SIZE, 1) }

// Supported topologies (opaque container)
nvstruct! {
	pub struct NV_MOSAIC_SUPPORTED_TOPOLOGIES {
		pub version: u32,
		pub totalCount: u32,
		pub topologies: [NV_MOSAIC_TOPOLOGY; NVAPI_MAX_MOSAIC_TOPOS],
	}
}
const NV_MOSAIC_SUPPORTED_TOPOLOGIES_SIZE: usize = std::mem::size_of::<NV_MOSAIC_SUPPORTED_TOPOLOGIES>();
nvversion! { NVAPI_MOSAIC_SUPPORTED_TOPOLOGIES_VER(NV_MOSAIC_SUPPORTED_TOPOLOGIES = NV_MOSAIC_SUPPORTED_TOPOLOGIES_SIZE, 1) }

// FFI Function bindings
use crate::NvAPI_Status;
use crate::types::NV_RECT;

// Grid topology structures
nvstruct! {
	pub struct NV_MOSAIC_GRID_TOPO_V1 {
		pub version: u32,
		pub rows: u32,
		pub columns: u32,
		pub displayCount: u32,
		// Bitfield in C: applyWithBezelCorrect:1, immersiveGaming:1, baseMosaic:1, driverReloadAllowed:1, acceleratePrimaryDisplay:1, reserved:27
		pub gridFlags: u32,
		pub displays: [NV_MOSAIC_GRID_TOPO_DISPLAY_V1; NV_MOSAIC_MAX_DISPLAYS],
		pub displaySettings: NV_MOSAIC_DISPLAY_SETTING_V1,
	}
}
const NV_MOSAIC_GRID_TOPO_V1_SIZE: usize = std::mem::size_of::<NV_MOSAIC_GRID_TOPO_V1>();
nvversion! { NV_MOSAIC_GRID_TOPO_VER1(NV_MOSAIC_GRID_TOPO_V1 = NV_MOSAIC_GRID_TOPO_V1_SIZE, 1) }

nvstruct! {
	pub struct NV_MOSAIC_GRID_TOPO_V2 {
		pub version: u32,
		pub rows: u32,
		pub columns: u32,
		pub displayCount: u32,
		// Bitfield in C adds pixelShift:1, reserved:26
		pub gridFlags: u32,
		pub displays: [NV_MOSAIC_GRID_TOPO_DISPLAY_V2; NV_MOSAIC_MAX_DISPLAYS],
		pub displaySettings: NV_MOSAIC_DISPLAY_SETTING_V1,
	}
}
const NV_MOSAIC_GRID_TOPO_V2_SIZE: usize = std::mem::size_of::<NV_MOSAIC_GRID_TOPO_V2>();
nvversion! { NV_MOSAIC_GRID_TOPO_VER2(NV_MOSAIC_GRID_TOPO_V2 = NV_MOSAIC_GRID_TOPO_V2_SIZE, 2) }
nvversion! { NV_MOSAIC_GRID_TOPO_VER = NV_MOSAIC_GRID_TOPO_VER2 }
pub type NV_MOSAIC_GRID_TOPO = NV_MOSAIC_GRID_TOPO_V2;

// Helpers for manipulating gridFlags
impl NV_MOSAIC_GRID_TOPO_V1 {
	#[inline]
	pub fn has_flag(&self, flag: u32) -> bool { (self.gridFlags & flag) != 0 }
	#[inline]
	pub fn set_flag(&mut self, flag: u32, enabled: bool) {
		if enabled { self.gridFlags |= flag; } else { self.gridFlags &= !flag; }
	}
}

impl NV_MOSAIC_GRID_TOPO_V2 {
	#[inline]
	pub fn has_flag(&self, flag: u32) -> bool { (self.gridFlags & flag) != 0 }
	#[inline]
	pub fn set_flag(&mut self, flag: u32, enabled: bool) {
		if enabled { self.gridFlags |= flag; } else { self.gridFlags &= !flag; }
	}
}

nvapi_fn! {
	pub type EnableCurrentMosaicTopologyFn = extern "C" fn(enable: u32) -> NvAPI_Status;
	pub unsafe fn NvAPI_EnableCurrentMosaicTopology;
}

nvapi_fn! {
	pub type GetCurrentMosaicTopologyFn = extern "C" fn(pMosaicTopo: *mut NV_MOSAIC_TOPOLOGY, pEnabled: *mut u32) -> NvAPI_Status;
	pub unsafe fn NvAPI_GetCurrentMosaicTopology;
}

nvapi_fn! {
	pub type GetSupportedMosaicTopologiesFn = extern "C" fn(pMosaicTopos: *mut NV_MOSAIC_SUPPORTED_TOPOLOGIES) -> NvAPI_Status;
	pub unsafe fn NvAPI_GetSupportedMosaicTopologies;
}

nvapi_fn! {
	pub type Mosaic_EnableCurrentTopoFn = extern "C" fn(enable: u32) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_EnableCurrentTopo;
}

nvapi_fn! {
	pub type Mosaic_GetCurrentTopoFn = extern "C" fn(pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF, pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING, pOverlapX: *mut i32, pOverlapY: *mut i32) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_GetCurrentTopo;
}

nvapi_fn! {
	pub type Mosaic_GetDisplayViewportsByResolutionFn = extern "C" fn(displayId: u32, srcWidth: u32, srcHeight: u32, viewports: *mut NV_RECT, bezelCorrected: *mut u8) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_GetDisplayViewportsByResolution;
}

nvapi_fn! {
	pub type Mosaic_GetOverlapLimitsFn = extern "C" fn(pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF, pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING, pMinOverlapX: *mut i32, pMaxOverlapX: *mut i32, pMinOverlapY: *mut i32, pMaxOverlapY: *mut i32) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_GetOverlapLimits;
}

nvapi_fn! {
	pub type Mosaic_GetSupportedTopoInfoFn = extern "C" fn(pSupportedTopoInfo: *mut NV_MOSAIC_SUPPORTED_TOPO_INFO, r#type: NV_MOSAIC_TOPO_TYPE) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_GetSupportedTopoInfo;
}

nvapi_fn! {
	pub type Mosaic_GetTopoGroupFn = extern "C" fn(pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF, pTopoGroup: *mut NV_MOSAIC_TOPO_GROUP) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_GetTopoGroup;
}

nvapi_fn! {
	pub type Mosaic_SetCurrentTopoFn = extern "C" fn(pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF, pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING, overlapX: i32, overlapY: i32, enable: u32) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_SetCurrentTopo;
}

nvapi_fn! {
	pub type SetCurrentMosaicTopologyFn = extern "C" fn(pMosaicTopo: *mut NV_MOSAIC_TOPOLOGY) -> NvAPI_Status;
	pub unsafe fn NvAPI_SetCurrentMosaicTopology;
}

// Grid-based Mosaic APIs
nvapi_fn! {
	pub type Mosaic_EnumDisplayGridsFn = extern "C" fn(pGridTopologies: *mut NV_MOSAIC_GRID_TOPO, pGridCount: *mut u32) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_EnumDisplayGrids;
}

nvapi_fn! {
	pub type Mosaic_SetDisplayGridsFn = extern "C" fn(pGridTopologies: *mut NV_MOSAIC_GRID_TOPO, gridCount: u32, setTopoFlags: u32) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_SetDisplayGrids;
}

nvapi_fn! {
	pub type Mosaic_ValidateDisplayGridsFn = extern "C" fn(setTopoFlags: u32, pGridTopologies: *mut NV_MOSAIC_GRID_TOPO, pTopoStatus: *mut NV_MOSAIC_DISPLAY_TOPO_STATUS, gridCount: u32) -> NvAPI_Status;
	pub unsafe fn NvAPI_Mosaic_ValidateDisplayGrids;
}
