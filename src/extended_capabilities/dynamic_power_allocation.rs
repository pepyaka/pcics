/*!
# Dynamic Power Allocation (DPA)

A common approach to managing power consumption is through a negotiation between the device
driver, operating system, and executing applications. Adding Dynamic Power Allocation for such
devices is anticipated to be done as an extension of that negotiation, through software mechanisms
that are outside of the scope of this specification. Some devices do not have a device specific driver
to manage power efficiently. The DPA Capability provides a mechanism to allocate power
dynamically for these types of devices.

## Struct diagram
<pre>
<a href="struct.DynamicPowerAllocation.html">DynamicPowerAllocation</a>
├─ <a href="struct.DpaCapability.html">DpaCapability</a>
│  ├─ <a href="enum.TransitionLatencyUnit.html">TransitionLatencyUnit</a>
│  └─ <a href="enum.PowerAllocationScale.html">PowerAllocationScale</a>
├─ <a href="struct.DpaStatus.html">DpaStatus</a>
├─ <a href="struct.DpaControl.html">DpaControl</a>
└─ <a href="struct.DpaPowerAllocationArray.html">DpaPowerAllocationArray</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::dynamic_power_allocation::*;
let data = [
    /* 00h */ 0x0c, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x03, 0x22, 0x55, 0xAA, // DPA Capability
    /* 08h */ 0x00, 0x11, 0x22, 0x33, // DPA Latency Indicator
    /* 0Ch */ 0x03, 0x01, // DPA Status
              0x03, 0x00, // DPA Control
    /* 10h */ 0x00, 0x11, 0x22, 0x33, // DPA Power Allocation Array
];

let result: DynamicPowerAllocation = data.as_slice().try_into().unwrap();

let sample = DynamicPowerAllocation {
    dpa_capability: DpaCapability {
        substate_max: 3,
        transition_latency_unit: TransitionLatencyUnit::Unit100ms,
        power_allocation_scale: PowerAllocationScale::Mul0_1,
        transition_latency_value_0: 0x55,
        transition_latency_value_1: 0xAA,
    },
    dpa_latency_indicator: 0x33221100,
    dpa_status: DpaStatus {
        substate_status: 3,
        substate_control_enabled: true,
    },
    dpa_control: DpaControl {
        substate_control: 3,
    },
    dpa_power_allocation_array: DpaPowerAllocationArray(&[0x00, 0x11, 0x22, 0x33]),
};

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P4, P8};
use snafu::Snafu;

use super::ExtendedCapabilityHeader;

#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum DynamicPowerAllocationError {
    #[snafu(display("capability, latency indicator, status and control fields are unreadable"))]
    Mandatory,
    #[snafu(display("number of entries must be equal to the Substate_Max plus one (expected: {expected}, found: {found})"))]
    DpaAllocationArray { expected: usize, found: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicPowerAllocation<'a> {
    pub dpa_capability: DpaCapability,
    /// Each bit indicates which Transition Latency Value is associated with
    /// the corresponding substate
    pub dpa_latency_indicator: u32,
    pub dpa_status: DpaStatus,
    pub dpa_control: DpaControl,
    pub dpa_power_allocation_array: DpaPowerAllocationArray<'a>,
}

impl<'a> TryFrom<&'a [u8]> for DynamicPowerAllocation<'a> {
    type Error = DynamicPowerAllocationError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        // Skip header
        let slice = slice
            .get(ExtendedCapabilityHeader::SIZE..)
            .unwrap_or_default();
        let Seq {
            head: Le((dpa_capability, dpa_latency_indicator, dpa_status, dpa_control)),
            tail,
        } = P4(slice)
            .try_into()
            .map_err(|_| DynamicPowerAllocationError::Mandatory)?;
        let dpa_capability @ DpaCapability { substate_max, .. } = From::<u32>::from(dpa_capability);
        let substate_max = substate_max as usize;
        tail.get(..substate_max + 1)
            .ok_or(DynamicPowerAllocationError::DpaAllocationArray {
                expected: substate_max,
                found: tail.len(),
            })
            .map(|slice| Self {
                dpa_capability,
                dpa_latency_indicator,
                dpa_control: From::<u16>::from(dpa_control),
                dpa_status: From::<u16>::from(dpa_status),
                dpa_power_allocation_array: DpaPowerAllocationArray(slice),
            })
    }
}

/// DPA Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpaCapability {
    /// Indicates the maximum substate number, which is the total number of
    /// supported substates minus one.
    pub substate_max: u8,
    pub transition_latency_unit: TransitionLatencyUnit,
    pub power_allocation_scale: PowerAllocationScale,
    /// This value is multiplied by the [Transition Latency Unit](TransitionLatencyUnit)
    /// to determine the maximum Transition Latency for the substate
    pub transition_latency_value_0: u8,
    /// This value is multiplied by the [Transition Latency Unit](TransitionLatencyUnit)
    /// to determine the maximum Transition Latency for the substate
    pub transition_latency_value_1: u8,
}

impl From<u32> for DpaCapability {
    fn from(dword: u32) -> Self {
        let Lsb((
            substate_max,
            (),
            transition_latency_unit,
            (),
            power_allocation_scale,
            (),
            transition_latency_value_0,
            transition_latency_value_1,
        )) = P8::<_, 5, 3, 2, 2, 2, 2, 8, 8>(dword).into();
        Self {
            substate_max,
            transition_latency_unit: From::<u8>::from(transition_latency_unit),
            power_allocation_scale: From::<u8>::from(power_allocation_scale),
            transition_latency_value_0,
            transition_latency_value_1,
        }
    }
}

/// A substate’s Transition Latency Value is multiplied by the Transition
/// Latency Unit to determine the maximum Transition Latency for the substate
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionLatencyUnit {
    /// 1 ms
    Unit1ms,
    /// 10 ms
    Unit10ms,
    /// 100 ms
    Unit100ms,
    /// Reserved
    Reserved,
}

impl From<u8> for TransitionLatencyUnit {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Unit1ms,
            0b01 => Self::Unit10ms,
            0b10 => Self::Unit100ms,
            0b11 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

/// The encodings provide the scale to determine power allocation per substate in Watts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerAllocationScale {
    /// 10.0x
    Mul10,
    /// 1.0x
    Mul1_0,
    /// 0.1x
    Mul0_1,
    /// 0.01x
    Mul0_01,
}

impl From<u8> for PowerAllocationScale {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Mul10,
            0b01 => Self::Mul1_0,
            0b10 => Self::Mul0_1,
            0b11 => Self::Mul0_01,
            _ => unreachable!(),
        }
    }
}

/// DPA Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpaStatus {
    /// Indicates current substate for this Function
    pub substate_status: u8,
    /// When this field is Set, the Substate Control field determines the
    /// current substate
    pub substate_control_enabled: bool,
}

impl From<u16> for DpaStatus {
    fn from(word: u16) -> Self {
        let Lsb((substate_status, (), substate_control_enabled, ())) =
            P4::<_, 5, 3, 1, 7>(word).into();
        Self {
            substate_status,
            substate_control_enabled,
        }
    }
}

/// DPA Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpaControl {
    pub substate_control: u8,
}

impl From<u16> for DpaControl {
    fn from(word: u16) -> Self {
        let Lsb((substate_control, ())) = P2::<_, 5, 11>(word).into();
        Self { substate_control }
    }
}

/// DPA Power Allocation Array
///
/// Each Substate Power Allocation register indicates the power allocation
/// value for its associated substate
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpaPowerAllocationArray<'a>(pub &'a [u8]);
