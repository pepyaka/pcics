//! Message Signaled Interrupts
//!
//! Message Signaled Interrupts (MSI) is an optional feature that enables a device function to
//! request service by writing a system-specified data value to a system-specified address (using a
//! PCI DWORD memory write transaction). System software initializes the message address and
//! message data (from here on referred to as the “vector”) during device configuration, allocating
//! one or more vectors to each MSI capable function.

use heterob::{bit_numbering::Lsb, endianness::Le, P3, P4, P5, P6, P8};
use snafu::prelude::*;

/// MSI Capability Structure for 32-bit Message Address
pub const MSI_32BIT_SIZE: usize = 2 + 4 + 2 + 2;
/// MSI Capability Structure for 64-bit Message Address
pub const MSI_64BIT_SIZE: usize = 2 + 4 + 4 + 2 + 2;
/// MSI Capability Structure for 32-bit Message Address and PVM
pub const MSI_32BIT_PVM_SIZE: usize = 2 + 4 + 2 + 2 + 4 + 4;
/// MSI Capability Structure for 64-bit Message Address and PVM
pub const MSI_64BIT_PVM_SIZE: usize = 2 + 4 + 4 + 2 + 2 + 4 + 4;

/// MSI Errors
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum MessageSignaledInterrupsError {
    MessageControl,
    Size32bit,
    Size64bit,
    Size32bitPerVector,
    Size64bitPerVector,
}

/// To request service, an MSI function writes the contents of the Message Data register to the
/// address specified by the contents of the Message Address register (and, optionally, the Message
/// Upper Address register for a 64-bit message address). A read of the address specified by the
/// contents of the Message Address register produces undefined results.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct MessageSignaledInterrups {
    pub message_control: MessageControl,
    pub message_address: MessageAddress,
    /// Message Data
    pub message_data: u16,
    /// Extended Message Data
    pub extended_message_data: u16,
    /// For each Mask bit that is set, the function is prohibited from sending the associated
    /// message
    pub mask_bits: Option<u32>,
    /// For each Pending bit that is set, the function has a pending associated message
    pub pending_bits: Option<u32>,
}
impl<'a> TryFrom<&'a [u8]> for MessageSignaledInterrups {
    type Error = MessageSignaledInterrupsError;
    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let mc = slice
            .get(..MessageControl::SIZE)
            .and_then(|slice| {
                let bytes = <[u8; MessageControl::SIZE]>::try_from(slice).ok()?;
                Some(u16::from_le_bytes(bytes))
            })
            .ok_or(MessageSignaledInterrupsError::MessageControl)?;
        let Lsb((
            msi_enable,
            mmc,
            mme,
            a_64_bit_address_capable,
            per_vector_masking_capable,
            extended_message_data_capable,
            extended_message_data_enable,
            (),
        )) = P8::<_, 1, 3, 3, 1, 1, 1, 1, 5>(mc).into();
        let message_control = MessageControl {
            msi_enable,
            multiple_message_capable: MultipleMessage(mmc),
            multiple_message_enable: MultipleMessage(mme),
            a_64_bit_address_capable,
            per_vector_masking_capable,
            extended_message_data_capable,
            extended_message_data_enable,
        };
        let msi = match (a_64_bit_address_capable, per_vector_masking_capable) {
            (false, false) => {
                let Le((addr, message_data, extended_message_data)) = slice
                    .get(MessageControl::SIZE..MSI_32BIT_SIZE)
                    .and_then(|slice| {
                        const SIZE: usize = MSI_32BIT_SIZE - MessageControl::SIZE;
                        let bytes = <[u8; SIZE]>::try_from(slice).ok()?;
                        Some(P3(bytes).into())
                    })
                    .ok_or(MessageSignaledInterrupsError::Size32bit)?;
                Self {
                    message_control,
                    message_address: MessageAddress::Dword(addr),
                    message_data,
                    extended_message_data,
                    mask_bits: None,
                    pending_bits: None,
                }
            }
            (true, false) => {
                let Le((addr_lo, addr_hi, message_data, extended_message_data)) = slice
                    .get(MessageControl::SIZE..MSI_64BIT_SIZE)
                    .and_then(|slice| {
                        const SIZE: usize = MSI_64BIT_SIZE - MessageControl::SIZE;
                        let bytes = <[u8; SIZE]>::try_from(slice).ok()?;
                        Some(P4(bytes).into())
                    })
                    .ok_or(MessageSignaledInterrupsError::Size64bit)?;
                let _: (u32, u32) = (addr_lo, addr_hi);
                Self {
                    message_control,
                    message_address: MessageAddress::Qword(
                        ((addr_hi as u64) << 32) | addr_lo as u64,
                    ),
                    message_data,
                    extended_message_data,
                    mask_bits: None,
                    pending_bits: None,
                }
            }
            (false, true) => {
                let Le((addr, message_data, extended_message_data, mb, pb)) = slice
                    .get(MessageControl::SIZE..MSI_32BIT_PVM_SIZE)
                    .and_then(|slice| {
                        const SIZE: usize = MSI_32BIT_PVM_SIZE - MessageControl::SIZE;
                        let bytes = <[u8; SIZE]>::try_from(slice).ok()?;
                        Some(P5(bytes).into())
                    })
                    .ok_or(MessageSignaledInterrupsError::Size32bitPerVector)?;
                Self {
                    message_control,
                    message_address: MessageAddress::Dword(addr),
                    message_data,
                    extended_message_data,
                    mask_bits: Some(mb),
                    pending_bits: Some(pb),
                }
            }
            (true, true) => {
                let Le((addr_lo, addr_hi, message_data, extended_message_data, mb, pb)) = slice
                    .get(MessageControl::SIZE..MSI_64BIT_PVM_SIZE)
                    .and_then(|slice| {
                        const SIZE: usize = MSI_64BIT_PVM_SIZE - MessageControl::SIZE;
                        let bytes = <[u8; SIZE]>::try_from(slice).ok()?;
                        Some(P6(bytes).into())
                    })
                    .ok_or(MessageSignaledInterrupsError::Size64bitPerVector)?;
                let _: (u32, u32) = (addr_lo, addr_hi);
                Self {
                    message_control,
                    message_address: MessageAddress::Qword(
                        ((addr_hi as u64) << 32) | addr_lo as u64,
                    ),
                    message_data,
                    extended_message_data,
                    mask_bits: Some(mb),
                    pending_bits: Some(pb),
                }
            }
        };
        Ok(msi)
    }
}

