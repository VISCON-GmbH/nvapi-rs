use crate::status::NvAPI_Status;
use crate::handles::NvPhysicalGpuHandle;

nvenum! {
    /// Used in NV_GPU_THERMAL_SETTINGS
    pub enum NV_THERMAL_TARGET / ThermalTarget {
        NVAPI_THERMAL_TARGET_NONE / None = 0,
        /// GPU core temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_GPU / Gpu = 1,
        /// GPU memory temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_MEMORY / Memory = 2,
        /// GPU power supply temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_POWER_SUPPLY / PowerSupply = 4,
        /// GPU board ambient temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_BOARD / Board = 8,
        /// Visual Computing Device Board temperature requires NvVisualComputingDeviceHandle
        NVAPI_THERMAL_TARGET_VCD_BOARD / VcdBoard = 9,
        /// Visual Computing Device Inlet temperature requires NvVisualComputingDeviceHandle
        NVAPI_THERMAL_TARGET_VCD_INLET / VcdInlet = 10,
        /// Visual Computing Device Outlet temperature requires NvVisualComputingDeviceHandle
        NVAPI_THERMAL_TARGET_VCD_OUTLET / VcdOutlet = 11,
        NVAPI_THERMAL_TARGET_ALL / All = 15,
        NVAPI_THERMAL_TARGET_UNKNOWN / Unknown = -1,
    }
}

nvenum_display! {
    ThermalTarget => {
        Gpu = "Core",
        Memory = "Memory",
        PowerSupply = "VRM",
        VcdBoard = "VCD Board",
        VcdInlet = "VCD Inlet",
        VcdOutlet = "VCD Outlet",
        _ = _,
    }
}

nvenum! {
    /// NV_GPU_THERMAL_SETTINGS
    pub enum NV_THERMAL_CONTROLLER / ThermalController {
        NVAPI_THERMAL_CONTROLLER_NONE / None = 0,
        NVAPI_THERMAL_CONTROLLER_GPU_INTERNAL / GpuInternal = 1,
        NVAPI_THERMAL_CONTROLLER_ADM1032 / ADM1032 = 2,
        NVAPI_THERMAL_CONTROLLER_MAX6649 / MAX6649 = 3,
        NVAPI_THERMAL_CONTROLLER_MAX1617 / MAX1617 = 4,
        NVAPI_THERMAL_CONTROLLER_LM99 / LM99 = 5,
        NVAPI_THERMAL_CONTROLLER_LM89 / LM89 = 6,
        NVAPI_THERMAL_CONTROLLER_LM64 / LM64 = 7,
        NVAPI_THERMAL_CONTROLLER_ADT7473 / ADT7473 = 8,
        NVAPI_THERMAL_CONTROLLER_SBMAX6649 / SBMAX6649 = 9,
        NVAPI_THERMAL_CONTROLLER_VBIOSEVT / VBIOSEVT = 10,
        NVAPI_THERMAL_CONTROLLER_OS / OS = 11,
        NVAPI_THERMAL_CONTROLLER_UNKNOWN / Unknown = -1,
    }
}

nvenum_display! {
    ThermalController => {
        GpuInternal = "Internal",
        _ = _,
    }
}

pub const NVAPI_MAX_THERMAL_SENSORS_PER_GPU: usize = 3;

nvstruct! {
    /// Used in NvAPI_GPU_GetThermalSettings()
    pub struct NV_GPU_THERMAL_SETTINGS_V1 {
        /// structure version
        pub version: u32,
        /// number of associated thermal sensors
        pub count: u32,
        pub sensor: [NV_GPU_THERMAL_SETTINGS_SENSOR; NVAPI_MAX_THERMAL_SENSORS_PER_GPU],
    }
}

nvstruct! {
    /// Anonymous struct in NV_GPU_THERMAL_SETTINGS
    pub struct NV_GPU_THERMAL_SETTINGS_SENSOR {
        /// internal, ADM1032, MAX6649...
        pub controller: NV_THERMAL_CONTROLLER,
        /// The min default temperature value of the thermal sensor in degree Celsius
        pub defaultMinTemp: i32,
        /// The max default temperature value of the thermal sensor in degree Celsius
        pub defaultMaxTemp: i32,
        /// The current temperature value of the thermal sensor in degree Celsius
        pub currentTemp: i32,
        /// Thermal sensor targeted @ GPU, memory, chipset, powersupply, Visual Computing Device, etc.
        pub target: NV_THERMAL_TARGET,
    }
}

pub type NV_GPU_THERMAL_SETTINGS_V2 = NV_GPU_THERMAL_SETTINGS_V1; // the only difference is the _SENSOR struct uses i32 instead of u32 fields
pub type NV_GPU_THERMAL_SETTINGS = NV_GPU_THERMAL_SETTINGS_V2;

