/*!
PCI Express Extended Capabilities

PCI Express Extended Capability registers are located in Configuration Space at offsets 256 or
greater or in the Root Complex Register Block (RCRB). These registers when located in the
Configuration Space are accessible using only the PCI Express Enhanced Configuration Access
Mechanism (ECAM).

Extended Capabilities list:
- [x] Null Capability (0000h)
- [x] [Advanced Error Reporting](advanced_error_reporting) (AER) (0001h)
- [x] [Virtual Channel](virtual_channel) (VC) – used if an MFVC Extended Cap structure is not present in the device (0002h)
- [x] [Device Serial Number](device_serial_number) (0003h)
- [ ] Power Budgeting (0004h)
- [ ] Root Complex Link Declaration (0005h)
- [ ] Root Complex Internal Link Control (0006h)
- [ ] Root Complex Event Collector Endpoint Association (0007h)
- [ ] Multi-Function Virtual Channel (MFVC) (0008h)
- [ ] Virtual Channel (VC) – used if an MFVC Extended Cap structure is present in the device (0009h)
- [ ] Root Complex Register Block (RCRB) Header (000Ah)
- [x] [Vendor-Specific Extended Capability](vendor_specific_extended_capability) (VSEC) (000Bh)
- [ ] Configuration Access Correlation (CAC) (000Ch)
- [ ] Access Control Services (ACS) (000Dh)
- [x] [Alternative Routing-ID Interpretation](alternative_routing_id_interpolation) (ARI) (000Eh)
- [x] [Address Translation Services](address_translation_services) (ATS) (000Fh)
- [ ] Single Root I/O Virtualization (SR-IOV) (0010h)
- [ ] Multi-Root I/O Virtualization (MR-IOV) (0011h)
- [ ] Multicast (0012h)
- [ ] Page Request Interface (PRI) (0013h)
- [ ] Reserved for AMD (0014h)
- [ ] Resizable BAR (0015h)
- [ ] Dynamic Power Allocation (DPA) (0016h)
- [ ] TPH Requester (0017h)
- [x] [Latency Tolerance Reporting (LTR)](latency_tolerance_reporting) (0018h)
- [x] [Secondary PCI Express](secondary_pci_express) (0019h)
- [ ] Protocol Multiplexing (PMUX) (001Ah)
- [x] [Process Address Space ID](process_address_space_id) (PASID) (001Bh)
- [ ] LN Requester (LNR) (001Ch)
- [x] [Downstream Port Containment](downstream_port_containment) (DPC) (001Dh)
- [x] [L1 PM Substates](l1_pm_substates) (001Eh)
- [x] [Precision Time Measurement](precision_time_measurement) (PTM) (001Fh)
- [ ] PCI Express over M-PHY (M-PCIe) (0020h)
- [ ] FRS Queueing (0021h)
- [ ] Readiness Time Reporting (0022h)
- [ ] Designated Vendor-Specific Extended Capability (0023h)
- [ ] VF Resizable BAR (0024h)
- [ ] Data Link Feature (0025h)
- [ ] Physical Layer 16.0 GT/s (0026h)
- [ ] Lane Margining at the Receiver (0027h)
- [ ] Hierarchy ID (0028h)
- [ ] Native PCIe Enclosure Management (NPEM) (0029h)
- [ ] Physical Layer 32.0 GT/s (002Ah)
- [ ] Alternate Protocol (002Bh)
- [ ] System Firmware Intermediary (SFI) (002Ch)
Others Reserved

## Example
lspci out:
```plaintext
Capabilities: [100 v1] Device Serial Number 11-22-33-44-55-66-77-88
```
parsed exetended capabilities:
```rust
# use pcics::extended_capabilities::{
#     DeviceSerialNumber,
#     ExtendedCapabilities,
#     ExtendedCapability,
#     ExtendedCapabilityKind,
# };

let data = [
    // 0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    0x03,0x00,0x01,0x00,0xfe,0x85,0x54,0x45,0x00,0x50,0x56,0xff,0x00,0x00,0x00,0x00 // 0x100
];
let result = ExtendedCapabilities::new(&data)
    .collect::<Vec<_>>();
let sample = vec![
    ExtendedCapability {
        version: 1,
        offset: 0x100,
        kind: ExtendedCapabilityKind::DeviceSerialNumber(DeviceSerialNumber {
            lower_dword: 0x88776655,
            upper_dword: 0x44332211,
        })
    },
];
```
*/



