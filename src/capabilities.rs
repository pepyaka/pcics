/*!
## PCI Capabilities

Each Capability structure has a Capability ID assigned by the PCI-SIG.

Capabilities list
- [x] Null Capability (00h)
- [x] [PCI Power Management Interface](power_management_interface) (01h)
- [ ] AGP (02h)
- [x] [VPD](vital_product_data) (03h)
- [x] [Slot Identification](slot_identification) (04h)
- [x] [Message Signaled Interrupts](message_signaled_interrups) (05h)
- [x] CompactPCI Hot Swap (06h)
- [ ] PCI-X (07h)
- [x] [HyperTransport](hypertransport) (08h)
- [x] [Vendor Specific](vendor_specific) (09h)
- [x] [Debug port](debug_port) (0Ah)
- [x] CompactPCI central resource control (0Bh)
- [x] PCI Hot-Plug (0Ch)
- [x] [PCI Bridge Subsystem Vendor ID](bridge_subsystem_vendor_id) (0Dh)
- [x] AGP 8x (0Eh)
- [x] Secure Device (0Fh)
- [x] [PCI Express](pci_express) (10h)
- [x] [MSI-X](msi_x) (11h)
- [x] [Serial ATA Data/Index Configuration](sata) (12h)
- [x] [Advanced Features](advanced_features) (AF) (13h)
- [ ] Enhanced Allocation (14h)
- [ ] Flattening Portal Bridge (15h)

Others Reserved

## Example

lspci out:
```plaintext
Capabilities: [80] MSI: Enable+ Count=1/1 Maskable- 64bit-
        Address: fee00358  Data: 0000
Capabilities: [70] Power Management version 3
        Flags: PMEClk- DSI- D1- D2- AuxCurrent=0mA PME(D0-,D1-,D2-,D3hot+,D3cold-)
        Status: D0 NoSoftRst+ PME-Enable- DSel=0 DScale=0 PME-
Capabilities: [a8] SATA HBA v1.0 BAR4 Offset=00000004
```

pcics capabilities:
```rust
# use pcics::capabilities::{
#     Capabilities,
#     Capability,
#     CapabilityKind,
#     MessageSignaledInterrups,
#     PowerManagementInterface,
#     Sata,
#     message_signaled_interrups as msi,
#     power_management_interface as pmi,
#     sata,
# };

let device_dependent_region = [
    // 0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    0x00,0x80,0x00,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x40
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x50
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x60
    0x01,0xa8,0x03,0x40,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x70
    0x05,0x70,0x01,0x00,0x58,0x03,0xe0,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x80
    0x60,0x1c,0x23,0x83,0x83,0x01,0x00,0x1c,0x20,0x02,0x1c,0x20,0x20,0x00,0x00,0x80, // 0x90
    0xa8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x12,0x00,0x10,0x00,0x48,0x00,0x00,0x00, // 0xa0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xb0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xc0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xd0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xe0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xb1,0x0f,0x06,0x08,0x00,0x00,0x00,0x00, // 0xf0
];

let result = Capabilities::new(&device_dependent_region, 0x80)
    .collect::<Vec<_>>();

let sample = vec![
    Capability {
        pointer: 0x80,
        kind: CapabilityKind::MessageSignaledInterrups(MessageSignaledInterrups {
            message_control: msi::MessageControl {
                enable: true,
                multiple_message_capable: msi::NumberOfVectors::One,
                multiple_message_enable: msi::NumberOfVectors::One,
                per_vector_masking_capable: false,
                reserved: 0,
            },
            message_address: msi::MessageAddress::Dword(0xfee00358),
            message_data: 0x0000,
            reserved: 0,
            mask_bits: None,
            pending_bits: None,
        })
    },
    Capability {
        pointer: 0x70,
        kind: CapabilityKind::PowerManagementInterface(PowerManagementInterface {
            capabilities: pmi::Capabilities {
                version: 0b11,
                pme_clock: false,
                reserved: false,
                device_specific_initialization: false,
                aux_current: pmi::AuxCurrent::SelfPowered,
                d1_support: false,
                d2_support: false,
                pme_support: pmi::PmeSupport {
                    d0: false,
                    d1: false,
                    d2: false,
                    d3_hot: true,
                    d3_cold: false,
                },
            },
            control: pmi::Control {
                power_state: pmi::PowerState::D0,
                reserved: 0b000010,
                no_soft_reset: true,
                pme_enabled: false,
                data_select: pmi::DataSelect::PowerConsumedD0,
                data_scale: pmi::DataScale::Unknown,
                pme_status: false,
            },
            bridge: pmi::Bridge {
                reserved: 0,
                b2_b3: false,
                bpcc_enabled: false,
            },
            data: 0,
        })
    },
    Capability {
        pointer: 0xa8,
        kind: CapabilityKind::Sata(Sata {
            revision: sata::Revision { major: 1, minor: 0 },
            bar_offset: sata::BarOffset(0x00000004),
            bar_location: sata::BarLocation::Bar4,
        })
    },
];

assert_eq!(sample, result);
```
*/



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
pub mod slot_identification;
pub use slot_identification::SlotIdentification;

