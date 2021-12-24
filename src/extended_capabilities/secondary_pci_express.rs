//! Secondary PCI Express
//!
//! The Secondary PCI Express Extended Capability structure must be implemented in any Function or
//! RCRB where any of the following are true:
//! - The Supported Link Speeds Vector field indicates that the Link supports Link Speeds of 8.0
//!   GT/s or higher
//! - Any bit in the Lower SKP OS Generation Supported Speeds Vector field is Set
//! - When Lane based errors are reported in the Lane Error Status register

use std::slice::Chunks;

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

use crate::capabilities::pci_express::{SupportedLinkSpeedsVector, TransmitterPreset, ReceiverPresetHint, LinkWidth};

use super::ECH_BYTES;

/// Lane Equalization Control offset
pub const ECL_OFFSET: usize = 0x0C;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondaryPciExpress<'a> {
    data: &'a [u8],
    pub link_control_3: LinkControl3,
    pub lane_error_status: LaneErrorStatus,
}
impl<'a> SecondaryPciExpress<'a> {
    pub fn equalization_control_lanes(&self, link_width: LinkWidth) -> EqualizationControlLanes<'a> {
        let start = ECL_OFFSET - ECH_BYTES;
        // One Lane Equalization Control 2 bytes width
        let end = start + link_width.value() * 2;
        EqualizationControlLanes::new(&self.data[start..end])
    }
}
impl<'a> TryRead<'a, Endian> for SecondaryPciExpress<'a> {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let spe = SecondaryPciExpress {
            data: bytes,
            link_control_3: bytes.read_with::<u32>(offset, endian)?.into(),
            lane_error_status: LaneErrorStatus(bytes.read_with::<u32>(offset, endian)?),
        };
        Ok((spe, *offset))
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct LinkControl3Proto {
    perform_equalization: bool,
    link_equalization_request_interrupt_enable: bool,
    rsvdp: B7,
    // LowerSkpOsGenerationVector
    speed_2_5_gtps: bool,
    speed_5_0_gtps: bool,
    speed_8_0_gtps: bool,
    speed_16_0_gtps: bool,
    speed_32_0_gtps: bool,
    speed_64_0_gtps: bool,
    rsvdp_2: B1,
    // LinkControl3 reserved
    rsvdp_3: B16,
}

/// Link Control 3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkControl3 {
    /// Perform Equalization
    pub perform_equalization: bool,
    /// Link Equalization Request Interrupt Enable
    pub link_equalization_request_interrupt_enable: bool,
    /// Corresponding to the current Link speed is Set, SKP Ordered Sets are scheduled at the rate
    /// defined for SRNS, overriding the rate required based on the clock tolerance architecture.
    pub lower_skp_os_generation_vector: SupportedLinkSpeedsVector,
}
impl From<LinkControl3Proto> for LinkControl3 {
    fn from(proto: LinkControl3Proto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        let _ = proto.rsvdp_3();
        Self {
            perform_equalization: proto.perform_equalization(),
            link_equalization_request_interrupt_enable:
                proto.link_equalization_request_interrupt_enable(),
            lower_skp_os_generation_vector: SupportedLinkSpeedsVector {
                speed_2_5_gtps: proto.speed_2_5_gtps(),
                speed_5_0_gtps: proto.speed_5_0_gtps(),
                speed_8_0_gtps: proto.speed_8_0_gtps(),
                speed_16_0_gtps: proto.speed_16_0_gtps(),
                speed_32_0_gtps: proto.speed_32_0_gtps(),
                speed_64_0_gtps: proto.speed_64_0_gtps(),
            },
        }
    }
}
impl From<u32> for LinkControl3 {
    fn from(dword: u32) -> Self { LinkControl3Proto::from(dword).into() }
}

/// The Lane Error Status register consists of a 32-bit vector, where each bit indicates if the
/// Lane with the corresponding Lane number detected an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneErrorStatus(pub u32);

