//! Predefined header region
//!
//! The predefined header region consists of fields that uniquely identify the device and allow the
//! device to be generically controlled. The predefined header portion of the Configuration Space
//! is divided into two parts. The first 16 bytes are defined the same for all types of devices.
//! The remaining bytes can have different layouts depending on the base function that the device
//! supports. 
//!
//! ## Usage
//! #### Type 00h Configuration Space Header
//!
//! ```plaintext
//! VGA compatible controller [0300]: Advanced Micro Devices, Inc. [AMD/ATI] RS880 [Radeon HD 4250] [1002:9715] (prog-if 00 [VGA controller])
//!         Subsystem: Fujitsu Technology Solutions Device [1734:11da]
//!         Control: I/O+ Mem+ BusMaster+ SpecCycle- MemWINV- VGASnoop- ParErr- Stepping- SERR- FastB2B- DisINTx-
//!         Status: Cap+ 66MHz- UDF- FastB2B- ParErr- DEVSEL=fast >TAbort- <TAbort- <MAbort- >SERR- <PERR- INTx-
//!         Latency: 0, Cache Line Size: 64 bytes
//!         Interrupt: pin A routed to IRQ 11
//!         Region 0: Memory at fc000000 (32-bit, prefetchable)
//!         Region 1: I/O ports at e000
//!         Region 2: Memory at ff500000 (32-bit, non-prefetchable)
//!         Region 5: Memory at ff400000 (32-bit, non-prefetchable)
//!         Capabilities: [50] Power Management version 3
//! ```
//! ```
//! # use pcics::header::*;
//! # use byte::{ ctx::LE, BytesExt, };
//! let data = [
//!     0x02,0x10,0x15,0x97,0x07,0x00,0x10,0x00,0x00,0x00,0x00,0x03,0x10,0x00,0x80,0x00,
//!     0x08,0x00,0x00,0xfc,0x01,0xe0,0x00,0x00,0x00,0x00,0x50,0xff,0x00,0x00,0x00,0x00,
//!     0x00,0x00,0x00,0x00,0x00,0x00,0x40,0xff,0x00,0x00,0x00,0x00,0x34,0x17,0xda,0x11,
//!     0x00,0x00,0x00,0x00,0x50,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0b,0x01,0x00,0x00,
//! ];
//! let result = data.read_with::<Header>(&mut 0, LE).unwrap();
//! let sample = Header {
//!     vendor_id: 0x1002,
//!     device_id: 0x9715,
//!     command: 0b0000_0000_0000_0111.into(),
//!     status: 0b0000_0000_0001_0000.into(),
//!     revision_id: 0,
//!     class_code: ClassCode {
//!         interface: 0x00,
//!         sub: 0x00,
//!         base: 0x03,
//!     },
//!     cache_line_size: 64 / 4, // DWORD = 4 bytes
//!     latency_timer: 0,
//!     bist: BuiltInSelfTest {
//!         is_capable: false,
//!         is_running: false,
//!         completion_code: 0x00,
//!     },
//!     capabilities_pointer: 0x50,
//!     is_multi_function: true,
//!     header_type: HeaderType::Normal(Normal {
//!         base_addresses: BaseAddressesNormal([
//!             0xfc000000 | 0b1000,
//!             0xe000 | 0b01,
//!             0xff500000 | 0b0000,
//!             0,
//!             0,
//!             0xff400000 | 0b0000,
//!         ]),
//!         cardbus_cis_pointer: 0x00,
//!         sub_vendor_id: 0x1734,
//!         sub_device_id: 0x11da,
//!         expansion_rom: Default::default(),
//!         min_grant: 0,
//!         max_latency: 0,
//!     }),
//!     interrupt_line: 0xb,
//!     interrupt_pin: InterruptPin::IntA,
//! };
//! assert_eq!(sample, result);
//!
//! ```

use core::convert::TryInto;

use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

mod command;
pub use command::Command;

mod status;
pub use status::Status;

mod class_code;
pub use class_code::ClassCode;

mod bar;
pub use bar::{
    BaseAddressType,
    BaseAddressesNormal,
    BaseAddressesBridge,
    BaseAddressesCardbus
};

mod bridge_control;
pub use bridge_control::BridgeControl;

mod cardbus_bridge_control;
pub use cardbus_bridge_control::CardbusBridgeControl;


