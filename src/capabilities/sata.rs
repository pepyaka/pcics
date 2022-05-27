/*!
# Serial ATA Data/Index Configuration

Cpability used for the Index-Data Pair mechanism.

## Struct diagram
[Sata]
- [Revision]
- [BarOffset]
- [BarLocation]

## Examples

> SATA HBA v1.0 BAR4 Offset=00000004

```rust
# use pcics::capabilities::sata::*;
let data = [0x12, 0x00, 0x10, 0x00, 0x48, 0x00, 0x00, 0x00,];
let result = data[2..].try_into().unwrap();
let sample = Sata {
    revision: Revision { minor: 0, major: 1 },
    bar_offset: BarOffset(0x04),
    bar_location: BarLocation::Bar4,
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P3};

use super::CapabilityDataError;

/// Slave/Primary Interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sata {
    pub revision: Revision,
    /// BAR Offset
    pub bar_offset: BarOffset,
    /// BAR Location
    pub bar_location: BarLocation,
}
impl TryFrom<&[u8]> for Sata {
    type Error = CapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((revision, rsvd, sata_cap1)),
            ..
        } = P3(slice).try_into().map_err(|_| CapabilityDataError {
            name: "Serial ATA",
            size: 6,
        })?;
        let _: u8 = rsvd;
        let Lsb((minor, major)) = P2::<u8, 4, 4>(revision).into();
        let Lsb((bar_location, bar_offset, ())) = P3::<u32, 4, 20, 8>(sata_cap1).into();
        Ok(Self {
            revision: Revision { minor, major },
            bar_offset: BarOffset(bar_offset),
            bar_location: From::<u8>::from(bar_location),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Revision {
    /// Minor Revision
    pub minor: u8,
    /// Major Revision
    pub major: u8,
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
