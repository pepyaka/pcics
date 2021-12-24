//! Process Address Space ID (PASID)
//!
//! The presence of a PASID Extended Capability indicates that the Endpoint supports sending and
//! receiving TLPs containing a PASID TLP Prefix.


use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessAddressSpaceId {
    pub pacid_capability: PacidCapability,
    pub pacid_control: PacidControl,
}
impl<'a> TryRead<'a, Endian> for ProcessAddressSpaceId {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let pacid = ProcessAddressSpaceId {
            pacid_capability: bytes.read_with::<u16>(offset, endian)?.into(),
            pacid_control: bytes.read_with::<u16>(offset, endian)?.into(),
        };
        Ok((pacid, *offset))
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct PacidCapabilityProto {
    rsvdp: B1,
    execute_permission_supported: bool,
    privileged_mode_supported: bool,
    rsvdp_2: B5,
    max_pasid_width: B5,
    rsvdp_3: B3,
}

/// PASID Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacidCapability {
    /// Execute Permission Supported
    pub execute_permission_supported: bool,
    /// Privileged Mode Supported
    pub privileged_mode_supported: bool,
    /// Max PASID Width
    pub max_pasid_width: u8,
}
impl From<PacidCapabilityProto> for PacidCapability {
    fn from(proto: PacidCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        let _ = proto.rsvdp_3();
        Self {
            execute_permission_supported: proto.execute_permission_supported(),
            privileged_mode_supported: proto.privileged_mode_supported(),
            max_pasid_width: proto.max_pasid_width(),
        }
    }
}
impl From<u16> for PacidCapability {
    fn from(word: u16) -> Self { PacidCapabilityProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct PacidControlProto {
    pasid_enable: bool,
    execute_permission_enable: bool,
    privileged_mode_enable: bool,
    rsvdp: B13,
}

/// PASID Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacidControl {
    /// PASID Enable
    pub pasid_enable: bool,
    /// Execute Permission Enable
    pub execute_permission_enable: bool,
    /// Privileged Mode Enable
    pub privileged_mode_enable: bool,
}
impl From<PacidControlProto> for PacidControl {
    fn from(proto: PacidControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            pasid_enable: proto.pasid_enable(),
            execute_permission_enable: proto.execute_permission_enable(),
            privileged_mode_enable: proto.privileged_mode_enable(),
        }
    }
}
impl From<u16> for PacidControl {
    fn from(word: u16) -> Self { PacidControlProto::from(word).into() }
}

