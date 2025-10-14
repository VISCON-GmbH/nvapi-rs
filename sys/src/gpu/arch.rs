use crate::handles::NvPhysicalGpuHandle;
use crate::status::NvAPI_Status;
use crate::ArgumentRangeError;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ArchitectureId(u32);

impl ArchitectureId {
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub const T2X: Self = Self::new(0xE0000020);
    pub const T3X: Self = Self::new(0xE0000030);
    pub const T4X: Self = Self::new(0xE0000040);
    pub const T12X: Self = Self::T4X;
    pub const NV40: Self = Self::new(0x00000040);
    pub const NV50: Self = Self::new(0x00000050);
    pub const G78: Self = Self::new(0x00000060);
    pub const G80: Self = Self::new(0x00000080);
    pub const G90: Self = Self::new(0x00000090);
    pub const GT200: Self = Self::new(0x000000A0);
    pub const GF100: Self = Self::new(0x000000C0);
    pub const GF110: Self = Self::new(0x000000D0);
    pub const GK100: Self = Self::new(0x000000E0);
    pub const GK110: Self = Self::new(0x000000F0);
    pub const GK200: Self = Self::new(0x00000100);
    pub const GM000: Self = Self::new(0x00000110);
    pub const GM200: Self = Self::new(0x00000120);
    pub const GP100: Self = Self::new(0x00000130);
    pub const GV100: Self = Self::new(0x00000140);
    pub const GV110: Self = Self::new(0x00000150);
    pub const TU100: Self = Self::new(0x00000160);
    pub const GA100: Self = Self::new(0x00000170);
    pub const AD100: Self = Self::new(0x00000190);
    pub const GB200: Self = Self::new(0x000001B0);

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub fn from_raw(raw: u32) -> Result<Self, ArgumentRangeError> {
        Ok(Self::new(raw))
    }

    pub fn values() -> impl Iterator<Item = Self> {
        ARCHITECTURE_VALUES.iter().copied()
    }

    pub fn name(&self) -> Option<&'static str> {
        match self.raw() {
            0xE0000020 => Some("T2X"),
            0xE0000030 => Some("T3X"),
            0xE0000040 => Some("T4X/T12X"),
            0x00000040 => Some("NV40"),
            0x00000050 => Some("NV50"),
            0x00000060 => Some("G78"),
            0x00000080 => Some("G80"),
            0x00000090 => Some("G90"),
            0x000000A0 => Some("GT200"),
            0x000000C0 => Some("GF100"),
            0x000000D0 => Some("GF110"),
            0x000000E0 => Some("GK100"),
            0x000000F0 => Some("GK110"),
            0x00000100 => Some("GK200"),
            0x00000110 => Some("GM000"),
            0x00000120 => Some("GM200"),
            0x00000130 => Some("GP100"),
            0x00000140 => Some("GV100"),
            0x00000150 => Some("GV110"),
            0x00000160 => Some("TU100"),
            0x00000170 => Some("GA100"),
            0x00000190 => Some("AD100"),
            0x000001B0 => Some("GB200"),
            _ => None,
        }
    }
}

impl From<ArchitectureId> for u32 {
    fn from(value: ArchitectureId) -> Self {
        value.raw()
    }
}

impl fmt::Display for ArchitectureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => write!(f, "{} (0x{:08x})", name, self.raw()),
            None => write!(f, "0x{:08x}", self.raw()),
        }
    }
}

const ARCHITECTURE_VALUES: [ArchitectureId; 23] = [
    ArchitectureId::T2X,
    ArchitectureId::T3X,
    ArchitectureId::T4X,
    ArchitectureId::NV40,
    ArchitectureId::NV50,
    ArchitectureId::G78,
    ArchitectureId::G80,
    ArchitectureId::G90,
    ArchitectureId::GT200,
    ArchitectureId::GF100,
    ArchitectureId::GF110,
    ArchitectureId::GK100,
    ArchitectureId::GK110,
    ArchitectureId::GK200,
    ArchitectureId::GM000,
    ArchitectureId::GM200,
    ArchitectureId::GP100,
    ArchitectureId::GV100,
    ArchitectureId::GV110,
    ArchitectureId::TU100,
    ArchitectureId::GA100,
    ArchitectureId::AD100,
    ArchitectureId::GB200,
];