/// Main structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// Identifies the manufacturer of the device. Where valid IDs are allocated by PCI-SIG to
    /// ensure uniqueness and 0xFFFF is an invalid value that will be returned on read accesses to
    /// Configuration Space registers of non-existent devices. 
    pub vendor_id: u16, 
    /// Identifies the particular device. Where valid IDs are allocated by the vendor
    pub device_id: u16,
    pub command: Command, 
    pub status: Status<'P'>,
    /// Device specific revision identifier.
    pub revision_id: u8,
    pub class_code: ClassCode, 
    /// Specifies the system cache line size in 32-bit units. A device can limit the number of
    /// cacheline sizes it can support, if a unsupported value is written to this field, the
    /// device will behave as if a value of 0 was written. 
    pub cache_line_size: u8, 
    /// Specifies the latency timer in units of PCI bus clocks.
    pub latency_timer: u8, 
    /// Used to identify a multi-function device
    pub is_multi_function: bool,
    pub header_type: HeaderType,
    pub bist: BuiltInSelfTest,
    /// Used to point to a linked list of new capabilities implemented by this device
    pub capabilities_pointer: u8,
    /// Specifies which input of the system interrupt controllers the device's interrupt pin is
    /// connected to and is implemented by any device that makes use of an interrupt pin. For
    /// the x86 architecture this register corresponds to the PIC IRQ numbers 0-15 (and not I/O
    /// APIC IRQ numbers) and a value of 0xFF defines no connection.
    pub interrupt_line: u8, 
    pub interrupt_pin: InterruptPin,
}
impl<'a> TryRead<'a, Endian> for Header {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let vendor_id = bytes.read_with::<u16>(offset, endian)?; 
        let device_id = bytes.read_with::<u16>(offset, endian)?;
        let command = bytes.read_with::<u16>(offset, endian)?.into(); 
        let status = bytes.read_with::<u16>(offset, endian)?.into();
        let revision_id = bytes.read_with::<u8>(offset, endian)?;
        let class_code = bytes.read_with::<ClassCode>(offset, endian)?; 
        let cache_line_size = bytes.read_with::<u8>(offset, endian)?; 
        let latency_timer = bytes.read_with::<u8>(offset, endian)?; 
        let htype = bytes.read_with::<u8>(offset, endian)?;
        let bist = bytes.read_with::<BuiltInSelfTest>(offset, endian)?;
        let (capabilities_pointer, interrupt_line, interrupt_pin);
        let is_multi_function = htype & 0x80 != 0;
        let header_type = match htype & !0x80 {
            0x00 => {
                HeaderType::Normal(Normal {
                    base_addresses: bytes.read_with::<BaseAddressesNormal>(offset, endian)?,
                    cardbus_cis_pointer: bytes.read_with::<u32>(offset, endian)?,
                    sub_vendor_id: bytes.read_with::<u16>(offset, endian)?,
                    sub_device_id: bytes.read_with::<u16>(offset, endian)?,
                    expansion_rom: bytes.read_with::<ExpansionRom>(offset, endian)?,
                    min_grant: {
                        capabilities_pointer = bytes.read_with::<u8>(offset, endian)? & !0b11;
                        let _reserved: [u8; 7] = bytes.read_with::<&[u8]>(offset, Bytes::Len(7))?
                            .try_into().unwrap_or_default();
                        interrupt_line = bytes.read_with::<u8>(offset, endian)?; 
                        interrupt_pin = bytes.read_with::<InterruptPin>(offset, endian)?;
                        bytes.read_with::<u8>(offset, endian)?
                    }, 
                    max_latency: bytes.read_with::<u8>(offset, endian)?, 
                })
            },
            0x01 => {
                let io_base = bytes.read_with::<u8>(&mut 0x1C, endian)?;
                let io_limit = bytes.read_with::<u8>(&mut 0x1D, endian)?;
                HeaderType::Bridge(Bridge {
                    base_addresses: bytes.read_with::<BaseAddressesBridge>(offset, endian)?,
                    primary_bus_number: bytes.read_with::<u8>(offset, endian)?,
                    secondary_bus_number: bytes.read_with::<u8>(offset, endian)?,
                    subordinate_bus_number: bytes.read_with::<u8>(offset, endian)?,
                    secondary_latency_timer: bytes.read_with::<u8>(offset, endian)?,
                    secondary_status: {
                        *offset += 2; // Skip IO Base and IO Limit
                        bytes.read_with::<u16>(offset, endian)?.into()
                    },
                    memory_base: bytes.read_with::<u16>(offset, endian)?,
                    memory_limit: bytes.read_with::<u16>(offset, endian)?,
                    prefetchable_memory: BridgePrefetchableMemory::new(
                        bytes.read_with::<u16>(offset, endian)?, // Prefetchable Memory Base
                        bytes.read_with::<u16>(offset, endian)?, // Prefetchable Memory Limit
                        bytes.read_with::<u32>(offset, endian)?, // Prefetchable Base Upper 32 Bits
                        bytes.read_with::<u32>(offset, endian)?, // Prefetchable Limit Upper 32 Bits
                    ),
                    io_address_range: BridgeIoAddressRange::new(
                        io_base, io_limit,
                        bytes.read_with::<u16>(offset, endian)?, // I/O Base Upper 16 Bits
                        bytes.read_with::<u16>(offset, endian)?, // I/O Limit Upper 16 Bits
                    ),
                    expansion_rom: {
                        capabilities_pointer = bytes.read_with::<u8>(offset, endian)? & !0b11;
                        let _reserved: [u8; 3] = bytes.read_with::<&[u8]>(offset, Bytes::Len(3))?
                            .try_into().unwrap_or_default();
                        bytes.read_with::<ExpansionRom>(offset, endian)?
                    },
                    bridge_control: {
                        interrupt_line = bytes.read_with::<u8>(offset, endian)?; 
                        interrupt_pin = bytes.read_with::<InterruptPin>(offset, endian)?;
                        bytes.read_with::<u16>(offset, endian)?.into()
                    },
                })
            },
            0x02 => {
                HeaderType::Cardbus(Cardbus {
                    base_addresses: bytes.read_with::<BaseAddressesCardbus>(offset, endian)?,
                    secondary_status: {
                        capabilities_pointer = bytes.read_with::<u8>(offset, endian)? & !0b11;
                        let _reserved = bytes.read_with::<u8>(offset, endian)?;
                        bytes.read_with::<u16>(offset, endian)?.into()
                    },
                    pci_bus_number: bytes.read_with::<u8>(offset, endian)?,
                    cardbus_bus_number: bytes.read_with::<u8>(offset, endian)?,
                    subordinate_bus_number: bytes.read_with::<u8>(offset, endian)?,
                    cardbus_latency_timer: bytes.read_with::<u8>(offset, endian)?,
                    memory_base_address_0: bytes.read_with::<u32>(offset, endian)?,
                    memory_limit_address_0: bytes.read_with::<u32>(offset, endian)?,
                    memory_base_address_1: bytes.read_with::<u32>(offset, endian)?,
                    memory_limit_address_1: bytes.read_with::<u32>(offset, endian)?,
                    io_access_address_range_0: bytes.read_with::<IoAccessAddressRange>(offset, endian)?,
                    io_access_address_range_1: bytes.read_with::<IoAccessAddressRange>(offset, endian)?,
                    bridge_control: {
                        interrupt_line = bytes.read_with::<u8>(offset, endian)?; 
                        interrupt_pin = bytes.read_with::<InterruptPin>(offset, endian)?;
                        bytes.read_with::<u16>(offset, endian)?.into()
                    },
                    subsystem_vendor_id: bytes.read_with::<u16>(offset, endian)?,
                    subsystem_device_id: bytes.read_with::<u16>(offset, endian)?,
                    legacy_mode_base_address: bytes.read_with::<u32>(offset, endian)?,
                })
            },
            _ => return Err(byte::Error::BadInput { err: "illegal header type" }),
        };
        let header = Header {
            vendor_id,
            device_id,
            command,
            status,
            revision_id,
            class_code,
            cache_line_size,
            latency_timer,
            is_multi_function,
            header_type,
            bist,
            capabilities_pointer,
            interrupt_line,
            interrupt_pin,
        };
        Ok((header, *offset))
    }
}
impl<'a> TryFrom<&'a [u8]> for Header {
    type Error = byte::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        bytes.read_with(&mut 0, LE)
    }
}



