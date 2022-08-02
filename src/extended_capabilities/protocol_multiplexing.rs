/*!
# Protocol Multiplexing (PMUX)

The presence of this capability indicates that the Port supports the optional
Protocol Multiplexing mechanism. This capability is optional and may be present
in any Downstream Port and in Function 0 of any Upstream Port. It must not be
present in non-zero Functions of Upstream Ports or in RCRBs.

## Struct diagram
<pre>
<a href="struct.ProtocolMultiplexing.html">ProtocolMultiplexing</a>
├─ <a href="struct.PmuxCapability.html">PmuxCapability</a>
│  └─ <a href="struct.PmuxSupportedLinkSpeeds.html">PmuxSupportedLinkSpeeds</a>
├─ <a href="struct.PmuxControl.html">PmuxControl</a>
├─ <a href="struct.PmuxStatus.html">PmuxStatus</a>
└─ <a href="struct.PmuxProtocolArray.html">PmuxProtocolArray</a>
   └─ <a href="struct.PmuxProtocolArrayEntry.html">PmuxProtocolArrayEntry (1 .. N)</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::protocol_multiplexing::*;
let data = [
    /* 00h */ 0x01, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x02, 0b1100, 0x00, 0x00, // PMUX Capability
    /* 08h */ 0x01, 0x02, 0x03, 0x04, // PMUX Control
    /* 0Ch */ 0b000, 0b001, 0b010, 0b100, // PMUX Status
    /* 10h */ 0x12, 0x34, 0x56, 0x78, // PMUX Protocol Array [1]
    /* 14h */ 0x11, 0x22, 0x33, 0x44, // PMUX Protocol Array [2]
];

let result: ProtocolMultiplexing = data.as_slice().try_into().unwrap();

let protocol_array_bytes = [
    PmuxProtocolArrayEntry {
        protocol_id: 0x3412,
        authority_id: 0x7856,
    },
    PmuxProtocolArrayEntry {
        protocol_id: 0x2211,
        authority_id: 0x4433,
    },
]
.map(Into::<[u8; 4]>::into)
.into_iter()
.flatten()
.collect::<Vec<_>>();

let sample = ProtocolMultiplexing {
    pmux_capability: PmuxCapability {
        pmux_protocol_array_size: 2,
        pmux_supported_link_speeds: PmuxSupportedLinkSpeeds {
            speed_2_5_gtps: false,
            speed_5_0_gtps: false,
            speed_8_0_gtps: true,
            speed_16_0_gtps: true,
        },
    },
    pmux_control: PmuxControl {
        pmux_channel_0_assignment: 1,
        pmux_channel_1_assignment: 2,
        pmux_channel_2_assignment: 3,
        pmux_channel_3_assignment: 4,
    },
    pmux_status: PmuxStatus {
        pmux_channel_0_disabled_link_speed: false,
        pmux_channel_0_disabled_link_width: false,
        pmux_channel_0_disabled_link_protocol_specific: false,
        pmux_channel_1_disabled_link_speed: true,
        pmux_channel_1_disabled_link_width: false,
        pmux_channel_1_disabled_link_protocol_specific: false,
        pmux_channel_2_disabled_link_speed: false,
        pmux_channel_2_disabled_link_width: true,
        pmux_channel_2_disabled_link_protocol_specific: false,
        pmux_channel_3_disabled_link_speed: false,
        pmux_channel_3_disabled_link_width: false,
        pmux_channel_3_disabled_link_protocol_specific: true,
    },
    pmux_protocol_array: PmuxProtocolArray::new(&protocol_array_bytes),
};

