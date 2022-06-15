/*!
# PCI-X

PCI-X, short for Peripheral Component Interconnect eXtended, is a computer bus
and expansion card standard that enhances the 32-bit PCI local bus for higher
bandwidth demanded mostly by servers and workstations.

## Struct diagram
<pre>
<a href="struct.PciX.html">PciX</a>
</pre>

## Examples

```rust
# use pcics::capabilities::flattening_portal_bridge::*;
// let data = [
// ];
// let result = data[4..].try_into().unwrap();
// assert_eq!(sample, result);
```
*/

use super::CapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciX;

impl TryFrom<&[u8]> for PciX {
    type Error = CapabilityDataError;

    fn try_from(_slice: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