/// Identifies the layout of the second part of the predefined header (beginning at byte 10h in
/// Configuration Space) and also whether or not the device contains multiple functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderType {
    Normal(Normal),
    Bridge(Bridge),
    Cardbus(Cardbus),
}
impl HeaderType {
    pub fn base_addresses(&self) -> bar::BaseAddresses {
        match self {
            Self::Normal(Normal { base_addresses, .. }) => base_addresses.clone().into(),
            Self::Bridge(Bridge { base_addresses, .. }) => base_addresses.clone().into(),
            Self::Cardbus(Cardbus { base_addresses, .. }) => base_addresses.clone().into(),
        }
    }
    pub fn expansion_rom(&self) -> Option<ExpansionRom> {
       match &self {
           Self::Normal(Normal { expansion_rom, .. }) => Some(expansion_rom.clone()),
           Self::Bridge(Bridge { expansion_rom, .. }) => Some(expansion_rom.clone()),
           _ => None,
       }
    }
}


/// General device (Type 00h)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Normal {
    pub base_addresses: BaseAddressesNormal,
    /// Points to the Card Information Structure and is used by devices that share silicon between CardBus and PCI. 
    pub cardbus_cis_pointer: u32,
    /// Subsystem Vendor ID
    pub sub_vendor_id: u16,
    /// Subsystem Device ID
    pub sub_device_id: u16,
    /// Expansion ROM
    pub expansion_rom: ExpansionRom,
    /// A read-only register that specifies the burst period length, in 1/4 microsecond units,
    /// that the device needs (assuming a 33 MHz clock rate).
    pub min_grant: u8, 
    /// A read-only register that specifies how often the device needs access to the PCI bus
    /// (in 1/4 microsecond units).
    pub max_latency: u8, 
}

/// PCI-to-PCI bridge (Type 01h)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bridge {
    /// Base Address Registers
    pub base_addresses: BaseAddressesBridge,
    /// Primary Bus Number
    pub primary_bus_number: u8,
    /// Secondary Bus Number
    pub secondary_bus_number: u8,
    /// Subordinate Bus Numbe
    pub subordinate_bus_number: u8,
    /// Secondary Latency Timer
    pub secondary_latency_timer: u8,
    pub io_address_range: BridgeIoAddressRange,
    /// Secondary Status
    pub secondary_status: Status<'B'>,
    /// Memory Base
    pub memory_base: u16,
    /// Memory Limit
    pub memory_limit: u16,
    pub prefetchable_memory: BridgePrefetchableMemory,
    /// Expansion ROM
    pub expansion_rom: ExpansionRom,
    pub bridge_control: BridgeControl,
}

