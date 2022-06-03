/*!
# TPH Requester

TLP Processing Hints is an optional feature that provides hints in Request TLP headers to
facilitate optimized processing of Requests that target Memory Space.

## Struct diagram
[TphRequester]
- [TphRequesterCapability]
  - [StTable]
- [TphRequesterControl]
  - [StModeSelect]
  - [TphRequesterEnable]

## Examples

```rust
# use pcics::extended_capabilities::tph_requester::*;
use pretty_assertions::assert_eq;
let data = [
    0x17, 0x00, 0x01, 0x1c, 0x05, 0x02, 0x07, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
let result: TphRequester = data[4..].try_into().unwrap();
let sample = TphRequester {
    tph_requester_capability: TphRequesterCapability {
        no_st_mode_supported: true,
        interrupt_vector_mode_supported: false,
        device_specific_mode_supported: true,
        extended_tph_requester_supported: false,
        st_table: StTable::Valid { size: 0x07, data: [0u8; 16].as_slice() },
    },
    tph_requester_control: TphRequesterControl {
        st_mode_select: StModeSelect::NoStMode,
        tph_requester_enable: TphRequesterEnable::NotPermitted,
    },
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P4, P9};

use super::ExtendedCapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphRequester<'a> {
    /// TPH Requester Capability
    pub tph_requester_capability: TphRequesterCapability<'a>,
    /// TPH Requester Control
    pub tph_requester_control: TphRequesterControl,
}

impl<'a> TryFrom<&'a [u8]> for TphRequester<'a> {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((tph_requester_capability, tph_requester_control)),
            tail,
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "TPH Requester",
                size: 8,
            })?;
        let Lsb((
            no_st_mode_supported,
            interrupt_vector_mode_supported,
            device_specific_mode_supported,
            (),
            extended_tph_requester_supported,
            st_table_location,
            (),
            st_table_size,
            (),
        )) = P9::<u32, 1, 1, 1, 5, 1, 2, 5, 11, 5>(tph_requester_capability).into();
        let _: (u8, u16) = (st_table_location, st_table_size);
        // 0Ch + (ST Table Size * 02h) + 02h
        let end = (st_table_size as usize) * 2 + 2;
        let st_table = match st_table_location {
            0b00 => StTable::NotPresent,
            0b01 => {
                if let (0..=63, Some(data)) = (st_table_size, tail.get(..end)) {
                    StTable::Valid {
                        size: st_table_size,
                        data,
                    }
                } else {
                    StTable::Invalid {
                        size: st_table_size,
                        data: tail,
                    }
                }
            }
            0b10 => StTable::MsiXTable {
                size: st_table_size,
            },
            0b11 => StTable::Reserved,
            _ => unreachable!(),
        };
        Ok(Self {
            tph_requester_capability: TphRequesterCapability {
                no_st_mode_supported,
                interrupt_vector_mode_supported,
                device_specific_mode_supported,
                extended_tph_requester_supported,
                st_table,
            },
            tph_requester_control: From::<u32>::from(tph_requester_control),
        })
    }
}

/// TPH Requester Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphRequesterCapability<'a> {
    /// No ST Mode Supported
    pub no_st_mode_supported: bool,
    /// Interrupt Vector Mode Supported
    pub interrupt_vector_mode_supported: bool,
    /// Device Specific Mode Supported
    pub device_specific_mode_supported: bool,
    /// Extended TPH Requester Supported
    pub extended_tph_requester_supported: bool,
    /// ST Table
    pub st_table: StTable<'a>,
}

/// Indicates if and where the ST Table is located
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StTable<'a> {
    /// ST Table is not present
    NotPresent,
    /// ST Table is located in the TPH Requester Capability structure and all bytes are readable
    Valid { size: u16, data: &'a [u8] },
    /// ST Table is located in the TPH Requester Capability structure, but has invalid size.
    ///
    /// Bytes slice may be shorter or limit of 64 entries exceeded
    Invalid { size: u16, data: &'a [u8] },
    /// ST Table is located in the MSI-X Table
    MsiXTable { size: u16 },
    /// Reserved
    Reserved,
}

/// Each implemented ST Entry is 16 bits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphStTableEntry {
    pub st_lower: u8,
    pub st_upper: u8,
}

/// TPH Requester Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphRequesterControl {
    /// ST Mode Select
    pub st_mode_select: StModeSelect,
    /// TPH Requester Enable
    pub tph_requester_enable: TphRequesterEnable,
}

impl From<u32> for TphRequesterControl {
    fn from(dword: u32) -> Self {
        let Lsb((st_mode_select, (), tph_requester_enable, ())) =
            P4::<_, 3, 5, 2, 22>(dword).into();
        Self {
            st_mode_select: From::<u8>::from(st_mode_select),
            tph_requester_enable: From::<u8>::from(tph_requester_enable),
        }
    }
}

/// Selects the ST Mode of operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StModeSelect {
    /// No ST Mode
    NoStMode,
    /// Interrupt Vector Mode
    InterruptVectorMode,
    /// Device Specific Mode
    DeviceSpecificMode,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for StModeSelect {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NoStMode,
            0b01 => Self::InterruptVectorMode,
            0b10 => Self::DeviceSpecificMode,
            v => Self::Reserved(v),
        }
    }
}

/// Controls the ability to issue Request TLPs using either TPH or Extended TPH
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TphRequesterEnable {
    /// Function operating as a Requester is not permitted to issue Requests with TPH or Extended
    /// TPH
    NotPermitted,
    /// Function operating as a Requester is permitted to issue Requests with TPH and is not
    /// permitted to issue Requests with Extended TPH
    TphPermitted,
    /// Reserved
    Reserved,
    /// Function operating as a Requester is permitted to issue Requests with TPH and Extended TPH
    TphAndExtendedTphPermitted,
}
impl From<u8> for TphRequesterEnable {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NotPermitted,
            0b01 => Self::TphPermitted,
            0b10 => Self::Reserved,
            0b11 => Self::TphAndExtendedTphPermitted,
            _ => unreachable!(),
        }
    }
}
