//! Debug port

use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// Debug port
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugPort {
    /// Indicates the byte offset (up to 4K) within the BAR indicated by BAR#
    pub offset: u16,
    /// Indicates which one of the possible 6 Base Address Register offsets contains the Debug Port
    /// registers
    pub bar_number: u8,
}
impl<'a> TryRead<'a, Endian> for DebugPort {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let word = bytes.read_with::<u16>(offset, endian)?;
        let dp = DebugPort {
            offset: word & 0x1fff,
            bar_number: (word >> 13) as u8,
        };
        Ok((dp, *offset))
    }
}