/// The I/O Base and I/O Limit registers define an address range that is used by the bridge to
/// determine when to forward I/O transactions from one interface to the other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeIoAddressRange {
    NotImplemented,
    IoAddr16 {
        base: u16,
        limit: u16,
    },
    IoAddr32 {
        base: u32,
        limit: u32,
    },
    Reserved {
        base: u8,
        limit: u8,
    },
}
impl BridgeIoAddressRange {
    pub fn new(io_base: u8, io_limit: u8, io_base_upper_16: u16, io_limit_upper_16: u16) -> Self {
        let base_capability = io_base & 0xf;
        let base_address = io_base & !0xf;
        let _limit_capability = io_limit & 0xf;
        let limit_address = io_limit & !0xf;
        match (base_capability, base_address) {
            (0x00, 0x00) => Self::NotImplemented,
            (0x00, _) => Self::IoAddr16 {
                base: (base_address as u16) << 8,
                limit: (limit_address as u16) << 8,
            },
            (0x01, _) => Self::IoAddr32 {
                base: ((base_address as u32) << 8) | ((io_base_upper_16 as u32) << 16),
                limit: ((limit_address as u32) << 8) | ((io_limit_upper_16 as u32) << 16),
            },
            _ => Self::Reserved {
                base: io_base,
                limit: io_limit,
            },
        }
    }
}

/// The Prefetchable Memory Base and Prefetchable Memory Limit registers define a prefetchable
/// memory address range which is used by the bridge to determine when to forward memory
/// transactions from one interface to the other
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgePrefetchableMemory {
    NotImplemented,
    MemAddr32 {
        base: u32,
        limit: u32,
    },
    MemAddr64 {
        base: u64,
        limit: u64,
    },
    Reserved {
        base: u16,
        limit: u16,
    },
}
impl BridgePrefetchableMemory {
    pub fn new(base: u16, limit: u16, base_upper_32: u32, limit_upper_32: u32) -> Self {
        let base_capability = base & 0xf;
        let base_address = base & !0xf;
        let _limit_capability = limit & 0xf;
        let limit_address = limit & !0xf;
        match (base_capability, base_address) {
            (0x00, 0x00) => Self::NotImplemented,
            (0x00, _) => Self::MemAddr32 {
                base: (base_address as u32) << 16,
                limit: (limit_address as u32) << 16,
            },
            (0x01, _) => Self::MemAddr64 {
                base: ((base_address as u64) << 16) | ((base_upper_32 as u64) << 32),
                limit: ((limit_address as u64) << 16) | ((limit_upper_32 as u64) << 32),
            },
            _ => Self::Reserved {
                base,
                limit,
            },
        }
    }
}

/// PCI-to-CardBus bridge (Type 02h)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cardbus {
    pub base_addresses: BaseAddressesCardbus,
    /// Secondary status
    pub secondary_status: Status<'C'>,
    /// PCI Bus Number
    pub pci_bus_number: u8,
    /// CardBus Bus Number
    pub cardbus_bus_number: u8,
    /// Subordinate Bus Number 
    pub subordinate_bus_number: u8,
    /// CardBus Latency Timer
    pub cardbus_latency_timer: u8,
    /// Memory Base #0
    pub memory_base_address_0: u32,
    /// Memory Limit #0
    pub memory_limit_address_0: u32,
    /// Memory Base Address #1
    pub memory_base_address_1: u32,
    /// Memory Limit #1
    pub memory_limit_address_1: u32,
    pub io_access_address_range_0: IoAccessAddressRange,
    pub io_access_address_range_1: IoAccessAddressRange,
    pub bridge_control: CardbusBridgeControl,
    /// Subsystem Vendor ID
    pub subsystem_vendor_id: u16,
    /// Subsystem Device ID
    pub subsystem_device_id: u16,
    /// PC Card 16 Bit IF Legacy Mode Base Address
    pub legacy_mode_base_address: u32,
}


/// Represents that status and allows control of a devices BIST (built-in self test).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BuiltInSelfTest {
    /// Device supports BIST
    pub is_capable: bool,
    /// When set to `true` the BIST is invoked. This bit is reset when BIST completes. If BIST does
    /// not complete after 2 seconds the device should be failed by system software.
    pub is_running: bool,
    /// Will return 0, after BIST execution, if the test completed successfully.
    pub completion_code: u8,
}
impl From<u8> for BuiltInSelfTest {
    fn from(data: u8) -> Self {
        Self {
            is_capable: data & 0b1000_0000 != 0,
            is_running: data & 0b0100_0000 != 0,
            completion_code: data & 0b1111,
        }
    }
}
impl From<BuiltInSelfTest> for u8 {
    fn from(bist: BuiltInSelfTest) -> Self {
        let mut result = bist.completion_code & 0b1111;
        if bist.is_capable {
            result |= 0b1000_0000;
        }
        if bist.is_running {
            result |= 0b0100_0000;
        }
        result
    }
}
impl<'a> TryRead<'a, Endian> for BuiltInSelfTest {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let byte = bytes.read_with::<u8>(offset, endian)?;
        let bist = BuiltInSelfTest {
            is_capable: byte & 0b1000_0000 != 0,
            is_running: byte & 0b0100_0000 != 0,
            completion_code: byte & 0b1111,
        };
        Ok((bist, *offset))
    }
}
        

