/*!
Debug port

# Examples

Capability value: `Debug port: BAR=1 offset=00a0`

```rust
# use pcics::capabilities::DebugPort;
let data = [0x0a, 0x98, 0xa0, 0x20];
let dp = data[2..].try_into().unwrap();
assert_eq!(DebugPort { offset: 0x00a0, bar_number: 1 }, dp);
```
*/

use heterob::{endianness::LeBytesTryInto, Seq};

use super::CapabilityDataError;

/// Debug port
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugPort {
    /// Indicates the byte offset (up to 4K) within the BAR indicated by BAR#
    pub offset: u16,
    /// Indicates which one of the possible 6 Base Address Register offsets contains the Debug Port
    /// registers
    pub bar_number: u8,
}
impl TryFrom<&[u8]> for DebugPort {
    type Error = CapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq { head, .. }: Seq<u16, _> =
            slice.le_bytes_try_into().map_err(|_| CapabilityDataError {
                name: "Debug port",
                size: 2,
            })?;
        Ok(DebugPort {
            offset: head & 0x1fff,
            bar_number: (head >> 13) as u8,
        })
    }
}
