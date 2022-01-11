//! Access Control Services (ACS)
//!
//! ACS defines a set of control points within a PCI Express topology to determine whether a TLP is
//! to be routed normally, blocked, or redirected.

use core::slice::Chunks;

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

use super::ECH_BYTES;


/// Egress Control Vector is DWORD
const ECV_BYTES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessControlServices<'a> {
    data: &'a [u8],
    /// ACS Capability
    pub acs_capability: AcsCapability,
    /// ACS Control
    pub acs_control: AcsControl,
}

impl<'a> AccessControlServices<'a> {
    pub fn egress_control_vectors(&self) -> EgressControlVectors<'a> {
        let size = self.acs_capability.egress_control_vector_size as usize;
        let start = 0x08 - ECH_BYTES;
        let end = start + size / (u32::BITS as usize) * ECV_BYTES;
        EgressControlVectors::new(&self.data[start..end], size)
    }
}

impl<'a> TryRead<'a, Endian> for AccessControlServices<'a> {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let acs = AccessControlServices {
            data: bytes,
            acs_capability: bytes.read_with::<u16>(offset, endian)?.into(),
            acs_control: bytes.read_with::<u16>(offset, endian)?.into(),
        };
        Ok((acs, *offset))
    }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct AcsCapabilityProto {
    acs_source_validation: bool,
    acs_translation_blocking: bool,
    acs_p2p_request_redirect: bool,
    acs_p2p_completion_redirect: bool,
    acs_upstream_forwarding: bool,
    acs_p2p_egress_control: bool,
    acs_direct_translated_p2p: bool,
    rsvdp: B1,
    egress_control_vector_size: u8,
}

/// ACS Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcsCapability {
    /// ACS Source Validation (V)
    pub acs_source_validation: bool,
    /// ACS Translation Blocking (B)
    pub acs_translation_blocking: bool,
    /// ACS P2P Request Redirect (R)
    pub acs_p2p_request_redirect: bool,
    /// ACS P2P Completion Redirect (C)
    pub acs_p2p_completion_redirect: bool,
    /// ACS Upstream Forwarding (U)
    pub acs_upstream_forwarding: bool,
    /// ACS P2P Egress Control (E)
    pub acs_p2p_egress_control: bool,
    /// ACS Direct Translated P2P (T)
    pub acs_direct_translated_p2p: bool,
    /// Egress Control Vector Size
    pub egress_control_vector_size: u8,
}
impl From<AcsCapabilityProto> for AcsCapability {
    fn from(proto: AcsCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            acs_source_validation: proto.acs_source_validation(),
            acs_translation_blocking: proto.acs_translation_blocking(),
            acs_p2p_request_redirect: proto.acs_p2p_request_redirect(),
            acs_p2p_completion_redirect: proto.acs_p2p_completion_redirect(),
            acs_upstream_forwarding: proto.acs_upstream_forwarding(),
            acs_p2p_egress_control: proto.acs_p2p_egress_control(),
            acs_direct_translated_p2p: proto.acs_direct_translated_p2p(),
            egress_control_vector_size: proto.egress_control_vector_size(),
        }
    }
}
impl From<u16> for AcsCapability {
    fn from(word: u16) -> Self { AcsCapabilityProto::from(word).into() }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct AcsControlProto {
    acs_source_validation_enable: bool,
    acs_translation_blocking_enable: bool,
    acs_p2p_request_redirect_enable: bool,
    acs_p2p_completion_redirect_enable: bool,
    acs_upstream_forwarding_enable: bool,
    acs_p2p_egress_control_enable: bool,
    acs_direct_translated_p2p_enable: bool,
    rsvdp: B9,
}

/// ACS Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcsControl {
    /// ACS Source Validation Enable (V)
    pub acs_source_validation_enable: bool,
    /// ACS Translation Blocking Enable (B)
    pub acs_translation_blocking_enable: bool,
    /// ACS P2P Request Redirect Enable (R)
    pub acs_p2p_request_redirect_enable: bool,
    /// ACS P2P Completion Redirect Enable (C)
    pub acs_p2p_completion_redirect_enable: bool,
    /// ACS Upstream Forwarding Enable (U)
    pub acs_upstream_forwarding_enable: bool,
    /// ACS P2P Egress Control Enable (E)
    pub acs_p2p_egress_control_enable: bool,
    /// ACS Direct Translated P2P Enable (T)
    pub acs_direct_translated_p2p_enable: bool,
}
impl From<AcsControlProto> for AcsControl {
    fn from(proto: AcsControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            acs_source_validation_enable: proto.acs_source_validation_enable(),
            acs_translation_blocking_enable: proto.acs_translation_blocking_enable(),
            acs_p2p_request_redirect_enable: proto.acs_p2p_request_redirect_enable(),
            acs_p2p_completion_redirect_enable: proto.acs_p2p_completion_redirect_enable(),
            acs_upstream_forwarding_enable: proto.acs_upstream_forwarding_enable(),
            acs_p2p_egress_control_enable: proto.acs_p2p_egress_control_enable(),
            acs_direct_translated_p2p_enable: proto.acs_direct_translated_p2p_enable(),
        }
    }
}
impl From<u16> for AcsControl {
    fn from(word: u16) -> Self { AcsControlProto::from(word).into() }
}


/// An iterator through bits controlled the blocking or redirecting of  peer-to-peer Requests
/// targeting the associated Port, Function, or Function Group.
#[derive(Debug, Clone)]
pub struct EgressControlVectors<'a> {
    chunks: Chunks<'a, u8>,
    dword: u32,
    mask: u32,
    size: usize,
}
impl<'a> EgressControlVectors<'a> {
    pub fn new(data: &'a [u8], size: usize) -> Self {
        Self { chunks: data.chunks(ECV_BYTES), dword: 0, mask: 1, size } 
    }
}
impl<'a> PartialEq for EgressControlVectors<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.chunks.clone().eq(other.chunks.clone())
        && self.dword == other.dword
        && self.mask == other.mask
        && self.size == other.size
    }
}
impl<'a> Eq for EgressControlVectors<'a> {}
impl<'a> Iterator for EgressControlVectors<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        if self.mask == 1 {
            let bytes = self.chunks.next()?;
            self.dword = u32::from_le_bytes(
                match bytes {
                    [a,b,c,d] => [*a,*b,*c,*d],
                    [a,b,c] => [*a,*b,*c,0],
                    [a,b] => [*a,*b,0,0],
                    [a] => [*a,0,0,0],
                    _ => unreachable!(),
                }
            );
        }
        let result = (self.dword & self.mask) != 0;
        self.mask = self.mask.rotate_left(1);
        self.size -= 1;
        Some(result)
    }
}


#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn egress_control_vectors() {
        let data = [0x00,0x0F,0xAA,0xFF,0x55,0x00,0x00,0x00];
        let result = EgressControlVectors::new(&data, 35).collect::<Vec<_>>();
        let sample = [
            0,0,0,0,0,0,0,0, 1,1,1,1,0,0,0,0, 0,1,0,1,0,1,0,1, 1,1,1,1,1,1,1,1, 
            1,0,1,0,1,0,1,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
        ].iter().take(35).map(|&v| v != 0).collect::<Vec<_>>();
        println!("{:?}", &sample);
        println!("{:?}", &result);
        assert_eq!(sample, result);
    }
}
