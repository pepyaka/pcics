//! PCI Power Management Interface
//!
//! This capability structure provides a standard interface to control power management features in
//! a PCI device. It is fully documented in the PCI Power Management Interface Specification.

use modular_bitfield::prelude::*;
use displaydoc::Display as DisplayDoc;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};



#[derive(Debug, PartialEq, Eq,)] 
pub struct PowerManagementInterface {
    pub capabilities: Capabilities,
    pub control: Control,
    pub bridge: Bridge,
    pub data: Option<Data>,
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct CapabilitiesProto {
    version: B3,
    pme_clock: bool,
    reserved: bool,
    device_specific_initialization: bool,
    #[bits = 3]
    aux_current: AuxCurrent,
    d1_support: bool,
    d2_support: bool,
    pme_support_d0: bool,
    pme_support_d1: bool,
    pme_support_d2: bool,
    pme_support_d3_hot: bool,
    pme_support_d3_cold: bool,
}

/// Provides information on the capabilities of the function related to power management
#[derive(Debug, PartialEq, Eq)]
pub struct Capabilities {
    /// Default value of 0b10 indicates that this function complies with Revision 1.1 of the PCI
    /// Power Management Interface Specification.
    pub version: u8,
    /// Indicates that the function relies on the presence of the PCI clock for PME# operation.
    pub pme_clock: bool,
    /// Reserved read-only.
    pub reserved: bool,
    /// Device Specific Initialization (DSI) bit indicates whether special initialization of this
    /// function is required before the generic class device driver is able to use it.
    pub device_specific_initialization: bool,
    pub aux_current: AuxCurrent,
    /// Supports the D1 Power Management State.
    pub d1_support: bool,
    /// Supports the D2 Power Management State.
    pub d2_support: bool,
    pub pme_support: PmeSupport,
}

impl From<CapabilitiesProto> for Capabilities {
    fn from(proto: CapabilitiesProto) -> Self {
        Self {
            version: proto.version(),
            pme_clock: proto.pme_clock(),
            reserved: proto.reserved(),
            device_specific_initialization: proto.device_specific_initialization(),
            aux_current: proto.aux_current(),
            d1_support: proto.d1_support(),
            d2_support: proto.d2_support(),
            pme_support: PmeSupport {
                d0: proto.pme_support_d0(),
                d1: proto.pme_support_d1(),
                d2: proto.pme_support_d2(),
                d3_hot: proto.pme_support_d3_hot(),
                d3_cold: proto.pme_support_d3_cold(),
            },
        }
    }
}
impl From<u16> for Capabilities {
    fn from(word: u16) -> Self { CapabilitiesProto::from(word).into() }
}

/// This 3 bit field reports the 3.3Vaux auxiliary current requirements for the PCI function.
/// he [Data] Register takes precedence over this field for 3.3Vaux current and value must be 0.
#[derive(DisplayDoc, BitfieldSpecifier, Debug, PartialEq, Eq)]
#[bits = 3]
pub enum AuxCurrent {
    /// 0mA
    SelfPowered,
    /// 55mA
    MaxCurrent55mA,
    /// 100mA
    MaxCurrent100mA,
    /// 160mA
    MaxCurrent160mA,
    /// 220mA
    MaxCurrent220mA,
    /// 270mA
    MaxCurrent270mA,
    /// 320mA
    MaxCurrent320mA,
    /// 375mA
    MaxCurrent375mA,
}

/// Indicates the power states in which the function may assert PME#.
#[derive(Debug, PartialEq, Eq)]
pub struct PmeSupport {
    /// PME# can be asserted from D0
    pub d0: bool,
    /// PME# can be asserted from D1
    pub d1: bool,
    /// PME# can be asserted from D2
    pub d2: bool,
    /// PME# can be asserted from D3 *hot*
    pub d3_hot: bool,
    /// PME# can be asserted from D3 *cold*
    pub d3_cold: bool,
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct ControlProto {
    #[bits = 2]
    power_state: PowerState,
    reserved: B6,
    pme_enabled: bool,
    #[bits = 4]
    data_select: DataSelect,
    #[bits = 2]
    data_scale: DataScale,
    pme_status: bool,
}

/// Used to manage the PCI function’s power management state as well as to enable/monitor PMEs.
#[derive(Debug, PartialEq, Eq)]
pub struct Control {
    pub power_state: PowerState,
    /// Reserved bits 07:02
    pub reserved: u8,
    /// PCI_PM_CTRL_NO_SOFT_RESET
    pub no_soft_reset: bool,
    /// Enables the function to assert PME#.
    pub pme_enabled: bool,
    /// This bit is set when the function would normally assert the PME# signal independent of the
    /// state of the [Control.pme_enabled] bit.
    pub pme_status: bool,
}

impl From<ControlProto> for Control {
    fn from(proto: ControlProto) -> Self {
        Self {
            power_state: proto.power_state(),
            reserved: proto.reserved(),
            no_soft_reset: ((proto.reserved() << 2) & 0x0008) != 0,
            pme_enabled: proto.pme_enabled(),
            pme_status: proto.pme_status(),
        }
    }
}
impl From<u16> for Control {
    fn from(word: u16) -> Self { ControlProto::from(word).into() }
}

/// Current power state.
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum PowerState {
    D0,
    D1,
    D2,
    D3Hot,
}


#[bitfield(bits = 8)]
#[repr(u8)]
pub struct BridgeProto {
    reserved: B6,
    b2_b3: bool,
    bpcc_enabled: bool,
}

/// PCI bridge specific functionality and is required for all PCI-toPCI bridges
#[derive(Debug, PartialEq, Eq)]
pub struct Bridge {
    /// Value at reset 0b000000
    pub reserved: u8,
    /// B2_B3# (b2/B3 support for D3hot)
    ///
    /// This field determines the action that is to occur as a direct result of programming the
    /// function to D3Hot
    pub b2_b3: bool,
    /// BPCC_En (Bus Power/Clock Control Enable)
    ///
    /// Indicates that the bus power/clock control mechanism is enabled
    pub bpcc_enabled: bool,
}

impl From<BridgeProto> for Bridge {
    fn from(proto: BridgeProto) -> Self {
        Self {
            reserved: proto.reserved(),
            b2_b3: proto.b2_b3(),
            bpcc_enabled: proto.bpcc_enabled(),
        }
    }
}
impl From<u8> for Bridge {
    fn from(byte: u8) -> Self { BridgeProto::from(byte).into() }
}

/// Register that provides a mechanism for the function to report state dependent operating data
/// such as power consumed or heat dissipation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data {
    pub value: u8,
    pub select: DataSelect,
    pub scale: DataScale,
}

/// Used to select which data is to be reported through the [Data] register and [DataScale].
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 4]
pub enum DataSelect {
    /// D0 Power Consumed
    PowerConsumedD0,
    /// D1 Power Consumed
    PowerConsumedD1,
    /// D2 Power Consumed
    PowerConsumedD2,
    /// D3 Power Consumed
    PowerConsumedD3,
    /// D0 Power Dissipated
    PowerDissipatedD0,
    /// D1 Power Dissipated
    PowerDissipatedD1,
    /// D2 Power Dissipated
    PowerDissipatedD2,
    /// D3 Power Dissipated
    PowerDissipatedD3,
    /// Common logic power consumption
    CommonLogic,
    /// TBD
    Reserved,
}

