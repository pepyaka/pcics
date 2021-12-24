//! Message Signaled Interrupts Extension (MSI-X)
//!
//! MSI-X defines a separate optional extension to basic MSI functionality. Compared to MSI, MSI-X
//! supports a larger maximum number of vectors per function, the ability for software to control
//! aliasing when fewer vectors are allocated than requested, plus the ability for each vector to
//! use an independent address and data value, specified by a table that resides in Memory Space.
//! However, most of the other characteristics of MSI-X are identical to those of MSI. 

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// In contrast to the [MSI](super::MessageSignaledInterrups) capability, which directly contains all of
/// the control/status information for the function's vectors, the MSI-X capability structure
/// instead points to an (MSI-X Table)[Table] structure and a (MSI-X Pending Bit Array
/// (PBA))[PendingBitArray] structure, each residing in Memory Space. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsiX {
    pub message_control: MessageControl,
    pub table: Table,
    pub pending_bit_array: PendingBitArray,
}
impl<'a> TryRead<'a, Endian> for MsiX {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let msix = MsiX {
            message_control: bytes.read_with::<u16>(offset, endian)?.into(),
            table: bytes.read_with::<u32>(offset, endian)?.into(),
            pending_bit_array: bytes.read_with::<u32>(offset, endian)?.into(),
        };
        Ok((msix, *offset))
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct MessageControlProto {
    table_size: B11,
    reserved: B3,
    function_mask: bool,
    msi_x_enable: bool,
}

/// Message Control for MSI-X 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageControl {
    /// Table Size
    pub table_size: u16,
    /// Function Mask
    pub function_mask: bool,
    /// MSI-X Enable
    pub msi_x_enable: bool,
}
impl From<MessageControlProto> for MessageControl {
    fn from(proto: MessageControlProto) -> Self {
        let _ = proto.reserved();
        Self {
            table_size: proto.table_size(),
            function_mask: proto.function_mask(),
            msi_x_enable: proto.msi_x_enable(),
        }
    }
}
impl From<u16> for MessageControl {
    fn from(word: u16) -> Self { MessageControlProto::from(word).into() }
}

/// BAR Indicator register (BIR) indicates which BAR, and a QWORD-aligned Offset indicates where
/// the structure begins relative to the base address associated with the BAR
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bir {
    Bar10h,
    Bar14h,
    Bar18h,
    Bar1Ch,
    Bar20h,
    Bar24h,
    Reserved(u8),
}
impl From<u8> for Bir {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::Bar10h,
            1 => Self::Bar14h,
            2 => Self::Bar18h,
            3 => Self::Bar1Ch,
            4 => Self::Bar20h,
            5 => Self::Bar24h,
            v => Self::Reserved(v),
        }
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct TableProto {
    bir: B3,
    offset: B29,
}

/// Table Offset/Table BIR for MSI-X
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub bir: Bir,
    /// Used as an offset from the address contained by one of the function’s Base Address
    /// registers to point to the base of the MSI-X Table
    pub offset: u32,
}
impl From<TableProto> for Table {
    fn from(proto: TableProto) -> Self {
        Self {
            bir: proto.bir().into(),
            offset: proto.offset() << 3,
        }
    }
}
impl From<u32> for Table {
    fn from(word: u32) -> Self { TableProto::from(word).into() }
}



#[bitfield(bits = 32)]
#[repr(u32)]
pub struct PendingBitArrayProto {
    bir: B3,
    offset: B29,
}

/// PBA Offset/PBA BIR for MSI-X
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingBitArray {
    pub bir: Bir,
    /// Used as an offset from the address contained by one of the function’s Base Address
    /// registers to point to the base of the MSI-X PBA.
    pub offset: u32,
}
impl From<PendingBitArrayProto> for PendingBitArray {
    fn from(proto: PendingBitArrayProto) -> Self {
        Self {
            bir: proto.bir().into(),
            offset: proto.offset() << 3,
        }
    }
}
impl From<u32> for PendingBitArray {
    fn from(word: u32) -> Self { PendingBitArrayProto::from(word).into() }
}
