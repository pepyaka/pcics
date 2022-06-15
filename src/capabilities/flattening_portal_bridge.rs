/*!
# Flattening Portal Bridge

The Flattening Portal Bridge (FPB) is an optional mechanism which can be used
to improve the scalability and runtime reallocation of Routing IDs and Memory
Space resources.

## Struct diagram
<pre>
<a href="struct.FlatteningPortalBridge.html">FlatteningPortalBridge</a>
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
pub struct FlatteningPortalBridge;

impl TryFrom<&[u8]> for FlatteningPortalBridge {
    type Error = CapabilityDataError;

    fn try_from(_slice: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