pub const NV_GPU_ARCHITECTURE_T2X: ArchitectureId = ArchitectureId::T2X;
pub const NV_GPU_ARCHITECTURE_T3X: ArchitectureId = ArchitectureId::T3X;
pub const NV_GPU_ARCHITECTURE_T4X: ArchitectureId = ArchitectureId::T4X;
pub const NV_GPU_ARCHITECTURE_T12X: ArchitectureId = ArchitectureId::T12X;
pub const NV_GPU_ARCHITECTURE_NV40: ArchitectureId = ArchitectureId::NV40;
pub const NV_GPU_ARCHITECTURE_NV50: ArchitectureId = ArchitectureId::NV50;
pub const NV_GPU_ARCHITECTURE_G78: ArchitectureId = ArchitectureId::G78;
pub const NV_GPU_ARCHITECTURE_G80: ArchitectureId = ArchitectureId::G80;
pub const NV_GPU_ARCHITECTURE_G90: ArchitectureId = ArchitectureId::G90;
pub const NV_GPU_ARCHITECTURE_GT200: ArchitectureId = ArchitectureId::GT200;
pub const NV_GPU_ARCHITECTURE_GF100: ArchitectureId = ArchitectureId::GF100;
pub const NV_GPU_ARCHITECTURE_GF110: ArchitectureId = ArchitectureId::GF110;
pub const NV_GPU_ARCHITECTURE_GK100: ArchitectureId = ArchitectureId::GK100;
pub const NV_GPU_ARCHITECTURE_GK110: ArchitectureId = ArchitectureId::GK110;
pub const NV_GPU_ARCHITECTURE_GK200: ArchitectureId = ArchitectureId::GK200;
pub const NV_GPU_ARCHITECTURE_GM000: ArchitectureId = ArchitectureId::GM000;
pub const NV_GPU_ARCHITECTURE_GM200: ArchitectureId = ArchitectureId::GM200;
pub const NV_GPU_ARCHITECTURE_GP100: ArchitectureId = ArchitectureId::GP100;
pub const NV_GPU_ARCHITECTURE_GV100: ArchitectureId = ArchitectureId::GV100;
pub const NV_GPU_ARCHITECTURE_GV110: ArchitectureId = ArchitectureId::GV110;
pub const NV_GPU_ARCHITECTURE_TU100: ArchitectureId = ArchitectureId::TU100;
pub const NV_GPU_ARCHITECTURE_GA100: ArchitectureId = ArchitectureId::GA100;
pub const NV_GPU_ARCHITECTURE_AD100: ArchitectureId = ArchitectureId::AD100;
pub const NV_GPU_ARCHITECTURE_GB200: ArchitectureId = ArchitectureId::GB200;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ArchitectureImplementationId(u32);

