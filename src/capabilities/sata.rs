//! Serial ATA Data/Index Configuration

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// Slave/Primary Interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sata {
    pub revision: Revision,
    /// BAR Offset
    pub bar_offset: BarOffset,
    /// BAR Location
    pub bar_location: BarLocation,
}
impl<'a> TryRead<'a, Endian> for Sata {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let revision = bytes.read_with::<u8>(offset, endian)?.into();
        let _reserved = bytes.read_with::<u8>(offset, endian)?;
        let sata_cap1_proto: SataCapability1Proto =
            bytes.read_with::<u32>(offset, endian)?.into();
        let _ = sata_cap1_proto.rsvdp();
        let sata = Sata {
            revision,
            bar_location: sata_cap1_proto.bar_location().into(),
            bar_offset: BarOffset(sata_cap1_proto.bar_offset()),
        };
        Ok((sata, *offset))
    }
}

#[bitfield(bits = 8)]
#[repr(u8)]
pub struct RevisionProto {
    minor: B4,
    major: B4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Revision {
    /// Minor Revision
    pub minor: u8,
    /// Major Revision
    pub major: u8,
}
impl From<RevisionProto> for Revision {
    fn from(proto: RevisionProto) -> Self {
        Self {
            minor: proto.minor(),
            major: proto.major(),
        }
    }
}
impl From<u8> for Revision {
    fn from(byte: u8) -> Self { RevisionProto::from(byte).into() }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct SataCapability1Proto {
    bar_location: B4,
    bar_offset: B20,
    rsvdp: B8,
}


/// Indicates the absolute PCI Configuration Register address of the BAR containing the Index-Data
/// Pair in Dword granularity
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarLocation {
    /// 10h (BAR0)
    Bar0,
    /// 14h (BAR1)
    Bar1,
    /// 18h (BAR2)
    Bar2,
    /// 1Ch (BAR3)
    Bar3,
    /// 20h (BAR4)
    Bar4,
    /// 24h (BAR5)
    Bar5,
    /// Index-Data Pair is implemented in Dwords directly following SATACR1 in the PCI
    /// configuration space
    SataCapability1,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for BarLocation {
    fn from(byte: u8) -> Self {
        match byte {
            0b0100 => Self::Bar0,
            0b0101 => Self::Bar1,
            0b0110 => Self::Bar2,
            0b0111 => Self::Bar3,
            0b1000 => Self::Bar4,
            0b1001 => Self::Bar5,
            0b1111 => Self::SataCapability1,
            v => Self::Reserved(v),
        }
    }
}


///  Indicates the offset into the BAR where the Index-Data Pair are located in Dword granularity
///
///  - Maximum if Index-Data Pair is implemented in IO Space from 0 – 64 KB
///  - Maximum if Index-Data Pair is memory mapped in the 0 – (1MB – 4) range)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarOffset(pub u32);
impl BarOffset {
    pub fn value(&self) -> u32 {
        self.0 * 4
    }
}