/// Scaling factor indicated to arrive at the value for the desired measurement.
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum DataScale {
    Unknown,
    /// 0.1x
    Tenth,
    /// 0.01x
    Hundredth,
    /// 0.001x
    Thousandth,
}


impl<'a> TryRead<'a, Endian> for PowerManagementInterface {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let capabilities = bytes.read_with::<u16>(offset, endian)?.into();
        let control_proto = ControlProto::from(bytes.read_with::<u16>(offset, endian)?);
        let bridge = bytes.read_with::<u8>(offset, endian)?.into();
        let data = {
            let value = bytes.read_with::<u8>(offset, endian)?;
            (value != 0).then(|| Data { 
                value,
                select: control_proto.data_select(),
                scale: control_proto.data_scale(),
            })
        };
        let pmi = PowerManagementInterface {
            capabilities,
            control: control_proto.into(),
            bridge,
            data,
        };
        Ok((pmi, *offset))
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn power_management_interface() {
        let data = [0x02,0x7e,0x00,0x00,0x40,0x00];
        // Capabilities: [c0] Power Management version 2
        //         Flags: PMEClk- DSI- D1+ D2+ AuxCurrent=0mA PME(D0+,D1+,D2+,D3hot+,D3cold-)
        //         Status: D0 NoSoftRst- PME-Enable- DSel=0 DScale=0 PME-
        //         Bridge: PM- B3+
        let result = data.read_with::<PowerManagementInterface>(&mut 0, LE).unwrap();
        let sample = PowerManagementInterface {
            capabilities: Capabilities {
                version: 0b10,
                pme_clock: false,
                reserved: false,
                device_specific_initialization: false,
                aux_current: AuxCurrent::SelfPowered,
                d1_support: true,
                d2_support: true,
                pme_support: PmeSupport {
                    d0: true,
                    d1: true,
                    d2: true,
                    d3_hot: true,
                    d3_cold: false,
                },
            },
            control: Control {
                power_state: PowerState::D0,
                reserved: 0b000000,
                no_soft_reset: false,
                pme_enabled: false,
                pme_status: false,
            },
            bridge: Bridge {
                reserved: 0,
                b2_b3: true,
                bpcc_enabled: false,
            },
            data: None,
        };
        assert_eq!(sample, result);
    }
}

