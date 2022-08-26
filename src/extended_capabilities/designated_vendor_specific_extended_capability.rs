/*!
# Designated Vendor-Specific Extended Capability (DVSEC)

The PCI Express Designated Vendor-Specific Extended Capability (DVSEC) is an optional
Extended Capability that is permitted to be implemented by any PCI Express Function or RCRB.
This allows PCI Express component vendors to use the Extended Capability mechanism to expose
vendor-specific registers that can be present in components by a variety of vendors.

## Struct diagram
<pre>
<a href="struct.DesignatedVendorSpecificExtendedCapability.html">DesignatedVendorSpecificExtendedCapability</a>
└─ <a href="enum.DvsecType.html">DvsecType</a>
   └─ <a href="compute_express_link/enum.ComputeExpressLink.html">ComputeExpressLink</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::designated_vendor_specific_extended_capability::*;
let data = [
    /* 00h */ 0x23, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x86, 0x80, 0x02, 0x01, // Designated Vendor-Specific Header 1
    /* 08h */ 0x23, 0x00,             // Designated Vendor-Specific Header 2
              0x00, 0x11,             // DVSEC vendor-specific registers
    /* 0Ch */ 0x22, 0x33, 0x44, 0x55,
];

let result: DesignatedVendorSpecificExtendedCapability = data.as_slice().try_into().unwrap();

let sample = DesignatedVendorSpecificExtendedCapability {
    dvsec_vendor_id: 0x8086,
    dvsec_revision: 2,
    dvsec_length: 0x10,
    dvsec_id: 0x23,
    dvsec_type: DvsecType::Unspecified(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]),
};

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P3};
use snafu::prelude::*;

use super::ExtendedCapabilityHeaderPlaceholder;

/// DVSEC Errors
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum DesignatedVendorSpecificExtendedCapabilityError {
    #[snafu(display("mandatory fields are unreadable"))]
    Mandatory,
    #[snafu(display(
        "Vendor-specific registers (VID: {:04x}, rev: {:02x}, ID: {:04x}) are unreadable. Length expected: {}, real: {}",
            dvsec_vendor_id,
            dvsec_revision,
            dvsec_id,
            dvsec_length,
            real,
    ))]
    VendorSpecificRegisters {
        dvsec_vendor_id: u16,
        dvsec_revision: u8,
        dvsec_length: u16,
        dvsec_id: u16,
        real: usize,
    },
    #[snafu(display("Compute Express Link error: {source}"))]
    ComputeExpressLink {
        source: compute_express_link::ComputeExpressLinkError,
    },
}

/// [DesignatedVendorSpecificExtendedCapabilityError] alias
pub type DvsecError = DesignatedVendorSpecificExtendedCapabilityError;

/// Designated Vendor-Specific Extended Capability (DVSEC)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedVendorSpecificExtendedCapability<'a> {
    /// Vendor ID associated with the vendor that defined the contents of this capability
    pub dvsec_vendor_id: u16,
    /// Vendor-defined version number that indicates the version of the DVSEC structure
    pub dvsec_revision: u8,
    /// Indicates the number of bytes in the entire DVSEC structure, including
    /// the PCI Express Extended Capability header, the DVSEC Header 1, DVSEC Header 2,
    /// and DVSEC vendor-specific registers
    pub dvsec_length: u16,
    /// Vendor-defined ID that indicates the nature and format of the DVSEC structure
    pub dvsec_id: u16,
    pub dvsec_type: DvsecType<'a>,
}

/// Contents of capability depends on Vendor ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DvsecType<'a> {
    Unspecified(&'a [u8]),
    ComputeExpressLink(ComputeExpressLink),
}

/// [DesignatedVendorSpecificExtendedCapability] alias
pub type Dvsec<'a> = DesignatedVendorSpecificExtendedCapability<'a>;

impl<'a> Dvsec<'a> {
    /// Min size in bytes (with Extended Capability Header)
    pub const MIN_SIZE: usize = 0x0a;
    /// Max size in bytes (with Extended Capability Header)
    pub const MAX_SIZE: usize = 0xfff - 0x100;
}

impl<'a> TryFrom<&'a [u8]> for Dvsec<'a> {
    type Error = DvsecError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((ExtendedCapabilityHeaderPlaceholder, dvsec_header_1, dvsec_id)),
            ..
        } = P3(slice).try_into().map_err(|_| DvsecError::Mandatory)?;
        let Lsb((dvsec_vendor_id, dvsec_revision, dvsec_length)) =
            P3::<u32, 16, 4, 12>(dvsec_header_1).into();
        let dvsec_vendor_specific_registers = slice
            .get(Self::MIN_SIZE..dvsec_length as usize)
            .ok_or(DvsecError::VendorSpecificRegisters {
                dvsec_vendor_id,
                dvsec_revision,
                dvsec_length,
                dvsec_id,
                real: slice.len(),
            })?;
        let dvsec_type = match dvsec_vendor_id {
            0x1e98 => ComputeExpressLink::try_new(dvsec_vendor_specific_registers, dvsec_id)
                .map(DvsecType::ComputeExpressLink)
                .context(ComputeExpressLinkSnafu)?,
            _ => DvsecType::Unspecified(dvsec_vendor_specific_registers),
        };
        Ok(Dvsec {
            dvsec_vendor_id,
            dvsec_revision,
            dvsec_length,
            dvsec_id,
            dvsec_type,
        })
    }
}

pub mod compute_express_link;
pub use compute_express_link::ComputeExpressLink;
