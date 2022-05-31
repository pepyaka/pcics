/*!
# Latency Tolerance Reporting (LTR)

The PCI Express Latency Tolerance Reporting (LTR) Capability is an optional Extended Capability
that allows software to provide platform latency information to components with Upstream Ports
(Endpoints and Switches), and is required for Switch Upstream Ports and Endpoints if the
Function supports the LTR mechanism. It is not applicable to Root Ports, Bridges, or Switch
Downstream Ports.

## Struct diagram
[LatencyToleranceReporting]

## Examples
> ```text
> Max snoop latency: 71680ns
> Max no snoop latency: 71680ns
> ```

```rust
# use pcics::extended_capabilities::latency_tolerance_reporting::*;
let data = [
    0x18, 0x00, 0x01, 0x1d, 0x46, 0x08, 0x46, 0x08,
];
let result = data[4..].try_into().unwrap();
let sample = LatencyToleranceReporting {
    max_snoop_latency: MaxLatency { value: 70, scale: 2 },
    max_no_snoop_latency: MaxLatency { value: 70, scale: 2 },
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P3};

use super::ExtendedCapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatencyToleranceReporting {
    /// Max Snoop Latency
    pub max_snoop_latency: MaxLatency,
    /// Max No-Snoop Latency
    pub max_no_snoop_latency: MaxLatency,
}
impl TryFrom<&[u8]> for LatencyToleranceReporting {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((max_snoop_latency, max_no_snoop_latency)),
            ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Latency Tolerance Reporting",
                size: 8,
            })?;
        Ok(Self {
            max_snoop_latency: From::<u16>::from(max_snoop_latency),
            max_no_snoop_latency: From::<u16>::from(max_no_snoop_latency),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxLatency {
    /// Specifies the maximum latency that a device is permitted to request
    pub value: u16,
    /// Provides a scale for the value contained within the value field
    pub scale: u8,
}
impl MaxLatency {
    pub fn value(&self) -> usize {
        match self.scale {
            scale @ 0..=5 => (self.value as usize) << (5 * scale),
            _ => 0,
        }
    }
}

impl From<u16> for MaxLatency {
    fn from(word: u16) -> Self {
        let Lsb((value, scale, ())) = P3::<_, 10, 3, 3>(word).into();
        Self { value, scale }
    }
}

impl From<MaxLatency> for u16 {
    fn from(data: MaxLatency) -> Self {
        (data.scale as u16) << 13 | data.value
    }
}
