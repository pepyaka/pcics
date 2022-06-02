/*!
# Vendor-Specific Extended Capability

The Vendor-Specific Extended Capability (VSEC) is an optional Extended Capability that is
permitted to be implemented by any PCI Express Function or RCRB.

## Struct diagram
[VendorSpecificExtendedCapability]
- [VsecHeader]
- [VsecRegisters]

## Examples

> ```text
> Vendor Specific Information: ID=0002 Rev=0 Len=00c <?>
> ```

```rust
# use pcics::extended_capabilities::vendor_specific_extended_capability::*;
let data = [
    0x0b, 0x00, 0x41, 0x14, 0x02, 0x00, 0xc0, 0x00, 0x07, 0x30, 0x00, 0x00,
];
let result = data[4..].try_into().unwrap();
let sample = VendorSpecificExtendedCapability {
    header: VsecHeader {
        vsec_id: 0x0002,
        vsec_rev: 0,
        vsec_length: 0x00c,
    },
    registers: VsecRegisters::Valid([0x07, 0x30, 0x00, 0x00].as_slice()),
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::LeBytesTryInto, Seq, P3};

use super::ExtendedCapabilityDataError;

/// Vendor-Specific Extended Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorSpecificExtendedCapability<'a> {
    pub header: VsecHeader,
    pub registers: VsecRegisters<'a>,
}
impl<'a> TryFrom<&'a [u8]> for VendorSpecificExtendedCapability<'a> {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq { head: header, tail } =
            slice
                .le_bytes_try_into()
                .map_err(|_| ExtendedCapabilityDataError {
                    name: "Vendor-Specific Extended Capability",
                    size: 4,
                })?;
        let Lsb((vsec_id, vsec_rev, vsec_length)) = P3::<u32, 16, 4, 12>(header).into();
        let header = VsecHeader {
            vsec_id,
            vsec_rev,
            vsec_length,
        };
        let registers = if let Some(len) = vsec_length.checked_sub(8) {
            if let Some(slice) = tail.get(..len.into()) {
                VsecRegisters::Valid(slice)
            } else {
                VsecRegisters::Incomplete(tail)
            }
        } else {
            VsecRegisters::InvalidLength(vsec_length)
        };
        Ok(Self { header, registers })
    }
}

/// Vendor-Specific Header
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VsecHeader {
    /// Vendor-defined ID number that indicates the nature and format of the VSEC structure.
    pub vsec_id: u16,
    /// Vendor-defined version number that indicates the version of the VSEC structure.
    pub vsec_rev: u8,
    /// Indicates the number of bytes in the entire VSEC structure, including the PCI Express
    /// Extended Capability header, the vendorspecific header, and the vendor-specific registers.
    pub vsec_length: u16,
}

/// Vendor-Specific Registers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VsecRegisters<'a> {
    Valid(&'a [u8]),
    /// VSEC Length too short (should be >= 8)
    InvalidLength(u16),
    /// Available data shorter than length in header
    Incomplete(&'a [u8]),
}
