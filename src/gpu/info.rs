use crate::sys::{self, driverapi};
use crate::types::{Kibibytes, RawConversion};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, fmt};

use super::{
    ArchitectureId,
    ArchitectureImplementationId,
    ChipRevision,
    DisplayIdsFlags,
    MonitorConnectorType,
    Vendor,
};
use crate::sys::gpu::{arch, display};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GpuArchitectureInfo {
    pub architecture: ArchitectureId,
    pub implementation: ArchitectureImplementationId,
    pub revision: ChipRevision,
}

impl RawConversion for arch::NV_GPU_ARCH_INFO {
    type Target = GpuArchitectureInfo;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(GpuArchitectureInfo {
            architecture: ArchitectureId::from_raw(self.architecture.raw())?,
            implementation: ArchitectureImplementationId::from_raw(self.implementation.raw())?,
            revision: ChipRevision::from_raw(self.revision.raw())?,
        })
    }
}

impl fmt::Debug for GpuArchitectureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct NamedId<'a> {
            name: Option<&'a str>,
            raw: u32,
        }

        impl<'a> fmt::Debug for NamedId<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.name {
                    Some(name) => write!(f, "{} (0x{:08x})", name, self.raw),
                    None => write!(f, "0x{:08x}", self.raw),
                }
            }
        }

        let arch_name = self.architecture.name();
        let impl_name = self.implementation.name_for_arch(self.architecture);
        let rev_name = self.revision.name();

        f.debug_struct("GpuArchitectureInfo")
            .field(
                "architecture",
                &NamedId {
                    name: arch_name,
                    raw: self.architecture.raw(),
                },
            )
            .field(
                "implementation",
                &NamedId {
                    name: impl_name,
                    raw: self.implementation.raw(),
                },
            )
            .field(
                "revision",
                &NamedId {
                    name: rev_name,
                    raw: self.revision.raw(),
                },
            )
            .finish()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PciIdentifiers {
    pub device_id: u32,
    pub subsystem_id: u32,
    pub revision_id: u32,
    pub ext_device_id: u32,
}

impl fmt::Display for PciIdentifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x} - {:08x} - {:08x} - {:x}",
            self.device_id, self.subsystem_id, self.ext_device_id, self.revision_id
        )
    }
}

impl PciIdentifiers {
    pub fn vendor_id(&self) -> u16 {
        self.ids().0
    }

    pub fn product_id(&self) -> u16 {
        self.ids().1
    }

    pub fn ids(&self) -> (u16, u16) {
        let pid = (self.device_id >> 16) as u16;
        let vid = self.device_id as u16;
        if vid == 0x10de && self.subsystem_id != 0 {
            let spid = (self.subsystem_id >> 16) as u16;
            (
                self.subsystem_id as u16,
                if spid == 0 {
                    // Colorful and Inno3D
                    pid
                } else {
                    spid
                },
            )
        } else {
            (vid, pid)
        }
    }

    pub fn vendor(&self) -> Result<Vendor, sys::ArgumentRangeError> {
        Vendor::from_raw(self.vendor_id() as _)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct MemoryInfo {
    pub dedicated: Kibibytes,
    pub dedicated_available: Kibibytes,
    pub system: Kibibytes,
    pub shared: Kibibytes,
    pub dedicated_available_current: Kibibytes,
    pub dedicated_evictions_size: Kibibytes,
    pub dedicated_evictions: u32,
}

impl RawConversion for driverapi::NV_DISPLAY_DRIVER_MEMORY_INFO {
    type Target = MemoryInfo;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(MemoryInfo {
            dedicated: Kibibytes(self.dedicatedVideoMemory),
            dedicated_available: Kibibytes(self.availableDedicatedVideoMemory),
            system: Kibibytes(self.systemVideoMemory),
            shared: Kibibytes(self.sharedSystemMemory),
            dedicated_available_current: Kibibytes(self.curAvailableDedicatedVideoMemory),
            dedicated_evictions_size: Kibibytes(self.dedicatedVideoMemoryEvictionsSize),
            dedicated_evictions: self.dedicatedVideoMemoryEvictionCount,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DriverModel {
    pub value: u32,
}

impl DriverModel {
    pub fn new(value: u32) -> Self {
        DriverModel { value }
    }

    pub fn wddm(&self) -> (u8, u8) {
        // 2.0 or 1.(value >> 8)
        let major = ((self.value >> 12) & 0xf) as u8;
        (
            major,
            if major == 2 {
                0
            } else {
                (self.value >> 8) as u8 & 0xf
            },
        )
    }
}

impl fmt::Display for DriverModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let wddm = self.wddm();
        write!(f, "WDDM {}.{:02}", wddm.0, wddm.1)
    }
}

impl fmt::Debug for DriverModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:08x})", self, self.value)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DisplayId {
    pub connector: MonitorConnectorType,
    pub display_id: u32,
    pub flags: DisplayIdsFlags,
}

impl RawConversion for display::NV_GPU_DISPLAYIDS {
    type Target = DisplayId;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(DisplayId {
            connector: MonitorConnectorType::from_raw(self.connectorType)?,
            display_id: self.displayId,
            flags: DisplayIdsFlags::from_bits_truncate(self.flags),
        })
    }
}