impl ArchitectureImplementationId {
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub const T20: Self = Self::new(0x00000000);
    pub const T30: Self = Self::new(0x00000000);
    pub const T35: Self = Self::new(0x00000005);
    pub const T40: Self = Self::new(0x00000000);
    pub const T124: Self = Self::new(0x00000000);
    pub const NV40: Self = Self::new(0x00000000);
    pub const NV41: Self = Self::new(0x00000001);
    pub const NV42: Self = Self::new(0x00000002);
    pub const NV43: Self = Self::new(0x00000003);
    pub const NV44: Self = Self::new(0x00000004);
    pub const NV44A: Self = Self::new(0x0000000A);
    pub const NV46: Self = Self::new(0x00000006);
    pub const NV47: Self = Self::new(0x00000007);
    pub const NV49: Self = Self::new(0x00000009);
    pub const NV4B: Self = Self::new(0x0000000B);
    pub const NV4C: Self = Self::new(0x0000000C);
    pub const NV4E: Self = Self::new(0x0000000E);
    pub const NV50: Self = Self::new(0x00000000);
    pub const NV63: Self = Self::new(0x00000003);
    pub const NV67: Self = Self::new(0x00000007);
    pub const G84: Self = Self::new(0x00000004);
    pub const G86: Self = Self::new(0x00000006);
    pub const G92: Self = Self::new(0x00000002);
    pub const G94: Self = Self::new(0x00000004);
    pub const G96: Self = Self::new(0x00000006);
    pub const G98: Self = Self::new(0x00000008);
    pub const GT200: Self = Self::new(0x00000000);
    pub const GT212: Self = Self::new(0x00000002);
    pub const GT214: Self = Self::new(0x00000004);
    pub const GT215: Self = Self::new(0x00000003);
    pub const GT216: Self = Self::new(0x00000005);
    pub const GT218: Self = Self::new(0x00000008);
    pub const MCP77: Self = Self::new(0x0000000A);
    pub const GT21C: Self = Self::new(0x0000000B);
    pub const MCP79: Self = Self::new(0x0000000C);
    pub const GT21A: Self = Self::new(0x0000000D);
    pub const MCP89: Self = Self::new(0x0000000F);
    pub const GF100: Self = Self::new(0x00000000);
    pub const GF104: Self = Self::new(0x00000004);
    pub const GF106: Self = Self::new(0x00000003);
    pub const GF108: Self = Self::new(0x00000001);
    pub const GF110: Self = Self::new(0x00000000);
    pub const GF116: Self = Self::new(0x00000006);
    pub const GF117: Self = Self::new(0x00000007);
    pub const GF118: Self = Self::new(0x00000008);
    pub const GF119: Self = Self::new(0x00000009);
    pub const GK104: Self = Self::new(0x00000004);
    pub const GK106: Self = Self::new(0x00000006);
    pub const GK107: Self = Self::new(0x00000007);
    pub const GK20A: Self = Self::new(0x0000000A);
    pub const GK110: Self = Self::new(0x00000000);
    pub const GK208: Self = Self::new(0x00000008);
    pub const GM204: Self = Self::new(0x00000004);
    pub const GM206: Self = Self::new(0x00000006);
    pub const GP100: Self = Self::new(0x00000000);
    pub const GP000: Self = Self::new(0x00000001);
    pub const GP102: Self = Self::new(0x00000002);
    pub const GP104: Self = Self::new(0x00000004);
    pub const GP106: Self = Self::new(0x00000006);
    pub const GP107: Self = Self::new(0x00000007);
    pub const GP108: Self = Self::new(0x00000008);
    pub const GV100: Self = Self::new(0x00000000);
    pub const GV10B: Self = Self::new(0x0000000B);
    pub const TU100: Self = Self::new(0x00000000);
    pub const TU102: Self = Self::new(0x00000002);
    pub const TU104: Self = Self::new(0x00000004);
    pub const TU106: Self = Self::new(0x00000006);
    pub const TU116: Self = Self::new(0x00000008);
    pub const TU117: Self = Self::new(0x00000007);
    pub const TU000: Self = Self::new(0x00000001);
    pub const GA100: Self = Self::new(0x00000000);
    pub const GA102: Self = Self::new(0x00000002);
    pub const GA104: Self = Self::new(0x00000004);
    pub const AD102: Self = Self::new(0x00000002);
    pub const AD103: Self = Self::new(0x00000003);
    pub const AD104: Self = Self::new(0x00000004);
    pub const GB202: Self = Self::new(0x00000002);

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub fn from_raw(raw: u32) -> Result<Self, ArgumentRangeError> {
        Ok(Self::new(raw))
    }