use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    BytesExt,
};

use super::ECS_OFFSET;

/// Extended Capability Header length in bytes
pub const ECH_BYTES: usize = 4;



#[bitfield(bits = 32)]
#[repr(u32)]
pub struct ExtendedCapabilityHeaderProto {
    id: u16,
    version: B4,
    offset: B12,
}

/// An iterator through *Extended Capabilities List*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtendedCapabilities<'a> {
    /// Extended Configuration Space
    ecs: &'a [u8],
    offset: u16,
}
impl<'a> ExtendedCapabilities<'a> {
    pub fn new(ecs: &'a [u8]) -> Self {
        Self { ecs, offset: 0x100 }
    }
}
impl<'a> Iterator for ExtendedCapabilities<'a> {
    type Item = ExtendedCapability<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == 0 {
            return None;
        }
        let offset = self.offset;
        let bytes = &self.ecs;
        let ecs_offset = &mut usize::from(self.offset).checked_sub(ECS_OFFSET)?;
        let header = bytes.read_with::<u32>(ecs_offset, LE).ok()?;
        if header == 0 {
            return None;
        }
        let header: ExtendedCapabilityHeaderProto = header.into();
        use ExtendedCapabilityKind::*;
        let kind = match header.id() {
            0x0000 => Null,
            0x0001 => AdvancedErrorReporting(bytes.read_with(ecs_offset, LE).ok()?),
            0x0002 => VirtualChannel(bytes.read_with(ecs_offset, LE).ok()?),
            0x0003 => DeviceSerialNumber(bytes.read_with(ecs_offset, LE).ok()?),
            0x0004 => PowerBudgeting(bytes.read_with(ecs_offset, LE).ok()?),
            0x0005 => RootComplexLinkDeclaration,
            0x0006 => RootComplexInternalLinkControl,
            0x0007 => RootComplexEventCollectorEndpointAssociation,
            0x0008 => MultiFunctionVirtualChannel,
            0x0009 => VirtualChannelMfvcPresent,
            0x000A => RootComplexRegisterBlock,
            0x000B => VendorSpecificExtendedCapability(bytes.read_with(ecs_offset, LE).ok()?),
            0x000C => ConfigurationAccessCorrelation,
            0x000D => AccessControlServices(bytes.read_with(ecs_offset, LE).ok()?),
            0x000E => AlternativeRoutingIdInterpretation(bytes.read_with(ecs_offset, LE).ok()?),
            0x000F => AddressTranslationServices(bytes.read_with(ecs_offset, LE).ok()?),
            0x0010 => SingleRootIoVirtualization,
            0x0011 => MultiRootIoVirtualization,
            0x0012 => Multicast,
            0x0013 => PageRequestInterface(bytes.read_with(ecs_offset, LE).ok()?),
            0x0014 => AmdReserved,
            0x0015 => ResizableBar,
            0x0016 => DynamicPowerAllocation,
            0x0017 => TphRequester(bytes.read_with(ecs_offset, LE).ok()?),
            0x0018 => LatencyToleranceReporting(bytes.read_with(ecs_offset, LE).ok()?),
            0x0019 => SecondaryPciExpress(bytes.read_with(ecs_offset, LE).ok()?),
            0x001A => ProtocolMultiplexing,
            0x001B => ProcessAddressSpaceId(bytes.read_with(ecs_offset, LE).ok()?),
            0x001C => LnRequester,
            0x001D => DownstreamPortContainment(bytes.read_with(ecs_offset, LE).ok()?),
            0x001E => L1PmSubstates(bytes.read_with(ecs_offset, LE).ok()?),
            0x001F => PrecisionTimeMeasurement(bytes.read_with(ecs_offset, LE).ok()?),
            0x0020 => PciExpressOverMphy,
            0x0021 => FrsQueueing,
            0x0022 => ReadinessTimeReporting,
            0x0023 => DesignatedVendorSpecificExtendedCapability,
            0x0024 => VFResizableBar,
            0x0025 => DataLinkFeature,
            0x0026 => PhysicalLayer16GTps,
            0x0027 => ReceiverLaneMargining,
            0x0028 => HierarchyId,
            0x0029 => NativePcieEnclosureManagement,
            0x002A => PhysicalLayer32GTps,
            0x002B => AlternateProtocol,
            0x002C => SystemFirmwareIntermediary,
                 v => Reserved(v),
        };
        let ecap = ExtendedCapability { kind, version: header.version(), offset, };
        self.offset = header.offset();
        Some(ecap)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedCapability<'a> {
    pub kind: ExtendedCapabilityKind<'a>,
    pub version: u8,
    pub offset: u16,
}
impl<'a> ExtendedCapability<'a> {
    pub fn id(&self) -> u16 {
        match self.kind {
            ExtendedCapabilityKind::Null => 0x0000,
            ExtendedCapabilityKind::AdvancedErrorReporting(_) => 0x0001,
            ExtendedCapabilityKind::VirtualChannel(_) => 0x0002,
            ExtendedCapabilityKind::DeviceSerialNumber(_) => 0x0003,
            ExtendedCapabilityKind::PowerBudgeting(_) => 0x0004,
            ExtendedCapabilityKind::RootComplexLinkDeclaration => 0x0005,
            ExtendedCapabilityKind::RootComplexInternalLinkControl => 0x0006,
            ExtendedCapabilityKind::RootComplexEventCollectorEndpointAssociation => 0x0007,
            ExtendedCapabilityKind::MultiFunctionVirtualChannel => 0x0008,
            ExtendedCapabilityKind::VirtualChannelMfvcPresent => 0x0009,
            ExtendedCapabilityKind::RootComplexRegisterBlock => 0x000A,
            ExtendedCapabilityKind::VendorSpecificExtendedCapability(_) => 0x000B,
            ExtendedCapabilityKind::ConfigurationAccessCorrelation => 0x000C,
            ExtendedCapabilityKind::AccessControlServices(_) => 0x000D,
            ExtendedCapabilityKind::AlternativeRoutingIdInterpretation(_) => 0x000E,
            ExtendedCapabilityKind::AddressTranslationServices(_) => 0x000F,
            ExtendedCapabilityKind::SingleRootIoVirtualization => 0x0010,
            ExtendedCapabilityKind::MultiRootIoVirtualization => 0x0011,
            ExtendedCapabilityKind::Multicast => 0x0012,
            ExtendedCapabilityKind::PageRequestInterface(_) => 0x0013,
            ExtendedCapabilityKind::AmdReserved => 0x0014,
            ExtendedCapabilityKind::ResizableBar => 0x0015,
            ExtendedCapabilityKind::DynamicPowerAllocation => 0x0016,
            ExtendedCapabilityKind::TphRequester(_) => 0x0017,
            ExtendedCapabilityKind::LatencyToleranceReporting(_) => 0x0018,
            ExtendedCapabilityKind::SecondaryPciExpress(_) => 0x0019,
            ExtendedCapabilityKind::ProtocolMultiplexing => 0x001A,
            ExtendedCapabilityKind::ProcessAddressSpaceId(_) => 0x001B,
            ExtendedCapabilityKind::LnRequester => 0x001C,
            ExtendedCapabilityKind::DownstreamPortContainment(_) => 0x001D,
            ExtendedCapabilityKind::L1PmSubstates(_) => 0x001E,
            ExtendedCapabilityKind::PrecisionTimeMeasurement(_) => 0x001F,
            ExtendedCapabilityKind::PciExpressOverMphy => 0x0020,
            ExtendedCapabilityKind::FrsQueueing => 0x0021,
            ExtendedCapabilityKind::ReadinessTimeReporting => 0x0022,
            ExtendedCapabilityKind::DesignatedVendorSpecificExtendedCapability => 0x0023,
            ExtendedCapabilityKind::VFResizableBar => 0x0024,
            ExtendedCapabilityKind::DataLinkFeature => 0x0025,
            ExtendedCapabilityKind::PhysicalLayer16GTps => 0x0026,
            ExtendedCapabilityKind::ReceiverLaneMargining => 0x0027,
            ExtendedCapabilityKind::HierarchyId => 0x0028,
            ExtendedCapabilityKind::NativePcieEnclosureManagement => 0x0029,
            ExtendedCapabilityKind::PhysicalLayer32GTps => 0x002A,
            ExtendedCapabilityKind::AlternateProtocol => 0x002B,
            ExtendedCapabilityKind::SystemFirmwareIntermediary => 0x002C,
            ExtendedCapabilityKind::Reserved(v) => v,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendedCapabilityKind<'a> {
    /// Null Capability – This capability contains no registers other than those in the Extended
    /// Capability Header
    Null,
    /// Advanced Error Reporting (AER)
    AdvancedErrorReporting(AdvancedErrorReporting),
    /// Virtual Channel (VC) – used if an MFVC Extended Cap structure is not present in the device
    VirtualChannel(VirtualChannel<'a>),
    /// Device Serial Number
    DeviceSerialNumber(DeviceSerialNumber),
    /// Power Budgeting
    PowerBudgeting(PowerBudgeting),
    /// Root Complex Link Declaration
    RootComplexLinkDeclaration,
    /// Root Complex Internal Link Control
    RootComplexInternalLinkControl,
    /// Root Complex Event Collector Endpoint Association
    RootComplexEventCollectorEndpointAssociation,
    /// Multi-Function Virtual Channel (MFVC)
    MultiFunctionVirtualChannel,
    /// Virtual Channel (VC) – used if an MFVC Extended Cap structure is present in the device
    VirtualChannelMfvcPresent,
    /// Root Complex Register Block (RCRB) Header
    RootComplexRegisterBlock,
    /// Vendor-Specific Extended Capability (VSEC)
    VendorSpecificExtendedCapability(VendorSpecificExtendedCapability<'a>),
    /// Configuration Access Correlation (CAC) – defined by the Trusted Configuration Space (TCS)
    /// for PCI Express ECN, which is no longer supported
    ConfigurationAccessCorrelation,
    /// Access Control Services (ACS)
    AccessControlServices(AccessControlServices<'a>),
    /// Alternative Routing-ID Interpretation (ARI)
    AlternativeRoutingIdInterpretation(AlternativeRoutingIdInterpretation),
    /// Address Translation Services (ATS)
    AddressTranslationServices(AddressTranslationServices),
    /// Single Root I/O Virtualization (SR-IOV)
    SingleRootIoVirtualization,
    /// Multi-Root I/O Virtualization (MR-IOV) – defined in the Multi-Root I/O Virtualization and
    /// Sharing Specification
    MultiRootIoVirtualization,
    /// Multicast
    Multicast,
    /// Page Request Interface (PRI)
    PageRequestInterface(PageRequestInterface),
    /// Reserved for AMD
    AmdReserved,
    /// Resizable BAR
    ResizableBar,
    /// Dynamic Power Allocation (DPA)
    DynamicPowerAllocation,
    /// TPH Requester
    TphRequester(TphRequester<'a>),
    /// Latency Tolerance Reporting (LTR)
    LatencyToleranceReporting(LatencyToleranceReporting),
    /// Secondary PCI Express
    SecondaryPciExpress(SecondaryPciExpress<'a>),
    /// Protocol Multiplexing (PMUX)
    ProtocolMultiplexing,
    /// Process Address Space ID (PASID)
    ProcessAddressSpaceId(ProcessAddressSpaceId),
    /// LN Requester (LNR)
    LnRequester,
    /// Downstream Port Containment (DPC)
    DownstreamPortContainment(DownstreamPortContainment),
    /// L1 PM Substates
    L1PmSubstates(L1PmSubstates),
    /// Precision Time Measurement (PTM)
    PrecisionTimeMeasurement(PrecisionTimeMeasurement),
    /// PCI Express over M-PHY (M-PCIe)
    PciExpressOverMphy,
    /// FRS Queueing
    FrsQueueing,
    /// Readiness Time Reporting
    ReadinessTimeReporting,
    /// Designated Vendor-Specific Extended Capability
    DesignatedVendorSpecificExtendedCapability,
    /// VF Resizable BAR
    VFResizableBar,
    /// Data Link Feature
    DataLinkFeature,
    /// Physical Layer 16.0 GT/s
    PhysicalLayer16GTps,
    /// Lane Margining at the Receiver
    ReceiverLaneMargining,
    /// Hierarchy ID
    HierarchyId,
    /// Native PCIe Enclosure Management (NPEM)
    NativePcieEnclosureManagement,
    /// Physical Layer 32.0 GT/s
    PhysicalLayer32GTps,
    /// Alternate Protocol
    AlternateProtocol,
    /// System Firmware Intermediary (SFI)
    SystemFirmwareIntermediary,
    Reserved(u16),
}

// 0001h Advanced Error Reporting (AER)
pub mod advanced_error_reporting;
pub use advanced_error_reporting::AdvancedErrorReporting;

// 0002h Virtual Channel (VC)
pub mod virtual_channel;
pub use virtual_channel::VirtualChannel;

// 0003h Device Serial Number
pub mod device_serial_number;
pub use device_serial_number::DeviceSerialNumber;

// 0004h Power Budgeting
pub mod power_budgeting;
pub use power_budgeting::PowerBudgeting;

// 000Bh Vendor-Specific Extended Capability (VSEC)
pub mod vendor_specific_extended_capability;
pub use vendor_specific_extended_capability::VendorSpecificExtendedCapability;

// 000Dh Access Control Services (ACS)
pub mod access_control_services;
pub use access_control_services::{AccessControlServices, EgressControlVectors};

// 000Eh Alternative Routing-ID Interpretation (ARI)
pub mod alternative_routing_id_interpolation;
pub use alternative_routing_id_interpolation::AlternativeRoutingIdInterpretation;

// 000Fh Address Translation Services (ATS)
pub mod address_translation_services;
pub use address_translation_services::AddressTranslationServices;

// 0013h Page Request Interface (PRI)
pub mod page_request_interface;
pub use page_request_interface::PageRequestInterface;

// 0017h TPH Requester
pub mod tph_requester;
pub use tph_requester::TphRequester;

// 0018h Latency Tolerance Reporting (LTR)
pub mod latency_tolerance_reporting;
pub use latency_tolerance_reporting::LatencyToleranceReporting;

// 0019h Secondary PCI Express
pub mod secondary_pci_express;
pub use secondary_pci_express::SecondaryPciExpress;

// 001Bh Process Address Space ID (PASID)
pub mod process_address_space_id;
pub use process_address_space_id::ProcessAddressSpaceId;

// 001Dh Downstream Port Containment (DPC)
pub mod downstream_port_containment;
pub use downstream_port_containment::DownstreamPortContainment;

// 001Eh L1 PM Substates
pub mod l1_pm_substates;
pub use l1_pm_substates::L1PmSubstates;

// 001Fh Precision Time Measurement (PTM)
pub mod precision_time_measurement;
pub use precision_time_measurement::PrecisionTimeMeasurement;



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn iterator() {
        // Capabilities: [100] Vendor Specific Information: ID=0002 Rev=0 Len=00c <?>
        // Capabilities: [110] Access Control Services
        // Capabilities: [148] Advanced Error Reporting
        // Capabilities: [1d0] Vendor Specific Information: ID=0003 Rev=1 Len=00a <?>
        // Capabilities: [250] Secondary PCI Express
        // Capabilities: [280] Vendor Specific Information: ID=0005 Rev=3 Len=018 <?>
        // Capabilities: [298] Vendor Specific Information: ID=0007 Rev=0 Len=024 <?>
        // Capabilities: [300] Vendor Specific Information: ID=0008 Rev=0 Len=038 <?>
        let ecaps = ExtendedCapabilities::new(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
                "/tests/data/device/8086:2030/config"
            ))
            [ECS_OFFSET..].try_into().unwrap()
        );
        let sample = vec![
            (0x100, 0x000b),
            (0x110, 0x000d),
            (0x148, 0x0001),
            (0x1d0, 0x000b),
            (0x250, 0x0019),
            (0x280, 0x000b),
            (0x298, 0x000b),
            (0x300, 0x000b),
        ];
        let result = ecaps.map(|ecap| (ecap.offset, ecap.id()))
            .collect::<Vec<_>>();
        assert_eq!(sample, result);
    }
}