assert_eq!(sample, result);
```
*/

use core::slice;

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P16, P2, P3, P4, P5, P8};
use snafu::Snafu;

use super::ExtendedCapabilityHeader;

#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum ProtocolMultiplexingError {
    #[snafu(display("capability, control and status fields are unreadable"))]
    Mandatory,
    #[snafu(display(
        "ureadable bytes for PMUX Protocol Array (expected: {expected}, found: {found})"
    ))]
    PmuxProtocolArray { expected: usize, found: usize },
}

/// Protocol Multiplexing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolMultiplexing<'a> {
    pub pmux_capability: PmuxCapability,
    pub pmux_control: PmuxControl,
    pub pmux_status: PmuxStatus,
    pub pmux_protocol_array: PmuxProtocolArray<'a>,
}

impl<'a> TryFrom<&'a [u8]> for ProtocolMultiplexing<'a> {
    type Error = ProtocolMultiplexingError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        // Skip header
        let slice = slice
            .get(ExtendedCapabilityHeader::SIZE..)
            .unwrap_or_default();
        let Seq {
            head: Le((pmux_capability, pmux_control, pmux_status)),
            tail,
        } = P3(slice)
            .try_into()
            .map_err(|_| ProtocolMultiplexingError::Mandatory)?;
        let pmux_capability @ PmuxCapability {
            pmux_protocol_array_size,
            ..
        } = From::<u32>::from(pmux_capability);
        let len = pmux_protocol_array_size as usize * PmuxProtocolArrayEntry::SIZE;
        tail.get(..len)
            .ok_or(ProtocolMultiplexingError::PmuxProtocolArray {
                expected: len,
                found: tail.len(),
            })
            .map(|slice| Self {
                pmux_capability,
                pmux_control: From::<u32>::from(pmux_control),
                pmux_status: From::<u32>::from(pmux_status),
                pmux_protocol_array: PmuxProtocolArray::new(slice),
            })
    }
}

/// PMUX Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmuxCapability {
    /// Indicates the size of this Function’s PMUX Protocol Array
    pub pmux_protocol_array_size: u8,
    pub pmux_supported_link_speeds: PmuxSupportedLinkSpeeds,
}

impl From<u32> for PmuxCapability {
    fn from(dword: u32) -> Self {
        let Lsb((pmux_protocol_array_size, (), pmux_supported_link_speeds, ())) =
            P4::<_, 6, 2, 8, 16>(dword).into();
        Self {
            pmux_protocol_array_size,
            pmux_supported_link_speeds: From::<u8>::from(pmux_supported_link_speeds),
        }
    }
}

/// Indicates the Link speed(s) where Protocol Multiplexing is supported
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmuxSupportedLinkSpeeds {
    /// 2.5 GT/s
    pub speed_2_5_gtps: bool,
    /// 5.0 GT/s
    pub speed_5_0_gtps: bool,
    /// 5.0 GT/s
    pub speed_8_0_gtps: bool,
    /// 16.0 GT/s
    pub speed_16_0_gtps: bool,
}

impl From<u8> for PmuxSupportedLinkSpeeds {
    fn from(byte: u8) -> Self {
        let Lsb((speed_2_5_gtps, speed_5_0_gtps, speed_8_0_gtps, speed_16_0_gtps, ())) =
            P5::<_, 1, 1, 1, 1, 4>(byte).into();
        Self {
            speed_2_5_gtps,
            speed_5_0_gtps,
            speed_8_0_gtps,
            speed_16_0_gtps,
        }
    }
}

/// PMUX Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmuxControl {
    /// Indicates the protocol assigned to PMUX Channel 0
    pub pmux_channel_0_assignment: u8,
    /// Indicates the protocol assigned to PMUX Channel 1
    pub pmux_channel_1_assignment: u8,
    /// Indicates the protocol assigned to PMUX Channel 2
    pub pmux_channel_2_assignment: u8,
    /// Indicates the protocol assigned to PMUX Channel 3
    pub pmux_channel_3_assignment: u8,
}

impl From<u32> for PmuxControl {
    fn from(dword: u32) -> Self {
        let Lsb((
            pmux_channel_0_assignment,
            (),
            pmux_channel_1_assignment,
            (),
            pmux_channel_2_assignment,
            (),
            pmux_channel_3_assignment,
            (),
        )) = P8::<_, 6, 2, 6, 2, 6, 2, 6, 2>(dword).into();
        Self {
            pmux_channel_0_assignment,
            pmux_channel_1_assignment,
            pmux_channel_2_assignment,
            pmux_channel_3_assignment,
        }
    }
}

/// PMUX Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmuxStatus {
    /// If Set, Channel 0 is disabled because the Current Link Speed is not
    /// supported by Protocol Multiplexing or by the protocol assigned to Channel 0
    pub pmux_channel_0_disabled_link_speed: bool,
    /// If Set, Channel 0 is disabled because the current Link Width is not
    /// supported by the protocol assigned to Channel 0
    pub pmux_channel_0_disabled_link_width: bool,
    /// If Set, Channel 0 is disabled for protocol specific reasons
    pub pmux_channel_0_disabled_link_protocol_specific: bool,
    /// If Set, Channel 1 is disabled because the Current Link Speed is not
    /// supported by Protocol Multiplexing or by the protocol assigned to Channel 1
    pub pmux_channel_1_disabled_link_speed: bool,
    /// If Set, Channel 1 is disabled because the current Link Width is not
    /// supported by the protocol assigned to Channel 1
    pub pmux_channel_1_disabled_link_width: bool,
    /// If Set, Channel 1 is disabled for protocol specific reasons
    pub pmux_channel_1_disabled_link_protocol_specific: bool,
    /// If Set, Channel 2 is disabled because the Current Link Speed is not
    /// supported by Protocol Multiplexing or by the protocol assigned to Channel 2
    pub pmux_channel_2_disabled_link_speed: bool,
    /// If Set, Channel 2 is disabled because the current Link Width is not
    /// supported by the protocol assigned to Channel 2
    pub pmux_channel_2_disabled_link_width: bool,
    /// If Set, Channel 2 is disabled for protocol specific reasons
    pub pmux_channel_2_disabled_link_protocol_specific: bool,
    /// If Set, Channel 3 is disabled because the Current Link Speed is not
    /// supported by Protocol Multiplexing or by the protocol assigned to Channel 3
    pub pmux_channel_3_disabled_link_speed: bool,
    /// If Set, Channel 3 is disabled because the current Link Width is not
    /// supported by the protocol assigned to Channel 3
    pub pmux_channel_3_disabled_link_width: bool,
    /// If Set, Channel 3 is disabled for protocol specific reasons
    pub pmux_channel_3_disabled_link_protocol_specific: bool,
}

impl From<u32> for PmuxStatus {
    fn from(dword: u32) -> Self {
        let Lsb((
            pmux_channel_0_disabled_link_speed,
            pmux_channel_0_disabled_link_width,
            pmux_channel_0_disabled_link_protocol_specific,
            (),
            pmux_channel_1_disabled_link_speed,
            pmux_channel_1_disabled_link_width,
            pmux_channel_1_disabled_link_protocol_specific,
            (),
            pmux_channel_2_disabled_link_speed,
            pmux_channel_2_disabled_link_width,
            pmux_channel_2_disabled_link_protocol_specific,
            (),
            pmux_channel_3_disabled_link_speed,
            pmux_channel_3_disabled_link_width,
            pmux_channel_3_disabled_link_protocol_specific,
            (),
        )) = P16::<_, 1, 1, 1, 5, 1, 1, 1, 5, 1, 1, 1, 5, 1, 1, 1, 5>(dword).into();
        Self {
            pmux_channel_0_disabled_link_speed,
            pmux_channel_0_disabled_link_width,
            pmux_channel_0_disabled_link_protocol_specific,
            pmux_channel_1_disabled_link_speed,
            pmux_channel_1_disabled_link_width,
            pmux_channel_1_disabled_link_protocol_specific,
            pmux_channel_2_disabled_link_speed,
            pmux_channel_2_disabled_link_width,
            pmux_channel_2_disabled_link_protocol_specific,
            pmux_channel_3_disabled_link_speed,
            pmux_channel_3_disabled_link_width,
            pmux_channel_3_disabled_link_protocol_specific,
        }
    }
}

/// An iterator through [PmuxProtocolArrayEntry]
#[derive(Debug, Clone)]
pub struct PmuxProtocolArray<'a>(pub slice::Chunks<'a, u8>);

impl<'a> PmuxProtocolArray<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self(slice.chunks(PmuxProtocolArrayEntry::SIZE))
    }
}

impl<'a> PartialEq for PmuxProtocolArray<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.clone().eq(other.0.clone())
    }
}

impl<'a> Eq for PmuxProtocolArray<'a> {}

impl<'a> Iterator for PmuxProtocolArray<'a> {
    type Item = PmuxProtocolArrayEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = self.0.next()?;
        let Seq {
            head: Le((protocol_id, authority_id)),
            ..
        } = P2(slice).try_into().ok()?;
        Some(PmuxProtocolArrayEntry {
            protocol_id,
            authority_id,
        })
    }
}

/// PMUX Protocol Array entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmuxProtocolArrayEntry {
    /// Designates a specific protocol and the mechanism by which that protocol
    /// is mapped onto Protocol Multiplexing
    pub protocol_id: u16,
    /// Designates the authority controlling the values used in the Protocol ID field
    pub authority_id: u16,
}

impl PmuxProtocolArrayEntry {
    /// Authority ID (u16) and Protocol ID (u16)
    pub const SIZE: usize = 4;
}

impl From<PmuxProtocolArrayEntry> for [u8; PmuxProtocolArrayEntry::SIZE] {
    fn from(entry: PmuxProtocolArrayEntry) -> Self {
        let [a, b] = entry.protocol_id.to_le_bytes();
        let [c, d] = entry.authority_id.to_le_bytes();
        [a, b, c, d]
    }
}