    pub fn values() -> impl Iterator<Item = Self> {
        IMPLEMENTATION_VALUES.iter().copied()
    }

	/// Get the name of the architecture implementation for a specific architecture.
    pub fn name_for_arch(&self, architecture: ArchitectureId) -> Option<&'static str> {
        match architecture.raw() {
            0xE0000020 => match self.raw() {
                0x00000000 => Some("T20"),
                _ => None,
            },
            0xE0000030 => match self.raw() {
                0x00000000 => Some("T30"),
                0x00000005 => Some("T35"),
                _ => None,
            },
            0xE0000040 => match self.raw() {
                0x00000000 => Some("T40/T124"),
                _ => None,
            },
            0x00000040 => match self.raw() {
                0x00000000 => Some("NV40"),
                0x00000001 => Some("NV41"),
                0x00000002 => Some("NV42"),
                0x00000003 => Some("NV43"),
                0x00000004 => Some("NV44"),
                0x0000000A => Some("NV44A"),
                0x00000006 => Some("NV46"),
                0x00000007 => Some("NV47"),
                0x00000009 => Some("NV49"),
                0x0000000B => Some("NV4B"),
                0x0000000C => Some("NV4C"),
                0x0000000E => Some("NV4E"),
                _ => None,
            },
            0x00000050 => match self.raw() {
                0x00000000 => Some("NV50"),
                0x00000003 => Some("NV63"),
                0x00000007 => Some("NV67"),
                _ => None,
            },
            0x00000080 => match self.raw() {
                0x00000004 => Some("G84"),
                0x00000006 => Some("G86"),
                _ => None,
            },
            0x00000090 => match self.raw() {
                0x00000002 => Some("G92"),
                0x00000004 => Some("G94"),
                0x00000006 => Some("G96"),
                0x00000008 => Some("G98"),
                _ => None,
            },
            0x000000A0 => match self.raw() {
                0x00000000 => Some("GT200"),
                0x00000002 => Some("GT212"),
                0x00000004 => Some("GT214"),
                0x00000003 => Some("GT215"),
                0x00000005 => Some("GT216"),
                0x00000008 => Some("GT218"),
                0x0000000A => Some("MCP77"),
                0x0000000B => Some("GT21C"),
                0x0000000C => Some("MCP79"),
                0x0000000D => Some("GT21A"),
                0x0000000F => Some("MCP89"),
                _ => None,
            },
            0x000000C0 => match self.raw() {
                0x00000000 => Some("GF100"),
                0x00000004 => Some("GF104"),
                0x00000003 => Some("GF106"),
                0x00000001 => Some("GF108"),
                _ => None,
            },
            0x000000D0 => match self.raw() {
                0x00000000 => Some("GF110"),
                0x00000006 => Some("GF116"),
                0x00000007 => Some("GF117"),
                0x00000008 => Some("GF118"),
                0x00000009 => Some("GF119"),
                _ => None,
            },
            0x000000E0 => match self.raw() {
                0x00000004 => Some("GK104"),
                0x00000006 => Some("GK106"),
                0x00000007 => Some("GK107"),
                0x0000000A => Some("GK20A"),
                _ => None,
            },
            0x000000F0 => match self.raw() {
                0x00000000 => Some("GK110"),
                _ => None,
            },
            0x00000100 => match self.raw() {
                0x00000008 => Some("GK208"),
                _ => None,
            },
            0x00000110 => match self.raw() {
                0x00000000 => Some("GM000"),
                _ => None,
            },
            0x00000120 => match self.raw() {
                0x00000004 => Some("GM204"),
                0x00000006 => Some("GM206"),
                _ => None,
            },
            0x00000130 => match self.raw() {
                0x00000000 => Some("GP100"),
                0x00000001 => Some("GP000"),
                0x00000002 => Some("GP102"),
                0x00000004 => Some("GP104"),
                0x00000006 => Some("GP106"),
                0x00000007 => Some("GP107"),
                0x00000008 => Some("GP108"),
                _ => None,
            },
            0x00000140 => match self.raw() {
                0x00000000 => Some("GV100"),
                0x0000000B => Some("GV10B"),
                _ => None,
            },
            0x00000160 => match self.raw() {
                0x00000000 => Some("TU100"),
                0x00000002 => Some("TU102"),
                0x00000004 => Some("TU104"),
                0x00000006 => Some("TU106"),
                0x00000008 => Some("TU116"),
                0x00000007 => Some("TU117"),
                0x00000001 => Some("TU000"),
                _ => None,
            },
            0x00000170 => match self.raw() {
                0x00000000 => Some("GA100"),
                0x00000002 => Some("GA102"),
                0x00000004 => Some("GA104"),
                _ => None,
            },
            0x00000190 => match self.raw() {
                0x00000002 => Some("AD102"),
                0x00000003 => Some("AD103"),
                0x00000004 => Some("AD104"),
                _ => None,
            },
            0x000001B0 => match self.raw() {
                0x00000002 => Some("GB202"),
                _ => None,
            },
            _ => None,
        }
    }
}

