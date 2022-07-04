/*!
# PCI Express Root Complex Internal Link Control

The PCI Express Root Complex Internal Link Control Capability is an optional Capability that
controls an internal Root Complex Link between two distinct Root Complex Components. This
Capability is valid for RCRBs that declare an Element Type field as Internal Root Complex Link in
the Element Self-Description register of the Root Complex Link Declaration Capability structure.

## Struct diagram
<pre>
<a href="struct.RootComplexInternalLinkControl.html">RootComplexInternalLinkControl</a>
├─ <a href="struct.RootComplexLinkCapabilities.html">RootComplexLinkCapabilities</a>
│  ├─ <a href="../../capabilities/pci_express/enum.LinkSpeed.html">LinkSpeed</a>
│  ├─ <a href="../../capabilities/pci_express/enum.LinkWidth.html">LinkWidth</a>
│  ├─ <a href="../../capabilities/pci_express/enum.ActiveStatePowerManagement.html">ActiveStatePowerManagement</a>
│  ├─ <a href="../../capabilities/pci_express/enum.L0sExitLatency.html">L0sExitLatency</a>
│  ├─ <a href="../../capabilities/pci_express/enum.L1ExitLatency.html">L1ExitLatency</a>
│  └─ <a href="../../capabilities/pci_express/struct.SupportedLinkSpeedsVector.html">SupportedLinkSpeedsVector</a>
├─ <a href="struct.RootComplexLinkControl.html">RootComplexLinkControl</a>
│  └─ <a href="../../capabilities/pci_express/enum.ActiveStatePowerManagement.html">ActiveStatePowerManagement</a>
└─ <a href="struct.RootComplexLinkStatus.html">RootComplexLinkStatus</a>
   ├─ <a href="../../capabilities/pci_express/enum.LinkSpeed.html">LinkSpeed</a>
   └─ <a href="../../capabilities/pci_express/enum.LinkWidth.html">LinkWidth</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::root_complex_internal_link_control::*;
# use pcics::capabilities::pci_express::*;
let data = [
    0x06, 0x00, 0x00, 0x00, // Extended Capability Header
    0x55, 0x55, 0x55, 0x55, // Root Complex Link Capabilities
    0x55, 0x55, // Root Complex Link Control
    0x55, 0x55, // Root Complex Link Status
];
let result = data[4..].try_into().unwrap();
let sample = RootComplexInternalLinkControl {
    root_complex_link_capabilities: RootComplexLinkCapabilities {
        max_link_speed: LinkSpeed::Rate32GTps,
        maximum_link_width: LinkWidth::Reserved(0x15),
        active_state_power_management_support: ActiveStatePowerManagement::L0s,
        l0s_exit_latency: L0sExitLatency::Ge1usAndLt2us,
        l1_exit_latency: L1ExitLatency::Ge2usAndLt4us,
        supported_link_speeds_vector: SupportedLinkSpeedsVector {
            speed_2_5_gtps: true,
            speed_5_0_gtps: false,
            speed_8_0_gtps: true,
            speed_16_0_gtps: false,
            speed_32_0_gtps: true,
            speed_64_0_gtps: false,
            reserved: true,
        },
    },
    root_complex_link_control: RootComplexLinkControl {
        active_state_power_management_control: ActiveStatePowerManagement::L0s,
        extended_synch: false,
    },
    root_complex_link_status: RootComplexLinkStatus {
        current_link_speed: LinkSpeed::Rate32GTps,
        negotiated_link_width: LinkWidth::Reserved(0x15),
    },
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P3, P4, P7};

use crate::capabilities::pci_express::{
    ActiveStatePowerManagement, L0sExitLatency, L1ExitLatency, LinkSpeed, LinkWidth,
    SupportedLinkSpeedsVector,
};

use super::ExtendedCapabilityDataError;

/// Root Complex Internal Link Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootComplexInternalLinkControl {
    /// Root Complex Link Capabilities
    pub root_complex_link_capabilities: RootComplexLinkCapabilities,
    /// Root Complex Link Control
    pub root_complex_link_control: RootComplexLinkControl,
    /// Root Complex Link Status
    pub root_complex_link_status: RootComplexLinkStatus,
}

impl RootComplexInternalLinkControl {
    pub const SIZE: usize = 4 + 2 + 2;
}

impl From<[u8; Self::SIZE]> for RootComplexInternalLinkControl {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((
            root_complex_link_capabilities,
            root_complex_link_control,
            root_complex_link_status,
        )) = P3(bytes).into();
        Self {
            root_complex_link_capabilities: From::<u32>::from(root_complex_link_capabilities),
            root_complex_link_control: From::<u16>::from(root_complex_link_control),
            root_complex_link_status: From::<u16>::from(root_complex_link_status),
        }
    }
}

impl TryFrom<&[u8]> for RootComplexInternalLinkControl {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq { head, .. } = slice.try_into().map_err(|_| ExtendedCapabilityDataError {
            name: "Root Complex Internal Link Control",
            size: Self::SIZE,
        })?;
        Ok(From::<[u8; Self::SIZE]>::from(head))
    }
}

/// Root Complex Link Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootComplexLinkCapabilities {
    pub max_link_speed: LinkSpeed,
    pub maximum_link_width: LinkWidth,
    pub active_state_power_management_support: ActiveStatePowerManagement,
    pub l0s_exit_latency: L0sExitLatency,
    pub l1_exit_latency: L1ExitLatency,
    pub supported_link_speeds_vector: SupportedLinkSpeedsVector,
}

impl From<u32> for RootComplexLinkCapabilities {
    fn from(dword: u32) -> Self {
        let Lsb((
            max_link_speed,
            maximum_link_width,
            active_state_power_management_support,
            l0s_exit_latency,
            l1_exit_latency,
            supported_link_speeds_vector,
            (),
        )) = P7::<_, 4, 6, 2, 3, 3, 7, 7>(dword).into();
        Self {
            max_link_speed: From::<u8>::from(max_link_speed),
            maximum_link_width: From::<u8>::from(maximum_link_width),
            active_state_power_management_support: From::<u8>::from(
                active_state_power_management_support,
            ),
            l0s_exit_latency: From::<u8>::from(l0s_exit_latency),
            l1_exit_latency: From::<u8>::from(l1_exit_latency),
            supported_link_speeds_vector: From::<u8>::from(supported_link_speeds_vector),
        }
    }
}

/// Root Complex Link Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootComplexLinkControl {
    pub active_state_power_management_control: ActiveStatePowerManagement,
    /// Forces the transmission of additional Ordered Sets when exiting the L0s
    /// state and when in the Recovery state
    pub extended_synch: bool,
}

impl From<u16> for RootComplexLinkControl {
    fn from(word: u16) -> Self {
        let Lsb((active_state_power_management_control, (), extended_synch, ())) =
            P4::<_, 2, 5, 1, 8>(word).into();
        Self {
            active_state_power_management_control: From::<u8>::from(
                active_state_power_management_control,
            ),
            extended_synch,
        }
    }
}

/// Root Complex Link Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootComplexLinkStatus {
    pub current_link_speed: LinkSpeed,
    pub negotiated_link_width: LinkWidth,
}

impl From<u16> for RootComplexLinkStatus {
    fn from(word: u16) -> Self {
        let Lsb((current_link_speed, negotiated_link_width, ())) = P3::<_, 4, 6, 6>(word).into();
        Self {
            current_link_speed: From::<u8>::from(current_link_speed),
            negotiated_link_width: From::<u8>::from(negotiated_link_width),
        }
    }
}
