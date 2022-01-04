//! PCI Capabilities
//! 
//! Each Capability structure has a Capability ID assigned by the PCI-SIG.
//!
//! Capabilities list
//! - [x] Null Capability (00h)
//! - [x] [PCI Power Management Interface](power_management_interface) (01h)
//! - [ ] AGP (02h)
//! - [x] [VPD](vital_product_data) (03h)
//! - [ ] Slot Identification (04h)
//! - [x] [Message Signaled Interrupts](message_signaled_interrups) (05h)
//! - [ ] CompactPCI Hot Swap (06h)
//! - [ ] PCI-X (07h)
//! - [x] [HyperTransport](Hypertransport) (08h)
//! - [x] [Vendor Specific](vendor_specific) (09h)
//! - [ ] Debug port (0Ah)
//! - [ ] CompactPCI central resource control (0Bh)
//! - [ ] PCI Hot-Plug (0Ch)
//! - [ ] PCI Bridge Subsystem Vendor ID (0Dh)
//! - [ ] AGP 8x (0Eh)
//! - [ ] Secure Device (0Fh)
//! - [x] [PCI Express](pci_express) (10h)
//! - [x] [MSI-X](msi_x) (11h)
//! - [x] [Serial ATA Data/Index Configuration](sata) (12h)
//! - [x] [Advanced Features](advanced_features) (AF) (13h)
//! - [ ] Enhanced Allocation (14h)
//! - [ ] Flattening Portal Bridge (15h)
//! 
//! Others Reserved

use byte::{
    ctx::LE,
    // TryRead,
    // TryWrite,
    BytesExt,
};

use super::DDR_OFFSET;

/// Each capability in the capability list consists of an 8-bit ID field assigned by the PCI SIG,
/// an 8 bit pointer in configuration space to the next capability.
pub const CAP_HEADER_LEN: usize = 2;

// 01h PCI Power Management Interface
pub mod power_management_interface;
pub use power_management_interface::PowerManagementInterface;

// 02h AGP

// 03h VPD
pub mod vital_product_data;
pub use vital_product_data::VitalProductData;

// 04h Slot Identification

// 05h Message Signaled Interrupts
pub mod message_signaled_interrups;
pub use message_signaled_interrups::MessageSignaledInterrups;

// 06h CompactPCI Hot Swap

// 07h PCI-X

// 08h HyperTransport
pub mod hypertransport;
pub use hypertransport::Hypertransport;

// 09h Vendor Specific
pub mod vendor_specific;
pub use vendor_specific::VendorSpecific;

// 0Ah Debug port

// 0Bh CompactPCI central resource control

// 0Ch PCI Hot-Plug

// 0Dh PCI Bridge Subsystem Vendor ID
pub mod bridge_subsystem_vendor_id;
pub use bridge_subsystem_vendor_id::BridgeSubsystemVendorId;

// 0Eh AGP 8x

// 0Fh Secure Device

// 10h PCI Express
pub mod pci_express;
pub use pci_express::PciExpress;

// 11h MSI-X
pub mod msi_x;
pub use msi_x::MsiX;

// 12h Serial ATA Data/Index Configuration
pub mod sata;
pub use sata::Sata;

// 13h Advanced Features (AF)
pub mod advanced_features;
pub use advanced_features::AdvancedFeatures;

// 14h Enhanced Allocation

// 15h Flattening Portal Bridge



