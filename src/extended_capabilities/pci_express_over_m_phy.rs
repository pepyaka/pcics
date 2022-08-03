/*!
# PCI Express over M-PHY (M-PCIe)

The M-PCIe Capability is an optional Extended Capability that is required for all ports and RCRBs
that support M-PCIe. It is not applicable to Root Complex Integrated Endpoints, or Root Complex
Event Collectors.

## Struct diagram
<pre>
<a href="struct.PciExpressOverMphy.html">PciExpressOverMphy</a>
 ├─ <a href="struct.MpcieCapabilities.html">MpcieCapabilities</a>
 │  ├─ <a href="struct.MpcieLinkSpeed.html">MpcieLinkSpeed</a>
 │  └─ <a href="struct.LaneWidthSupported.html">LaneWidthSupported x 2</a>
 ├─ <a href="struct.MpcieControl.html">MpcieControl</a>
 ├─ <a href="struct.MpcieStatus.html">MpcieStatus</a>
 │  ├─ <a href="struct.MpcieLinkSpeed.html">MpcieLinkSpeed</a>
 │  └─ <a href="enum.LaneWidthStatus.html">LaneWidthStatus x 2</a>
 ├─ <a href="struct.MpciePhyControlAddress.html">MpciePhyControlAddress</a>
 │  └─ <a href="enum.PhyLocation.html">PhyLocation</a>
 └─ <a href="struct.MpciePhyControlData.html">MpciePhyControlData</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::pci_express_over_m_phy::*;
let data = [
    /* 00h */ 0x20, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x01, 0x80, 0x0f, 0x0f, // M-PCIe Capabilities
    /* 08h */ 0x01, 0x00, 0x00, 0x00, // M-PCIe Control
    /* 0Ch */ 0x01, 0x80, 0x0C, 0x0C, // M-PCIe Status
    /* 10h */ 0x00, 0x11, 0x22, 0x33, // M-PCIe LANE Error Status
    /* 14h */ 0x55, 0x00, 0x0A, 0xC1, // M-PCIe Phy Control Address
    /* 18h */ 0x13, 0x00, 0x00, 0x40, // M-PCIe Phy Control Data
];

let result: PciExpressOverMphy = data.as_slice().try_into().unwrap();

let sample = PciExpressOverMphy {
    mpcie_capabilities: MpcieCapabilities {
        mpcie_link_speed_capability: MpcieLinkSpeed {
            hs_g1: true,
            hs_g2: false,
        },
        configuration_software_supported: true,
        maximum_tx_lane_width_capability: LaneWidthSupported {
            x1: true,
            x2: true,
            x4: true,
            x8: true,
            x12: false,
            x16: false,
            x32: false,
        },
        maximum_rx_lane_width_capability: LaneWidthSupported {
            x1: true,
            x2: true,
            x4: true,
            x8: true,
            x12: false,
            x16: false,
            x32: false,
        },
    },
    mpcie_control: MpcieControl {
        mpcie_target_link_speed_control: MpcieLinkSpeed {
            hs_g1: true,
            hs_g2: false,
        },
    },
    mpcie_status: MpcieStatus {
        mpcie_current_link_speed_status: MpcieLinkSpeed {
            hs_g1: true,
            hs_g2: false,
        },
        mpcie_configuration_software_status: true,
        tx_lane_width_status: LaneWidthStatus::X12,
        rx_lane_width_status: LaneWidthStatus::X12,
    },
    mpcie_lane_error_status: 0x33221100,
    mpcie_phy_control_address: MpciePhyControlAddress {
        lower_addr: 0x55,
        upper_addr: 0x2a,
        phy_location: PhyLocation::LocalPhy,
        read: true,
        config: true,
    },
    mpcie_phy_control_data: MpciePhyControlData {
        phy_register_data: 0x13,
        phy_control_error: false,
        rrap_abort_a: true,
        phy_control_pending: false,
    },
};

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P5, P6, P7, P9};