impl From<ArchitectureImplementationId> for u32 {
    fn from(value: ArchitectureImplementationId) -> Self {
        value.raw()
    }
}

const IMPLEMENTATION_VALUES: [ArchitectureImplementationId; 16] = [
    ArchitectureImplementationId::T20,
    ArchitectureImplementationId::NV41,
    ArchitectureImplementationId::NV42,
    ArchitectureImplementationId::NV43,
    ArchitectureImplementationId::NV44,
    ArchitectureImplementationId::T35,
    ArchitectureImplementationId::NV46,
    ArchitectureImplementationId::NV47,
    ArchitectureImplementationId::GT218,
    ArchitectureImplementationId::NV49,
    ArchitectureImplementationId::NV44A,
    ArchitectureImplementationId::NV4B,
    ArchitectureImplementationId::NV4C,
    ArchitectureImplementationId::GT21A,
    ArchitectureImplementationId::NV4E,
    ArchitectureImplementationId::MCP89,
];

pub const NV_GPU_ARCH_IMPLEMENTATION_T20: ArchitectureImplementationId =
    ArchitectureImplementationId::T20;
pub const NV_GPU_ARCH_IMPLEMENTATION_T30: ArchitectureImplementationId =
    ArchitectureImplementationId::T30;
pub const NV_GPU_ARCH_IMPLEMENTATION_T35: ArchitectureImplementationId =
    ArchitectureImplementationId::T35;
pub const NV_GPU_ARCH_IMPLEMENTATION_T40: ArchitectureImplementationId =
    ArchitectureImplementationId::T40;
pub const NV_GPU_ARCH_IMPLEMENTATION_T124: ArchitectureImplementationId =
    ArchitectureImplementationId::T124;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV40: ArchitectureImplementationId =
    ArchitectureImplementationId::NV40;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV41: ArchitectureImplementationId =
    ArchitectureImplementationId::NV41;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV42: ArchitectureImplementationId =
    ArchitectureImplementationId::NV42;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV43: ArchitectureImplementationId =
    ArchitectureImplementationId::NV43;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV44: ArchitectureImplementationId =
    ArchitectureImplementationId::NV44;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV44A: ArchitectureImplementationId =
    ArchitectureImplementationId::NV44A;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV46: ArchitectureImplementationId =
    ArchitectureImplementationId::NV46;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV47: ArchitectureImplementationId =
    ArchitectureImplementationId::NV47;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV49: ArchitectureImplementationId =
    ArchitectureImplementationId::NV49;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV4B: ArchitectureImplementationId =
    ArchitectureImplementationId::NV4B;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV4C: ArchitectureImplementationId =
    ArchitectureImplementationId::NV4C;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV4E: ArchitectureImplementationId =
    ArchitectureImplementationId::NV4E;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV50: ArchitectureImplementationId =
    ArchitectureImplementationId::NV50;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV63: ArchitectureImplementationId =
    ArchitectureImplementationId::NV63;
