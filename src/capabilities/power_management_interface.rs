/*!
# PCI Power Management Interface

This capability structure provides a standard interface to control power management features in
a PCI device. It is fully documented in the PCI Power Management Interface Specification.

## Struct diagram
[PowerManagementInterface]
- [Capabilities]
  - [AuxCurrent]
  - [PmeSupport]
- [Control]
  - [PowerState]
  - [DataSelect]
  - [DataScale]
- [Bridge]

## Examples

> Power Management version 3
Flags: PMEClk- DSI- D1- D2- AuxCurrent=0mA PME(D0-,D1-,D2-,D3hot-,D3cold-)
Status: D0 NoSoftRst+ PME-Enable- DSel=0 DScale=0 PME-

```rust
# use pcics::capabilities::power_management_interface::*;
let data = [0x01, 0x00, 0x03, 0x00, 0x08, 0x00, 0x00, 0x00,];
let result = data[2..].try_into().unwrap();
let sample = PowerManagementInterface {
    capabilities: Capabilities {
        version: 3,
        pme_clock: false,
        immediate_readiness_on_return_to_d0: false,
        device_specific_initialization: false,
        aux_current: AuxCurrent::SelfPowered,
        d1_support: false,
        d2_support: false,
        pme_support: PmeSupport {
            d0: false,
            d1: false,
            d2: false,
            d3_hot: false,
            d3_cold: false,
        },
    },
    control: Control {
        power_state: PowerState::D0,
        no_soft_reset: true,
        pme_enabled: false,
        data_select: DataSelect::PowerConsumedD0,
        data_scale: DataScale::Unknown,
        pme_status: false,
    },
    bridge: Bridge {
        reserved: 0,
        b2_b3: false,
        bpcc_enabled: false,
    },
    data: 0,
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P12, P3, P4, P8};

use super::CapabilityDataError;

use displaydoc::Display as DisplayDoc;

#[derive(Debug, PartialEq, Eq)]
pub struct PowerManagementInterface {
    pub capabilities: Capabilities,
    pub control: Control,
    pub bridge: Bridge,
    pub data: u8,
}
impl PowerManagementInterface {
    pub fn data(&self) -> Option<Data> {
        if self.data == 0 {
            None
        } else {
            Some(Data {
                value: self.data,
                select: self.control.data_select,
                scale: self.control.data_scale,
            })
        }
    }
}
impl TryFrom<&[u8]> for PowerManagementInterface {
    type Error = CapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((capabilities, control, bridge, data)),
            ..
        } = P4(slice).try_into().map_err(|_| CapabilityDataError {
            name: "Power Management Interface",
            size: 6,
        })?;
        let Lsb((
            version,
            pme_clock,
            immediate_readiness_on_return_to_d0,
            device_specific_initialization,
            aux_current,
            d1_support,
            d2_support,
            d0,
            d1,
            d2,
            d3_hot,
            d3_cold,
        )) = P12::<u16, 3, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1>(capabilities).into();
        let Lsb((
            power_state,
            (),
            no_soft_reset,
            (),
            pme_enabled,
            data_select,
            data_scale,
            pme_status,
        )) = P8::<u16, 2, 1, 1, 4, 1, 4, 2, 1>(control).into();
        let Lsb((reserved, b2_b3, bpcc_enabled)) = P3::<u8, 6, 1, 1>(bridge).into();
        Ok(Self {
            capabilities: Capabilities {
                version,
                pme_clock,
                immediate_readiness_on_return_to_d0,
                device_specific_initialization,
                aux_current: From::<u8>::from(aux_current),
                d1_support,
                d2_support,
                pme_support: PmeSupport {
                    d0,
                    d1,
                    d2,
                    d3_hot,
                    d3_cold,
                },
            },
            control: Control {
                power_state: From::<u8>::from(power_state),
                no_soft_reset,
                pme_enabled,
                data_select: From::<u8>::from(data_select),
                data_scale: From::<u8>::from(data_scale),
                pme_status,
            },
            bridge: Bridge {
                reserved,
                b2_b3,
                bpcc_enabled,
            },
            data,
        })
    }
}

/// Provides information on the capabilities of the function related to power management
#[derive(Debug, PartialEq, Eq)]
pub struct Capabilities {
    /// Default value of 0b10 indicates that this function complies with Revision 1.1 of the PCI
    /// Power Management Interface Specification.
    pub version: u8,
    /// Indicates that the function relies on the presence of the PCI clock for PME# operation.
    pub pme_clock: bool,
    /// Function is guaranteed to be ready to successfully complete valid accesses immediately
    /// after being set to D0
    pub immediate_readiness_on_return_to_d0: bool,
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

/// This 3 bit field reports the 3.3Vaux auxiliary current requirements for the PCI function.
/// he [Data] Register takes precedence over this field for 3.3Vaux current and value must be 0.
#[derive(DisplayDoc, Debug, PartialEq, Eq)]
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
impl From<u8> for AuxCurrent {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::SelfPowered,
            1 => Self::MaxCurrent55mA,
            2 => Self::MaxCurrent100mA,
            3 => Self::MaxCurrent160mA,
            4 => Self::MaxCurrent220mA,
            5 => Self::MaxCurrent270mA,
            6 => Self::MaxCurrent320mA,
            7 => Self::MaxCurrent375mA,
            _ => unreachable!(),
        }
    }
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

/// Used to manage the PCI function’s power management state as well as to enable/monitor PMEs.
#[derive(Debug, PartialEq, Eq)]
pub struct Control {
    pub power_state: PowerState,
    /// State of the Function after writing the PowerState field to transition the Function
    /// from D3hot to D0.
    pub no_soft_reset: bool,
    /// Enables the function to assert PME#.
    pub pme_enabled: bool,
    pub data_select: DataSelect,
    pub data_scale: DataScale,
    /// This bit is set when the function would normally assert the PME# signal independent of the
    /// state of the [Control.pme_enabled] bit.
    pub pme_status: bool,
}

/// Current power state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    D0,
    D1,
    D2,
    D3Hot,
}
impl From<u8> for PowerState {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::D0,
            0b01 => Self::D1,
            0b10 => Self::D2,
            0b11 => Self::D3Hot,
            _ => unreachable!(),
        }
    }
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

/// Register that provides a mechanism for the function to report state dependent operating data
/// such as power consumed or heat dissipation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data {
    pub value: u8,
    pub select: DataSelect,
    pub scale: DataScale,
}

/// Used to select which data is to be reported through the [Data] register and [DataScale].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Reserved(u8),
}
impl From<u8> for DataSelect {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::PowerConsumedD0,
            1 => Self::PowerConsumedD1,
            2 => Self::PowerConsumedD2,
            3 => Self::PowerConsumedD3,
            4 => Self::PowerDissipatedD0,
            5 => Self::PowerDissipatedD1,
            6 => Self::PowerDissipatedD2,
            7 => Self::PowerDissipatedD3,
            8 => Self::CommonLogic,
            v => Self::Reserved(v),
        }
    }
}
impl From<DataSelect> for u8 {
    fn from(data: DataSelect) -> Self {
        match data {
            DataSelect::PowerConsumedD0 => 0,
            DataSelect::PowerConsumedD1 => 1,
            DataSelect::PowerConsumedD2 => 2,
            DataSelect::PowerConsumedD3 => 3,
            DataSelect::PowerDissipatedD0 => 4,
            DataSelect::PowerDissipatedD1 => 5,
            DataSelect::PowerDissipatedD2 => 6,
            DataSelect::PowerDissipatedD3 => 7,
            DataSelect::CommonLogic => 8,
            DataSelect::Reserved(v) => v,
        }
    }
}

/// Scaling factor indicated to arrive at the value for the desired measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataScale {
    Unknown,
    /// 0.1x
    Tenth,
    /// 0.01x
    Hundredth,
    /// 0.001x
    Thousandth,
}
impl From<u8> for DataScale {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Unknown,
            0b01 => Self::Tenth,
            0b10 => Self::Hundredth,
            0b11 => Self::Thousandth,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn power_management_interface() {
        let data = [0x02, 0x7e, 0x00, 0x00, 0x40, 0x00];
        // Capabilities: [c0] Power Management version 2
        //         Flags: PMEClk- DSI- D1+ D2+ AuxCurrent=0mA PME(D0+,D1+,D2+,D3hot+,D3cold-)
        //         Status: D0 NoSoftRst- PME-Enable- DSel=0 DScale=0 PME-
        //         Bridge: PM- B3+
        let result = data.as_slice().try_into().unwrap();
        let sample = PowerManagementInterface {
            capabilities: Capabilities {
                version: 0b10,
                pme_clock: false,
                immediate_readiness_on_return_to_d0: false,
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
                no_soft_reset: false,
                pme_enabled: false,
                data_select: DataSelect::PowerConsumedD0,
                data_scale: DataScale::Unknown,
                pme_status: false,
            },
            bridge: Bridge {
                reserved: 0,
                b2_b3: true,
                bpcc_enabled: false,
            },
            data: 0,
        };
        assert_eq!(sample, result);
    }
}
