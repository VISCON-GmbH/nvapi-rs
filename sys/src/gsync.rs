use crate::handles::NvGSyncDeviceHandle;
use crate::handles::NvPhysicalGpuHandle;
use crate::NvAPI_Status;

nvstruct! {
    pub struct NV_GSYNC_CAPABILITIES_V1 {
    pub version: u32,
        boardId: u32,
        revision: u32,
        pub capFlags: u32,
    }
}

const NV_GSYNC_CAPABILITIES_V1_SIZE: usize = 4 * 4;

nvstruct! {
    pub struct NV_GSYNC_CAPABILITIES_V2 {
    pub v1: NV_GSYNC_CAPABILITIES_V1,
        extendedRevision: u32,
    }
}

nvinherit! { NV_GSYNC_CAPABILITIES_V2(v1: NV_GSYNC_CAPABILITIES_V1) }

const NV_GSYNC_CAPABILITIES_V2_SIZE: usize = NV_GSYNC_CAPABILITIES_V1_SIZE + 4;

pub type NV_GSYNC_CAPABILITIES = NV_GSYNC_CAPABILITIES_V2;

nvversion! { NV_GSYNC_CAPABILITIES_VER_1(NV_GSYNC_CAPABILITIES_V1 = NV_GSYNC_CAPABILITIES_V1_SIZE, 1) }
nvversion! { NV_GSYNC_CAPABILITIES_VER_2(NV_GSYNC_CAPABILITIES_V2 = NV_GSYNC_CAPABILITIES_V2_SIZE, 2) }
nvversion! { NV_GSYNC_CAPABILITIES_VER = NV_GSYNC_CAPABILITIES_VER_2 }

nvenum! {
    pub enum NVAPI_GSYNC_DISPLAY_SYNC_STATE / DisplaySyncState {
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_UNSYNCED / Unsynced = 0,
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_SLAVE / Slave = 1,
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_MASTER / Master = 2,
    }
}

nvstruct! {
    pub struct NV_GSYNC_DISPLAY {
    pub version: u32,
    pub displayId: u32,
    // C bitfield: isMasterable:1, reserved:31 — represented as a single u32
    pub isMasterable: u32,
    pub syncState: NVAPI_GSYNC_DISPLAY_SYNC_STATE,
    }
}

const NV_GSYNC_DISPLAY_SIZE: usize = std::mem::size_of::<NV_GSYNC_DISPLAY>();
nvversion! { NV_GSYNC_DISPLAY_VER(NV_GSYNC_DISPLAY = NV_GSYNC_DISPLAY_SIZE, 1) }

nvenum! {
    pub enum NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR / TopologyConnector {
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_NONE / None = 0,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_PRIMARY / Primary = 1,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_SECONDARY / Secondary = 2,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_TERTIARY / Tertiary = 3,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_QUARTERNARY / Quarternary = 4,
    }
}

nvstruct! {
    pub struct NV_GSYNC_GPU {
    pub version: u32,
    pub hPhysicalGpu: NvPhysicalGpuHandle,
        connector: NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR,
    pub hProxyPhysicalGpu: NvPhysicalGpuHandle,
    // C bitfield: isSynced:1, reserved:31 — represented as a single u32
    isSynced: u32,
    }
}

const NV_GSYNC_GPU_SIZE: usize = std::mem::size_of::<NV_GSYNC_GPU>();
nvversion! { NV_GSYNC_GPU_VER(NV_GSYNC_GPU = NV_GSYNC_GPU_SIZE, 1) }

nvenum! {
    pub enum NVAPI_GSYNC_POLARITY / Polarity {
        NVAPI_GSYNC_POLARITY_RISING_EDGE / RisingEdge = 0,
        NVAPI_GSYNC_POLARITY_FALLING_EDGE / FallingEdge = 1,
        NVAPI_GSYNC_POLARITY_BOTH_EDGES / BothEdges = 2,
    }
}

