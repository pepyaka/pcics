//! Latency Tolerance Reporting (LTR)
//!
//! The PCI Express Latency Tolerance Reporting (LTR) Capability is an optional Extended Capability
//! that allows software to provide platform latency information to components with Upstream Ports
//! (Endpoints and Switches), and is required for Switch Upstream Ports and Endpoints if the
//! Function supports the LTR mechanism. It is not applicable to Root Ports, Bridges, or Switch
//! Downstream Ports.

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatencyToleranceReporting {
    /// Max Snoop Latency
    pub max_snoop_latency: MaxLatency,
    /// Max No-Snoop Latency
    pub max_no_snoop_latency: MaxLatency,
}
impl<'a> TryRead<'a, Endian> for LatencyToleranceReporting {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let ltr = LatencyToleranceReporting {
            max_snoop_latency: bytes.read_with::<u16>(offset, endian)?.into(),
            max_no_snoop_latency: bytes.read_with::<u16>(offset, endian)?.into(),
        };
        Ok((ltr, *offset))
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct MaxLatencyProto {
    value: B10,
    scale: B3,
    rsvdp: B3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxLatency {
    /// Specifies the maximum latency that a device is permitted to request
    pub value: u16,
    /// Provides a scale for the value contained within the value field
    pub scale: u8,
}
impl MaxLatency {
    pub fn value(&self) -> usize {
        match self.scale {
            scale @ 0..=5 =>
                (self.value as usize) << (5 * scale),
            _ => 0,
        }
    }
}
impl From<MaxLatencyProto> for MaxLatency {
    fn from(proto: MaxLatencyProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            value: proto.value(),
            scale: proto.scale(),
        }
    }
}
impl From<u16> for MaxLatency {
    fn from(word: u16) -> Self { MaxLatencyProto::from(word).into() }
}

