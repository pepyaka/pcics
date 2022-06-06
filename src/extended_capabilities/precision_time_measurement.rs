/*!
# Precision Time Measurement (PTM)

Precision Time Measurement (PTM) enables precise coordination of events across multiple
components with independent local time clocks.

## Struct diagram
[PrecisionTimeMeasurement]
- [PtmCapability]
- [PtmControl]

## Examples

> ```text
> PTMCap: Requester:- Responder:+ Root:+
> PTMClockGranularity: 4ns
> PTMControl: Enabled:- RootSelected:-
> PTMEffectiveGranularity: Unknown
> ```

```rust
# use pcics::extended_capabilities::precision_time_measurement::*;
let data = [
    0x1f, 0x00, 0x01, 0x22, 0x06, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
let sample = PrecisionTimeMeasurement {
    ptm_capability: PtmCapability {
        ptm_requester_capable: false,
        ptm_responder_capable: true,
        ptm_root_capable: true,
        local_clock_granularity: 4,
    },
    ptm_control: PtmControl {
        ptm_enable: false,
        root_select: false,
        effective_granularity: 0x00,
    },
};
let result = data[4..].try_into().unwrap();
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P5, P6};

use super::ExtendedCapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecisionTimeMeasurement {
    /// PTM Capability
    pub ptm_capability: PtmCapability,
    /// PTM Control
    pub ptm_control: PtmControl,
}
impl TryFrom<&[u8]> for PrecisionTimeMeasurement {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((ptm_capability, ptm_control)),
            ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Precision Time Measurement",
                size: 8,
            })?;
        Ok(Self {
            ptm_capability: From::<u32>::from(ptm_capability),
            ptm_control: From::<u32>::from(ptm_control),
        })
    }
}

/// Describes a Function’s support for Precision Time Measurement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtmCapability {
    /// PTM Requester Capable
    pub ptm_requester_capable: bool,
    /// PTM Responder Capable
    pub ptm_responder_capable: bool,
    /// PTM Root Capable
    pub ptm_root_capable: bool,
    /// Local Clock Granularity
    pub local_clock_granularity: u8,
}

impl From<u32> for PtmCapability {
    fn from(dword: u32) -> Self {
        let Lsb((
            ptm_requester_capable,
            ptm_responder_capable,
            ptm_root_capable,
            (),
            local_clock_granularity,
            (),
        )) = P6::<_, 1, 1, 1, 5, 8, 16>(dword).into();
        Self {
            ptm_requester_capable,
            ptm_responder_capable,
            ptm_root_capable,
            local_clock_granularity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtmControl {
    /// PTM Enable
    pub ptm_enable: bool,
    /// Root Select
    pub root_select: bool,
    /// Effective Granularity
    pub effective_granularity: u8,
}

impl From<u32> for PtmControl {
    fn from(dword: u32) -> Self {
        let Lsb((ptm_enable, root_select, (), effective_granularity, ())) =
            P5::<_, 1, 1, 6, 8, 16>(dword).into();
        Self {
            ptm_enable,
            root_select,
            effective_granularity,
        }
    }
}