// 05h Message Signaled Interrupts
pub mod message_signaled_interrups;
pub use message_signaled_interrups::MessageSignaledInterrups;

/// 06h CompactPCI Hot Swap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactPciHotSwap;

// 07h PCI-X

// 08h HyperTransport
pub mod hypertransport;
pub use hypertransport::Hypertransport;

// 09h Vendor Specific
pub mod vendor_specific;
pub use vendor_specific::VendorSpecific;

// 0Ah Debug port
pub mod debug_port;
pub use debug_port::DebugPort;

/// 0Bh CompactPCI central resource control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactPciResourceControl;

/// PCI Hot-Plug
///
/// This ID indicates that the associated device conforms to the Standard Hot-Plug Controller model
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciHotPlug;

// 0Dh PCI Bridge Subsystem Vendor ID
pub mod bridge_subsystem_vendor_id;
pub use bridge_subsystem_vendor_id::BridgeSubsystemVendorId;

/// 0Eh AGP 8x
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agp8x;

/// 0Fh Secure Device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecureDevice;

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
            0x04 => data.read_with(offset, LE).map(CapabilityKind::SlotIdentification).ok()?,
            0x05 => data.read_with(offset, LE).map(CapabilityKind::MessageSignaledInterrups).ok()?,
            0x06 => CapabilityKind::CompactPciHotSwap(CompactPciHotSwap),
            0x08 => data.read_with(offset, LE).map(CapabilityKind::Hypertransport).ok()?,
            0x09 => data.read_with(offset, LE).map(CapabilityKind::VendorSpecific).ok()?,
            0x0a => data.read_with(offset, LE).map(CapabilityKind::DebugPort).ok()?,
            0x0b => CapabilityKind::CompactPciResourceControl(CompactPciResourceControl),
            0x0c => CapabilityKind::PciHotPlug(PciHotPlug),
            0x0d => data.read_with(offset, LE).map(CapabilityKind::BridgeSubsystemVendorId).ok()?,
            0x0f => CapabilityKind::SecureDevice(SecureDevice),
            0x10 => data.read_with(offset, LE).map(CapabilityKind::PciExpress).ok()?,
            0x11 => data.read_with(offset, LE).map(CapabilityKind::MsiX).ok()?,
            0x12 => data.read_with(offset, LE).map(CapabilityKind::Sata).ok()?,
            0x13 => data.read_with(offset, LE).map(CapabilityKind::AdvancedFeatures).ok()?,
            v => CapabilityKind::Reserved(v),
        };
        Some(Capability { pointer, kind })
    }
}

/// Capability structure
#[derive(Debug, PartialEq, Eq)]
pub struct Capability<'a> {
    pub pointer: u8,
    pub kind: CapabilityKind<'a>,
}

/// Capability ID assigned by the PCI-SIG
#[derive(Debug, PartialEq, Eq)]
pub enum CapabilityKind<'a> {
    /// 00h Null Capability
    ///
    /// This capability contains no registers. It may be present in any Function. Functions may
    /// contain multiple instances of this capability.
    NullCapability,
    /// 01h PCI Power Management Interface
    PowerManagementInterface(PowerManagementInterface),
    // AcceleratedGraphicsPort(AcceleratedGraphicsPort),
    VitalProductData(VitalProductData),
    SlotIdentification(SlotIdentification),
    MessageSignaledInterrups(MessageSignaledInterrups),
    CompactPciHotSwap(CompactPciHotSwap),
    // PciX(PciX),
    Hypertransport(Hypertransport),
    VendorSpecific(VendorSpecific<'a>),
    DebugPort(DebugPort),
    CompactPciResourceControl(CompactPciResourceControl),
    PciHotPlug(PciHotPlug),
    BridgeSubsystemVendorId(BridgeSubsystemVendorId),
    Agp8x(Agp8x),
    SecureDevice(SecureDevice),
    PciExpress(PciExpress),
    /// 11h MSI-X
    MsiX(MsiX),
    /// 12h Serial ATA Data/Index Configuration
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
        let sample = vec![
            Capability {
                pointer: 0x50,
                kind: CapabilityKind::PowerManagementInterface(
                    data.read_with(&mut (0x50 + 2), LE).unwrap()
                )
            },
            Capability {
                pointer: 0x80,
                kind: CapabilityKind::VendorSpecific(
                    data.read_with(&mut (0x80 + 2), LE).unwrap()
                )
            },
            Capability {
                pointer: 0x60,
                kind: CapabilityKind::MessageSignaledInterrups(
                    data.read_with(&mut (0x60 + 2), LE).unwrap()
                )
            },

        ];
        assert_eq!(sample, result);
    }
}
