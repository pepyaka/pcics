/*!
# Compute Express Link

CXL is a dynamic multi-protocol technology designed to support accelerators and
memory devices.

## Struct diagram
<pre>
<a href="enum.ComputeExpressLink.html">ComputeExpressLink</a>
├─ <a href="struct.PcieDvsecForCxlDevice.html">PcieDvsecForCxlDevice</a>
├─ <a href="struct.NonCxlFunctionMapDvsec.html">NonCxlFunctionMapDvsec</a>
├─ <a href="struct.Cxl20ExtensionsDvsecForPorts.html">Cxl20ExtensionsDvsecForPorts</a>
├─ <a href="struct.GpfDvsecForCxlPorts.html">GpfDvsecForCxlPorts</a>
├─ <a href="struct.GpfDvsecForCxlDevices.html">GpfDvsecForCxlDevices</a>
├─ <a href="struct.PcieDvsecForFlexBusPort.html">PcieDvsecForFlexBusPort</a>
├─ <a href="struct.RegisterLocatorDvsec.html">RegisterLocatorDvsec</a>
├─ <a href="struct.MldDvsec.html">MldDvsec</a>
└─ <a href="struct.PcieDvsecForTestCapability.html">PcieDvsecForTestCapability</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::designated_vendor_specific_extended_capability::{
#     Dvsec, DvsecType,
#     compute_express_link::*
# };
let data = [
    /* 00h */ 0x23, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x98, 0x1E, 0xC1, 0x00, // Designated Vendor-Specific Header 1
    /* 08h */ 0x55, 0x55,             // Designated Vendor-Specific Header 2
              0x55, 0x55,             // Reserved
];

let result: Dvsec = data.as_slice().try_into().unwrap();

let sample = Dvsec {
    dvsec_vendor_id: 0x1e98,
    dvsec_revision: 1,
    dvsec_length: 0x0C,
    dvsec_id: 0x5555,
    dvsec_type: DvsecType::ComputeExpressLink(ComputeExpressLink::Undefined(0x5555))
};

assert_eq!(sample, result);
*/

use heterob::{endianness::Le, Seq, P15, U16};
use snafu::Snafu;

/// Compute Express Link (CXL) Errors
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum ComputeExpressLinkError {
    #[snafu(display("PCIe DVSEC for CXL Device"))]
    PcieDvsecForCxlDevice,
}

/// Compute Express Link (CXL)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeExpressLink {
    PcieDvsecForCxlDevice(PcieDvsecForCxlDevice),
    NonCxlFunctionMapDvsec(NonCxlFunctionMapDvsec),
    Cxl20ExtensionsDvsecForPorts(Cxl20ExtensionsDvsecForPorts),
    GpfDvsecForCxlPorts(GpfDvsecForCxlPorts),
    GpfDvsecForCxlDevices(GpfDvsecForCxlDevices),
    PcieDvsecForFlexBusPort(PcieDvsecForFlexBusPort),
    RegisterLocatorDvsec(RegisterLocatorDvsec),
    MldDvsec(MldDvsec),
    PcieDvsecForTestCapability(PcieDvsecForTestCapability),
    /// Not defined by CXL specification
    Undefined(u16),
}

impl ComputeExpressLink {
    pub fn try_new(slice: &[u8], id: u16) -> Result<Self, ComputeExpressLinkError> {
        let result = match id {
            0x00 => {
                let Seq {
                    head:
                        Le((
                            U16(cxl_capability),
                            U16(cxl_control),
                            U16(cxl_status),
                            U16(cxl_control2),
                            U16(cxl_status2),
                            U16(cxl_lock),
                            U16(cxl_capability2),
                            range_1_size_high,
                            range_1_size_low,
                            range_1_base_high,
                            range_1_base_low,
                            range_2_size_high,
                            range_2_size_low,
                            range_2_base_high,
                            range_2_base_low,
                        )),
                    ..
                } = P15(slice)
                    .try_into()
                    .map_err(|_| ComputeExpressLinkError::PcieDvsecForCxlDevice)?;
                Self::PcieDvsecForCxlDevice(PcieDvsecForCxlDevice {
                    cxl_capability,
                    cxl_control,
                    cxl_status,
                    cxl_control2,
                    cxl_status2,
                    cxl_lock,
                    cxl_capability2,
                    cxl_range_1_size: CxlRangeSize::new(range_1_size_low, range_1_size_high),
                    cxl_range_1_base: CxlRangeBase::new(range_1_base_low, range_1_base_high),
                    cxl_range_2_size: CxlRangeSize::new(range_2_size_low, range_2_size_high),
                    cxl_range_2_base: CxlRangeBase::new(range_2_base_low, range_2_base_high),
                })
            }
            0x02 => Self::NonCxlFunctionMapDvsec(NonCxlFunctionMapDvsec),
            0x03 => Self::Cxl20ExtensionsDvsecForPorts(Cxl20ExtensionsDvsecForPorts),
            0x04 => Self::GpfDvsecForCxlPorts(GpfDvsecForCxlPorts),
            0x05 => Self::GpfDvsecForCxlDevices(GpfDvsecForCxlDevices),
            0x07 => Self::PcieDvsecForFlexBusPort(PcieDvsecForFlexBusPort),
            0x08 => Self::RegisterLocatorDvsec(RegisterLocatorDvsec),
            0x09 => Self::MldDvsec(MldDvsec),
            0x0A => Self::PcieDvsecForTestCapability(PcieDvsecForTestCapability),
            id => Self::Undefined(id),
        };
        Ok(result)
    }
}

pub mod pcie_dvsec_for_cxl_device;
use self::pcie_dvsec_for_cxl_device::{CxlRangeBase, CxlRangeSize, PcieDvsecForCxlDevice};

mod non_cxl_function_map_dvsec {
    /// Non-CXL Function Map DVSEC
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NonCxlFunctionMapDvsec;
}
use non_cxl_function_map_dvsec::NonCxlFunctionMapDvsec;

mod cxl_2_0_extensions_dvsec_for_ports {
    /// CXL 2.0 Extensions DVSEC for Ports
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Cxl20ExtensionsDvsecForPorts;
}
use cxl_2_0_extensions_dvsec_for_ports::Cxl20ExtensionsDvsecForPorts;

mod gpf_dvsec_for_cxl_ports {
    /// GPF DVSEC for CXL Ports
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct GpfDvsecForCxlPorts;
}
use gpf_dvsec_for_cxl_ports::GpfDvsecForCxlPorts;

mod gpf_dvsec_for_cxl_devices {
    /// GPF DVSEC for CXL Devices
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct GpfDvsecForCxlDevices;
}
use gpf_dvsec_for_cxl_devices::GpfDvsecForCxlDevices;

mod pcie_dvsec_for_flex_bus_port {
    /// PCIe DVSEC for Flex Bus Port
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PcieDvsecForFlexBusPort;
}
use pcie_dvsec_for_flex_bus_port::PcieDvsecForFlexBusPort;

mod register_locator_dvsec {
    /// Register Locator DVSEC
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegisterLocatorDvsec;
}
use register_locator_dvsec::RegisterLocatorDvsec;

mod mld_dvsec {
    /// MLD DVSEC
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MldDvsec;
}
use mld_dvsec::MldDvsec;

mod pcie_dvsec_for_test_capability {
    /// PCIe DVSEC for Test Capability
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PcieDvsecForTestCapability;
}
use pcie_dvsec_for_test_capability::PcieDvsecForTestCapability;
