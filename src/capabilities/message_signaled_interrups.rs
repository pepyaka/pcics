//! Message Signaled Interrupts
//!
//! Message Signaled Interrupts (MSI) is an optional feature that enables a device function to
//! request service by writing a system-specified data value to a system-specified address (using a
//! PCI DWORD memory write transaction). System software initializes the message address and
//! message data (from here on referred to as the “vector”) during device configuration, allocating
//! one or more vectors to each MSI capable function.

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// To request service, an MSI function writes the contents of the Message Data register to the
/// address specified by the contents of the Message Address register (and, optionally, the Message
/// Upper Address register for a 64-bit message address). A read of the address specified by the
/// contents of the Message Address register produces undefined results. 
#[derive(Default, Debug, PartialEq, Eq,)] 
pub struct MessageSignaledInterrups {
    pub message_control: MessageControl,
    pub message_address: MessageAddress,
    /// System-specified message data
    pub message_data: u16,
    /// The contents or undefined states or information are not defined at this time. Using any
    /// reserved area in the PCI specification is not permitted.
    pub reserved: u16,
    /// For each Mask bit that is set, the function is prohibited from sending the associated
    /// message
    pub mask_bits: Option<u32>,
    /// For each Pending bit that is set, the function has a pending associated message
    pub pending_bits: Option<u32>,
}

impl<'a> TryRead<'a, Endian> for MessageSignaledInterrups {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let mcp: MessageControlProto = bytes.read_with::<u16>(offset, endian)?.into();
        let message_address = bytes.read_with::<u32>(offset, endian)?;
        let msi = match (mcp.is_64bit_address_capable(), mcp.per_vector_masking_capable()) {
            (false, false) => Self {
                message_control: mcp.into(),
                message_address: MessageAddress::Dword(message_address),
                message_data: bytes.read_with::<u16>(offset, endian)?,
                reserved: bytes.read_with::<u16>(offset, endian)?,
                mask_bits: None,
                pending_bits: None,
            },
            (true, false) => Self {
                message_control: mcp.into(),
                message_address: MessageAddress::Qword({
                    let message_upper_address = 
                        bytes.read_with::<u32>(offset, endian)? as u64;
                    message_address as u64 | message_upper_address << 32
                }),
                message_data: bytes.read_with::<u16>(offset, endian)?,
                reserved: bytes.read_with::<u16>(offset, endian)?,
                mask_bits: None,
                pending_bits: None,
            },
            (false, true) => Self {
                message_control: mcp.into(),
                message_address: MessageAddress::Dword(message_address),
                message_data: bytes.read_with::<u16>(offset, endian)?,
                reserved: bytes.read_with::<u16>(offset, endian)?,
                mask_bits: bytes.read_with::<u32>(offset, endian).ok(),
                pending_bits: bytes.read_with::<u32>(offset, endian).ok(),
            },
            (true, true) => Self {
                message_control: mcp.into(),
                message_address: MessageAddress::Qword({
                    let message_upper_address = 
                        bytes.read_with::<u32>(offset, endian)? as u64;
                    message_address as u64 | message_upper_address << 32
                }),
                message_data: bytes.read_with::<u16>(offset, endian)?,
                reserved: bytes.read_with::<u16>(offset, endian)?,
                mask_bits: bytes.read_with::<u32>(offset, endian).ok(),
                pending_bits: bytes.read_with::<u32>(offset, endian).ok(),
            },
        };
        Ok((msi, *offset))
    }
}

/// Provides system software control over MSI.
#[derive(Default, Debug, PartialEq, Eq,)] 
pub struct MessageControl {
    pub enable: bool,
    pub multiple_message_capable: NumberOfVectors,
    pub multiple_message_enable: NumberOfVectors,
    pub per_vector_masking_capable: bool,
    pub reserved: u8,

}

/// System-specified message address
#[derive(Debug, PartialEq, Eq,)] 
pub enum MessageAddress {
    Dword(u32),
    Qword(u64),
}

impl Default for MessageAddress {
    fn default() -> Self {
        Self::Dword(Default::default())
    }
}

/// The number of requested vectors must be aligned to a power of two (if a function requires three
/// vectors, it requests four by initializing this field to “010”).
#[derive(Clone, Copy, Debug, PartialEq, Eq,)] 
#[repr(u8)]
pub enum NumberOfVectors {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
    Reserved = u8::MAX,
}

