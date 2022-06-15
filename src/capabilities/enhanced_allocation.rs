/*!
# Enhanced Allocation

The Enhanced Allocation (EA) Capability is an optional Capability that allows
the allocation of I/O, Memory and Bus Number resources in ways not possible
with the BAR and Base/Limit mechanisms in the Type 0 and Type 1 Configuration
Headers.

## Struct diagram
<pre>
<a href="struct.EnhancedAllocation.html">EnhancedAllocation</a>
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
pub struct EnhancedAllocation;

impl TryFrom<&[u8]> for EnhancedAllocation {
    type Error = CapabilityDataError;

    fn try_from(_slice: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
