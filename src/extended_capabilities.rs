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
- [x] [Power Budgeting](power_budgeting) (0004h)
- [ ] Root Complex Link Declaration (0005h)
- [ ] Root Complex Internal Link Control (0006h)
- [ ] Root Complex Event Collector Endpoint Association (0007h)
- [ ] Multi-Function Virtual Channel (MFVC) (0008h)
- [x] [Virtual Channel](virtual_channel) (VC) – used if an MFVC Extended Cap structure is present in the device (0009h)
- [ ] Root Complex Register Block (RCRB) Header (000Ah)
- [x] [Vendor-Specific Extended Capability](vendor_specific_extended_capability) (VSEC) (000Bh)
- [ ] Configuration Access Correlation (CAC) (000Ch)
- [x] [Access Control Services](access_control_services) (ACS) (000Dh)
- [x] [Alternative Routing-ID Interpretation](alternative_routing_id_interpolation) (ARI) (000Eh)
- [x] [Address Translation Services](address_translation_services) (ATS) (000Fh)
- [x] [Single Root I/O Virtualization](single_root_io_virtualization) (SR-IOV) (0010h)
- [ ] Multi-Root I/O Virtualization (MR-IOV) (0011h)
- [ ] Multicast (0012h)
- [x] [Page Request Interface](page_request_interface) (PRI) (0013h)
- [ ] Reserved for AMD (0014h)
- [ ] Resizable BAR (0015h)
- [ ] Dynamic Power Allocation (DPA) (0016h)
- [x] [TPH Requester](tph_requester) (0017h)
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
pcics extended capabilities:
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



use byte::{
    ctx::*,
    BytesExt,
};
use heterob::{P3, bit_numbering::LsbInto};
use snafu::prelude::*;

use super::ECS_OFFSET;

/// Extended Capability Header length in bytes
pub const ECH_BYTES: usize = 4;


/// Extended capability parsing error
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum ExtendedCapabilityError {
    #[snafu(display("extended capability offset should be greater than 0xFF"))]
    Offset,
    #[snafu(display("[{offset:03x}] extended capability header shorter than u32"))]
    Header { offset: u16 },
    #[snafu(display("[{offset:03x}] extended capability has empty header"))]
    EmptyHeader { offset: u16 },
    #[snafu(display("`byte` crate error"))]
    ByteCrate { err: byte::Error },
    #[snafu(display("[{offset:03x}] {source} data read error"))]
    Data { offset: u16, source: ExtendedCapabilityDataError },
    #[snafu(display("[{offset:03x}] Root Complex Link Declaration error: {source}"))]
    RootComplexLinkDeclaration {
        offset: u16,
        source: root_complex_link_declaration::RootComplexLinkDeclarationError,
    },
    #[snafu(display("[{offset:03x}] Single Root I/O Virtualization error: {source}"))]
    SingleRootIoVirtualization {
        offset: u16,
        source: single_root_io_virtualization::SingleRootIoVirtualizationError,
    },
    #[snafu(display("[{offset:03x}] Advanced Error Reporting error: {source}"))]
    AdvancedErrorReporting {
        offset: u16,
        source: advanced_error_reporting::AdvancedErrorReportingError,
    },
}
impl From<byte::Error> for ExtendedCapabilityError {
    fn from(be: byte::Error) -> Self {
        Self::ByteCrate { err: be }
    }
}

/// Common error for reading capability data
#[derive(Snafu, Debug, Clone, Copy, PartialEq, Eq)]
#[snafu(display("{name} ({size} bytes)"))]
pub struct ExtendedCapabilityDataError {
    name: &'static str,
    size: usize,
}

