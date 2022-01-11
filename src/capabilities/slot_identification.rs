//! Slot Identification
//!
//! This Capability structure identifies a bridge that provides external expansion capabilities

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


/// Slot Identification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotIdentification {
    expansion_slot: ExpansionSlot,
    /// Contains the physical chassis number for the slots on this bridge’s secondary interface
    chassis_number: u8,
}
impl<'a> TryRead<'a, Endian> for SlotIdentification {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let si = SlotIdentification {
            expansion_slot: bytes.read_with::<u8>(offset, endian)?.into(),
            chassis_number: bytes.read_with::<u8>(offset, endian)?,
        };
        Ok((si, *offset))
    }
}

#[bitfield(bits = 8)]
#[repr(u8)]
pub struct ExpansionSlotProto {
    expansion_slots_provided: B5,
    first_in_chassis: bool,
    rsvdp: B2,
}

/// Provides information used by system software in calculating the slot number of a device plugged
/// into a PCI slot in an expansion chassis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpansionSlot {
    /// Number of PCI expansion slots located directly on the secondary interface of this bridge
    expansion_slots_provided: u8,
    /// Indicates that this bridge is the first in an expansion chassis
    first_in_chassis: bool,
}
impl From<ExpansionSlotProto> for ExpansionSlot {
    fn from(proto: ExpansionSlotProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            expansion_slots_provided: proto.expansion_slots_provided(),
            first_in_chassis: proto.first_in_chassis(),
        }
    }
}
impl From<u8> for ExpansionSlot {
    fn from(byte: u8) -> Self { ExpansionSlotProto::from(byte).into() }
}
