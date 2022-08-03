/*!
# LN Requester (LNR)

An LN Requester (LNR) is a client subsystem in an Endpoint that sends LN Read/Write
Requests and receives LN Messages.

## Struct diagram
<pre>
<a href="struct.LnRequester.html">LnRequester</a>
├─ <a href="struct.LnrCapability.html">LnrCapability</a>
└─ <a href="struct.LnrControl.html">LnrControl</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::ln_requester::*;
let data = [
    /* 00h */ 0x1C, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x01, 0x13, // LNR Capability
              0x01, 0x13, // LNR Control
];

let result: LnRequester = data.as_slice().try_into().unwrap();

let sample = LnRequester {
    lnr_capability: LnrCapability {
        lnr_64_supported: true,
        lnr_128_supported: false,
        lnr_registration_max: 0x13,
    },
    lnr_control: LnrControl {
        lnr_enable: true,
        lnr_cls: false,
        lnr_registration_limit: 0x13,
    },
};

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P3, P5};

use super::{ExtendedCapabilityDataError, ExtendedCapabilityHeaderPlaceholder};

/// LN Requester
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LnRequester {
    pub lnr_capability: LnrCapability,
    pub lnr_control: LnrControl,
}

impl LnRequester {
    /// Size in bytes (with Extended Capability Header)
    pub const SIZE: usize = 0x8;
}

impl From<[u8; Self::SIZE]> for LnRequester {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((ExtendedCapabilityHeaderPlaceholder, lnr_capability, lnr_control)) =
            P3(bytes).into();
        Self {
            lnr_capability: From::<u16>::from(lnr_capability),
            lnr_control: From::<u16>::from(lnr_control),
        }
    }
}

impl TryFrom<&[u8]> for LnRequester {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq { head, .. } = slice.try_into().map_err(|_| ExtendedCapabilityDataError {
            name: "LN Requester",
            size: Self::SIZE,
        })?;
        Ok(From::<[u8; Self::SIZE]>::from(head))
    }
}

/// LNR Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LnrCapability {
    /// Endpoint supports LN protocol for 64-byte cachelines as a Requester
    pub lnr_64_supported: bool,
    /// Endpoint supports LN protocol for 128-byte cachelines as a Requester
    pub lnr_128_supported: bool,
    /// Encoded as a power of 2, indicates the maximum number of cachelines
    /// that this LN Requester is capable of registering concurrently
    pub lnr_registration_max: u8,
}

impl From<u16> for LnrCapability {
    fn from(word: u16) -> Self {
        let Lsb((lnr_64_supported, lnr_128_supported, (), lnr_registration_max, ())) =
            P5::<_, 1, 1, 6, 5, 3>(word).into();
        Self {
            lnr_64_supported,
            lnr_128_supported,
            lnr_registration_max,
        }
    }
}

/// LNR Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LnrControl {
    /// Endpoint is enabled to operate as an LN Requester
    pub lnr_enable: bool,
    /// Indicates the cacheline size used with LN protocol by this Requester.
    pub lnr_cls: bool,
    /// Encoded as a power of 2, imposes a limit on the number of cachelines
    /// that this LN Requester is permitted to register concurrently
    pub lnr_registration_limit: u8,
}

impl From<u16> for LnrControl {
    fn from(word: u16) -> Self {
        let Lsb((lnr_enable, lnr_cls, (), lnr_registration_limit, ())) =
            P5::<_, 1, 1, 6, 5, 3>(word).into();
        Self {
            lnr_enable,
            lnr_cls,
            lnr_registration_limit,
        }
    }
}