/// An iterator through *Extended Capabilities List*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtendedCapabilities<'a> {
    /// Extended Configuration Space
    ecs: &'a [u8],
    /// PCI Configuration Space offset
    next_capability_offset: u16,
}
impl<'a> ExtendedCapabilities<'a> {
    pub fn new(ecs: &'a [u8]) -> Self {
        Self { ecs, next_capability_offset: ECS_OFFSET as u16 }
    }
}
impl<'a> Iterator for ExtendedCapabilities<'a> {
    type Item = ExtendedCapabilityResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_capability_offset == 0 {
            return None;
        }
        match parse_ecap(self.ecs, &mut self.next_capability_offset) {
            Err(ExtendedCapabilityError::EmptyHeader { .. }) => None,
            v => Some(v),
        }
    }
}

type ExtendedCapabilityResult<'a> = Result<ExtendedCapability<'a>, ExtendedCapabilityError>;
fn parse_ecap<'a>(bytes: &'a [u8], next_capability_offset: &mut u16) -> ExtendedCapabilityResult<'a> {
        let offset = *next_capability_offset;
        let ecs_offset = (offset as usize).checked_sub(ECS_OFFSET)
            .ok_or_else(|| {
                *next_capability_offset = 0;
                ExtendedCapabilityError::Offset
            })?;
        let ecap_data_offset = ecs_offset + ECH_BYTES;
        let dword = &bytes.get(ecs_offset .. ecap_data_offset)
            // We can use unwrap on already length checked slice
            .map(|slice| u32::from_le_bytes(slice.try_into().unwrap()))
            .ok_or_else(|| {
                *next_capability_offset = 0;
                ExtendedCapabilityError::Header { offset }
            })?;
        if *dword == 0 {
            return Err(ExtendedCapabilityError::EmptyHeader { offset });
        }
        let (id, version, next_cap_offset) = P3::<_, 16, 4, 12>(*dword).lsb_into();
        *next_capability_offset = next_cap_offset;

        let ecap_data = &bytes[ecap_data_offset..];
        let ecap_data_offset = &mut ecap_data_offset.clone(); // ecap u32 sized

        use ExtendedCapabilityKind as Kind;
        let kind = match id {
            0x0000 => Kind::Null,
            0x0001 => ecap_data.try_into().map(Kind::AdvancedErrorReporting)
                        .context(AdvancedErrorReportingSnafu { offset })?,
            0x0002 => Kind::VirtualChannel(bytes.read_with(ecap_data_offset, LE)?),
            0x0003 => ecap_data.try_into().map(Kind::DeviceSerialNumber)
                        .context(DataSnafu { offset })?,
            0x0004 => ecap_data.try_into().map(Kind::PowerBudgeting)
                        .context(DataSnafu { offset })?,
            0x0005 => ecap_data.try_into().map(Kind::RootComplexLinkDeclaration)
                        .context(RootComplexLinkDeclarationSnafu { offset })?,
            0x0006 => Kind::RootComplexInternalLinkControl,
            0x0007 => Kind::RootComplexEventCollectorEndpointAssociation,
            0x0008 => Kind::MultiFunctionVirtualChannel,
            0x0009 => Kind::VirtualChannelMfvcPresent(bytes.read_with(ecap_data_offset, LE)?),
            0x000A => Kind::RootComplexRegisterBlock,
            0x000B => Kind::VendorSpecificExtendedCapability(bytes.read_with(ecap_data_offset, LE)?),
            0x000C => Kind::ConfigurationAccessCorrelation,
            0x000D => Kind::AccessControlServices(bytes.read_with(ecap_data_offset, LE)?),
            0x000E => Kind::AlternativeRoutingIdInterpretation(bytes.read_with(ecap_data_offset, LE)?),
            0x000F => Kind::AddressTranslationServices(bytes.read_with(ecap_data_offset, LE)?),
            0x0010 => ecap_data.try_into().map(Kind::SingleRootIoVirtualization)
                        .context(DataSnafu { offset })?,
            0x0011 => Kind::MultiRootIoVirtualization,
            0x0012 => Kind::Multicast,
            0x0013 => ecap_data.try_into().map(Kind::PageRequestInterface)
                        .context(DataSnafu { offset })?,
            0x0014 => Kind::AmdReserved,
            0x0015 => Kind::ResizableBar,
            0x0016 => Kind::DynamicPowerAllocation,
            0x0017 => Kind::TphRequester(bytes.read_with(ecap_data_offset, LE)?),
            0x0018 => ecap_data.try_into().map(Kind::LatencyToleranceReporting)
                        .context(DataSnafu { offset })?,
            0x0019 => Kind::SecondaryPciExpress(bytes.read_with(ecap_data_offset, LE)?),
            0x001A => Kind::ProtocolMultiplexing,
            0x001B => Kind::ProcessAddressSpaceId(bytes.read_with(ecap_data_offset, LE)?),
            0x001C => Kind::LnRequester,
            0x001D => Kind::DownstreamPortContainment(bytes.read_with(ecap_data_offset, LE)?),
            0x001E => ecap_data.try_into().map(Kind::L1PmSubstates)
                        .context(DataSnafu { offset })?,
            0x001F => Kind::PrecisionTimeMeasurement(bytes.read_with(ecap_data_offset, LE)?),
            0x0020 => Kind::PciExpressOverMphy,
            0x0021 => Kind::FrsQueueing,
            0x0022 => Kind::ReadinessTimeReporting,
            0x0023 => Kind::DesignatedVendorSpecificExtendedCapability,
            0x0024 => Kind::VFResizableBar,
            0x0025 => Kind::DataLinkFeature,
            0x0026 => Kind::PhysicalLayer16GTps,
            0x0027 => Kind::ReceiverLaneMargining,
            0x0028 => Kind::HierarchyId,
            0x0029 => Kind::NativePcieEnclosureManagement,
            0x002A => Kind::PhysicalLayer32GTps,
            0x002B => Kind::AlternateProtocol,
            0x002C => Kind::SystemFirmwareIntermediary,
                 v => Kind::Reserved(v),
        };
        Ok(ExtendedCapability { kind, version, offset })
}

