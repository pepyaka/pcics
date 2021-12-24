//! Precision Time Measurement (PTM)
//!
//! Precision Time Measurement (PTM) enables precise coordination of events across multiple
//! components with independent local time clocks.

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecisionTimeMeasurement {
    /// PTM Capability
    pub ptm_capability: PtmCapability,
    /// PTM Control
    pub ptm_control: PtmControl,
}
impl<'a> TryRead<'a, Endian> for PrecisionTimeMeasurement {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let ptm = PrecisionTimeMeasurement {
            ptm_capability: bytes.read_with::<u32>(offset, endian)?.into(),
            ptm_control: bytes.read_with::<u32>(offset, endian)?.into(),
        };
        Ok((ptm, *offset))
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct PtmCapabilityProto {
    ptm_requester_capable: bool,
    ptm_responder_capable: bool,
    ptm_root_capable: bool,
    rsvdp: B5,
    local_clock_granularity: u8,
    rsvdp_2: B16,
}
impl From<PtmCapability> for PtmCapabilityProto {
    fn from(data: PtmCapability) -> Self {
        Self::new()
            .with_ptm_requester_capable(data.ptm_requester_capable)
            .with_ptm_responder_capable(data.ptm_responder_capable)
            .with_ptm_root_capable(data.ptm_root_capable)
            .with_rsvdp(0)
            .with_local_clock_granularity(data.local_clock_granularity)
            .with_rsvdp_2(0)
    }
}

/// Describes a Function’s support for Precision Time Measurement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtmCapability {
    /// PTM Requester Capable
    pub ptm_requester_capable: bool,
    /// PTM Responder Capable
    pub ptm_responder_capable: bool,
    /// PTM Root Capable
    pub ptm_root_capable: bool,
    /// Local Clock Granularity
    pub local_clock_granularity: u8,
}
impl From<PtmCapabilityProto> for PtmCapability {
    fn from(proto: PtmCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            ptm_requester_capable: proto.ptm_requester_capable(),
            ptm_responder_capable: proto.ptm_responder_capable(),
            ptm_root_capable: proto.ptm_root_capable(),
            local_clock_granularity: proto.local_clock_granularity(),
        }
    }
}
impl From<u32> for PtmCapability {
    fn from(dword: u32) -> Self { PtmCapabilityProto::from(dword).into() }
}
impl From<PtmCapability> for u32 {
    fn from(data: PtmCapability) -> Self { PtmCapabilityProto::from(data).into() }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct PtmControlProto {
    ptm_enable: bool,
    root_select: bool,
    rsvdp: B6,
    effective_granularity: u8,
    rsvdp_2: B16,
}
impl From<PtmControl> for PtmControlProto {
    fn from(data: PtmControl) -> Self {
        Self::new()
            .with_ptm_enable(data.ptm_enable)
            .with_root_select(data.root_select)
            .with_rsvdp(0)
            .with_effective_granularity(data.effective_granularity)
            .with_rsvdp_2(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtmControl {
    /// PTM Enable
    pub ptm_enable: bool,
    /// Root Select
    pub root_select: bool,
    /// Effective Granularity
    pub effective_granularity: u8,
}
impl From<PtmControlProto> for PtmControl {
    fn from(proto: PtmControlProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            ptm_enable: proto.ptm_enable(),
            root_select: proto.root_select(),
            effective_granularity: proto.effective_granularity(),
        }
    }
}
impl From<u32> for PtmControl {
    fn from(dword: u32) -> Self { PtmControlProto::from(dword).into() }
}
impl From<PtmControl> for u32 {
    fn from(data: PtmControl) -> Self { PtmControlProto::from(data).into() }
}
