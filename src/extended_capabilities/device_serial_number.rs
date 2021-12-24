//! Device Serial Number Capability
//!
//! The PCI Express Device Serial Number Capability is an optional Extended Capability that may be
//! implemented by any PCI Express device Function. The Device Serial Number is a read-only 64-bit
//! value that is unique for a given PCI Express device.

use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

/// The Serial Number register is a 64-bit field that contains the IEEE defined 64-bit extended
/// unique identifier (EUI-64™).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSerialNumber {
    /// PCI Express Device Serial Number (1st DW)
    pub lower_dword: u32,
    /// PCI Express Device Serial Number (2nd DW)
    pub upper_dword: u32,
}
impl<'a> TryRead<'a, Endian> for DeviceSerialNumber {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let dsn = DeviceSerialNumber {
            lower_dword: bytes.read_with::<u32>(offset, endian)?,
            upper_dword: bytes.read_with::<u32>(offset, endian)?,
        };
        Ok((dsn, *offset))
    }
}
