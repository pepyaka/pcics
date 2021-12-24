//! Advanced Features
//!
//! For conventional PCI devices integrated into a PCI Express Root Complex, the Advanced Features
//! (AF) capability provides mechanisms for using advanced features originally devleoped for PCI
//! Express

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancedFeatures {
    /// AF Structure Length (Bytes). Shall return a value of 06h.
    pub length: u8,
    pub capabilities: Capabilities,
    pub control: Control,
    pub status: Status,
}
impl<'a> TryRead<'a, Endian> for AdvancedFeatures {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let af = AdvancedFeatures {
            length: bytes.read_with::<u8>(offset, endian)?,
            capabilities: bytes.read_with::<u8>(offset, endian)?.into(),
            control: bytes.read_with::<u8>(offset, endian)?.into(),
            status: bytes.read_with::<u8>(offset, endian)?.into(),
        };
        Ok((af, *offset))
    }
}

#[bitfield(bits = 8)]
#[repr(u8)]
pub struct CapabilitiesProto {
    transactions_pending: bool,
    function_level_reset: bool,
    reserved: B6,
}

/// AF Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    /// Indicate support for the Transactions Pending (TP) bit. TP must be supported if FLR is
    /// supported.
    pub transactions_pending: bool,
    /// indicate support for Function Level Reset (FLR).
    pub function_level_reset: bool,
}
impl From<CapabilitiesProto> for Capabilities {
    fn from(proto: CapabilitiesProto) -> Self {
        let _ = proto.reserved();
        Self {
            transactions_pending: proto.transactions_pending(),
            function_level_reset: proto.function_level_reset(),
        }
    }
}
impl From<u8> for Capabilities {
    fn from(byte: u8) -> Self { CapabilitiesProto::from(byte).into() }
}

#[bitfield(bits = 8)]
#[repr(u8)]
pub struct ControlProto {
    initiate_flr: bool,
    reserved: B7,
}

/// AF Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
    /// A write of 1b initiates Function Level Reset (FLR). The value read by software from this
    /// bit shall always be 0b.
    pub initiate_flr: bool,
}
impl From<ControlProto> for Control {
    fn from(proto: ControlProto) -> Self {
        let _ = proto.reserved();
        Self {
            initiate_flr: proto.initiate_flr(),
        }
    }
}
impl From<u8> for Control {
    fn from(byte: u8) -> Self { ControlProto::from(byte).into() }
}


#[bitfield(bits = 8)]
#[repr(u8)]
pub struct StatusProto {
    transactions_pending: bool,
    reserved: B7,
}

/// AF Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    /// Indicates that the Function has issued one or more non-posted transactions which have not
    /// been completed, including non-posted transactions that a target has terminated with Retry
    pub transactions_pending: bool,
}
impl From<StatusProto> for Status {
    fn from(proto: StatusProto) -> Self {
        let _ = proto.reserved();
        Self {
            transactions_pending: proto.transactions_pending(),
        }
    }
}
impl From<u8> for Status {
    fn from(byte: u8) -> Self { StatusProto::from(byte).into() }
}
