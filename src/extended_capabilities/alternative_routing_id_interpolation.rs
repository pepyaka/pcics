/*!
# Alternative Routing-ID Interpretation (ARI)

Routing IDs, Requester IDs, and Completer IDs are 16-bit identifiers traditionally composed of
20 three fields: an 8-bit Bus Number, a 5-bit Device Number, and a 3-bit Function Number. With
ARI, the 16-bit field is interpreted as two fields instead of three: an 8-bit Bus Number and an
8-bit Function Number – the Device Number field is eliminated. This new interpretation enables
an ARI Device to support up to 256 Functions [0..255] instead of 8 Functions [0..7].

## Struct diagram
[AlternativeRoutingIdInterpretation]
- [AriCapability]
- [AriControl]

## Examples

> ```text
> ARICap: MFVC- ACS-, Next Function: 1
> ARICtl: MFVC- ACS-, Function Group: 0
> ```

```rust
# use pcics::extended_capabilities::alternative_routing_id_interpolation::*;
let data = [
    0x0e, 0x00, 0x01, 0x16, 0x00, 0x01, 0x00, 0x00,
];
let result = data[4..].try_into().unwrap();
let sample = AlternativeRoutingIdInterpretation {
    ari_capability: AriCapability {
        mfvc_function_groups_capability: false,
        acs_function_groups_capability: false,
        next_function_number: 1,
    },
    ari_control: AriControl {
        mfvc_function_groups_enable: false,
        acs_function_groups_enable: false,
        function_group: 0,
    }
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P4, P5};

use super::ExtendedCapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeRoutingIdInterpretation {
    pub ari_capability: AriCapability,
    pub ari_control: AriControl,
}
impl TryFrom<&[u8]> for AlternativeRoutingIdInterpretation {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((ari_capability, ari_control)),
            ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Alternative Routing-ID Interpretation",
                size: 4,
            })?;
        Ok(Self {
            ari_capability: From::<u16>::from(ari_capability),
            ari_control: From::<u16>::from(ari_control),
        })
    }
}

/// ARI Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AriCapability {
    /// MFVC Function Groups Capability (M)
    pub mfvc_function_groups_capability: bool,
    /// ACS Function Groups Capability (A)
    pub acs_function_groups_capability: bool,
    /// Next Function Number
    pub next_function_number: u8,
}

impl From<u16> for AriCapability {
    fn from(word: u16) -> Self {
        let Lsb((
            mfvc_function_groups_capability,
            acs_function_groups_capability,
            (),
            next_function_number,
        )) = P4::<_, 1, 1, 6, 8>(word).into();
        Self {
            mfvc_function_groups_capability,
            acs_function_groups_capability,
            next_function_number,
        }
    }
}

/// ARI Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AriControl {
    /// MFVC Function Groups Enable (M)
    pub mfvc_function_groups_enable: bool,
    /// ACS Function Groups Enable (A)
    pub acs_function_groups_enable: bool,
    /// Function Group
    pub function_group: u8,
}

impl From<u16> for AriControl {
    fn from(word: u16) -> Self {
        let Lsb((mfvc_function_groups_enable, acs_function_groups_enable, (), function_group, ())) =
            P5::<_, 1, 1, 2, 3, 9>(word).into();
        Self {
            mfvc_function_groups_enable,
            acs_function_groups_enable,
            function_group,
        }
    }
}