use super::{ExtendedCapabilityDataError, ExtendedCapabilityHeader};

/// M-PCIe Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciExpressOverMphy {
    pub mpcie_capabilities: MpcieCapabilities,
    pub mpcie_control: MpcieControl,
    pub mpcie_status: MpcieStatus,
    pub mpcie_lane_error_status: u32,
    pub mpcie_phy_control_address: MpciePhyControlAddress,
    pub mpcie_phy_control_data: MpciePhyControlData,
}

impl PciExpressOverMphy {
    /// Size in bytes (with Extended Capability Header)
    pub const SIZE: usize = 0x1C;
}

impl TryFrom<&[u8]> for PciExpressOverMphy {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        // Skip header
        let slice = slice
            .get(ExtendedCapabilityHeader::SIZE..)
            .unwrap_or_default();
        let Seq {
            head:
                Le((
                    mpcie_capabilities,
                    mpcie_control,
                    mpcie_status,
                    mpcie_lane_error_status,
                    mpcie_phy_control_address,
                    mpcie_phy_control_data,
                )),
            ..
        } = P6(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "M-PCIe",
                size: Self::SIZE,
            })?;
        Ok(Self {
            mpcie_capabilities: From::<u32>::from(mpcie_capabilities),
            mpcie_control: From::<u32>::from(mpcie_control),
            mpcie_status: From::<u32>::from(mpcie_status),
            mpcie_lane_error_status,
            mpcie_phy_control_address: From::<u32>::from(mpcie_phy_control_address),
            mpcie_phy_control_data: From::<u32>::from(mpcie_phy_control_data),
        })
    }
}

/// M-PCIe Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpcieCapabilities {
    /// M-PCIe Link Speed Capability
    pub mpcie_link_speed_capability: MpcieLinkSpeed,
    /// Configuration.Software Supported
    pub configuration_software_supported: bool,
    /// Maximum TX LANE Width Capability
    pub maximum_tx_lane_width_capability: LaneWidthSupported,
    /// Maximum RX LANE Width Capability
    pub maximum_rx_lane_width_capability: LaneWidthSupported,
}

impl From<u32> for MpcieCapabilities {
    fn from(dword: u32) -> Self {
        let Lsb((lsc, (), configuration_software_supported, tx, rx)) =
            P5::<_, 2, 13, 1, 8, 8>(dword).into();
        Self {
            mpcie_link_speed_capability: From::<u8>::from(lsc),
            configuration_software_supported,
            maximum_tx_lane_width_capability: From::<u8>::from(tx),
            maximum_rx_lane_width_capability: From::<u8>::from(rx),
        }
    }
}

/// M-PCIe Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpcieControl {
    /// M-PCIe Target Link Speed Control
    pub mpcie_target_link_speed_control: MpcieLinkSpeed,
}

impl From<u32> for MpcieControl {
    fn from(dword: u32) -> Self {
        let Lsb((tlsc, ())) = P2::<_, 2, 30>(dword).into();
        Self {
            mpcie_target_link_speed_control: From::<u8>::from(tlsc),
        }
    }
}

/// M-PCIe LANE Error Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpcieStatus {
    /// M-PCIe Current Link Speed Status
    pub mpcie_current_link_speed_status: MpcieLinkSpeed,
    /// M-PCIe Configuration.Software Status
    pub mpcie_configuration_software_status: bool,
    /// TX LANE Width Status
    pub tx_lane_width_status: LaneWidthStatus,
    /// RX LANE Width Status
    pub rx_lane_width_status: LaneWidthStatus,
}

impl From<u32> for MpcieStatus {
    fn from(dword: u32) -> Self {
        let Lsb((clss, (), mpcie_configuration_software_status, tx, rx)) =
            P5::<_, 2, 13, 1, 8, 8>(dword).into();
        Self {
            mpcie_current_link_speed_status: From::<u8>::from(clss),
            mpcie_configuration_software_status,
            tx_lane_width_status: From::<u8>::from(tx),
            rx_lane_width_status: From::<u8>::from(rx),
        }
    }
}

