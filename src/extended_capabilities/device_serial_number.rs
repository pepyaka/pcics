/*!
# Device Serial Number Capability

The PCI Express Device Serial Number Capability is an optional Extended Capability that may be
implemented by any PCI Express device Function. The Device Serial Number is a read-only 64-bit
value that is unique for a given PCI Express device.

## Struct diagram
[DeviceSerialNumber]

## Examples

```rust
# use pcics::extended_capabilities::device_serial_number::*;
let data = [
    0x03, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
];
let result = data[4..].try_into().unwrap();
let sample = DeviceSerialNumber {
   lower_dword: 0x44332211,
   upper_dword: 0x88776655,
};
assert_eq!(sample, result);
```
*/

use heterob::{endianness::Le, Seq, P2};

use super::ExtendedCapabilityDataError;

/// The Serial Number register is a 64-bit field that contains the IEEE defined 64-bit extended
/// unique identifier (EUI-64™).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSerialNumber {
    /// PCI Express Device Serial Number (1st DW)
    pub lower_dword: u32,
    /// PCI Express Device Serial Number (2nd DW)
    pub upper_dword: u32,
}
impl TryFrom<&[u8]> for DeviceSerialNumber {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((lower_dword, upper_dword)),
            ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Device Serial Number",
                size: 8,
            })?;
        Ok(Self {
            lower_dword,
            upper_dword,
        })
    }
}
