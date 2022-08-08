/*!
# Readiness Time Reporting

The Readiness Time Reporting Extended Capability provides an optional mechanism for
describing the time required for a Device or Function to become Configuration-ready.

## Struct diagram
<pre>
<a href="struct.ReadinessTimeReporting.html">ReadinessTimeReporting</a>
└─ <a href="struct.ReadinessTime.html">ReadinessTime x 4</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::readiness_time_reporting::*;
let data = [
    /* 00h */ 0x22, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x11, 0x22, 0x33, 0x80, // Readiness Time Reporting 1
    /* 08h */ 0x44, 0x55, 0x66, 0x00, // Readiness Time Reporting 2
];

let result: ReadinessTimeReporting = data.as_slice().try_into().unwrap();

let sample = ReadinessTimeReporting {
    reset_time: ReadinessTime {
        value: 0x11,
        scale: 0b001,
    },
    dl_up_time: ReadinessTime {
        value: 0x32 | 0x100,
        scale: 0b001,
    },
    valid: true,
    flr_time: ReadinessTime {
        value: 0x44 | 0x100,
        scale: 0b010,
    },
    d3hot_to_d0_time: ReadinessTime {
        value: 0x65,
        scale: 0b011,
    },
};

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P3, P4};

use super::{ExtendedCapabilityDataError, ExtendedCapabilityHeader};

/// Readiness Time Reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessTimeReporting {
    /// The time the Function requires to become Configuration-Ready after the
    /// completion of Conventional Reset
    pub reset_time: ReadinessTime,
    /// The time the Function requires to become Configuration-Ready after the
    /// Downstream Port above the Function reports Data Link Layer Link Active
    pub dl_up_time: ReadinessTime,
    /// Indicates that all time values in this capability are valid
    pub valid: bool,
    /// The time that the Function requires to become Configuration-Ready after
    /// it was issued an FLR
    pub flr_time: ReadinessTime,
    /// The time that the Function requires after it is directed from D3<sub>hot</sub> to
    /// D0 before it is Configuration-Ready and has returned to either D0<sub>uninitialized</sub> or
    /// D0<sub>active</sub> state
    pub d3hot_to_d0_time: ReadinessTime,
}

impl ReadinessTimeReporting {
    /// Size in bytes (with Extended Capability Header)
    pub const SIZE: usize = 0x0c;
}

impl TryFrom<&[u8]> for ReadinessTimeReporting {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        // Skip header
        let slice = slice
            .get(ExtendedCapabilityHeader::SIZE..)
            .unwrap_or_default();
        let Seq {
            head: Le((r1, r2)), ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Readiness Time Reporting",
                size: Self::SIZE,
            })?;
        let Lsb((reset_time, dl_up_time, (), valid)) = P4::<u32, 12, 12, 7, 1>(r1).into();
        let Lsb((flr_time, d3hot_to_d0_time, ())) = P3::<u32, 12, 12, 8>(r2).into();
        Ok(Self {
            reset_time: From::<u16>::from(reset_time),
            dl_up_time: From::<u16>::from(dl_up_time),
            valid,
            flr_time: From::<u16>::from(flr_time),
            d3hot_to_d0_time: From::<u16>::from(d3hot_to_d0_time),
        })
    }
}

/// Readiness Time Encoding
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessTime {
    pub value: u16,
    pub scale: u8,
}

impl ReadinessTime {
    /// The actual time value is *[Value](Self::value) x 32[<sup>scale</sup>](Self::scale)*
    pub fn actual_time_value(&self) -> u64 {
        (self.value as u64) << (self.scale * 5)
    }
}

impl From<u16> for ReadinessTime {
    fn from(word: u16) -> Self {
        let Lsb((value, scale, ())) = P3::<_, 9, 3, 4>(word).into();
        Self { value, scale }
    }
}
