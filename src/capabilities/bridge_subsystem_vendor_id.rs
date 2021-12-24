//! PCI Bridge Subsystem Vendor ID

use byte::{
    ctx::*,
    self,
    TryRead,
    BytesExt,
};

/// PCI Bridge Subsystem Vendor ID
/// ```
/// # use pcics::capabilities::BridgeSubsystemVendorId;
/// # use pretty_assertions::assert_eq;
/// # use byte::{ ctx::*, self, TryRead, BytesExt, };
/// let data = [0x00,0x00,0x11,0x22,0x33,0x44];
///
/// let result = data.read_with::<BridgeSubsystemVendorId>(&mut 0, LE).unwrap();
/// 
/// let sample = BridgeSubsystemVendorId {
///     reserved: 0x0000,
///     subsystem_vendor_id: 0x2211,
///     subsystem_id: 0x4433,
/// };
/// assert_eq!(sample, result);
/// ```


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubsystemVendorId {
    pub reserved: u16,
    /// PCI Bridge Subsystem Vendor ID
    pub subsystem_vendor_id: u16,
    /// PCI-Bridge subsystem device id register 
    pub subsystem_id: u16,
}

impl<'a> TryRead<'a, Endian> for BridgeSubsystemVendorId {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let bsv = Self {
            reserved: bytes.read_with::<u16>(offset, endian)?,
            subsystem_vendor_id: bytes.read_with::<u16>(offset, endian)?,
            subsystem_id: bytes.read_with::<u16>(offset, endian)?,
        };
        Ok((bsv, *offset))
    }
}