/// Extended Capability
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
            ExtendedCapabilityKind::RootComplexLinkDeclaration(_) => 0x0005,
            ExtendedCapabilityKind::RootComplexInternalLinkControl => 0x0006,
            ExtendedCapabilityKind::RootComplexEventCollectorEndpointAssociation => 0x0007,
            ExtendedCapabilityKind::MultiFunctionVirtualChannel => 0x0008,
            ExtendedCapabilityKind::VirtualChannelMfvcPresent(_) => 0x0009,
            ExtendedCapabilityKind::RootComplexRegisterBlock => 0x000A,
            ExtendedCapabilityKind::VendorSpecificExtendedCapability(_) => 0x000B,
            ExtendedCapabilityKind::ConfigurationAccessCorrelation => 0x000C,
            ExtendedCapabilityKind::AccessControlServices(_) => 0x000D,
            ExtendedCapabilityKind::AlternativeRoutingIdInterpretation(_) => 0x000E,
            ExtendedCapabilityKind::AddressTranslationServices(_) => 0x000F,
            ExtendedCapabilityKind::SingleRootIoVirtualization(_) => 0x0010,
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
    RootComplexLinkDeclaration(RootComplexLinkDeclaration<'a>),
    /// Root Complex Internal Link Control
    RootComplexInternalLinkControl,
    /// Root Complex Event Collector Endpoint Association
    RootComplexEventCollectorEndpointAssociation,
    /// Multi-Function Virtual Channel (MFVC)
    MultiFunctionVirtualChannel,
    /// Virtual Channel (VC) – used if an MFVC Extended Cap structure is present in the device
    VirtualChannelMfvcPresent(VirtualChannel<'a>),
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
    SingleRootIoVirtualization(SingleRootIoVirtualization),
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

// 0002h/0009h Virtual Channel (VC)
pub mod virtual_channel;
pub use virtual_channel::VirtualChannel;

// 0003h Device Serial Number
pub mod device_serial_number;
pub use device_serial_number::DeviceSerialNumber;

// 0004h Power Budgeting
pub mod power_budgeting;
pub use power_budgeting::PowerBudgeting;

// 0005h Root Complex Link Declaration
pub mod root_complex_link_declaration;
pub use root_complex_link_declaration::RootComplexLinkDeclaration;

// 0006h Root Complex Internal Link Control
// 0007h Root Complex Event Collector Endpoint Association
// 0008h Multi-Function Virtual Channel (MFVC)
// 000Ah Root Complex Register Block (RCRB) Header

// 000Bh Vendor-Specific Extended Capability (VSEC)
pub mod vendor_specific_extended_capability;
pub use vendor_specific_extended_capability::VendorSpecificExtendedCapability;

// 000Ch Configuration Access Correlation (CAC) – defined by the Trusted Configuration Space (TCS) for PCI Express ECN, which is no longer supported

// 000Dh Access Control Services (ACS)
pub mod access_control_services;
pub use access_control_services::AccessControlServices;

// 000Eh Alternative Routing-ID Interpretation (ARI)
pub mod alternative_routing_id_interpolation;
pub use alternative_routing_id_interpolation::AlternativeRoutingIdInterpretation;

// 000Fh Address Translation Services (ATS)
pub mod address_translation_services;
pub use address_translation_services::AddressTranslationServices;

// 0010h Single Root I/O Virtualization (SR-IOV)
pub mod single_root_io_virtualization;
pub use single_root_io_virtualization::SingleRootIoVirtualization;

// 0011h Multi-Root I/O Virtualization (MR-IOV) – defined in the Multi-Root I/O Virtualization and Sharing Specification
// 0012h Multicast

// 0013h Page Request Interface (PRI)
pub mod page_request_interface;
pub use page_request_interface::PageRequestInterface;

// 0014h Reserved for AMD
// 0015h Resizable BAR
// 0016h Dynamic Power Allocation (DPA)

// 0017h TPH Requester
pub mod tph_requester;
pub use tph_requester::TphRequester;

// 0018h Latency Tolerance Reporting (LTR)
pub mod latency_tolerance_reporting;
pub use latency_tolerance_reporting::LatencyToleranceReporting;

// 0019h Secondary PCI Express
pub mod secondary_pci_express;
pub use secondary_pci_express::SecondaryPciExpress;

// 001Ah Protocol Multiplexing (PMUX)

// 001Bh Process Address Space ID (PASID)
pub mod process_address_space_id;
pub use process_address_space_id::ProcessAddressSpaceId;

// 001Ch LN Requester (LNR)

// 001Dh Downstream Port Containment (DPC)
pub mod downstream_port_containment;
pub use downstream_port_containment::DownstreamPortContainment;

// 001Eh L1 PM Substates
pub mod l1_pm_substates;
pub use l1_pm_substates::L1PmSubstates;

// 001Fh Precision Time Measurement (PTM)
pub mod precision_time_measurement;
pub use precision_time_measurement::PrecisionTimeMeasurement;

// 0020h PCI Express over M-PHY (M-PCIe)
// 0021h FRS Queueing
// 0022h Readiness Time Reporting
// 0023h Designated Vendor-Specific Extended Capability
// 0024h VF Resizable BAR
// 0025h Data Link Feature
// 0026h Physical Layer 16.0 GT/s
// 0027h Lane Margining at the Receiver
// 0028h Hierarchy ID
// 0029h Native PCIe Enclosure Management (NPEM)
// 002Ah Physical Layer 32.0 GT/s
// 002Bh Alternate Protocol
// 002Ch System Firmware Intermediary (SFI)



#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
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
            Ok((0x100, 0x000b)),
            Ok((0x110, 0x000d)),
            Ok((0x148, 0x0001)),
            Ok((0x1d0, 0x000b)),
            Ok((0x250, 0x0019)),
            Ok((0x280, 0x000b)),
            Ok((0x298, 0x000b)),
            Ok((0x300, 0x000b)),
        ];
        let result = ecaps.clone().map(|ecap| ecap.map(|ecap| (ecap.offset, ecap.id())))
            .collect::<Vec<_>>();
        assert_eq!(sample, result);
    }
}