/// Specifies which interrupt pin the device uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptPin {
    Unused,
    IntA,
    IntB,
    IntC,
    IntD,
    Reserved(u8),
}
impl Default for InterruptPin {
    fn default() -> Self { Self::Unused }
}
impl<'a> TryRead<'a, Endian> for InterruptPin {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let byte = bytes.read_with::<u8>(offset, endian)?;
        let interrupt_pin = match byte {
            0x00 => Self::Unused,
            0x01 => Self::IntA,
            0x02 => Self::IntB,
            0x03 => Self::IntC,
            0x04 => Self::IntD,
            v => Self::Reserved(v),
        };
        Ok((interrupt_pin, *offset))
    }
}
impl From<u8> for InterruptPin {
    fn from(data: u8) -> Self {
        match data {
            0x00 => Self::Unused,
            0x01 => Self::IntA,
            0x02 => Self::IntB,
            0x03 => Self::IntC,
            0x04 => Self::IntD,
            v => Self::Reserved(v),
        }
    }
}
impl From<InterruptPin> for u8 {
    fn from(pin: InterruptPin) -> Self {
        match pin {
            InterruptPin::Unused => 0x00,
            InterruptPin::IntA => 0x01,
            InterruptPin::IntB => 0x02,
            InterruptPin::IntC => 0x03,
            InterruptPin::IntD => 0x04,
            InterruptPin::Reserved(v) => v,
        }
    }
}


/// The IO Base Register and I/O Limit Register defines the address range that is used by the
/// bridge to determine when to forward an I/O transaction to the CardBus. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoAccessAddressRange {
    Addr16Bit {
        base: u16,
        limit: u16
    },
    Addr32Bit {
        base: u32,
        limit: u32
    },
    Unknown,
}
impl Default for IoAccessAddressRange {
    fn default() -> Self { IoAccessAddressRange::Unknown }
}
impl<'a> TryRead<'a, Endian> for IoAccessAddressRange {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let base_lower = bytes.read_with::<u16>(offset, endian)?;
        let base_upper = bytes.read_with::<u16>(offset, endian)?;
        let limit_lower = bytes.read_with::<u16>(offset, endian)?;
        let limit_upper = bytes.read_with::<u16>(offset, endian)?;
        let io_access_address_range = match base_lower & 0b11 {
            0x00 => Self::Addr16Bit {
                base: base_lower & !0b11,
                limit: limit_lower
            },
            0x01 => {
                let base = u32::from_le_bytes({
                    let lower = (base_lower & !0b11).to_le_bytes();
                    let upper = base_upper.to_le_bytes();
                    [lower[0], lower[1], upper[0], upper[1]]
                });
                let limit = u32::from_le_bytes({
                    let lower = limit_lower.to_le_bytes();
                    let upper = limit_upper.to_le_bytes();
                    [lower[0], lower[1], upper[0], upper[1]]
                });
                Self::Addr32Bit { base, limit }
            },
            _ => Self::Unknown,
        };
        Ok((io_access_address_range, *offset))
    }
}
impl From<[[u16;2]; 2]> for IoAccessAddressRange {
    fn from(data: [[u16;2]; 2]) -> Self {
        let [[base_lower, base_upper], [limit_lower, limit_upper]] = data;
        match base_lower & 0b11 {
            0x00 => Self::Addr16Bit {
                base: base_lower & !0b11,
                limit: limit_lower
            },
            0x01 => {
                let base = u32::from_le_bytes({
                    let lower = (base_lower & !0b11).to_le_bytes();
                    let upper = base_upper.to_le_bytes();
                    [lower[0], lower[1], upper[0], upper[1]]
                });
                let limit = u32::from_le_bytes({
                    let lower = limit_lower.to_le_bytes();
                    let upper = limit_upper.to_le_bytes();
                    [lower[0], lower[1], upper[0], upper[1]]
                });
                Self::Addr32Bit { base, limit }
            },
            _ => Self::Unknown,
        }
    }
}
impl From<IoAccessAddressRange > for [[u16;2]; 2] {
    fn from(data: IoAccessAddressRange) -> Self {
        match data {
            IoAccessAddressRange::Addr16Bit { base, limit } => {
                [[base & !0b11, 0], [limit, 0]]
            },
            IoAccessAddressRange::Addr32Bit { base, limit } => {
                let base = base.to_le_bytes();
                let base_upper = u16::from_le_bytes([base[0] & !0b11, base[1]]);
                let base_lower = u16::from_le_bytes([base[2], base[3]]);
                let limit = limit.to_le_bytes();
                let limit_upper = u16::from_le_bytes([limit[0], limit[1]]);
                let limit_lower = u16::from_le_bytes([limit[2], limit[3]]);
                [[base_upper, base_lower], [limit_upper, limit_lower]]
            },
            _ => unreachable!(),
        }
    }
}
        

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExpansionRom {
    pub address: u32,
    pub is_enabled: bool,
}
impl<'a> TryRead<'a, Endian> for ExpansionRom {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let dword = bytes.read_with::<u32>(offset, endian)?;
        let expansion_rom = ExpansionRom { 
            address: dword & !0x7ff,
            is_enabled: dword & 1 != 0,
        };
        Ok((expansion_rom, *offset))
    }
}
impl From<u32> for ExpansionRom {
    fn from(dword: u32) -> Self {
        Self { 
            address: dword & !0x7ff,
            is_enabled: dword & 1 != 0,
        }
    }
}
impl From<ExpansionRom> for u32 {
    fn from(rom: ExpansionRom) -> Self {
        rom.address | (rom.is_enabled as u32)
    }
}