/// Provides system software control over MSI.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct MessageControl {
    pub msi_enable: bool,
    pub multiple_message_capable: MultipleMessage,
    pub multiple_message_enable: MultipleMessage,
    pub a_64_bit_address_capable: bool,
    pub per_vector_masking_capable: bool,
    pub extended_message_data_capable: bool,
    pub extended_message_data_enable: bool,
}
impl MessageControl {
    pub const SIZE: usize = 2;
}

/// System-specified message address
#[derive(Debug, PartialEq, Eq)]
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
#[derive(Default, Debug, PartialEq, Eq)]
pub struct MultipleMessage(pub u8);
impl MultipleMessage {
    pub fn number_of_vectors(&self) -> u8 {
        1 << self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn message_address_32bit() {
        let mut data =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k")).clone();
        let control = 0b0_0000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].try_into().unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                msi_enable: false,
                multiple_message_capable: MultipleMessage(0),
                multiple_message_enable: MultipleMessage(0),
                per_vector_masking_capable: false,
                a_64_bit_address_capable: false,
                extended_message_data_capable: false,
                extended_message_data_enable: false,
            },
            message_address: MessageAddress::Dword(0x95f8e4dc),
            message_data: 0xcb86,
            mask_bits: None,
            pending_bits: None,
            extended_message_data: 0x1a87,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn message_address_64bit() {
        let mut data =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k")).clone();
        let control = 0b0_1000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].try_into().unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                msi_enable: false,
                multiple_message_capable: MultipleMessage(0),
                multiple_message_enable: MultipleMessage(0),
                per_vector_masking_capable: false,
                a_64_bit_address_capable: true,
                extended_message_data_capable: false,
                extended_message_data_enable: false,
            },
            message_address: MessageAddress::Qword(0x1A87CB8695F8E4DC),
            message_data: 0x5eb6,
            mask_bits: None,
            pending_bits: None,
            extended_message_data: 0x3273,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn message_address_32bit_per_vector_masking() {
        let mut data =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k")).clone();
        let control = 0b1_0000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].try_into().unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                msi_enable: false,
                multiple_message_capable: MultipleMessage(0),
                multiple_message_enable: MultipleMessage(0),
                per_vector_masking_capable: true,
                a_64_bit_address_capable: false,
                extended_message_data_capable: false,
                extended_message_data_enable: false,
            },
            message_address: MessageAddress::Dword(0x95f8e4dc),
            message_data: 0xcb86,
            mask_bits: Some(0x32735EB6),
            pending_bits: Some(0x6AE226D5),
            extended_message_data: 0x1a87,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn message_address_64bit_per_vector_masking() {
        let mut data =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/random/4k")).clone();
        let control = 0b1_1000_0000u16.to_le_bytes();
        data[2] = control[0];
        data[3] = control[1];
        let result: MessageSignaledInterrups = data[2..].try_into().unwrap();
        let sample = MessageSignaledInterrups {
            message_control: MessageControl {
                msi_enable: false,
                multiple_message_capable: MultipleMessage(0),
                multiple_message_enable: MultipleMessage(0),
                per_vector_masking_capable: true,
                a_64_bit_address_capable: true,
                extended_message_data_capable: false,
                extended_message_data_enable: false,
            },
            message_address: MessageAddress::Qword(0x1A87CB8695F8E4DC),
            message_data: 0x5eb6,
            extended_message_data: 0x3273,
            mask_bits: Some(0x6AE226D5),
            pending_bits: Some(0x5FFFEED8),
        };
        assert_eq!(sample, result);
    }
}