nvenum! {
    pub enum NVAPI_GSYNC_VIDEO_MODE / VideoMode {
        NVAPI_GSYNC_VIDEO_MODE_NONE / None = 0,
        NVAPI_GSYNC_VIDEO_MODE_TTL / TTL = 1,
        NVAPI_GSYNC_VIDEO_MODE_NTSCPALSECAM / NtscPalCam = 2,
        NVAPI_GSYNC_VIDEO_MODE_HDTV / Hdtv = 3,
        NVAPI_GSYNC_VIDEO_MODE_COMPOSITE / Composite = 4,
    }
}

nvenum! {
    pub enum NVAPI_GSYNC_SYNC_SOURCE / SyncSource {
        NVAPI_GSYNC_SYNC_SOURCE_VSYNC / VSync = 0,
        NVAPI_GSYNC_SYNC_SOURCE_HOUSESYNC / HouseSync = 1,
    }
}

nvstruct! {
    pub struct NV_GSYNC_DELAY {
        pub version: u32,
        pub numLines: u32,
        pub numPixels: u32,
        maxLines: u32,
        minPixels: u32,
    }
}

const NV_GSYNC_DELAY_SIZE: usize = std::mem::size_of::<NV_GSYNC_DELAY>();

nvversion! { NV_GSYNC_DELAY_VER(NV_GSYNC_DELAY = NV_GSYNC_DELAY_SIZE, 1) }

nvstruct! {
    pub struct NV_GSYNC_CONTROL_PARAMS {
    pub version: u32,
        pub polarity: NVAPI_GSYNC_POLARITY,
        pub vmode: NVAPI_GSYNC_VIDEO_MODE,
        pub interval: u32,
        pub source: NVAPI_GSYNC_SYNC_SOURCE,
        pub interlaceMode: u32,
        pub syncSourceIsOutput: u32,
        reserved: u32,
        syncSkew: NV_GSYNC_DELAY,
        startupDelay: NV_GSYNC_DELAY,
    }
}

const NV_GSYNC_CONTROL_PARAMS_SIZE: usize = std::mem::size_of::<NV_GSYNC_CONTROL_PARAMS>();

nvversion! { NV_GSYNC_CONTROL_PARAMS_VER(NV_GSYNC_CONTROL_PARAMS = NV_GSYNC_CONTROL_PARAMS_SIZE, 1) }

nvenum! {
    pub enum NVAPI_GSYNC_DELAY_TYPE / DelayType {
        NVAPI_GSYNC_DELAY_TYPE_UNKNOWN / Unknown = 0,
        NVAPI_GSYNC_DELAY_TYPE_SYNC_SKEW / SyncSkew = 1,
        NVAPI_GSYNC_DELAY_TYPE_STARTUP / Startup = 2,
    }
}

nvstruct! {
    #[derive(Default)]
    pub struct NV_GSYNC_STATUS {
        pub version: u32,
        pub bIsSynced: u32,
        pub bIsStereoSynced: u32,
        pub bIsSyncSignalAvailable: u32,
    }
}

const NV_GSYNC_STATUS_SIZE: usize = std::mem::size_of::<NV_GSYNC_STATUS>();
nvversion! { NV_GSYNC_STATUS_VER(NV_GSYNC_STATUS = NV_GSYNC_STATUS_SIZE, 1) }

nvenum! {
    pub enum NVAPI_GSYNC_RJ45_IO / RJ45_IO {
        NVAPI_GSYNC_RJ45_OUTPUT / Output = 0,
        NVAPI_GSYNC_RJ45_INPUT 	/ Input = 1,
        NVAPI_GSYNC_RJ45_UNUSED / Unused = 2,
    }
}

pub const NVAPI_MAX_RJ45_PER_GSYNC: usize = 2;

