/*!
# Multicast

Multicast is an optional normative functionality that is controlled by the Multicast Capability
structure. The Multicast Capability is applicable to Root Ports, RCRBs, Switch Ports, Endpoint
Functions, and RCiEPs. It is not applicable to PCI Express to PCI/PCI-X Bridges.

## Struct diagram
<pre>
<a href="struct.Multicast.html">Multicast</a>
├─ <a href="struct.MulticastCapability.html">MulticastCapability</a>
├─ <a href="struct.MulticastControl.html">MulticastControl</a>
├─ <a href="struct.McBaseAddress.html">McBaseAddress</a>
└─ <a href="struct.McOverlayBar.html">McOverlayBar</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::multicast::*;
let data = [
    /* 00h */ 0x0c, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x2a, 0x8f, // Multicast Capability
              0x20, 0x80, // Multicast Control
    /* 08h */ 0x0f, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, // MC_Base_Address
    /* 10h */ 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, // MC_Receive
    /* 18h */ 0x22, 0x22, 0x22, 0x22, 0x33, 0x33, 0x33, 0x33, // MC_Block_All
    /* 20h */ 0x44, 0x44, 0x44, 0x44, 0x55, 0x55, 0x55, 0x55, // MC_Block_Untranslated
    /* 28h */ 0x0a, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, // MC_Overlay_BAR
];

let result: Multicast = data.as_slice().try_into().unwrap();

let sample = Multicast {
    multicast_capability: MulticastCapability {
        mc_max_group: 42,
        mc_window_size_requested: 0x0f,
        mc_ecrc_regeneration_supported: true,
    },
    multicast_control: MulticastControl {
        mc_num_group: 32,
        mc_enable: true,
    },
    mc_base_address: McBaseAddress {
        mc_index_position: 15,
        mc_base_address: 0x7060504030201000,
    },
    mc_receive: 0x1111111100000000,
    mc_block_all: 0x3333333322222222,
    mc_block_untranslated: 0x5555555544444444,
    mc_overlay_bar: Some(McOverlayBar {
        mc_overlay_size: 10,
        mc_overlay_bar: 0x7766554433221100,
    }),
};

assert_eq!(sample, result);
```
*/

use heterob::{
    bit_numbering::Lsb,
    endianness::{Le, LeBytesTryInto},
    Seq, P2, P3, P5, P10,
};

use super::{ExtendedCapabilityDataError, ExtendedCapabilityHeaderPlaceholder};

/// Multicast
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multicast {
    pub multicast_capability: MulticastCapability,
    pub multicast_control: MulticastControl,
    pub mc_base_address: McBaseAddress,
    /// Provides a bit vector denoting which Multicast groups the Function
    /// should accept, or in the case of Switch and Root Complex Ports, forward
    /// Multicast TLPs
    pub mc_receive: u64,
    /// Provides a bit vector denoting which Multicast groups the Function should block
    pub mc_block_all: u64,
    /// Used to determine whether or not a TLP that includes an Untranslated
    /// Address should be blocked
    pub mc_block_untranslated: u64,
    pub mc_overlay_bar: Option<McOverlayBar>,
}

impl Multicast {
    pub const SIZE: usize = 0x30;
}

impl TryFrom<&[u8]> for Multicast {
    type Error = ExtendedCapabilityDataError;
    fn try_from<'a>(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head:
                Le((
                    ExtendedCapabilityHeaderPlaceholder,
                    multicast_capability,
                    multicast_control,
                    mc_base_address,
                    rcv_l,
                    rcv_h,
                    blk_all_l,
                    blk_all_h,
                    blk_untr_l,
                    blk_untr_h,
                )),
            tail,
        } = P10(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Multicast",
                size: Self::SIZE,
            })?;
        let _: (u32, u32, u32, u32, u32, u32) =
            (rcv_l, rcv_h, blk_all_l, blk_all_h, blk_untr_l, blk_untr_h);
        let multicast_capability: MulticastCapability = From::<u16>::from(multicast_capability);
        let mc_overlay_bar = tail
            .le_bytes_try_into()
            .map(|Seq { head, .. }| From::<u64>::from(head))
            .ok();
        Ok(Self {
            multicast_capability,
            multicast_control: From::<u16>::from(multicast_control),
            mc_base_address: From::<u64>::from(mc_base_address),
            mc_receive: (rcv_l as u64) | (rcv_h as u64) << 32,
            mc_block_all: (blk_all_l as u64) | (blk_all_h as u64) << 32,
            mc_block_untranslated: (blk_untr_l as u64) | (blk_untr_h as u64) << 32,
            mc_overlay_bar,
        })
    }
}

/// Multicast Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MulticastCapability {
    /// Maximum number of Multicast Groups that the component supports, encoded as M-1
    pub mc_max_group: u8,
    /// In Endpoints, Multicast Window size requested
    pub mc_window_size_requested: u8,
    /// Indicates that ECRC regeneration is supported
    pub mc_ecrc_regeneration_supported: bool,
}

impl From<u16> for MulticastCapability {
    fn from(word: u16) -> Self {
        let Lsb((mc_max_group, (), mc_window_size_requested, (), mc_ecrc_regeneration_supported)) =
            P5::<_, 6, 2, 6, 1, 1>(word).into();
        Self {
            mc_max_group,
            mc_window_size_requested,
            mc_ecrc_regeneration_supported,
        }
    }
}

/// Multicast Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MulticastControl {
    /// Indicates the number of Multicast Groups configured for use, encoded as N-1
    pub mc_num_group: u8,
    /// When Set, the Multicast mechanism is enabled for the component
    pub mc_enable: bool,
}

impl From<u16> for MulticastControl {
    fn from(word: u16) -> Self {
        let Lsb((mc_num_group, (), mc_enable)) = P3::<_, 6, 9, 1>(word).into();
        Self {
            mc_num_group,
            mc_enable,
        }
    }
}

/// The MC_Base_Address register contains the MC_Base_Address and the MC_Index_Position
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McBaseAddress {
    /// The location of the LSB of the Multicast Group number within the address
    pub mc_index_position: u8,
    /// The base address of the Multicast address range
    pub mc_base_address: u64,
}

impl From<u64> for McBaseAddress {
    fn from(qword: u64) -> Self {
        let Lsb((mc_index_position, (), mc_base_address)) = P3::<_, 6, 6, 52>(qword).into();
        let _: u64 = mc_base_address;
        Self {
            mc_index_position,
            mc_base_address: mc_base_address << 12,
        }
    }
}

/// The MC_Overlay_BAR is required in Switch and Root Complex Ports that
/// support the Multicast Capability and not implemented in Endpoints
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McOverlayBar {
    pub mc_overlay_size: u8,
    pub mc_overlay_bar: u64,
}

impl From<u64> for McOverlayBar {
    fn from(qword: u64) -> Self {
        let Lsb((mc_overlay_size, mc_overlay_bar)) = P2::<_, 6, 58>(qword).into();
        let _: u64 = mc_overlay_bar;
        Self {
            mc_overlay_size,
            mc_overlay_bar: mc_overlay_bar << 6,
        }
    }
}
