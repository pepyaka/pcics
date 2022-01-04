//! Alternative Routing-ID Interpretation (ARI)
//!
//! Routing IDs, Requester IDs, and Completer IDs are 16-bit identifiers traditionally composed of
//! 20 three fields: an 8-bit Bus Number, a 5-bit Device Number, and a 3-bit Function Number. With
//! ARI, the 16-bit field is interpreted as two fields instead of three: an 8-bit Bus Number and an
//! 8-bit Function Number – the Device Number field is eliminated. This new interpretation enables
//! an ARI Device to support up to 256 Functions [0..255] instead of 8 Functions [0..7].


use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeRoutingIdInterpretation {
    pub ari_capability: AriCapability,
    pub ari_control: AriControl,
}
impl<'a> TryRead<'a, Endian> for AlternativeRoutingIdInterpretation {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let ari = AlternativeRoutingIdInterpretation {
            ari_capability: bytes.read_with::<u16>(offset, endian)?.into(),
            ari_control: bytes.read_with::<u16>(offset, endian)?.into(),
        };
        Ok((ari, *offset))
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct AriCapabilityProto {
    mfvc_function_groups_capability: bool,
    acs_function_groups_capability: bool,
    rsvdp: B6,
    next_function_number: u8,
}

/// ARI Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AriCapability {
    /// MFVC Function Groups Capability (M)
    pub mfvc_function_groups_capability: bool,
    /// ACS Function Groups Capability (A)
    pub acs_function_groups_capability: bool,
    /// Next Function Number
    pub next_function_number: u8,
}
impl From<AriCapabilityProto> for AriCapability {
    fn from(proto: AriCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            mfvc_function_groups_capability: proto.mfvc_function_groups_capability(),
            acs_function_groups_capability: proto.acs_function_groups_capability(),
            next_function_number: proto.next_function_number(),
        }
    }
}
impl From<u16> for AriCapability {
    fn from(word: u16) -> Self { AriCapabilityProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct AriControlProto {
    mfvc_function_groups_enable: bool,
    acs_function_groups_enable: bool,
    function_group: B6,
    rsvdp: B8,
}

/// ARI Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AriControl {
    /// MFVC Function Groups Enable (M)
    pub mfvc_function_groups_enable: bool,
    /// ACS Function Groups Enable (A)
    pub acs_function_groups_enable: bool,
    /// Function Group
    pub function_group: u8,
}
impl From<AriControlProto> for AriControl {
    fn from(proto: AriControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            mfvc_function_groups_enable: proto.mfvc_function_groups_enable(),
            acs_function_groups_enable: proto.acs_function_groups_enable(),
            function_group: proto.function_group(),
        }
    }
}
impl From<u16> for AriControl {
    fn from(word: u16) -> Self { AriControlProto::from(word).into() }
}