/// M-PCIe Phy Control Address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpciePhyControlAddress {
    /// Corresponds to Lower Addr\[7:0\] in an RRAP packet.
    pub lower_addr: u8,
    /// Corresponds to Upper Addr\[5:0\] in an RRAP packet.
    pub upper_addr: u8,
    pub phy_location: PhyLocation,
    /// Initiates a read operation of the register described by Phy Location,
    /// UpperAddr and LowerAddr.
    pub read: bool,
    /// If Set, the Downstream Port LTSSM will stay in the Configuration.Software state
    pub config: bool,
}

impl From<u32> for MpciePhyControlAddress {
    fn from(dword: u32) -> Self {
        let Lsb((lower_addr, (), upper_addr, (), upper_addr_5, phy_location, (), read, config)) =
            P9::<_, 8, 8, 5, 3, 1, 3, 2, 1, 1>(dword).into();
        let _: (u8, u8) = (upper_addr, upper_addr_5);
        Self {
            lower_addr,
            upper_addr: upper_addr | (upper_addr_5 << 5),
            phy_location: From::<u8>::from(phy_location),
            read,
            config,
        }
    }
}

/// M-PCIe Phy Control Data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpciePhyControlData {
    /// Phy Register Data
    pub phy_register_data: u8,
    /// Phy Control operation completes in error
    pub phy_control_error: bool,
    /// Contains the value of the A bit in the associated RRAP Response packet
    pub rrap_abort_a: bool,
    /// Phy Control operation is started and is Cleared when that operation completes
    pub phy_control_pending: bool,
}

impl From<u32> for MpciePhyControlData {
    fn from(dword: u32) -> Self {
        let Lsb((phy_register_data, (), phy_control_error, rrap_abort_a, phy_control_pending)) =
            P5::<_, 8, 21, 1, 1, 1>(dword).into();
        Self {
            phy_register_data,
            phy_control_error,
            rrap_abort_a,
            phy_control_pending,
        }
    }
}

/// M-PHY HS-GEARs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpcieLinkSpeed {
    /// HS-G1
    pub hs_g1: bool,
    /// HS-G2
    pub hs_g2: bool,
}

impl From<u8> for MpcieLinkSpeed {
    fn from(byte: u8) -> Self {
        let Lsb((hs_g1, hs_g2)) = P2::<_, 1, 1>(byte).into();
        Self { hs_g1, hs_g2 }
    }
}

/// Specifies the supported LANE Width
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneWidthSupported {
    pub x1: bool,
    pub x2: bool,
    pub x4: bool,
    pub x8: bool,
    pub x12: bool,
    pub x16: bool,
    pub x32: bool,
}

impl From<u8> for LaneWidthSupported {
    fn from(byte: u8) -> Self {
        let Lsb((x1, x2, x4, x8, x12, x16, x32)) = P7::<_, 1, 1, 1, 1, 1, 1, 1>(byte).into();
        Self {
            x1,
            x2,
            x4,
            x8,
            x12,
            x16,
            x32,
        }
    }
}

/// LANE Width Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaneWidthStatus {
    X1,
    X2,
    X4,
    X8,
    X12,
    X16,
    X32,
    Reserved(u8),
}

impl From<u8> for LaneWidthStatus {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000_0001 => Self::X1,
            0b0000_0010 => Self::X2,
            0b0000_0100 => Self::X4,
            0b0000_1000 => Self::X8,
            0b0000_1100 => Self::X12,
            0b0001_0000 => Self::X16,
            0b0010_0000 => Self::X32,
            v => Self::Reserved(v),
        }
    }
}

/// Indicates the location along the Link of the Phy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhyLocation {
    LocalPhy,
    RemotePhy,
    Reserved(u8),
}

impl From<u8> for PhyLocation {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::LocalPhy,
            0b001 => Self::RemotePhy,
            v => Self::Reserved(v),
        }
    }
}
