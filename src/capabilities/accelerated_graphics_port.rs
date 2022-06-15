/*!
# Accelerated Graphics Port

Accelerated Graphics Port (AGP) is a parallel expansion card standard, designed
for attaching a video card to a computer system to assist in the acceleration
of 3D computer graphics.

## Struct diagram
<pre>
<a href="struct.AcceleratedGraphicsPort.html">AcceleratedGraphicsPort</a>
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
pub struct AcceleratedGraphicsPort;

impl TryFrom<&[u8]> for AcceleratedGraphicsPort {
    type Error = CapabilityDataError;

    fn try_from(_slice: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
