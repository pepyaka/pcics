/*!
# Advanced Features

For conventional PCI devices integrated into a PCI Express Root Complex, the Advanced Features
(AF) capability provides mechanisms for using advanced features originally devleoped for PCI
Express


## Struct diagram
[AdvancedFeatures]
- [Capabilities]
- [Control]
- [Status]

## Examples

> AFCap: TP+ FLR+, AFCtrl: FLR-, AFStatus: TP-

```rust
# use pcics::capabilities::advanced_features::*;
let data = [0x13, 0x00, 0x06, 0x03, 0x00, 0x00,];
let result = data[2..].try_into().unwrap();
let sample = AdvancedFeatures {
    length: 6,
    capabilities: Capabilities {
        transactions_pending: true,
        function_level_reset: true,
    },
    control: Control {
        initiate_flr: false,
    },
    status: Status {
        transactions_pending: false,
    },
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P3, P4};

use super::CapabilityDataError;

/// Advanced Features
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancedFeatures {
    /// AF Structure Length (Bytes). Shall return a value of 06h.
    pub length: u8,
    pub capabilities: Capabilities,
    pub control: Control,
    pub status: Status,
}
impl TryFrom<&[u8]> for AdvancedFeatures {
    type Error = CapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((length, capabilities, control, status)),
            ..
        } = P4(slice).try_into().map_err(|_| CapabilityDataError {
            name: "Advanced Features",
            size: 4,
        })?;
        let Lsb((cap_tp, function_level_reset, ())) = P3::<u8, 1, 1, 6>(capabilities).into();
        let Lsb((initiate_flr, ())) = P2::<u8, 1, 7>(control).into();
        let Lsb((st_tp, ())) = P2::<u8, 1, 7>(status).into();
        Ok(Self {
            length,
            capabilities: Capabilities {
                transactions_pending: cap_tp,
                function_level_reset,
            },
            control: Control { initiate_flr },
            status: Status {
                transactions_pending: st_tp,
            },
        })
    }
}

/// AF Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    /// Indicate support for the Transactions Pending (TP) bit. TP must be supported if FLR is
    /// supported.
    pub transactions_pending: bool,
    /// indicate support for Function Level Reset (FLR).
    pub function_level_reset: bool,
}

/// AF Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
    /// A write of 1b initiates Function Level Reset (FLR). The value read by software from this
    /// bit shall always be 0b.
    pub initiate_flr: bool,
}

/// AF Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    /// Indicates that the Function has issued one or more non-posted transactions which have not
    /// been completed, including non-posted transactions that a target has terminated with Retry
    pub transactions_pending: bool,
}
