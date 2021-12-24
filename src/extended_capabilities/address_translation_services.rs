//! Address Translation Services (ATS)

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressTranslationServices {
    /// ATS Capability
    pub ats_capability: AtsCapability,
    /// ATS Control
    pub ats_control: AtsControl,
}
impl<'a> TryRead<'a, Endian> for AddressTranslationServices {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let ats = AddressTranslationServices {
            ats_capability: bytes.read_with::<u16>(offset, endian)?.into(),
            ats_control: bytes.read_with::<u16>(offset, endian)?.into(),
        };
        Ok((ats, *offset))
    }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct AtsCapabilityProto {
    invalidate_queue_depth: B5,
    page_aligned_request: bool,
    global_invalidate_supported: bool,
    rsvdp: B9,
}

/// ATS Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtsCapability {
    /// Invalidate Queue Depth
    pub invalidate_queue_depth: u8,
    /// Page Aligned Request
    pub page_aligned_request: bool,
    /// Global Invalidate Supported
    pub global_invalidate_supported: bool,
}
impl From<AtsCapabilityProto> for AtsCapability {
    fn from(proto: AtsCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            invalidate_queue_depth: proto.invalidate_queue_depth(),
            page_aligned_request: proto.page_aligned_request(),
            global_invalidate_supported: proto.global_invalidate_supported(),
        }
    }
}
impl From<u16> for AtsCapability {
    fn from(word: u16) -> Self { AtsCapabilityProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct AtsControlProto {
    smallest_translation_unit: B5,
    rsvdp: B10,
    enable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtsControl {
    /// Smallest Translation Unit (STU)
    pub smallest_translation_unit: u8,
    /// Enable (E)
    pub enable: bool,
}
impl From<AtsControlProto> for AtsControl {
    fn from(proto: AtsControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            smallest_translation_unit: proto.smallest_translation_unit(),
            enable: proto.enable(),
        }
    }
}
impl From<u16> for AtsControl {
    fn from(word: u16) -> Self { AtsControlProto::from(word).into() }
}

