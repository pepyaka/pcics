//! Power Budgeting
//!
//! The PCI Express Power Budgeting Capability allows the system to allocate power to devices that
//! are added to the system at runtime.

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerBudgeting {
    /// Data Select
    pub data_select: u8,
    /// Data
    pub data: Data,
    /// Power Budget Capability
    pub power_budget_capability: PowerBudgetCapability,
}
impl<'a> TryRead<'a, Endian> for PowerBudgeting {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let pb = PowerBudgeting {
            data_select: bytes.read_with::<u8>(offset, endian)?,
            data: {
                let _rsvdp = bytes.read_with::<&[u8]>(offset, Bytes::Len(3))?;
                bytes.read_with::<u32>(offset, endian)?.into()
            },
            power_budget_capability: bytes.read_with::<u8>(offset, endian)?.into(),
        };
        Ok((pb, *offset))
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct DataProto {
    base_power: B8,
    data_scale: B2,
    pm_sub_state: B3,
    pm_state: B2,
    operation_condition_type: B3,
    power_rail: B3,
    rsvdp: B11,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    /// Base Power
    pub base_power: BasePower,
    /// Data Scale
    pub data_scale: DataScale,
    /// PM Sub State
    pub pm_sub_state: PmSubState,
    /// PM State
    pub pm_state: PmState,
    /// Type
    pub operation_condition_type: OperationConditionType,
    /// Power Rail
    pub power_rail: PowerRail,
}
impl From<DataProto> for Data {
    fn from(proto: DataProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            base_power: proto.base_power().into(),
            data_scale: proto.data_scale().into(),
            pm_sub_state: proto.pm_sub_state().into(),
            pm_state: proto.pm_state().into(),
            operation_condition_type: proto.operation_condition_type().into(),
            power_rail: proto.power_rail().into(),
        }
    }
}
impl From<u32> for Data {
    fn from(dword: u32) -> Self { DataProto::from(dword).into() }
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
}
impl From<u8> for PowerRail {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Power12v,
            0b01 => Self::Power3_3v,
            0b10 => Self::Power1_5vOr1_8v,
            0b11 => Self::Thermal,
            _ => unreachable!(),
        }
    }
}


#[bitfield(bits = 8)]
#[repr(u8)]
pub struct PowerBudgetCapabilityProto {
    system_allocated: bool,
    rsvdp: B7,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerBudgetCapability {
    /// System Allocated
    pub system_allocated: bool,
}
impl From<PowerBudgetCapabilityProto> for PowerBudgetCapability {
    fn from(proto: PowerBudgetCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            system_allocated: proto.system_allocated(),
        }
    }
}
impl From<u8> for PowerBudgetCapability {
    fn from(byte: u8) -> Self { PowerBudgetCapabilityProto::from(byte).into() }
}
