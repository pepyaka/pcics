/*!
# Vendor Specific

Allow device vendors to use the capability mechanism for vendor specific information. The
layout of the information is vendor specific, except that the byte immediately following the
“Next” pointer in the capability structure is defined to be a length field.
An example vendor specific usage is a device that is configured in the final
manufacturing steps as either a 32-bit or 64-bit PCI agent and the Vendor Specific capability
structure tells the device driver which features the device supports.

## Struct diagram
<pre>
<a href="enum.VendorSpecific.html">VendorSpecific</a>
└─ <a href="enum.Virtio.html">Virtio</a>
</pre>

## Examples

> ```plaintext
> VirtIO: CommonCfg
>     BAR=4 offset=00000000 size=00001000
> ```

```rust
# use pcics::{Header, capabilities::vendor_specific::*};
let mut header: Header = [0; 0x40].try_into().unwrap();
header.vendor_id = 0x1af4;
header.device_id = 0x1048;

let data = [
    0x09, 0x7c, // Header
    0x10, // Length
    0x01, // Virtio type
    0x04, // BAR
    0x00, 0x00, 0x00,       // Reserved
    0x00, 0x00, 0x00, 0x00, // Offset
    0x00, 0x10, 0x00, 0x00, // Size
];
let result = VendorSpecific::try_new(&data[2..], &header).unwrap();
let sample = VendorSpecific::Virtio(Virtio::CommonCfg {
    bar: 4,
    offset: 0x00,
    size: 0x1000,
});
assert_eq!(sample, result);
*/

use snafu::prelude::*;
use heterob::{endianness::Le, P5};

use crate::header::Header;

#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum VendorSpecificError {
    #[snafu(display("length byte is unreadable"))]
    LengthUnreadable,
    #[snafu(display("length should be > 2, not {val}"))]
    Length { val: u8 },
    #[snafu(display("unable to get {size} bytes data"))]
    Data { size: usize },
    #[snafu(display("Virtio size shold be > 12"))]
    Virtio,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VendorSpecific<'a> {
    /// Only vendor-specific data. Without Cap ID, Next Ptr and length itself
    Unspecified(&'a [u8]),
    /// Known vendor-specific capabilities
    Virtio(Virtio),
}

impl<'a> VendorSpecific<'a> {
    pub fn try_new(slice: &'a [u8], header: &'a Header) -> Result<Self, VendorSpecificError> {
        let size: usize = slice
            .first()
            .ok_or(VendorSpecificError::LengthUnreadable)
            // slice already without cap_id and next_ptr
            .and_then(|l| {
                l.checked_sub(2)
                    .ok_or(VendorSpecificError::Length { val: *l })
            })?
            .into();
        let slice = slice
            .get(1..size)
            .ok_or(VendorSpecificError::Data { size })?;
        let result = match (header.vendor_id, header.device_id) {
            (0x1af4, 0x1000..=0x107f) => slice.try_into().map(Self::Virtio)?,
            _ => Self::Unspecified(slice),
        };
        Ok(result)
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
impl Virtio {
    /// Identifies the structure: `[u8]`
    /// `[u8]`: Where to find it: `[u8]`
    /// `[[u8;3]]`: Pad to full dword: `[[u8;3]]`
    /// Offset within bar: `[u32]`
    /// Length of the structure, in bytes: `[u32]`
    pub const SIZE: usize = 1 + 1 + 3 + 4 + 4; // 13 bytes
}
impl<'a> TryFrom<&'a [u8]> for Virtio {
    type Error = VendorSpecificError;
    fn try_from(slice: &'a [u8]) -> Result<Virtio, Self::Error> {
        let bytes = slice.get(..Virtio::SIZE)
            .and_then(|slice| <[u8;Virtio::SIZE]>::try_from(slice).ok())
            .ok_or(VendorSpecificError::Virtio)?;
        let Le((cfg_type, bar, padding, offset, size)) = P5(bytes).into();
        let _: [u8;3] = padding;
        let result = match cfg_type {
            1u8 => Self::CommonCfg { bar, offset, size },
            2 => {
                let multiplier = slice.get(Virtio::SIZE .. Virtio::SIZE + 4)
                    .and_then(|slice| <[u8;4]>::try_from(slice).ok())
                    .map(u32::from_le_bytes);
                Self::Notify { bar, offset, size, multiplier }
            },
            3 => Self::Isr { bar, offset, size },
            4 => Self::DeviceCfg { bar, offset, size },
            _ => Self::Unknown { bar, offset, size },
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn virtio() {
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
        let vc: Virtio = data[3..].try_into().unwrap();
        let sample = Virtio::Notify {
            bar: 4, offset: 0x00003000, size: 0x00001000,
            multiplier: Some(0x00000004)
        };
        assert_eq!(sample, vc);
    }
}
