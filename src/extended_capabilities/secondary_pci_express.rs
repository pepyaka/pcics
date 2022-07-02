/*!
# Secondary PCI Express

The Secondary PCI Express Extended Capability structure must be implemented in any Function or
RCRB where any of the following are true:
- The Supported Link Speeds Vector field indicates that the Link supports Link Speeds of 8.0
  GT/s or higher
- Any bit in the Lower SKP OS Generation Supported Speeds Vector field is Set
- When Lane based errors are reported in the Lane Error Status register

## Struct diagram
[SecondaryPciExpress]
- [LinkControl3]
  - [SupportedLinkSpeedsVector]
- [LaneErrorStatus]

## Examples

> ```text
> LnkCtl3: LnkEquIntrruptEn- PerformEqu-
> LaneErrStat: LaneErr at lane: 0 1 2 3
> ```

```rust
# use pcics::extended_capabilities::secondary_pci_express::*;
# use pcics::capabilities::pci_express::SupportedLinkSpeedsVector;
use pretty_assertions::assert_eq;
let data = [
    0x19, 0x00, 0x01, 0x28, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x78, 0x27, 0x78, 0x27,
    0x78, 0x27, 0x78, 0x27,
];

let result: SecondaryPciExpress = data[4..].try_into().unwrap();

let mut sample = result.clone();
sample.link_control_3 = LinkControl3 {
    perform_equalization: false,
    link_equalization_request_interrupt_enable: false,
    lower_skp_os_generation_vector: SupportedLinkSpeedsVector {
        speed_2_5_gtps: false,
        speed_5_0_gtps: false,
        speed_8_0_gtps: false,
        speed_16_0_gtps: false,
        speed_32_0_gtps: false,
        speed_64_0_gtps: false,
        reserved: false,
    },
};
sample.lane_error_status = LaneErrorStatus(0x0f);

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P11, P2, P6};

use super::ExtendedCapabilityDataError;

use core::slice::Chunks;

use crate::capabilities::pci_express::{
    LinkWidth, ReceiverPresetHint, SupportedLinkSpeedsVector, TransmitterPreset,
};

/// Lane Equalization Control offset
pub const ECL_OFFSET: usize = 0x0C;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondaryPciExpress<'a> {
    ecl_data: &'a [u8],
    pub link_control_3: LinkControl3,
    pub lane_error_status: LaneErrorStatus,
}
impl<'a> SecondaryPciExpress<'a> {
    pub fn equalization_control_lanes(
        &self,
        link_width: LinkWidth,
    ) -> EqualizationControlLanes<'a> {
        // One Lane Equalization Control 2 bytes width
        let end = (u8::from(link_width) as usize) * 2;
        EqualizationControlLanes::new(&self.ecl_data[..end])
    }
}
impl<'a> TryFrom<&'a [u8]> for SecondaryPciExpress<'a> {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((link_control_3, lane_error_status)),
            tail,
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Secondary PCI Express",
                size: 8,
            })?;
        Ok(Self {
            ecl_data: tail,
            link_control_3: From::<u32>::from(link_control_3),
            lane_error_status: LaneErrorStatus(lane_error_status),
        })
    }
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

impl From<u32> for LinkControl3 {
    fn from(dword: u32) -> Self {
        let Lsb((
            perform_equalization,
            link_equalization_request_interrupt_enable,
            (),
            speed_2_5_gtps,
            speed_5_0_gtps,
            speed_8_0_gtps,
            speed_16_0_gtps,
            speed_32_0_gtps,
            speed_64_0_gtps,
            reserved,
            (),
        )) = P11::<_, 1, 1, 7, 1, 1, 1, 1, 1, 1, 1, 16>(dword).into();
        Self {
            perform_equalization,
            link_equalization_request_interrupt_enable,
            lower_skp_os_generation_vector: SupportedLinkSpeedsVector {
                speed_2_5_gtps,
                speed_5_0_gtps,
                speed_8_0_gtps,
                speed_16_0_gtps,
                speed_32_0_gtps,
                speed_64_0_gtps,
                reserved,
            },
        }
    }
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
        Self {
            chunks: bytes.chunks(2),
        }
    }
}
impl<'a> Iterator for EqualizationControlLanes<'a> {
    type Item = LaneEqualizationControl;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: [u8; 2] = self.chunks.next()?.try_into().ok()?;
        Some(u16::from_le_bytes(bytes).into())
    }
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

impl From<u16> for LaneEqualizationControl {
    fn from(word: u16) -> Self {
        let Lsb((
            downstream_port_transmitter_preset,
            downstream_port_receiver_preset_hint,
            (),
            upstream_port_transmitter_preset,
            upstream_port_receiver_preset_hint,
            (),
        )) = P6::<_, 4, 3, 1, 4, 3, 1>(word).into();
        Self {
            downstream_port_transmitter_preset: From::<u8>::from(
                downstream_port_transmitter_preset,
            ),
            downstream_port_receiver_preset_hint: From::<u8>::from(
                downstream_port_receiver_preset_hint,
            ),
            upstream_port_transmitter_preset: From::<u8>::from(upstream_port_transmitter_preset),
            upstream_port_receiver_preset_hint: From::<u8>::from(
                upstream_port_receiver_preset_hint,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    // Capabilities: [250 v1] Secondary PCI Express
    //         LnkCtl3: LnkEquIntrruptEn-, PerformEqu-
    //         LaneErrStat: LaneErr at lane: 0 1 3 4 5 6 7 10 11
    const DATA: [u8; 3 * 16] = [
        0x19, 0x00, 0x01, 0x28, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x0c, 0x00, 0x00, 0x77, 0x27, 0x77,
        0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27,
        0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x77, 0x27, 0x00,
        0x00, 0x00, 0x00,
    ];

    #[test]
    fn lane_error_status() {
        let result = DATA[4..].try_into().unwrap();
        let sample = SecondaryPciExpress {
            ecl_data: &DATA[4 + 4 + 4..],
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
                    reserved: false,
                },
            },
            lane_error_status: LaneErrorStatus(0b1100_1111_1011),
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn equalization_control_lanes() {
        let spe: SecondaryPciExpress = DATA[4..].try_into().unwrap();
        let result = spe
            .equalization_control_lanes(LinkWidth::X8)
            .collect::<Vec<_>>();
        let sample = std::iter::repeat(LaneEqualizationControl {
            downstream_port_transmitter_preset: TransmitterPreset::P7,
            downstream_port_receiver_preset_hint: ReceiverPresetHint::Reserved,
            upstream_port_transmitter_preset: TransmitterPreset::P7,
            upstream_port_receiver_preset_hint: ReceiverPresetHint::Minus8dB,
        })
        .take(8)
        .collect::<Vec<_>>();
        assert_eq!(sample, result);
    }
}
