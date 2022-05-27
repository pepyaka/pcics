/*!
# Vital Product Data

Vital Product Data (VPD) is the information that uniquely defines items such as the hardware,
software, and microcode elements of a system.

## Struct diagram
[VitalProductData]

## Examples

```rust
# use pcics::capabilities::vital_product_data::*;
let data = [0x03, 0x58, 0x8c, 0x81, 0x00, 0x00, 0x00, 0x78,];
let result = data[2..].try_into().unwrap();
let sample = VitalProductData {
    vpd_address: 0x018c,
    transfer_completed: true,
    vpd_data: 0x78000000,
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2};

use super::CapabilityDataError;

/// Vital Product Data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VitalProductData {
    /// DWORD-aligned byte address of the VPD to be accessed
    pub vpd_address: u16,
    /// Indicate when the transfer of data between the VPD Data register and the storage component
    /// is completed
    pub transfer_completed: bool,
    /// VPD Data
    pub vpd_data: u32,
}
impl TryFrom<&[u8]> for VitalProductData {
    type Error = CapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((word, vpd_data)),
            ..
        } = P2(slice).try_into().map_err(|_| CapabilityDataError {
            name: "Vital Product Data",
            size: 6,
        })?;
        let Lsb((vpd_address, transfer_completed)) = P2::<u16, 15, 1>(word).into();
        Ok(Self {
            vpd_address,
            transfer_completed,
            vpd_data,
        })
    }
}
