/*!
# Slot Identification

This Capability structure identifies a bridge that provides external expansion capabilities

## Struct diagram
[SlotIdentification]
- [ExpansionSlot]

## Examples

> Slot ID: 2 slots, First+, chassis 0x02

```rust
# use pcics::capabilities::slot_identification::*;
let data = [0x04, 0x00, 0x22, 0x02];
let result = data[2..].try_into().unwrap();
let sample = SlotIdentification {
    expansion_slot: ExpansionSlot {
        expansion_slots_provided: 2,
        first_in_chassis: true,
    },
    chassis_number: 2,
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P3};

use super::CapabilityDataError;

/// Slot Identification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotIdentification {
    pub expansion_slot: ExpansionSlot,
    /// Contains the physical chassis number for the slots on this bridge’s secondary interface
    pub chassis_number: u8,
}
impl<'a> TryFrom<&'a [u8]> for SlotIdentification {
    type Error = CapabilityDataError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((expansion_slot, chassis_number)),
            ..
        } = P2(slice).try_into().map_err(|_| CapabilityDataError {
            name: "Slot Identification",
            size: 2,
        })?;
        let Lsb((expansion_slots_provided, first_in_chassis, ())) =
            P3::<u8, 5, 1, 2>(expansion_slot).into();
        let expansion_slot = ExpansionSlot {
            expansion_slots_provided,
            first_in_chassis,
        };
        Ok(Self {
            expansion_slot,
            chassis_number,
        })
    }
}

/// Provides information used by system software in calculating the slot number of a device plugged
/// into a PCI slot in an expansion chassis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpansionSlot {
    /// Number of PCI expansion slots located directly on the secondary interface of this bridge
    pub expansion_slots_provided: u8,
    /// Indicates that this bridge is the first in an expansion chassis
    pub first_in_chassis: bool,
}