/// An iterator through *Capabilities List*
///
/// Used to point to a linked list of new capabilities implemented by this device. This
/// register is only valid if the “Capabilities List” bit in the [crate::header::Status] Register is set. If
/// implemented, the bottom two bits are reserved and should be set to 00b.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities<'a> {
    data: &'a [u8],
    pointer: u8,
}
impl<'a> Capabilities<'a> {
    pub fn new(data: &'a [u8], pointer: u8) -> Self {
        Self { data, pointer }
    }
}
impl<'a> Iterator for Capabilities<'a> {
    type Item = Capability<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop iterating if next pointer is null
        if self.pointer == 0 {
            return None;
        }
        let pointer = self.pointer;
        let data = &self.data;
        // Capability data resides in Device dependent region (0x34 offset)
        let offset = &mut usize::from(pointer).checked_sub(DDR_OFFSET)?;
        // 8-bit ID field assigned by the PCI SIG
        let cap_id = data.read_with::<u8>(offset, LE).ok()?;
        // an 8 bit pointer in configuration space to the next capability
        self.pointer = data.read_with::<u8>(offset, LE).ok()?;
        let kind = match cap_id {
            0x00 => CapabilityKind::NullCapability,
            0x01 => data.read_with(offset, LE).map(CapabilityKind::PowerManagementInterface).ok()?,
            0x03 => data.read_with(offset, LE).map(CapabilityKind::VitalProductData).ok()?,
            0x05 => data.read_with(offset, LE).map(CapabilityKind::MessageSignaledInterrups).ok()?,
            0x08 => data.read_with(offset, LE).map(CapabilityKind::Hypertransport).ok()?,
            0x09 => data.read_with(offset, LE).map(CapabilityKind::VendorSpecific).ok()?,
            0x0d => data.read_with(offset, LE).map(CapabilityKind::BridgeSubsystemVendorId).ok()?,
            0x10 => data.read_with(offset, LE).map(CapabilityKind::PciExpress).ok()?,
            0x11 => data.read_with(offset, LE).map(CapabilityKind::MsiX).ok()?,
            0x12 => data.read_with(offset, LE).map(CapabilityKind::Sata).ok()?,
            0x13 => data.read_with(offset, LE).map(CapabilityKind::AdvancedFeatures).ok()?,
            v => CapabilityKind::Reserved(v),
        };
        Some(Capability { pointer, kind })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Capability<'a> {
    pub pointer: u8,
    pub kind: CapabilityKind<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CapabilityKind<'a> {
    /// Null Capability
    ///
    /// This capability contains no registers. It may be present in any Function. Functions may
    /// contain multiple instances of this capability.
    NullCapability,
    PowerManagementInterface(PowerManagementInterface),
    // AcceleratedGraphicsPort(AcceleratedGraphicsPort),
    VitalProductData(VitalProductData),
    // SlotIndetification(SlotIndetification),
    MessageSignaledInterrups(MessageSignaledInterrups),
    // CompactPciHotSwap(CompactPciHotSwap),
    // PciX(PciX),
    Hypertransport(Hypertransport),
    VendorSpecific(VendorSpecific<'a>),
    // DebugPort(DebugPort),
    // CompactPciResourceControl(CompactPciResourceControl),
    // PciHotPlug(PciHotPlug),
    BridgeSubsystemVendorId(BridgeSubsystemVendorId),
    // AcceleratedGraphicsPort8X(AcceleratedGraphicsPort8X),
    // SecureDevice(SecureDevice),
    PciExpress(PciExpress),
    MsiX(MsiX),
    Sata(Sata),
    AdvancedFeatures(AdvancedFeatures),
    Reserved(u8),
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::ECS_OFFSET;

    use super::*;
    #[test]
    fn capabilities() {
        // Capabilities: [50] Power Management version 3
        //         Flags: PMEClk- DSI- D1- D2- AuxCurrent=55mA PME(D0-,D1-,D2-,D3hot+,D3cold+)
        //         Status: D0 NoSoftRst+ PME-Enable- DSel=0 DScale=0 PME-
        // Capabilities: [80] Vendor Specific Information: Len=14 <?>
        // Capabilities: [60] MSI: Enable+ Count=1/1 Maskable- 64bit+
        //         Address: 00000000fee00578  Data: 0000
        let data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/device/8086:9dc8/config"));
        let ddr = &data[DDR_OFFSET..ECS_OFFSET];
        let offset = data[0x34];
        let result = Capabilities::new(ddr, offset).collect::<Vec<_>>();
        use CapabilityKind::*;
        let sample = vec![
            Capability { 
                pointer: 0x50, 
                kind: PowerManagementInterface(data.read_with(&mut (0x50 + 2), LE).unwrap()) 
            },
            Capability { 
                pointer: 0x80, 
                kind: VendorSpecific(data.read_with(&mut (0x80 + 2), LE).unwrap()) 
            },
            Capability { 
                pointer: 0x60, 
                kind: MessageSignaledInterrups(data.read_with(&mut (0x60 + 2), LE).unwrap()) 
            },

        ];
        assert_eq!(sample, result);
    }
}
