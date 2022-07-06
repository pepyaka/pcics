/*!
# Root Complex Event Collector Endpoint Association

The PCI Express Root Complex Event Collector Endpoint Association Capability is implemented
by Root Complex Event Collectors.

It declares the RCiEPs supported by the Root Complex Event Collector on the same Logical Bus on
which the Root Complex Event Collector is located. A Root Complex Event Collector must
implement the Root Complex Event Collector Endpoint Association Capability; no other PCI
Express device Function is permitted to implement this Capability.

## Struct diagram
<pre>
<a href="struct.RootComplexEventCollectorEndpointAssociation.html">RootComplexEventCollectorEndpointAssociation</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::root_complex_event_collector_endpoint_association::*;
let data = [
    0x07, 0x00, 0x00, 0x00, // Extended Capability Header
    0x44, 0x33, 0x22, 0x11, // Association Bitmap for RCiEPs
];
let result = data[4..].try_into().unwrap();
let sample = RootComplexEventCollectorEndpointAssociation {
    association_bitmap_for_rcieps: 0x11223344
};
assert_eq!(sample, result);
```
*/

use heterob::{endianness::Le, Seq};

use super::ExtendedCapabilityDataError;

/// Root Complex Event Collector Endpoint Association
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootComplexEventCollectorEndpointAssociation {
    /// The Association Bitmap for RCiEPs is a read-only register that sets
    /// the bits corresponding to the Device Numbers of RCiEPs supported by the Root
    /// Complex Event Collector on the same Logical Bus as the Event Collector itself.
    pub association_bitmap_for_rcieps: u32,
}

impl RootComplexEventCollectorEndpointAssociation {
    pub const SIZE: usize = 4;
}

impl From<[u8; Self::SIZE]> for RootComplexEventCollectorEndpointAssociation {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le(association_bitmap_for_rcieps) = bytes.into();
        Self {
            association_bitmap_for_rcieps,
        }
    }
}

impl TryFrom<&[u8]> for RootComplexEventCollectorEndpointAssociation {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq { head, .. } = slice.try_into().map_err(|_| ExtendedCapabilityDataError {
            name: "Root Complex Event Collector Endpoint Association",
            size: Self::SIZE,
        })?;
        Ok(From::<[u8; Self::SIZE]>::from(head))
    }
}
