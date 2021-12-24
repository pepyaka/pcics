//! Vendor-Specific Extended Capability
//!
//! The Vendor-Specific Extended Capability (VSEC) is an optional Extended Capability that is
//! permitted to be implemented by any PCI Express Function or RCRB.

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// Vendor-Specific Extended Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorSpecificExtendedCapability<'a> {
    pub header: VsecHeader,
    /// Vendor-Specific Registers
    pub registers: &'a [u8],
}
impl<'a> TryRead<'a, Endian> for VendorSpecificExtendedCapability<'a> {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let header: VsecHeader = bytes.read_with::<u32>(offset, endian)?.into();
        let len = (header.vsec_length - 8).into();
        let vsec = VendorSpecificExtendedCapability {
            header,
            registers: bytes.read_with::<&[u8]>(offset, Bytes::Len(len))?,
        };
        Ok((vsec, *offset))
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct VsecHeaderProto {
    vsec_id: u16,
    vsec_rev: B4,
    vsec_length: B12,
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
impl From<VsecHeaderProto> for VsecHeader {
    fn from(proto: VsecHeaderProto) -> Self {
        Self {
            vsec_id: proto.vsec_id(),
            vsec_rev: proto.vsec_rev(),
            vsec_length: proto.vsec_length(),
        }
    }
}
impl From<u32> for VsecHeader {
    fn from(dword: u32) -> Self { VsecHeaderProto::from(dword).into() }
}