pub const NV_GPU_ARCH_IMPLEMENTATION_NV67: ArchitectureImplementationId =
    ArchitectureImplementationId::NV67;
pub const NV_GPU_ARCH_IMPLEMENTATION_G84: ArchitectureImplementationId =
    ArchitectureImplementationId::G84;
pub const NV_GPU_ARCH_IMPLEMENTATION_G86: ArchitectureImplementationId =
    ArchitectureImplementationId::G86;
pub const NV_GPU_ARCH_IMPLEMENTATION_G92: ArchitectureImplementationId =
    ArchitectureImplementationId::G92;
pub const NV_GPU_ARCH_IMPLEMENTATION_G94: ArchitectureImplementationId =
    ArchitectureImplementationId::G94;
pub const NV_GPU_ARCH_IMPLEMENTATION_G96: ArchitectureImplementationId =
    ArchitectureImplementationId::G96;
pub const NV_GPU_ARCH_IMPLEMENTATION_G98: ArchitectureImplementationId =
    ArchitectureImplementationId::G98;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT200: ArchitectureImplementationId =
    ArchitectureImplementationId::GT200;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT212: ArchitectureImplementationId =
    ArchitectureImplementationId::GT212;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT214: ArchitectureImplementationId =
    ArchitectureImplementationId::GT214;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT215: ArchitectureImplementationId =
    ArchitectureImplementationId::GT215;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT216: ArchitectureImplementationId =
    ArchitectureImplementationId::GT216;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT218: ArchitectureImplementationId =
    ArchitectureImplementationId::GT218;
pub const NV_GPU_ARCH_IMPLEMENTATION_MCP77: ArchitectureImplementationId =
    ArchitectureImplementationId::MCP77;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT21C: ArchitectureImplementationId =
    ArchitectureImplementationId::GT21C;
pub const NV_GPU_ARCH_IMPLEMENTATION_MCP79: ArchitectureImplementationId =
    ArchitectureImplementationId::MCP79;
pub const NV_GPU_ARCH_IMPLEMENTATION_GT21A: ArchitectureImplementationId =
    ArchitectureImplementationId::GT21A;
pub const NV_GPU_ARCH_IMPLEMENTATION_MCP89: ArchitectureImplementationId =
    ArchitectureImplementationId::MCP89;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF100: ArchitectureImplementationId =
    ArchitectureImplementationId::GF100;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF104: ArchitectureImplementationId =
    ArchitectureImplementationId::GF104;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF106: ArchitectureImplementationId =
    ArchitectureImplementationId::GF106;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF108: ArchitectureImplementationId =
    ArchitectureImplementationId::GF108;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF110: ArchitectureImplementationId =
    ArchitectureImplementationId::GF110;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF116: ArchitectureImplementationId =
    ArchitectureImplementationId::GF116;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF117: ArchitectureImplementationId =
    ArchitectureImplementationId::GF117;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF118: ArchitectureImplementationId =
    ArchitectureImplementationId::GF118;
pub const NV_GPU_ARCH_IMPLEMENTATION_GF119: ArchitectureImplementationId =
    ArchitectureImplementationId::GF119;
pub const NV_GPU_ARCH_IMPLEMENTATION_GK104: ArchitectureImplementationId =
    ArchitectureImplementationId::GK104;
pub const NV_GPU_ARCH_IMPLEMENTATION_GK106: ArchitectureImplementationId =
    ArchitectureImplementationId::GK106;
pub const NV_GPU_ARCH_IMPLEMENTATION_GK107: ArchitectureImplementationId =
    ArchitectureImplementationId::GK107;
pub const NV_GPU_ARCH_IMPLEMENTATION_GK20A: ArchitectureImplementationId =
    ArchitectureImplementationId::GK20A;