impl Default for NumberOfVectors {
    fn default() -> Self {
        Self::One
    }
}
impl From<u8> for NumberOfVectors {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Four,
            3 => Self::Eight,
            4 => Self::Sixteen,
            5 => Self::ThirtyTwo,
            _ => Self::Reserved,
        }
    }
}


/// Common Capability Structure for Message Address
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct MessageControlProto {
    enable: bool,
    multiple_message_capable: B3,
    multiple_message_enable: B3,
    is_64bit_address_capable: bool,
    per_vector_masking_capable: bool,
    reserved: B7,
}

impl From<MessageControlProto> for MessageControl {
    fn from(proto: MessageControlProto) -> Self {
        Self {
            enable: proto.enable(),
            multiple_message_capable: proto.multiple_message_capable().into(),
            multiple_message_enable: proto.multiple_message_enable().into(),
            per_vector_masking_capable: proto.per_vector_masking_capable(),
            reserved: proto.reserved(),
        }
    }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    //#[test]
    //fn msg_addr() {
    //    let data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k"));
    //    //println!("{}", hexdump!(data));
    //    let mac = MsgAddrCommon::from_bytes(data[0x2..0x8].try_into().unwrap());
    //    assert_eq!(true, mac.ctrl_enable());
    //    assert_eq!(1, mac.ctrl_multiple_message_capable());
    //    assert_eq!(6, mac.ctrl_multiple_message_enable());
    //    assert_eq!(false, mac.ctrl_is64bit());
    //    assert_eq!(false, mac.ctrl_per_vector_masking_capable());
    //    assert_eq!(66, mac.ctrl_reserved());
    //    assert_eq!(0x95f8e4dc, mac.address());
    //}

    #[test]
    fn number_of_vectors() {
        for i in 0..6 {
            assert_eq!(1 << i, NumberOfVectors::from(i) as u8);
        }
    }

    #[test]
    fn message_address_32bit() {
        let mut data = 
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k"))
            .clone();
        let control = 0b0_0000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].read_with(&mut 0, LE).unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                enable: false,
                multiple_message_capable: NumberOfVectors::One,
                multiple_message_enable: NumberOfVectors::One,
                per_vector_masking_capable: false,
                reserved: 0,
            },
            message_address: MessageAddress::Dword(0x95f8e4dc),
            message_data: 0xcb86,
            reserved: 0x1a87,
            mask_bits: None,
            pending_bits: None,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn message_address_64bit() {
        let mut data = 
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k"))
            .clone();
        let control = 0b0_1000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].read_with(&mut 0, LE).unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                enable: false,
                multiple_message_capable: NumberOfVectors::One,
                multiple_message_enable: NumberOfVectors::One,
                per_vector_masking_capable: false,
                reserved: 0,
            },
            message_address: MessageAddress::Qword(0x1A87CB8695F8E4DC),
            message_data: 0x5eb6,
            reserved: 0x3273,
            mask_bits: None,
            pending_bits: None,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn message_address_32bit_per_vector_masking() {
        let mut data = 
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k"))
            .clone();
        let control = 0b1_0000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].read_with(&mut 0, LE).unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                enable: false,
                multiple_message_capable: NumberOfVectors::One,
                multiple_message_enable: NumberOfVectors::One,
                per_vector_masking_capable: true,
                reserved: 0,
            },
            message_address: MessageAddress::Dword(0x95f8e4dc),
            message_data: 0xcb86,
            reserved: 0x1a87,
            mask_bits: Some(0x32735EB6),
            pending_bits: Some(0x6AE226D5),
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn message_address_64bit_per_vector_masking() {
        let mut data = 
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k"))
            .clone();
        let control = 0b1_1000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].read_with(&mut 0, LE).unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                enable: false,
                multiple_message_capable: NumberOfVectors::One,
                multiple_message_enable: NumberOfVectors::One,
                per_vector_masking_capable: true,
                reserved: 0,
            },
            message_address: MessageAddress::Qword(0x1A87CB8695F8E4DC),
            message_data: 0x5eb6,
            reserved: 0x3273,
            mask_bits: Some(0x6AE226D5),
            pending_bits: Some(0x5FFFEED8),
        };
        assert_eq!(sample, result);
    }
}