/// An iterator through Lane Equalization Controls
pub struct EqualizationControlLanes<'a> {
    chunks: Chunks<'a, u8>,
}
impl<'a> EqualizationControlLanes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { chunks: bytes.chunks(2) }
    }
}
impl<'a> Iterator for EqualizationControlLanes<'a> {
    type Item = LaneEqualizationControl;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: [u8; 2] = self.chunks.next()?.try_into().ok()?;
        Some(u16::from_le_bytes(bytes).into())
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LaneEqualizationControlProto {
    downstream_port_transmitter_preset: B4,
    downstream_port_receiver_preset_hint: B3,
    rsvdz: B1,
    upstream_port_transmitter_preset: B4,
    upstream_port_receiver_preset_hint: B3,
    rsvdz_2: B1,
}

/// The Lane Equalization Control register consists of control fields required for per-Lane 8.0
/// GT/s equalization and the number of entries in this register are sized by Maximum Link Width
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneEqualizationControl {
    /// Downstream Port 8.0 GT/s Transmitter Preset
    downstream_port_transmitter_preset: TransmitterPreset,
    /// Downstream Port 8.0 GT/s Receiver Preset Hint
    downstream_port_receiver_preset_hint: ReceiverPresetHint,
    /// Upstream Port 8.0 GT/s Transmitter Preset
    upstream_port_transmitter_preset: TransmitterPreset,
    /// Upstream Port 8.0 GT/s Receiver Preset Hint
    upstream_port_receiver_preset_hint: ReceiverPresetHint,
}
impl From<LaneEqualizationControlProto> for LaneEqualizationControl {
    fn from(proto: LaneEqualizationControlProto) -> Self {
        let _ = proto.rsvdz();
        let _ = proto.rsvdz_2();
        Self {
            downstream_port_transmitter_preset: proto.downstream_port_transmitter_preset().into(),
            downstream_port_receiver_preset_hint: proto.downstream_port_receiver_preset_hint().into(),
            upstream_port_transmitter_preset: proto.upstream_port_transmitter_preset().into(),
            upstream_port_receiver_preset_hint: proto.upstream_port_receiver_preset_hint().into(),
        }
    }
}
impl From<u16> for LaneEqualizationControl {
    fn from(word: u16) -> Self { LaneEqualizationControlProto::from(word).into() }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    // Capabilities: [250 v1] Secondary PCI Express
    //         LnkCtl3: LnkEquIntrruptEn-, PerformEqu-
    //         LaneErrStat: LaneErr at lane: 0 1 3 4 5 6 7 10 11
    const DATA: [u8; 3 * 16] = [
        0x19,0x00,0x01,0x28,0x00,0x00,0x00,0x00,0xfb,0x0c,0x00,0x00,0x77,0x27,0x77,0x27,
        0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,
        0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,0x77,0x27,0x00,0x00,0x00,0x00,
    ];

    #[test]
    fn lane_error_status() {
        let result = DATA[ECH_BYTES..].read_with::<SecondaryPciExpress>(&mut 0, LE).unwrap();
        let sample = SecondaryPciExpress {
            data: &DATA[ECH_BYTES..],
            link_control_3: LinkControl3 {
                perform_equalization: false,
                link_equalization_request_interrupt_enable: false,
                lower_skp_os_generation_vector: SupportedLinkSpeedsVector {
                    speed_2_5_gtps: false,
                    speed_5_0_gtps: false,
                    speed_8_0_gtps: false,
                    speed_16_0_gtps: false,
                    speed_32_0_gtps: false,
                    speed_64_0_gtps: false,
                },
            },
            lane_error_status: LaneErrorStatus(0b1100_1111_1011),
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn equalization_control_lanes() {
        let spe = DATA[ECH_BYTES..].read_with::<SecondaryPciExpress>(&mut 0, LE).unwrap();
        let result = spe.equalization_control_lanes(LinkWidth::X8)
            .collect::<Vec<_>>();
        let sample = std::iter::repeat(LaneEqualizationControl {
            downstream_port_transmitter_preset: TransmitterPreset::P7,
            downstream_port_receiver_preset_hint: ReceiverPresetHint::Reserved,
            upstream_port_transmitter_preset: TransmitterPreset::P7,
            upstream_port_receiver_preset_hint: ReceiverPresetHint::Minus8dB,
        }).take(8).collect::<Vec<_>>();
        assert_eq!(sample, result);
    }
}