pub const NV_GPU_ARCH_IMPLEMENTATION_GK110: ArchitectureImplementationId =
    ArchitectureImplementationId::GK110;
pub const NV_GPU_ARCH_IMPLEMENTATION_GK208: ArchitectureImplementationId =
    ArchitectureImplementationId::GK208;
pub const NV_GPU_ARCH_IMPLEMENTATION_GM204: ArchitectureImplementationId =
    ArchitectureImplementationId::GM204;
pub const NV_GPU_ARCH_IMPLEMENTATION_GM206: ArchitectureImplementationId =
    ArchitectureImplementationId::GM206;
pub const NV_GPU_ARCH_IMPLEMENTATION_GP100: ArchitectureImplementationId =
    ArchitectureImplementationId::GP100;
pub const NV_GPU_ARCH_IMPLEMENTATION_GP000: ArchitectureImplementationId =
    ArchitectureImplementationId::GP000;
pub const NV_GPU_ARCH_IMPLEMENTATION_GP102: ArchitectureImplementationId =
    ArchitectureImplementationId::GP102;
pub const NV_GPU_ARCH_IMPLEMENTATION_GP104: ArchitectureImplementationId =
    ArchitectureImplementationId::GP104;
pub const NV_GPU_ARCH_IMPLEMENTATION_GP106: ArchitectureImplementationId =
    ArchitectureImplementationId::GP106;
pub const NV_GPU_ARCH_IMPLEMENTATION_GP107: ArchitectureImplementationId =
    ArchitectureImplementationId::GP107;
pub const NV_GPU_ARCH_IMPLEMENTATION_GP108: ArchitectureImplementationId =
    ArchitectureImplementationId::GP108;
pub const NV_GPU_ARCH_IMPLEMENTATION_GV100: ArchitectureImplementationId =
    ArchitectureImplementationId::GV100;
pub const NV_GPU_ARCH_IMPLEMENTATION_GV10B: ArchitectureImplementationId =
    ArchitectureImplementationId::GV10B;
pub const NV_GPU_ARCH_IMPLEMENTATION_TU100: ArchitectureImplementationId =
    ArchitectureImplementationId::TU100;
pub const NV_GPU_ARCH_IMPLEMENTATION_TU102: ArchitectureImplementationId =
    ArchitectureImplementationId::TU102;
pub const NV_GPU_ARCH_IMPLEMENTATION_TU104: ArchitectureImplementationId =
    ArchitectureImplementationId::TU104;
pub const NV_GPU_ARCH_IMPLEMENTATION_TU106: ArchitectureImplementationId =
    ArchitectureImplementationId::TU106;
pub const NV_GPU_ARCH_IMPLEMENTATION_TU116: ArchitectureImplementationId =
    ArchitectureImplementationId::TU116;
pub const NV_GPU_ARCH_IMPLEMENTATION_TU117: ArchitectureImplementationId =
    ArchitectureImplementationId::TU117;
pub const NV_GPU_ARCH_IMPLEMENTATION_TU000: ArchitectureImplementationId =
    ArchitectureImplementationId::TU000;
pub const NV_GPU_ARCH_IMPLEMENTATION_GA100: ArchitectureImplementationId =
    ArchitectureImplementationId::GA100;
pub const NV_GPU_ARCH_IMPLEMENTATION_GA102: ArchitectureImplementationId =
    ArchitectureImplementationId::GA102;
pub const NV_GPU_ARCH_IMPLEMENTATION_GA104: ArchitectureImplementationId =
    ArchitectureImplementationId::GA104;
pub const NV_GPU_ARCH_IMPLEMENTATION_AD102: ArchitectureImplementationId =
    ArchitectureImplementationId::AD102;
pub const NV_GPU_ARCH_IMPLEMENTATION_AD103: ArchitectureImplementationId =
    ArchitectureImplementationId::AD103;
