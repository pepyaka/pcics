#![doc = include_str!("../README.md")]

//! #### Design
//!
//! Library has 3 significant modules
//! - [PCI 3.0 Compatible Configuration Space Header](header)
//! - [PCI Configuration Space Capabilities](capabilities)
//! - [Extended Configuration Space Capabilities](extended_capabilities)
//!
//! ### Usage
//!
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
//! # use pretty_assertions::assert_eq;
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


pub mod header;
pub use header::{Header, HeaderType};

pub mod capabilities;
pub use capabilities::Capabilities;

pub mod extended_capabilities;
pub use extended_capabilities::ExtendedCapabilities;


/// Device dependent region starts at 0x40 offset
pub const DDR_OFFSET: usize = 0x40;
/// Extended configuration space starts at 0x100 offset
pub const ECS_OFFSET: usize = 0x100;
/// Device dependent region length
pub const DDR_LENGTH: usize = ECS_OFFSET - DDR_OFFSET;
/// Extended configuration space length
pub const ECS_LENGTH: usize = 4096 - ECS_OFFSET;


