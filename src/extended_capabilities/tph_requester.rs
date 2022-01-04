//! TPH Requester
//!
//! TLP Processing Hints is an optional feature that provides hints in Request TLP headers to
//! facilitate optimized processing of Requests that target Memory Space.


use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphRequester<'a> {
    data: &'a [u8],
    /// TPH Requester Capability
    pub tph_requester_capability: TphRequesterCapability,
    /// TPH Requester Control
    pub tph_requester_control: TphRequesterControl,
}

impl<'a> TphRequester<'a> {
    pub fn tph_st_table(&self) -> TphStTable {
        TphStTable::new(self.data)
    }
}
impl<'a> TryRead<'a, Endian> for TphRequester<'a> {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let tphr = TphRequester {
            data: bytes,
            tph_requester_capability: bytes.read_with::<u32>(offset, endian)?.into(),
            tph_requester_control: bytes.read_with::<u32>(offset, endian)?.into(),
        };
        Ok((tphr, *offset))
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct TphRequesterCapabilityProto {
    no_st_mode_supported: bool,
    interrupt_vector_mode_supported: bool,
    device_specific_mode_supported: bool,
    rsvdp: B5,
    extended_tph_requester_supported: bool,
    st_table_location: B2,
    rsvdp_2: B5,
    st_table_size: B11,
    rsvdp_3: B5,
}

/// TPH Requester Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphRequesterCapability {
    /// No ST Mode Supported
    pub no_st_mode_supported: bool,
    /// Interrupt Vector Mode Supported
    pub interrupt_vector_mode_supported: bool,
    /// Device Specific Mode Supported
    pub device_specific_mode_supported: bool,
    /// Extended TPH Requester Supported
    pub extended_tph_requester_supported: bool,
    /// ST Table Location
    pub st_table_location: StTableLocation,
    /// ST Table Size
    pub st_table_size: u16,
}
impl From<TphRequesterCapabilityProto> for TphRequesterCapability {
    fn from(proto: TphRequesterCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        let _ = proto.rsvdp_3();
        Self {
            no_st_mode_supported: proto.no_st_mode_supported(),
            interrupt_vector_mode_supported: proto.interrupt_vector_mode_supported(),
            device_specific_mode_supported: proto.device_specific_mode_supported(),
            extended_tph_requester_supported: proto.extended_tph_requester_supported(),
            st_table_location: proto.st_table_location().into(),
            st_table_size: proto.st_table_size(),
        }
    }
}
impl From<u32> for TphRequesterCapability {
    fn from(dword: u32) -> Self { TphRequesterCapabilityProto::from(dword).into() }
}

/// Indicates if and where the ST Table is located
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StTableLocation {
    /// ST Table is not present
    NotPresent,
    /// ST Table is located in the TPH Requester Capability structure
    TphRequesterCapability,
    /// ST Table is located in the MSI-X Table
    MsiXTable,
    /// Reserved
    Reserved,
}
impl From<u8> for StTableLocation {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NotPresent,
            0b01 => Self::TphRequesterCapability,
            0b10 => Self::MsiXTable,
            0b11 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct TphRequesterControlProto {
    st_mode_select: B3,
    rsvdp: B5,
    tph_requester_enable: B2,
    rsvdp_2: B22,
}

/// TPH Requester Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphRequesterControl {
    /// ST Mode Select
    pub st_mode_select: StModeSelect,
    /// TPH Requester Enable
    pub tph_requester_enable: TphRequesterEnable,
}
impl From<TphRequesterControlProto> for TphRequesterControl {
    fn from(proto: TphRequesterControlProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            st_mode_select: proto.st_mode_select().into(),
            tph_requester_enable: proto.tph_requester_enable().into(),
        }
    }
}
impl From<u32> for TphRequesterControl {
    fn from(dword: u32) -> Self { TphRequesterControlProto::from(dword).into() }
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

/// TPH ST Table - an iterator through ST Table entries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphStTable<'a>(&'a [u8]);
impl<'a> TphStTable<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }
}
impl<'a> Iterator for TphStTable<'a> {
    type Item = TphStTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Each implemented ST Entry is 16 bits 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TphStTableEntry {
    pub st_lower: u8,
    pub st_upper: u8,
}