nvstruct! {
    pub struct NV_GSYNC_STATUS_PARAMS_V1 {
    pub version: u32,
         pub refreshRate: u32,
         RJ45_IO: [NVAPI_GSYNC_RJ45_IO; NVAPI_MAX_RJ45_PER_GSYNC],
         RJ45_Ethernet: [u32; NVAPI_MAX_RJ45_PER_GSYNC],
         houseSyncIncoming: u32,
         pub bHouseSync: u32,
    }
}

const NV_GSYNC_STATUS_PARAMS_V1_SIZE: usize = std::mem::size_of::<NV_GSYNC_STATUS_PARAMS_V1>();

nvstruct! {
    pub struct NV_GSYNC_STATUS_PARAMS_V2 {
    pub v1: NV_GSYNC_STATUS_PARAMS_V1,
        bInternalSlave: u32,
        reserved: u32,
    }
}

nvinherit! { NV_GSYNC_STATUS_PARAMS_V2(v1: NV_GSYNC_STATUS_PARAMS_V1) }

const NV_GSYNC_STATUS_PARAMS_V2_SIZE: usize = std::mem::size_of::<NV_GSYNC_STATUS_PARAMS_V2>();

// Default to V1 for broader compatibility. Some cards/drivers/sync boards return
// NVAPI_INCOMPATIBLE_STRUCT_VERSION for V2.
// TODO: Tested with 2x Quadro P4000 and Quadro Sync 2 with older Firmware (2.02). 
// Investigate if newer GPUs and Q Sync firmware works with either version.
pub type NV_GSYNC_STATUS_PARAMS = NV_GSYNC_STATUS_PARAMS_V1;

nvversion! { NV_GSYNC_STATUS_PARAMS_VER_1(NV_GSYNC_STATUS_PARAMS_V1 = NV_GSYNC_STATUS_PARAMS_V1_SIZE, 1) }
nvversion! { NV_GSYNC_STATUS_PARAMS_VER_2(NV_GSYNC_STATUS_PARAMS_V2 = NV_GSYNC_STATUS_PARAMS_V2_SIZE, 2) }
nvversion! { NV_GSYNC_STATUS_PARAMS_VER = NV_GSYNC_STATUS_PARAMS_VER_1 }

nvapi_fn! {
    pub type GSync_EnumSyncDevicesFn = extern "C" fn(nvGSyncHandles: *mut [NvGSyncDeviceHandle; super::types::NVAPI_MAX_GSYNC_DEVICES], gsyncCount: *mut u32) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_EnumSyncDevices;
}

nvapi_fn! {
    pub type GSync_QueryCapabilitiesFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pNvGSyncCapabilities: *mut NV_GSYNC_CAPABILITIES) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_QueryCapabilities;
}

nvapi_fn! {
    pub type GSync_GetTopologyFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, gsyncGpuCount: *mut u32, gsyncGPUs: *mut NV_GSYNC_GPU, gsyncDisplayCount: *mut u32, gsyncDisplays: *mut NV_GSYNC_DISPLAY) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetTopology;
}

nvapi_fn! {
    pub type GSync_SetSyncStateSettingsFn = extern "C" fn(gsyncDisplayCount: u32, pGsyncDisplays: *const NV_GSYNC_DISPLAY, flags: u32) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_SetSyncStateSettings;
}

nvapi_fn! {
    pub type GSync_GetControlParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetControlParameters;
}

nvapi_fn! {
    pub type GSync_SetControlParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_SetControlParameters;
}

nvapi_fn! {
    // Parameter should be pointer?
    pub type GSync_AdjustSyncDelayFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, delayType: NVAPI_GSYNC_DELAY_TYPE, pGsyncDelay: *mut NV_GSYNC_DELAY, syncSteps: *mut u32) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_AdjustSyncDelay;
}

nvapi_fn! {
    pub type GSync_GetSyncStatusFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, hPhysicalGpu: NvPhysicalGpuHandle, status: *mut NV_GSYNC_STATUS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetSyncStatus;
}

nvapi_fn! {
    pub type GSync_GetStatusParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pStatusParams: *mut NV_GSYNC_STATUS_PARAMS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetStatusParameters;
}
