/*!
# Root Complex Register Block (RCRB) Header

The PCI Express RCRB Header Capability is an optional Extended Capability that may be
implemented in an RCRB to provide a Vendor ID and Device ID for the RCRB and to permit the
management of parameters that affect the behavior of Root Complex functionality associated with
the RCRB.

## Struct diagram
<pre>
<a href="struct.RootComplexRegisterBlockHeader.html">RootComplexRegisterBlockHeader</a>
├─ <a href="struct.RcrbCapabilities.html">RcrbCapabilities</a>
└─ <a href="struct.RcrbControl.html">RcrbControl</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::root_complex_register_block_header::*;
let data = [
    /* 00h */ 0x0a, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x86, 0x80,             // Vendor ID
              0x34, 0x3e,             // Device ID
    /* 08h */ 0b01, 0x00, 0x00, 0x00, // RCRB Capabilities
    /* 10h */ 0b01, 0x00, 0x00, 0x00, // RCRB Control
    /* 14h */ 0x00, 0x00, 0x00, 0x00, // RsvdZ
];

let result: RootComplexRegisterBlockHeader = data.as_slice().try_into().unwrap();

let sample = RootComplexRegisterBlockHeader {
    vendor_id: 0x8086,
    device_id: 0x3e34,
    rcrb_capabilities: RcrbCapabilities {
        crs_software_visibility: true,
    },
    rcrb_control: RcrbControl {
        crs_software_visibility_enable: true,
    },
};

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P2, P6};

use super::{ExtendedCapabilityDataError, ExtendedCapabilityHeaderPlaceholder};

/// RCRB Header
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootComplexRegisterBlockHeader {
    /// Vendor ID – PCI-SIG assigned.
    ///
    /// Analogous to the equivalent field in PCI-compatible Configuration Space.
    /// This field provides a means to associate an RCRB with a particular vendor.
    pub vendor_id: u16,
    /// Device ID - Vendor assigned.
    ///
    /// Analogous to the equivalent field in PCI-compatible Configuration Space.
    /// This field provides a means for a vendor to classify a particular RCRB.
    pub device_id: u16,
    pub rcrb_capabilities: RcrbCapabilities,
    pub rcrb_control: RcrbControl,
}

impl RootComplexRegisterBlockHeader {
    /// Size in bytes (with Extended Capability Header)
    pub const SIZE: usize = 0x14;
}

impl From<[u8; Self::SIZE]> for RootComplexRegisterBlockHeader {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((
            ExtendedCapabilityHeaderPlaceholder,
            vendor_id,
            device_id,
            rcrb_capabilities,
            rcrb_control,
            rsvdz,
        )) = P6(bytes).into();
        let _: u32 = rsvdz;
        Self {
            vendor_id,
            device_id,
            rcrb_capabilities: From::<u32>::from(rcrb_capabilities),
            rcrb_control: From::<u32>::from(rcrb_control),
        }
    }
}

impl TryFrom<&[u8]> for RootComplexRegisterBlockHeader {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq { head, .. } = slice.try_into().map_err(|_| ExtendedCapabilityDataError {
            name: "RCRB Header",
            size: Self::SIZE,
        })?;
        Ok(From::<[u8; Self::SIZE]>::from(head))
    }
}

/// RCRB Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcrbCapabilities {
    /// CRS Software Visibility
    ///
    /// Indicates that the Root Complex is capable of returning Configuration
    /// Request Retry Status (CRS) Completion Status to software for all Root Ports and
    /// integrated devices associated with this RCRB
    pub crs_software_visibility: bool,
}

impl From<u32> for RcrbCapabilities {
    fn from(dword: u32) -> Self {
        let Lsb((crs_software_visibility, ())) = P2::<_, 1, 31>(dword).into();
        Self {
            crs_software_visibility,
        }
    }
}

/// RCRB Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcrbControl {
    /// CRS Software Visibility Enable
    ///
    /// Enables the Root Complex to return Configuration Request Retry Status
    /// (CRS) Completion Status to software for all Root Ports and integrated devices
    /// associated with this RCRB
    pub crs_software_visibility_enable: bool,
}

impl From<u32> for RcrbControl {
    fn from(dword: u32) -> Self {
        let Lsb((crs_software_visibility_enable, ())) = P2::<_, 1, 31>(dword).into();
        Self {
            crs_software_visibility_enable,
        }
    }
}
