#![doc = include_str!("../README.md")]

pub mod header;
pub use header::Header;

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
