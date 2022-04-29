//! Message Signaled Interrupts Extension (MSI-X)
//!
//! MSI-X defines a separate optional extension to basic MSI functionality. Compared to MSI, MSI-X
//! supports a larger maximum number of vectors per function, the ability for software to control
//! aliasing when fewer vectors are allocated than requested, plus the ability for each vector to
//! use an independent address and data value, specified by a table that resides in Memory Space.
//! However, most of the other characteristics of MSI-X are identical to those of MSI.

use heterob::{bit_numbering::Lsb, endianness::Le, P2, P3, P4};

use super::CapabilityDataError;

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
impl MsiX {
    pub const SIZE: usize = 2 + 4 + 4;
}
impl From<[u8; MsiX::SIZE]> for MsiX {
    fn from(bytes: [u8; MsiX::SIZE]) -> Self {
        let Le((mc, t, pba)) = P3(bytes).into();
        Self {
            message_control: From::<u16>::from(mc),
            table: From::<u32>::from(t),
            pending_bit_array: From::<u32>::from(pba),
        }
    }
}
impl<'a> TryFrom<&'a [u8]> for MsiX {
    type Error = CapabilityDataError;
    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        slice
            .get(..Self::SIZE)
            .and_then(|slice| <[u8; Self::SIZE]>::try_from(slice).ok())
            .ok_or(CapabilityDataError {
                name: "MSI-X",
                size: Self::SIZE,
            })
            .map(Self::from)
    }
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
impl From<u16> for MessageControl {
    fn from(word: u16) -> Self {
        let Lsb((table_size, (), function_mask, msi_x_enable)) = P4::<_, 11, 3, 1, 1>(word).into();
        Self {
            table_size,
            function_mask,
            msi_x_enable,
        }
    }
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

/// Table Offset/Table BIR for MSI-X
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub bir: Bir,
    /// Used as an offset from the address contained by one of the function’s Base Address
    /// registers to point to the base of the MSI-X Table
    pub offset: u32,
}
impl From<u32> for Table {
    fn from(word: u32) -> Self {
        let Lsb((bir, offset)) = P2::<_, 3, 29>(word).into();
        let _: u32 = offset;
        Self {
            bir: From::<u8>::from(bir),
            offset: offset << 3,
        }
    }
}

/// PBA Offset/PBA BIR for MSI-X
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingBitArray {
    pub bir: Bir,
    /// Used as an offset from the address contained by one of the function’s Base Address
    /// registers to point to the base of the MSI-X PBA.
    pub offset: u32,
}
impl From<u32> for PendingBitArray {
    fn from(word: u32) -> Self {
        let Lsb((bir, offset)) = P2::<_, 3, 29>(word).into();
        let _: u32 = offset;
        Self {
            bir: From::<u8>::from(bir),
            offset: offset << 3,
        }
    }
}
