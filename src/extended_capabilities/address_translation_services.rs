/*!
# Address Translation Services (ATS)

ATS extends the PCIe protocol to support an address translation agent (TA) that
translates DMA addresses to cached addresses in the device.

## Struct diagram
[AddressTranslationServices]
- [AtsCapability]
- [AtsControl]

## Examples

```rust
# use pcics::extended_capabilities::address_translation_services::*;
let data = [
    0x0f, 0x00, 0x00, 0x00, 0x24, 0x00, 0x02, 0x00,
];
let result = data[4..].try_into().unwrap();
let sample = AddressTranslationServices {
    ats_capability: AtsCapability {
        invalidate_queue_depth: 4,
        page_aligned_request: true,
        global_invalidate_supported: false,
    },
    ats_control: AtsControl {
        smallest_translation_unit: 2,
        enable: false,
    }
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P3, P4};

use super::ExtendedCapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressTranslationServices {
    /// ATS Capability
    pub ats_capability: AtsCapability,
    /// ATS Control
    pub ats_control: AtsControl,
}
impl TryFrom<&[u8]> for AddressTranslationServices {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((ats_capability, ats_control)),
            ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Address Translation Services",
                size: 4,
            })?;
        Ok(Self {
            ats_capability: From::<u16>::from(ats_capability),
            ats_control: From::<u16>::from(ats_control),
        })
    }
}

/// ATS Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtsCapability {
    /// Invalidate Queue Depth
    pub invalidate_queue_depth: u8,
    /// Page Aligned Request
    pub page_aligned_request: bool,
    /// Global Invalidate Supported
    pub global_invalidate_supported: bool,
}

impl From<u16> for AtsCapability {
    fn from(word: u16) -> Self {
        let Lsb((invalidate_queue_depth, page_aligned_request, global_invalidate_supported, ())) =
            P4::<_, 5, 1, 1, 9>(word).into();
        Self {
            invalidate_queue_depth,
            page_aligned_request,
            global_invalidate_supported,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtsControl {
    /// Smallest Translation Unit (STU)
    pub smallest_translation_unit: u8,
    /// Enable (E)
    pub enable: bool,
}

impl From<u16> for AtsControl {
    fn from(word: u16) -> Self {
        let Lsb((smallest_translation_unit, (), enable)) = P3::<_, 5, 10, 1>(word).into();
        Self {
            smallest_translation_unit,
            enable,
        }
    }
}