#[cfg(test)]
mod tests {
    use byte::*;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn io_access_address_range() {
        let zeros = [[ 0x00, 0x00 ], [ 0x00, 0x00 ]];
        assert_eq!(IoAccessAddressRange::Addr16Bit { base: 0, limit: 0 }, zeros.into(), "All zeros");

        let a16 = [[ 0x50, 0x00 ], [ 0x60, 0x00 ]];
        assert_eq!(IoAccessAddressRange::Addr16Bit { base: 0x50, limit: 0x60 }, a16.into(), "16 Bit");

        let a32 = [[ 0x51, 0x50 ], [ 0x60, 0x60 ]];
        assert_eq!(IoAccessAddressRange::Addr32Bit { base: 0x500050, limit: 0x600060 }, a32.into(), "32 Bit");

        let unkn = [[ 0x52, 0x50 ], [ 0x60, 0x60 ]];
        assert_eq!(IoAccessAddressRange::Unknown, unkn.into(), "Unknown");
    }

    #[test]
    fn header_type_normal() {
        // SATA controller [0106]: Intel Corporation Q170/Q150/B150/H170/H110/Z170/CM236 Chipset SATA Controller [AHCI Mode] [8086:a102] (rev 31) (prog-if 01 [AHCI 1.0])
        // Subsystem: Dell Device [1028:06a5]
        // Control: I/O+ Mem+ BusMaster+ SpecCycle- MemWINV- VGASnoop- ParErr+ Stepping- SERR+ FastB2B- DisINTx+
        // Status: Cap+ 66MHz+ UDF- FastB2B+ ParErr- DEVSEL=medium >TAbort- <TAbort- <MAbort- >SERR- <PERR- INTx-
        // Latency: 0
        // Interrupt: pin A routed to IRQ 30
        // Region 0: Memory at 93014000 (32-bit, non-prefetchable) [size=8K]
        // Region 1: Memory at 93017000 (32-bit, non-prefetchable) [size=256]
        // Region 2: I/O ports at 3040 [size=8]
        // Region 3: I/O ports at 3048 [size=4]
        // Region 4: I/O ports at 3020 [size=32]
        // Region 5: Memory at 93016000 (32-bit, non-prefetchable) [size=2K]
        // Capabilities: [80] MSI: Enable+ Count=1/1 Maskable- 64bit-
        //         Address: fee003b8  Data: 0000
        // Capabilities: [70] Power Management version 3
        //         Flags: PMEClk- DSI- D1- D2- AuxCurrent=0mA PME(D0-,D1-,D2-,D3hot+,D3cold-)
        //         Status: D0 NoSoftRst+ PME-Enable- DSel=0 DScale=0 PME-
        // Capabilities: [a8] SATA HBA v1.0 BAR4 Offset=00000004
        // Kernel driver in use: ahci
        // Kernel modules: ahci
        let data = [
            0x86, 0x80, 0x02, 0xa1, 0x47, 0x05, 0xb0, 0x02, 0x31, 0x01, 0x06, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x40, 0x01, 0x93, 0x00, 0x70, 0x01, 0x93, 0x41, 0x30, 0x00, 0x00, 0x49, 0x30, 0x00, 0x00,
            0x21, 0x30, 0x00, 0x00, 0x00, 0x60, 0x01, 0x93, 0x00, 0x00, 0x00, 0x00, 0x28, 0x10, 0xa5, 0x06,
            0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x01, 0x00, 0x00,
        ];
        let result: Header = data.read_with(&mut 0, LE).unwrap();
        let sample = Header {
            vendor_id: 0x8086,
            device_id: 0xa102,
            command: 0b0000_0101_0100_0111.into(),
            status: 0b0000_0010_1011_0000.into(),
            revision_id: 0x31,
            class_code: ClassCode {
                interface: 0x01,
                sub: 0x06,
                base: 0x01,
            },
            cache_line_size: 0,
            latency_timer: 0,
            bist: BuiltInSelfTest {
                is_capable: false,
                is_running: false,
                completion_code: 0x00,
            },
            capabilities_pointer: 0x80,
            is_multi_function: false,
            header_type: HeaderType::Normal(Normal {
                base_addresses: BaseAddressesNormal([
                    0x93014000,
                    0x93017000,
                    0x3041,
                    0x3049,
                    0x3021,
                    0x93016000
                ]),
                cardbus_cis_pointer: 0x00,
                sub_vendor_id: 0x1028,
                sub_device_id: 0x06a5,
                expansion_rom: Default::default(),
                min_grant: 0,
                max_latency: 0,
            }),
            interrupt_line: 0xb,
            interrupt_pin: InterruptPin::IntA,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn header_type_bridge() {
        // PCI bridge [0604]: Renesas Technology Corp. SH7758 PCIe Switch [PS] [1912:001d] (prog-if 00 [Normal decode])
        // Control: I/O+ Mem+ BusMaster+ SpecCycle- MemWINV- VGASnoop- ParErr- Stepping- SERR- FastB2B- DisINTx-
        // Status: Cap+ 66MHz- UDF- FastB2B- ParErr- DEVSEL=fast >TAbort- <TAbort- <MAbort- >SERR- <PERR- INTx-
        // Latency: 0
        // BIST result: 00
        // Bus: primary=04, secondary=05, subordinate=08, sec-latency=0
        // I/O behind bridge: 0000f000-00000fff
        // Memory behind bridge: 92000000-929fffff
        // Prefetchable memory behind bridge: 0000000091000000-0000000091ffffff
        // Secondary status: 66MHz- FastB2B- ParErr- DEVSEL=fast >TAbort- <TAbort- <MAbort- <SERR- <PERR-
        // BridgeCtl: Parity+ SERR+ NoISA- VGA+ MAbort- >Reset- FastB2B-
        //         PriDiscTmr- SecDiscTmr- DiscTmrStat- DiscTmrSERREn-
        // Capabilities: [40] Power Management version 3
        //         Flags: PMEClk- DSI- D1- D2- AuxCurrent=0mA PME(D0+,D1-,D2-,D3hot+,D3cold+)
        //         Status: D0 NoSoftRst+ PME-Enable- DSel=0 DScale=0 PME-
        // Capabilities: [50] MSI: Enable- Count=1/1 Maskable- 64bit+
        //         Address: 0000000000000000  Data: 0000
        // Capabilities: [70] Express (v2) Upstream Port, MSI 00
        //         DevCap: MaxPayload 128 bytes, PhantFunc 0
        //                 ExtTag+ AttnBtn- AttnInd- PwrInd- RBE+ SlotPowerLimit 0.000W
        //         DevCtl: Report errors: Correctable- Non-Fatal+ Fatal+ Unsupported+
        //                 RlxdOrd+ ExtTag+ PhantFunc- AuxPwr- NoSnoop+
        //                 MaxPayload 128 bytes, MaxReadReq 128 bytes
        //         DevSta: CorrErr+ UncorrErr- FatalErr- UnsuppReq+ AuxPwr- TransPend-
        //         LnkCap: Port #0, Speed 2.5GT/s, Width x1, ASPM L0s, Exit Latency L0s unlimited, L1 unlimited
        //                 ClockPM- Surprise- LLActRep- BwNot- ASPMOptComp+
        //         LnkCtl: ASPM Disabled; Disabled- CommClk+
        //                 ExtSynch- ClockPM- AutWidDis- BWInt- AutBWInt-
        //         LnkSta: Speed 2.5GT/s, Width x1, TrErr- Train- SlotClk+ DLActive- BWMgmt- ABWMgmt-
        //         DevCap2: Completion Timeout: Not Supported, TimeoutDis-, LTR-, OBFF Not Supported
        //         DevCtl2: Completion Timeout: 50us to 50ms, TimeoutDis-, LTR-, OBFF Disabled
        //         LnkCtl2: Target Link Speed: 2.5GT/s, EnterCompliance- SpeedDis-
        //                  Transmit Margin: Normal Operating Range, EnterModifiedCompliance- ComplianceSOS-
        //                  Compliance De-emphasis: -6dB
        //         LnkSta2: Current De-emphasis Level: -6dB, EqualizationComplete-, EqualizationPhase1-
        //                  EqualizationPhase2-, EqualizationPhase3-, LinkEqualizationRequest-
        // Capabilities: [b0] Subsystem: Renesas Technology Corp. SH7758 PCIe Switch [PS] [1912:001d]
        // Capabilities: [100 v1] Advanced Error Reporting
        //         UESta:  DLP- SDES- TLP- FCP- CmpltTO- CmpltAbrt- UnxCmplt- RxOF- MalfTLP- ECRC- UnsupReq- ACSViol-
        //         UEMsk:  DLP- SDES- TLP- FCP- CmpltTO- CmpltAbrt+ UnxCmplt+ RxOF- MalfTLP- ECRC- UnsupReq- ACSViol-
        //         UESvrt: DLP+ SDES+ TLP+ FCP+ CmpltTO- CmpltAbrt- UnxCmplt- RxOF+ MalfTLP+ ECRC+ UnsupReq- ACSViol-
        //         CESta:  RxErr- BadTLP- BadDLLP- Rollover- Timeout- NonFatalErr+
        //         CEMsk:  RxErr+ BadTLP+ BadDLLP+ Rollover+ Timeout+ NonFatalErr+
        //         AERCap: First Error Pointer: 00, GenCap+ CGenEn- ChkCap+ ChkEn-
        // Kernel driver in use: pcieport
        let data = [
            0x12, 0x19, 0x1d, 0x00, 0x07, 0x00, 0x10, 0x00, 0x00, 0x00, 0x04, 0x06, 0x00, 0x00, 0x01, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x05, 0x08, 0x00, 0xf1, 0x01, 0x00, 0x00,
            0x00, 0x92, 0x90, 0x92, 0x01, 0x91, 0xf1, 0x91, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x1b, 0x00,
        ];
        let result: Header = data.read_with(&mut 0, LE).unwrap();
        // println!("{:#04X?}", &result);
        let sample = Header {
            vendor_id: 0x1912,
            device_id: 0x001d,
            command: 0b0000_0000_0000_0111.into(),
            status: 0b0000_0000_0001_0000.into(),
            revision_id: 0x00,
            class_code: ClassCode {
                interface: 0x00,
                sub: 0x04,
                base: 0x06,
            },
            cache_line_size: 0,
            latency_timer: 0,
            capabilities_pointer: 0x40,
            bist: BuiltInSelfTest {
                is_capable: true,
                is_running: false,
                completion_code: 0x00,
            },
            is_multi_function: false,
            header_type: HeaderType::Bridge(Bridge {
                base_addresses: BaseAddressesBridge([0; 2]),
                primary_bus_number: 0x04,
                secondary_bus_number: 0x05,
                subordinate_bus_number: 0x08,
                secondary_latency_timer: 0x00,
                io_address_range: BridgeIoAddressRange::IoAddr32 { 
                    base: 0xf000,
                    limit: 0x0 // lspci define output as io_limit+0xfff
                },
                secondary_status: 0x0000.into(),
                memory_base: (0x92000000u32 >> 16) as u16,
                memory_limit: ((0x929fffffu32 - 0xfffff) >> 16) as u16,
                prefetchable_memory: BridgePrefetchableMemory::MemAddr64 {
                    base: 0x91000000,
                    limit: 0x91ffffff - 0xfffff,
                },
                expansion_rom: Default::default(),
                bridge_control: 0b0000_0000_0001_1011.into(),
            }),
            interrupt_line: 0xff, 
            interrupt_pin: InterruptPin::Unused,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn header_type_cardbus() {
        // Random data with some bytes fixed
        //
        // CardBus bridge [0607]: Device [df8e:05ee] (rev 37)
        // Subsystem: Device [3322:5544]
        // Control: I/O- Mem- BusMaster+ SpecCycle- MemWINV+ VGASnoop+ ParErr- Stepping+ SERR- FastB2B- DisINTx-
        // Status: Cap+ 66MHz+ UDF+ FastB2B- ParErr+ DEVSEL=medium >TAbort+ <TAbort- <MAbort- >SERR+ <PERR- INTx+
        // Latency: 41, Cache Line Size: 968 bytes
        // Interrupt: pin Z routed to IRQ 6
        // Region 0: Memory at 35f88000 (32-bit, non-prefetchable) [disabled]
        // Bus: primary=6d, secondary=ba, subordinate=fe, sec-latency=252
        // Memory window 0: 11f54000-22475fff [disabled] (prefetchable)
        // Memory window 1: 33853000-44d0cfff [disabled]
        // I/O window 0: 00000060-00000073 [disabled]
        // I/O window 1: 00060060-00070073 [disabled]
        // BridgeCtl: Parity+ SERR- ISA+ VGA- MAbort- >Reset+ 16bInt- PostWrite+
        // 16-bit legacy interface ports at 3322
        let data = [
            0x8e, 0xdf, 0xee, 0x05, 0xb4, 0x00, 0x78, 0x4b, 0x37, 0x00, 0x07, 0x06, 0xf2, 0x29, 0x82, 0x00,
            0x00, 0x80, 0xf8, 0x35, 0x80, 0x00, 0x00, 0x00, 0x6d, 0xba, 0xfe, 0xfc, 0x00, 0x40, 0xf5, 0x11,
            0x00, 0x50, 0x47, 0x22, 0x00, 0x30, 0x85, 0x33, 0x00, 0xc0, 0xd0, 0x44, 0x60, 0x00, 0x00, 0x00,
            0x70, 0x00, 0x00, 0x00, 0x61, 0x00, 0x06, 0x00, 0x70, 0x00, 0x07, 0x00, 0x06, 0x1a, 0x45, 0x05,
            0x22, 0x33, 0x44, 0x55, 0x22, 0x33, 0x00, 0x00,
        ];
        let result: Header = data.read_with(&mut 0, LE).unwrap();
        println!("{:02X?}", &data);
        let sample = Header {
            vendor_id: 0xdf8e,
            device_id: 0x05ee,
            command: 0b0000_0000_1011_0100.into(),
            status: 0b0100_1011_0111_1000.into(),
            revision_id: 0x37,
            class_code: ClassCode {
                interface: 0x00,
                sub: 0x07,
                base: 0x06,
            },
            cache_line_size: 0xf2,
            latency_timer: 41,
            capabilities_pointer: 0x80,
            is_multi_function: true,
            header_type: HeaderType::Cardbus(Cardbus {
                base_addresses: BaseAddressesCardbus([0x35f88000]),
                secondary_status: 0x0000.into(),
                pci_bus_number: 0x6d,
                cardbus_bus_number: 0xba,
                subordinate_bus_number: 0xfe,
                cardbus_latency_timer: 252,
                memory_base_address_0: 0x11f54000,
                memory_limit_address_0: 0x22475fff - 0xfff,
                memory_base_address_1: 0x33853000,
                memory_limit_address_1: 0x44d0cfff - 0xfff,
                io_access_address_range_0: IoAccessAddressRange::Addr16Bit {
                    base: 0x0060,
                    // Don't know why '+ 3' here https://github.com/pciutils/pciutils/blob/5bdf63b6b1bc35b59c4b3f47f7ca83ca1868155b/lspci.c#L683
                    limit: 0x0073 - 3,
                },
                io_access_address_range_1: IoAccessAddressRange::Addr32Bit {
                    base: 0x00060060,
                    // Don't know why '+ 3' here https://github.com/pciutils/pciutils/blob/5bdf63b6b1bc35b59c4b3f47f7ca83ca1868155b/lspci.c#L683
                    limit: 0x00070073 - 3,
                },
                bridge_control: 0b0000_0101_0100_0101.into(),
                subsystem_vendor_id: 0x3322,
                subsystem_device_id: 0x5544,
                legacy_mode_base_address: 0x3322,
            }),
            bist: BuiltInSelfTest {
                is_capable: false,
                is_running: false,
                completion_code: 0x00,
            },
            interrupt_line: 0x06, 
            interrupt_pin: InterruptPin::Reserved(0x1a),
        };
        assert_eq!(sample, result);
    }
}
