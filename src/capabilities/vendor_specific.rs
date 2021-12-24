//! Vendor Specific
//!
//! Allow device vendors to use the capability mechanism for vendor specific information. The
//! layout of the information is vendor specific, except that the byte immediately following the
//! “Next” pointer in the capability structure is defined to be a length field. 
//! An example vendor specific usage is a device that is configured in the final
//! manufacturing steps as either a 32-bit or 64-bit PCI agent and the Vendor Specific capability
//! structure tells the device driver which features the device supports. 

use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// Only vendor-specific data length. Without Cap ID, Next Ptr and length itself
#[derive(Debug, PartialEq, Eq)]
pub struct VendorSpecific<'a>(pub &'a [u8]);
impl<'a> VendorSpecific<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }
    pub fn vendor_capability(&self, vendor_id: u16, device_id: u16) -> VendorCapabilty<'a> {
        let data = self.0;
        let offset = &mut 0;
        match (vendor_id, device_id) {
            (0x1af4, 0x1000..=0x107f) => {
                if let Ok(virtio) = data.read_with::<Virtio>(offset, LE) {
                    VendorCapabilty::Virtio(virtio)
                } else {
                    VendorCapabilty::Unspecified(data)
                }
            },
            _ => VendorCapabilty::Unspecified(data),
        }
    }
}
impl<'a> TryRead<'a, Endian> for VendorSpecific<'a> {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let length = bytes.read_with::<u8>(offset, endian)? as usize - 1;
        let vs = VendorSpecific(bytes.read_with(offset, Bytes::Len(length))?);
        Ok((vs, *offset))
    }
}

/// Known vendor-specific capabilities
#[derive(Debug, PartialEq, Eq)]
pub enum VendorCapabilty<'a> {
    Unspecified(&'a [u8]),
    Virtio(Virtio),
}
impl<'a> VendorCapabilty<'a> {
    /// Vendor Specific capability depends on Vendor ID and Device ID
    pub fn new(data: &'a [u8], vendor_id: u16, device_id: u16) -> Self {
        let offset = &mut 0;
        match (vendor_id, device_id) {
            (0x1af4, 0x1000..=0x107f) => {
                if let Ok(virtio) = data.read_with::<Virtio>(offset, LE) {
                    Self::Virtio(virtio)
                } else {
                    Self::Unspecified(data)
                }
            },
            _ => Self::Unspecified(data),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Virtio {
    CommonCfg { bar: u8, offset: u32, size: u32 },
    Notify {
        bar: u8,
        offset: u32,
        size: u32,
        multiplier: Option<u32>,
    },
    Isr { bar: u8, offset: u32, size: u32 },
    DeviceCfg { bar: u8, offset: u32, size: u32 },
    Unknown { bar: u8, offset: u32, size: u32 },
}
impl<'a> TryRead<'a, Endian> for Virtio {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset_ = &mut 0;
        let type_ = bytes.read_with::<u8>(offset_, endian)?;
        let bar = bytes.read_with::<u8>(offset_, endian)?;
        *offset_ += 3; // aligned by u32 ?
        let offset = bytes.read_with::<u32>(offset_, endian)?;
        let size = bytes.read_with::<u32>(offset_, endian)?;
        let virtio = match type_ {
            1 => Self::CommonCfg { bar, offset, size },
            2 => {
                let multiplier = bytes.read_with::<u32>(offset_, endian).ok();
                Self::Notify { bar, offset, size, multiplier }
            },
            3 => Self::Isr { bar, offset, size },
            4 => Self::DeviceCfg { bar, offset, size },
            _ => Self::Unknown { bar, offset, size },
        };
        Ok((virtio, *offset_))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn vendor_capability() {
        // Capabilities: [b4] Vendor Specific Information: VirtIO: Notify
        //         BAR=4 offset=00003000 size=00001000 multiplier=00000004
        let data = [
            0x09, // Vendor Specific ID = 0x09
            0xa4, // next capabilities pointer
            0x14, // length = 20
            0x02, // Virtio type
            0x04, // BAR
            0x00, 0x00, 0x00,       // skipped
            0x00, 0x30, 0x00, 0x00, // offset
            0x00, 0x10, 0x00, 0x00, // size
            0x04, 0x00, 0x00, 0x00  // multiplier
        ];
        let vc = VendorCapabilty::new(&data[3..], 0x1af4, 0x1045);
        let sample = VendorCapabilty::Virtio(
            Virtio::Notify { bar: 4, offset: 0x00003000, size: 0x00001000, multiplier: Some(0x00000004) }
        );
        assert_eq!(sample, vc);
    }
}
