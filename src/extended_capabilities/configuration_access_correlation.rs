/*!
# Configuration Access Correlation (CAC)

The PCI Express Configuration Access Correlation (CAC) Capability is an optional Extended
Capability that must be implemented by any PCI Express device Function that provides a Trusted
Configuration Space.

## Struct diagram
<pre>
<a href="struct.ConfigurationAccessCorrelation.html">ConfigurationAccessCorrelation</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::configuration_access_correlation::*;
let data = [
    /* 00h */ 0x0c, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x00, 0x11, 0x22, 0x33, // Device Correlation
];

let result: ConfigurationAccessCorrelation = data.as_slice().try_into().unwrap();

let sample = ConfigurationAccessCorrelation {
    device_correlation: 0x33221100,
};

assert_eq!(sample, result);
```
*/

use heterob::{endianness::Le, Seq, P2};

use super::{ExtendedCapabilityDataError, ExtendedCapabilityHeaderPlaceholder};

/// Configuration Access Correlation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationAccessCorrelation {
    /// Device Correlation
    ///
    /// Contains a value written from within the TSE via the Configuration
    /// Access Correlation Trusted Capability.
    pub device_correlation: u32,
}

impl ConfigurationAccessCorrelation {
    /// Size in bytes (with Extended Capability Header)
    pub const SIZE: usize = 0x8;
}

impl From<[u8; Self::SIZE]> for ConfigurationAccessCorrelation {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((ExtendedCapabilityHeaderPlaceholder, device_correlation)) = P2(bytes).into();
        Self { device_correlation }
    }
}

impl TryFrom<&[u8]> for ConfigurationAccessCorrelation {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq { head, .. } = slice.try_into().map_err(|_| ExtendedCapabilityDataError {
            name: "Configuration Access Correlation",
            size: Self::SIZE,
        })?;
        Ok(From::<[u8; Self::SIZE]>::from(head))
    }
}