const NV_GPU_THERMAL_SETTINGS_V1_SIZE: usize = 4 * 2 + (4 * 5) * NVAPI_MAX_THERMAL_SENSORS_PER_GPU;
nvversion! { NV_GPU_THERMAL_SETTINGS_VER_1(NV_GPU_THERMAL_SETTINGS_V1 = NV_GPU_THERMAL_SETTINGS_V1_SIZE, 1) }
nvversion! { NV_GPU_THERMAL_SETTINGS_VER_2(NV_GPU_THERMAL_SETTINGS_V2 = NV_GPU_THERMAL_SETTINGS_V1_SIZE, 2) }
nvversion! { NV_GPU_THERMAL_SETTINGS_VER = NV_GPU_THERMAL_SETTINGS_VER_2 }

nvapi_fn! {
    pub type GPU_GetThermalSettingsFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, sensorIndex: u32, pThermalSettings: *mut NV_GPU_THERMAL_SETTINGS) -> NvAPI_Status;

    /// This function retrieves the thermal information of all thermal sensors or specific thermal sensor associated with the selected GPU.
    ///
    /// Thermal sensors are indexed 0 to NVAPI_MAX_THERMAL_SENSORS_PER_GPU-1.
    /// - To retrieve specific thermal sensor info, set the sensorIndex to the required thermal sensor index.
    /// - To retrieve info for all sensors, set sensorIndex to NVAPI_THERMAL_TARGET_ALL.
    pub unsafe fn NvAPI_GPU_GetThermalSettings;
}

/// Undocumented API
pub mod private {
    use crate::status::NvAPI_Status;
    use crate::handles::NvPhysicalGpuHandle;

    pub const NVAPI_MAX_THERMAL_INFO_ENTRIES: usize = 4;

    nvstruct! {
        pub struct NV_GPU_THERMAL_INFO_ENTRY {
            pub controller: super::NV_THERMAL_CONTROLLER,
            pub unknown: u32,
            pub minTemp: i32,
            pub defaultTemp: i32,
            pub maxTemp: i32,
            pub defaultFlags: u32,
        }
    }
    const NV_GPU_THERMAL_INFO_ENTRY_SIZE: usize = 4 * 6;

    nvstruct! {
        pub struct NV_GPU_THERMAL_INFO_V2 {
            pub version: u32,
            pub count: u8,
            pub flags: u8,
            pub padding: [u8; 2],
            pub entries: [NV_GPU_THERMAL_INFO_ENTRY; NVAPI_MAX_THERMAL_INFO_ENTRIES],
        }
    }
    const NV_GPU_THERMAL_INFO_V2_SIZE: usize = 4 * 2 + NV_GPU_THERMAL_INFO_ENTRY_SIZE * NVAPI_MAX_THERMAL_INFO_ENTRIES;

    pub type NV_GPU_THERMAL_INFO = NV_GPU_THERMAL_INFO_V2;

    nvversion! { NV_GPU_THERMAL_INFO_VER_2(NV_GPU_THERMAL_INFO_V2 = NV_GPU_THERMAL_INFO_V2_SIZE, 2) }
    nvversion! { NV_GPU_THERMAL_INFO_VER = NV_GPU_THERMAL_INFO_VER_2 }

    nvapi_fn! {
        pub unsafe fn NvAPI_GPU_ClientThermalPoliciesGetInfo(hPhysicalGPU: NvPhysicalGpuHandle, pThermalInfo: *mut NV_GPU_THERMAL_INFO) -> NvAPI_Status;
    }

    pub const NVAPI_MAX_THERMAL_LIMIT_ENTRIES: usize = 4;

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY {
            pub controller: super::NV_THERMAL_CONTROLLER,
            pub value: u32,
            pub flags: u32,
        }
    }
    const NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_SIZE: usize = 4 * 3;

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2 {
            pub version: u32,
            pub flags: u32,
            pub entries: [NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY; NVAPI_MAX_THERMAL_LIMIT_ENTRIES],
        }
    }
    const NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2_SIZE: usize = 4 * 2 + NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_SIZE * NVAPI_MAX_THERMAL_LIMIT_ENTRIES;

    pub type NV_GPU_CLIENT_THERMAL_POLICIES_STATUS = NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2;

    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_VER_2(NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2 = NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2_SIZE, 2) }
    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_VER = NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_VER_2 }

    nvapi_fn! {
        pub unsafe fn NvAPI_GPU_ClientThermalPoliciesGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pThermalLimit: *mut NV_GPU_CLIENT_THERMAL_POLICIES_STATUS) -> NvAPI_Status;
    }

    nvapi_fn! {
        pub unsafe fn NvAPI_GPU_ClientThermalPoliciesSetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pThermalLimit: *const NV_GPU_CLIENT_THERMAL_POLICIES_STATUS) -> NvAPI_Status;
    }
}
