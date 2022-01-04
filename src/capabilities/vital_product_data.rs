//! Vital Product Data
//!
//! Vital Product Data (VPD) is the information that uniquely defines items such as the hardware,
//! software, and microcode elements of a system.

use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// Vital Product Data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VitalProductData {
    /// DWORD-aligned byte address of the VPD to be accessed
    pub vpd_address: u16,
    /// Indicate when the transfer of data between the VPD Data register and the storage component
    /// is completed
    pub transfer_completed: bool,
    /// VPD Data
    pub vpd_data: u32,
}
impl<'a> TryRead<'a, Endian> for VitalProductData {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let word = bytes.read_with::<u16>(offset, endian)?;
        let vpd = VitalProductData {
            vpd_address: word & !0x8000,
            transfer_completed: (word & 0x8000) != 0,
            vpd_data: bytes.read_with::<u32>(offset, endian)?,
        };
        Ok((vpd, *offset))
    }
}
