/*!
# Process Address Space ID (PASID)

The presence of a PASID Extended Capability indicates that the Endpoint supports sending and
receiving TLPs containing a PASID TLP Prefix.

## Struct diagram
[ProcessAddressSpaceId]
- [PacidCapability]
- [PacidControl]

## Examples

```rust
# use pcics::extended_capabilities::process_address_space_id::*;
let data = [
    0x1b, 0x00, 0x00, 0x00, 0x02, 0x04, 0x03, 0x00,
];
let result = data[4..].try_into().unwrap();
let sample = ProcessAddressSpaceId {
    pacid_capability: PacidCapability {
        execute_permission_supported: true,
        privileged_mode_supported: false,
        max_pasid_width: 0x04,
    },
    pacid_control: PacidControl {
        pasid_enable: true,
        execute_permission_enable: true,
        privileged_mode_enable: false,
    },
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P4, P6};

use super::ExtendedCapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessAddressSpaceId {
    pub pacid_capability: PacidCapability,
    pub pacid_control: PacidControl,
}
impl TryFrom<&[u8]> for ProcessAddressSpaceId {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((pacid_capability, pacid_control)),
            ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Process Address Space ID",
                size: 4,
            })?;
        Ok(Self {
            pacid_capability: From::<u16>::from(pacid_capability),
            pacid_control: From::<u16>::from(pacid_control),
        })
    }
}

/// PASID Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacidCapability {
    /// Execute Permission Supported
    pub execute_permission_supported: bool,
    /// Privileged Mode Supported
    pub privileged_mode_supported: bool,
    /// Max PASID Width
    pub max_pasid_width: u8,
}

impl From<u16> for PacidCapability {
    fn from(word: u16) -> Self {
        let Lsb((
            (),
            execute_permission_supported,
            privileged_mode_supported,
            (),
            max_pasid_width,
            (),
        )) = P6::<_, 1, 1, 1, 5, 5, 3>(word).into();
        Self {
            execute_permission_supported,
            privileged_mode_supported,
            max_pasid_width,
        }
    }
}

/// PASID Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacidControl {
    /// PASID Enable
    pub pasid_enable: bool,
    /// Execute Permission Enable
    pub execute_permission_enable: bool,
    /// Privileged Mode Enable
    pub privileged_mode_enable: bool,
}

impl From<u16> for PacidControl {
    fn from(word: u16) -> Self {
        let Lsb((pasid_enable, execute_permission_enable, privileged_mode_enable, ())) =
            P4::<_, 1, 1, 1, 13>(word).into();
        Self {
            pasid_enable,
            execute_permission_enable,
            privileged_mode_enable,
        }
    }
}
