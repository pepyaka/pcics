/*!
# Power Budgeting

The PCI Express Power Budgeting Capability allows the system to allocate power to devices that
are added to the system at runtime.

## Struct diagram
[PowerBudgeting]
- [Data]
  - [BasePower]
  - [DataScale]
  - [PmSubState]
  - [PmState]
  - [OperationConditionType]
  - [PowerRail]
- [PowerBudgetCapability]

## Examples

```rust
# use pcics::extended_capabilities::power_budgeting::*;
let data = [
    0x04, 0x00, 0x01, 0x16, 0x00, 0x00, 0x00, 0x00,
    0x1b, 0x81, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00,
];
let result = data[4..].try_into().unwrap();
let sample = PowerBudgeting {
    data_select: 0x00,
    data: Data {
        base_power: BasePower::Value(0x1b),
        data_scale: DataScale::Deci,
        pm_sub_state: PmSubState::Default,
        pm_state: PmState::D0,
        operation_condition_type: OperationConditionType::Maximum,
        power_rail: PowerRail::Power3_3v,
    },
    power_budget_capability: PowerBudgetCapability {
        system_allocated: true,
    },
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P5, P7};

use super::ExtendedCapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerBudgeting {
    /// Data Select
    pub data_select: u8,
    pub data: Data,
    pub power_budget_capability: PowerBudgetCapability,
}
impl TryFrom<&[u8]> for PowerBudgeting {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((data_select, rsvdp_0, data, power_budget_capability, rsvdp_1)),
            ..
        } = P5(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Power Budgeting",
                size: 12,
            })?;
        let _: ([u8; 3], [u8; 3]) = (rsvdp_0, rsvdp_1);
        let Lsb((
            base_power,
            data_scale,
            pm_sub_state,
            pm_state,
            operation_condition_type,
            power_rail,
            (),
        )) = P7::<u32, 8, 2, 3, 2, 3, 3, 11>(data).into();
        let Lsb((system_allocated, ())) = P2::<u8, 1, 7>(power_budget_capability).into();
        Ok(Self {
            data_select,
            data: Data {
                base_power: From::<u8>::from(base_power),
                data_scale: From::<u8>::from(data_scale),
                pm_sub_state: From::<u8>::from(pm_sub_state),
                pm_state: From::<u8>::from(pm_state),
                operation_condition_type: From::<u8>::from(operation_condition_type),
                power_rail: From::<u8>::from(power_rail),
            },
            power_budget_capability: PowerBudgetCapability { system_allocated },
        })
    }
}

/// Power Budgeting Data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub base_power: BasePower,
    pub data_scale: DataScale,
    pub pm_sub_state: PmSubState,
    pub pm_state: PmState,
    pub operation_condition_type: OperationConditionType,
    pub power_rail: PowerRail,
}

/// Specifies in watts the base power value in the given operating condition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasePower {
    Value(u8),
    Gt239Le250,
    Gt250Le275,
    Gt275Le300,
    Gt300,
}
impl From<u8> for BasePower {
    fn from(byte: u8) -> Self {
        match byte {
            v @ 0x00..=0xEF => Self::Value(v),
            0xF0 => Self::Gt239Le250,
            0xF1 => Self::Gt250Le275,
            0xF2 => Self::Gt275Le300,
            _ => Self::Gt300,
        }
    }
}

/// Specifies the scale to apply to the Base Power value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataScale {
    /// 1.0x
    One,
    /// 0.1x
    Deci,
    /// 0.01x
    Centi,
    /// 0.001x
    Milli,
}
impl DataScale {
    pub fn multiplier(&self) -> f64 {
        match self {
            Self::One => 1.0,
            Self::Deci => 0.1,
            Self::Centi => 0.01,
            Self::Milli => 0.001,
        }
    }
}
impl From<u8> for DataScale {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::One,
            0b01 => Self::Deci,
            0b10 => Self::Centi,
            0b11 => Self::Milli,
            _ => unreachable!(),
        }
    }
}

/// Specifies the power management sub state of the operating condition being described
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PmSubState {
    /// Default Sub State
    Default,
    /// Device Specific Sub State
    Specific(u8),
}
impl From<u8> for PmSubState {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Default,
            v => Self::Specific(v),
        }
    }
}

/// Specifies the power management state of the operating condition being described
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PmState {
    D0,
    D1,
    D2,
    D3,
}
impl From<u8> for PmState {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::D0,
            0b01 => Self::D1,
            0b10 => Self::D2,
            0b11 => Self::D3,
            _ => unreachable!(),
        }
    }
}

/// Specifies the type of the operating condition being described
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationConditionType {
    /// PME Aux
    PmeAux,
    /// Auxiliary
    Auxiliary,
    /// Idle
    Idle,
    /// Sustained
    Sustained,
    /// Sustained – Emergency Power Reduction State
    SustainedEmergencyPowerReductionState,
    /// Maximum – Emergency Power Reduction State
    MaximumEmergencyPowerReductionState,
    /// Maximum
    Maximum,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for OperationConditionType {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::PmeAux,
            0b001 => Self::Auxiliary,
            0b010 => Self::Idle,
            0b011 => Self::Sustained,
            0b100 => Self::SustainedEmergencyPowerReductionState,
            0b101 => Self::MaximumEmergencyPowerReductionState,
            0b111 => Self::Maximum,
            v => Self::Reserved(v),
        }
    }
}

/// Specifies the thermal load or power rail of the operating condition being described
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerRail {
    /// Power (12V)
    Power12v,
    /// Power (3.3V)
    Power3_3v,
    /// Power (1.5V or 1.8V)
    Power1_5vOr1_8v,
    /// Thermal
    Thermal,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for PowerRail {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Power12v,
            0b01 => Self::Power3_3v,
            0b10 => Self::Power1_5vOr1_8v,
            0b11 => Self::Thermal,
            v => Self::Reserved(v),
        }
    }
}

/// Power Budget Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerBudgetCapability {
    /// System Allocated
    pub system_allocated: bool,
}