pub const NV_GPU_ARCH_IMPLEMENTATION_AD104: ArchitectureImplementationId =
    ArchitectureImplementationId::AD104;
pub const NV_GPU_ARCH_IMPLEMENTATION_GB202: ArchitectureImplementationId =
    ArchitectureImplementationId::GB202;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ChipRevision(u32);

impl ChipRevision {
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub const EMULATION_QT: Self = Self::new(0x00000000);
    pub const EMULATION_FPGA: Self = Self::new(0x00000001);
    pub const A01: Self = Self::new(0x00000011);
    pub const A02: Self = Self::new(0x00000012);
    pub const A03: Self = Self::new(0x00000013);
    pub const UNKNOWN: Self = Self::new(0xFFFFFFFF);

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub fn from_raw(raw: u32) -> Result<Self, ArgumentRangeError> {
        Ok(Self::new(raw))
    }

    pub fn values() -> impl Iterator<Item = Self> {
        CHIP_REVISION_VALUES.iter().copied()
    }

    pub fn name(&self) -> Option<&'static str> {
        match self.raw() {
            0x00000000 => Some("Emulation QT"),
            0x00000001 => Some("Emulation FPGA"),
            0x00000011 => Some("A01"),
            0x00000012 => Some("A02"),
            0x00000013 => Some("A03"),
            0xFFFFFFFF => Some("Unknown"),
            _ => None,
        }
    }
}

impl From<ChipRevision> for u32 {
    fn from(value: ChipRevision) -> Self {
        value.raw()
    }
}

const CHIP_REVISION_VALUES: [ChipRevision; 6] = [
    ChipRevision::EMULATION_QT,
    ChipRevision::EMULATION_FPGA,
    ChipRevision::A01,
    ChipRevision::A02,
    ChipRevision::A03,
    ChipRevision::UNKNOWN,
];

pub const NV_GPU_CHIP_REV_EMULATION_QT: ChipRevision = ChipRevision::EMULATION_QT;
pub const NV_GPU_CHIP_REV_EMULATION_FPGA: ChipRevision = ChipRevision::EMULATION_FPGA;
pub const NV_GPU_CHIP_REV_A01: ChipRevision = ChipRevision::A01;
pub const NV_GPU_CHIP_REV_A02: ChipRevision = ChipRevision::A02;
pub const NV_GPU_CHIP_REV_A03: ChipRevision = ChipRevision::A03;
pub const NV_GPU_CHIP_REV_UNKNOWN: ChipRevision = ChipRevision::UNKNOWN;

nvstruct! {
    pub struct NV_GPU_ARCH_INFO_V1 {
        pub version: u32,
        pub architecture: ArchitectureId,
        pub implementation: ArchitectureImplementationId,
        pub revision: ChipRevision,
    }
}

pub type NV_GPU_ARCH_INFO_V2 = NV_GPU_ARCH_INFO_V1;
pub type NV_GPU_ARCH_INFO = NV_GPU_ARCH_INFO_V2;

const NV_GPU_ARCH_INFO_V1_SIZE: usize = 4 * 4;
nvversion! { NV_GPU_ARCH_INFO_VER_1(NV_GPU_ARCH_INFO_V1 = NV_GPU_ARCH_INFO_V1_SIZE, 1) }
nvversion! { NV_GPU_ARCH_INFO_VER_2(NV_GPU_ARCH_INFO_V2 = NV_GPU_ARCH_INFO_V1_SIZE, 2) }
nvversion! { NV_GPU_ARCH_INFO_VER = NV_GPU_ARCH_INFO_VER_2 }

nvapi_fn! {
    pub type GPU_GetArchInfoFn = extern "C" fn(hPhysicalGpu: NvPhysicalGpuHandle, pGpuArchInfo: *mut NV_GPU_ARCH_INFO) -> NvAPI_Status;

    /// Retrieves architecture, implementation, and chip revision information for the specified GPU.
    pub unsafe fn NvAPI_GPU_GetArchInfo;
}
