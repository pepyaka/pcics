/*!
PCI Express Extended Capabilities

PCI Express Extended Capability registers are located in Configuration Space at offsets 256 or
greater or in the Root Complex Register Block (RCRB). These registers when located in the
Configuration Space are accessible using only the PCI Express Enhanced Configuration Access
Mechanism (ECAM).

Extended Capabilities list:
- [x] [Null Capability](ExtendedCapabilityKind::Null) (0000h)
- [x] [Advanced Error Reporting (AER)](advanced_error_reporting) (0001h)
- [x] [Virtual Channel (VC)](virtual_channel) (0002h) - used if an MFVC Extended Cap structure is **not** present in the device
- [x] [Device Serial Number](device_serial_number) (0003h)
- [x] [Power Budgeting](power_budgeting) (0004h)
- [x] [Root Complex Link Declaration](root_complex_link_declaration) (0005h)
- [x] [Root Complex Internal Link Control](root_complex_internal_link_control) (0006h)
- [x] [Root Complex Event Collector Endpoint Association](root_complex_event_collector_endpoint_association) (0007h)
- [x] [Multi-Function Virtual Channel (MFVC)](multifunction_virtual_channel) (0008h)
- [x] [Virtual Channel (VC)](virtual_channel) (0009h) – used if an MFVC Extended Cap structure is present in the device
- [x] [Root Complex Register Block (RCRB) Header](root_complex_register_block_header) (000Ah)
- [x] [Vendor-Specific Extended Capability (VSEC)](vendor_specific_extended_capability) (000Bh)
- [x] [Configuration Access Correlation (CAC)](configuration_access_correlation) (000Ch)
- [x] [Access Control Services (ACS)](access_control_services) (000Dh)
- [x] [Alternative Routing-ID Interpretation (ARI)](alternative_routing_id_interpolation) (000Eh)
- [x] [Address Translation Services (ATS)](address_translation_services) (000Fh)
- [x] [Single Root I/O Virtualization (SR-IOV)](single_root_io_virtualization) (0010h)
- [ ] [Multi-Root I/O Virtualization (MR-IOV)](multi_root_io_virtualization) (0011h)
- [x] [Multicast](multicast) (0012h)
- [x] [Page Request Interface (PRI)](page_request_interface) (0013h)
- [x] [Reserved for AMD](reserved_for_amd) (0014h)
- [x] [Resizable BAR](resizable_bar) (0015h)
- [x] [Dynamic Power Allocation (DPA)](dynamic_power_allocation) (0016h)
- [x] [TPH Requester](tph_requester) (0017h)
- [x] [Latency Tolerance Reporting (LTR)](latency_tolerance_reporting) (0018h)
- [x] [Secondary PCI Express](secondary_pci_express) (0019h)
- [ ] [Protocol Multiplexing (PMUX)](protocol_multiplexing) (001Ah)
- [x] [Process Address Space ID (PASID)](process_address_space_id) (001Bh)
- [ ] [LN Requester (LNR)](ln_requester) (001Ch)
- [x] [Downstream Port Containment (DPC)](downstream_port_containment) (001Dh)
- [x] [L1 PM Substates](l1_pm_substates) (001Eh)
- [x] [Precision Time Measurement (PTM)](precision_time_measurement) (001Fh)
- [ ] [PCI Express over M-PHY (M-PCIe)](pci_express_over_m_phy) (0020h)
- [ ] [FRS Queueing](frs_queueing) (0021h)
- [ ] [Readiness Time Reporting](readiness_time_reporting) (0022h)
- [ ] [Designated Vendor-Specific Extended Capability](designated_vendor_specific_extended_capability) (0023h)
- [ ] [VF Resizable BAR](vf_resizable_bar) (0024h)
- [ ] [Data Link Feature](data_link_feature) (0025h)
- [ ] [Physical Layer 16.0 GT/s](physical_layer_16_gtps) (0026h)
- [ ] [Lane Margining at the Receiver](lane_margining_at_the_receiver) (0027h)
- [ ] [Hierarchy ID](hierarchy_id) (0028h)
- [ ] [Native PCIe Enclosure Management (NPEM)](native_pcie_enclosure_management) (0029h)
- [ ] [Physical Layer 32.0 GT/s](physical_layer_32_gtps) (002Ah)
- [ ] [Alternate Protocol](alternate_protocol) (002Bh)
- [ ] [System Firmware Intermediary (SFI)](system_firmware_intermediary) (002Ch)
- [ ] [Shadow Functions](shadow_functions) (002Dh)
- [ ] [Data Object Exchange](data_object_exchange) (002Eh)
- [ ] [Device 3](device_3) (002Fh)
- [ ] [Integrity and Data Encryption (IDE)](integrity_and_data_encryption) (0030h)
- [ ] [Physical Layer 64.0 GT/s Capability](physical_layer_64_gtps) (0031h)
- [ ] [Flit Logging](flit_logging) (0032h)
- [ ] [Flit Performance Measurement](flit_performance_measurement) (0033h)
- [ ] [Flit Error Injection](flit_error_injection) (0034h)

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



use heterob::{P3, bit_numbering::{LsbInto, Lsb}, endianness::FromLeBytes};
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
    #[snafu(display("[{offset:03x}] {source} data read error"))]
    Data { offset: u16, source: ExtendedCapabilityDataError },
    #[snafu(display("[{offset:03x}] Root Complex Link Declaration error: {source}"))]
    RootComplexLinkDeclaration {
        offset: u16,
        source: root_complex_link_declaration::RootComplexLinkDeclarationError,
    },
    #[snafu(display("[{offset:03x}] Multi-Function Virtual Channel error: {source}"))]
    MultifunctionVirtualChannel {
        offset: u16,
        source: multifunction_virtual_channel::MultifunctionVirtualChannelError,
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
    #[snafu(display("[{offset:03x}] Downstream Port Containment error: {source}"))]
    DownstreamPortContainment {
        offset: u16,
        source: downstream_port_containment::DownstreamPortContainmentError,
    },
    #[snafu(display("[{offset:03x}] Resizable BAR error: {source}"))]
    ResizableBar {
        offset: u16,
        source: resizable_bar::ResizableBarError,
    },
    #[snafu(display("[{offset:03x}] Dynamic Power Allocation error: {source}"))]
    DynamicPowerAllocation {
        offset: u16,
        source: dynamic_power_allocation::DynamicPowerAllocationError,
    },
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

/// PCI Express Extended Capability Header
///
/// All PCI Express Extended Capabilities must begin with a PCI Express
/// Extended Capability header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtendedCapabilityHeader {
    /// PCI Express Extended Capability ID
    ///
    /// This field is a PCI-SIG defined ID number that indicates the nature and
    /// format of the Extended Capability
    pub extended_capability_id: u16,
    /// Capability Version
    ///
    /// This field is a PCI-SIG defined version number that indicates the
    /// version of the Capability structure present
    pub capability_version: u8,
    /// Next Capability Offset
    ///
    /// This field contains the offset to the next PCI Express Capability
    /// structure or 000h if no other items exist in the linked list of Capabilities.
    pub next_capability_offset: u16,
}

impl From<u32> for ExtendedCapabilityHeader {
    fn from(dword: u32) -> Self {
        let Lsb((
            extended_capability_id,
            capability_version,
            next_capability_offset,
        )) = P3::<_, 16, 4, 12>(dword).into();
        Self {
            extended_capability_id,
            capability_version,
            next_capability_offset,
        }
    }
}

impl ExtendedCapabilityHeader {
    /// Extended Capability Header is DWORD
    pub const SIZE: usize = 4;
}

struct ExtendedCapabilityHeaderPlaceholder;
impl FromLeBytes<4> for ExtendedCapabilityHeaderPlaceholder {
    fn from_le_bytes(_: [u8;4]) -> Self {
        Self
    }
}

type ExtendedCapabilityResult<'a> = Result<ExtendedCapability<'a>, ExtendedCapabilityError>;

fn parse_ecap<'a>(
    bytes: &'a [u8],
    next_capability_offset: &mut u16,
) -> ExtendedCapabilityResult<'a> {
    let offset = *next_capability_offset;
    let ecs_offset = (offset as usize).checked_sub(ECS_OFFSET).ok_or_else(|| {
        *next_capability_offset = 0;
        ExtendedCapabilityError::Offset
    })?;
    let ecap_data_offset = ecs_offset + ECH_BYTES;
    let dword = &bytes
        .get(ecs_offset..ecap_data_offset)
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

    use ExtendedCapabilityKind as Kind;
    let kind = match id {
        0x0000 => Kind::Null,
        0x0001 => ecap_data
            .try_into()
            .map(Kind::AdvancedErrorReporting)
            .context(AdvancedErrorReportingSnafu { offset })?,
        0x0002 => ecap_data
            .try_into()
            .map(Kind::VirtualChannel)
            .context(DataSnafu { offset })?,
        0x0003 => ecap_data
            .try_into()
            .map(Kind::DeviceSerialNumber)
            .context(DataSnafu { offset })?,
        0x0004 => ecap_data
            .try_into()
            .map(Kind::PowerBudgeting)
            .context(DataSnafu { offset })?,
        0x0005 => ecap_data
            .try_into()
            .map(Kind::RootComplexLinkDeclaration)
            .context(RootComplexLinkDeclarationSnafu { offset })?,
        0x0006 => ecap_data
            .try_into()
            .map(Kind::RootComplexInternalLinkControl)
            .context(DataSnafu { offset })?,
        0x0007 => ecap_data
            .try_into()
            .map(Kind::RootComplexEventCollectorEndpointAssociation)
            .context(DataSnafu { offset })?,
        // MFVC use data with PCI Express Extended Capability Header for simpler calculations
        0x0008 => bytes
            .try_into()
            .map(Kind::MultifunctionVirtualChannel)
            .context(MultifunctionVirtualChannelSnafu { offset })?,
        0x0009 => ecap_data
            .try_into()
            .map(Kind::VirtualChannelMfvcPresent)
            .context(DataSnafu { offset })?,
        0x000A => bytes
            .try_into()
            .map(Kind::RootComplexRegisterBlockHeader)
            .context(DataSnafu { offset })?,
        0x000B => ecap_data
            .try_into()
            .map(Kind::VendorSpecificExtendedCapability)
            .context(DataSnafu { offset })?,
        0x000C => bytes
            .try_into()
            .map(Kind::ConfigurationAccessCorrelation)
            .context(DataSnafu { offset })?,
        0x000D => ecap_data
            .try_into()
            .map(Kind::AccessControlServices)
            .context(DataSnafu { offset })?,
        0x000E => ecap_data
            .try_into()
            .map(Kind::AlternativeRoutingIdInterpretation)
            .context(DataSnafu { offset })?,
        0x000F => ecap_data
            .try_into()
            .map(Kind::AddressTranslationServices)
            .context(DataSnafu { offset })?,
        0x0010 => ecap_data
            .try_into()
            .map(Kind::SingleRootIoVirtualization)
            .context(DataSnafu { offset })?,
        0x0011 => Kind::MultiRootIoVirtualization(MultiRootIoVirtualization),
        0x0012 => bytes
            .try_into()
            .map(Kind::Multicast)
            .context(DataSnafu { offset })?,
        0x0013 => ecap_data
            .try_into()
            .map(Kind::PageRequestInterface)
            .context(DataSnafu { offset })?,
        0x0014 => Kind::ReservedForAmd(ReservedForAmd),
        0x0015 => bytes
            .try_into()
            .map(Kind::ResizableBar)
            .context(ResizableBarSnafu { offset })?,
        0x0016 => bytes
            .try_into()
            .map(Kind::DynamicPowerAllocation)
            .context(DynamicPowerAllocationSnafu { offset })?,
        0x0017 => ecap_data
            .try_into()
            .map(Kind::TphRequester)
            .context(DataSnafu { offset })?,
        0x0018 => ecap_data
            .try_into()
            .map(Kind::LatencyToleranceReporting)
            .context(DataSnafu { offset })?,
        0x0019 => ecap_data
            .try_into()
            .map(Kind::SecondaryPciExpress)
            .context(DataSnafu { offset })?,
        0x001A => Kind::ProtocolMultiplexing(ProtocolMultiplexing),
        0x001B => ecap_data
            .try_into()
            .map(Kind::ProcessAddressSpaceId)
            .context(DataSnafu { offset })?,
        0x001C => Kind::LnRequester(LnRequester),
        0x001D => ecap_data
            .try_into()
            .map(Kind::DownstreamPortContainment)
            .context(DownstreamPortContainmentSnafu { offset })?,
        0x001E => ecap_data
            .try_into()
            .map(Kind::L1PmSubstates)
            .context(DataSnafu { offset })?,
        0x001F => ecap_data
            .try_into()
            .map(Kind::PrecisionTimeMeasurement)
            .context(DataSnafu { offset })?,
        0x0020 => Kind::PciExpressOverMphy(PciExpressOverMphy),
        0x0021 => Kind::FrsQueueing(FrsQueueing),
        0x0022 => Kind::ReadinessTimeReporting(ReadinessTimeReporting),
        0x0023 => Kind::DesignatedVendorSpecificExtendedCapability(
            DesignatedVendorSpecificExtendedCapability,
        ),
        0x0024 => Kind::VfResizableBar(VfResizableBar),
        0x0025 => Kind::DataLinkFeature(DataLinkFeature),
        0x0026 => Kind::PhysicalLayer16GTps(PhysicalLayer16GTps),
        0x0027 => Kind::LaneMarginingAtTheReceiver(LaneMarginingAtTheReceiver),
        0x0028 => Kind::HierarchyId(HierarchyId),
        0x0029 => Kind::NativePcieEnclosureManagement(NativePcieEnclosureManagement),
        0x002A => Kind::PhysicalLayer32GTps(PhysicalLayer32GTps),
        0x002B => Kind::AlternateProtocol(AlternateProtocol),
        0x002C => Kind::SystemFirmwareIntermediary(SystemFirmwareIntermediary),
        v => Kind::Reserved(v),
    };
    Ok(ExtendedCapability {
        kind,
        version,
        offset,
    })
}


/// Extended Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedCapability<'a> {
    pub kind: ExtendedCapabilityKind<'a>,
    pub version: u8,
    pub offset: u16,
}
impl<'a> ExtendedCapability<'a> {
    /// Extended Capability Header length in bytes
    pub const HEADER_SIZE: usize = 4;
    pub fn id(&self) -> u16 {
        match self.kind {
            ExtendedCapabilityKind::Null => 0x0000,
            ExtendedCapabilityKind::AdvancedErrorReporting(_) => 0x0001,
            ExtendedCapabilityKind::VirtualChannel(_) => 0x0002,
            ExtendedCapabilityKind::DeviceSerialNumber(_) => 0x0003,
            ExtendedCapabilityKind::PowerBudgeting(_) => 0x0004,
            ExtendedCapabilityKind::RootComplexLinkDeclaration(_) => 0x0005,
            ExtendedCapabilityKind::RootComplexInternalLinkControl(_) => 0x0006,
            ExtendedCapabilityKind::RootComplexEventCollectorEndpointAssociation(_) => 0x0007,
            ExtendedCapabilityKind::MultifunctionVirtualChannel(_) => 0x0008,
            ExtendedCapabilityKind::VirtualChannelMfvcPresent(_) => 0x0009,
            ExtendedCapabilityKind::RootComplexRegisterBlockHeader(_) => 0x000A,
            ExtendedCapabilityKind::VendorSpecificExtendedCapability(_) => 0x000B,
            ExtendedCapabilityKind::ConfigurationAccessCorrelation(_) => 0x000C,
            ExtendedCapabilityKind::AccessControlServices(_) => 0x000D,
            ExtendedCapabilityKind::AlternativeRoutingIdInterpretation(_) => 0x000E,
            ExtendedCapabilityKind::AddressTranslationServices(_) => 0x000F,
            ExtendedCapabilityKind::SingleRootIoVirtualization(_) => 0x0010,
            ExtendedCapabilityKind::MultiRootIoVirtualization(_) => 0x0011,
            ExtendedCapabilityKind::Multicast(_) => 0x0012,
            ExtendedCapabilityKind::PageRequestInterface(_) => 0x0013,
            ExtendedCapabilityKind::ReservedForAmd(_) => 0x0014,
            ExtendedCapabilityKind::ResizableBar(_) => 0x0015,
            ExtendedCapabilityKind::DynamicPowerAllocation(_) => 0x0016,
            ExtendedCapabilityKind::TphRequester(_) => 0x0017,
            ExtendedCapabilityKind::LatencyToleranceReporting(_) => 0x0018,
            ExtendedCapabilityKind::SecondaryPciExpress(_) => 0x0019,
            ExtendedCapabilityKind::ProtocolMultiplexing(_) => 0x001A,
            ExtendedCapabilityKind::ProcessAddressSpaceId(_) => 0x001B,
            ExtendedCapabilityKind::LnRequester(_) => 0x001C,
            ExtendedCapabilityKind::DownstreamPortContainment(_) => 0x001D,
            ExtendedCapabilityKind::L1PmSubstates(_) => 0x001E,
            ExtendedCapabilityKind::PrecisionTimeMeasurement(_) => 0x001F,
            ExtendedCapabilityKind::PciExpressOverMphy(_) => 0x0020,
            ExtendedCapabilityKind::FrsQueueing(_) => 0x0021,
            ExtendedCapabilityKind::ReadinessTimeReporting(_) => 0x0022,
            ExtendedCapabilityKind::DesignatedVendorSpecificExtendedCapability(_) => 0x0023,
            ExtendedCapabilityKind::VfResizableBar(_) => 0x0024,
            ExtendedCapabilityKind::DataLinkFeature(_) => 0x0025,
            ExtendedCapabilityKind::PhysicalLayer16GTps(_) => 0x0026,
            ExtendedCapabilityKind::LaneMarginingAtTheReceiver(_) => 0x0027,
            ExtendedCapabilityKind::HierarchyId(_) => 0x0028,
            ExtendedCapabilityKind::NativePcieEnclosureManagement(_) => 0x0029,
            ExtendedCapabilityKind::PhysicalLayer32GTps(_) => 0x002A,
            ExtendedCapabilityKind::AlternateProtocol(_) => 0x002B,
            ExtendedCapabilityKind::SystemFirmwareIntermediary(_) => 0x002C,
            ExtendedCapabilityKind::ShadowFunctions(_) => 0x002D,
            ExtendedCapabilityKind::DataObjectExchange(_) => 0x002E,
            ExtendedCapabilityKind::Device3(_) => 0x002F,
            ExtendedCapabilityKind::IntegrityAndDataEncryption(_) => 0x0030,
            ExtendedCapabilityKind::PhysicalLayer64GTps(_) => 0x0031,
            ExtendedCapabilityKind::FlitLogging(_) => 0x0032,
            ExtendedCapabilityKind::FlitPerformanceMeasurement(_) => 0x0033,
            ExtendedCapabilityKind::FlitErrorInjection(_) => 0x0034,
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
    RootComplexInternalLinkControl(RootComplexInternalLinkControl),
    /// Root Complex Event Collector Endpoint Association
    RootComplexEventCollectorEndpointAssociation(RootComplexEventCollectorEndpointAssociation),
    /// Multi-Function Virtual Channel (MFVC)
    MultifunctionVirtualChannel(MultifunctionVirtualChannel<'a>),
    /// Virtual Channel (VC) – used if an MFVC Extended Cap structure is present in the device
    VirtualChannelMfvcPresent(VirtualChannel<'a>),
    /// Root Complex Register Block (RCRB) Header
    RootComplexRegisterBlockHeader(RootComplexRegisterBlockHeader),
    /// Vendor-Specific Extended Capability (VSEC)
    VendorSpecificExtendedCapability(VendorSpecificExtendedCapability<'a>),
    /// Configuration Access Correlation (CAC) – defined by the Trusted Configuration Space (TCS)
    /// for PCI Express ECN, which is no longer supporte(ECNd
    ConfigurationAccessCorrelation(ConfigurationAccessCorrelation),
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
    MultiRootIoVirtualization(MultiRootIoVirtualization),
    /// Multicast
    Multicast(Multicast),
    /// Page Request Interface (PRI)
    PageRequestInterface(PageRequestInterface),
    /// Reserved for AMD
    ReservedForAmd(ReservedForAmd),
    /// Resizable BAR
    ResizableBar(ResizableBar<'a>),
    /// Dynamic Power Allocation (DPA)
    DynamicPowerAllocation(DynamicPowerAllocation<'a>),
    /// TPH Requester
    TphRequester(TphRequester<'a>),
    /// Latency Tolerance Reporting (LTR)
    LatencyToleranceReporting(LatencyToleranceReporting),
    /// Secondary PCI Express
    SecondaryPciExpress(SecondaryPciExpress<'a>),
    /// Protocol Multiplexing (PMUX)
    ProtocolMultiplexing(ProtocolMultiplexing),
    /// Process Address Space ID (PASID)
    ProcessAddressSpaceId(ProcessAddressSpaceId),
    /// LN Requester (LNR)
    LnRequester(LnRequester),
    /// Downstream Port Containment (DPC)
    DownstreamPortContainment(DownstreamPortContainment),
    /// L1 PM Substates
    L1PmSubstates(L1PmSubstates),
    /// Precision Time Measurement (PTM)
    PrecisionTimeMeasurement(PrecisionTimeMeasurement),
    /// PCI Express over M-PHY (M-PCIe)
    PciExpressOverMphy(PciExpressOverMphy),
    /// FRS Queueing
    FrsQueueing(FrsQueueing),
    /// Readiness Time Reporting
    ReadinessTimeReporting(ReadinessTimeReporting),
    /// Designated Vendor-Specific Extended Capability
    DesignatedVendorSpecificExtendedCapability(DesignatedVendorSpecificExtendedCapability),
    /// VF Resizable BAR
    VfResizableBar(VfResizableBar),
    /// Data Link Feature
    DataLinkFeature(DataLinkFeature),
    /// Physical Layer 16.0 GT/s
    PhysicalLayer16GTps(PhysicalLayer16GTps),
    /// Lane Margining at the Receiver
    LaneMarginingAtTheReceiver(LaneMarginingAtTheReceiver),
    /// Hierarchy ID
    HierarchyId(HierarchyId),
    /// Native PCIe Enclosure Management (NPEM)
    NativePcieEnclosureManagement(NativePcieEnclosureManagement),
    /// Physical Layer 32.0 GT/s
    PhysicalLayer32GTps(PhysicalLayer32GTps),
    /// Alternate Protocol
    AlternateProtocol(AlternateProtocol),
    /// System Firmware Intermediary (SFI)
    SystemFirmwareIntermediary(SystemFirmwareIntermediary),
    /// Shadow Functions
    ShadowFunctions(ShadowFunctions),
    /// Data Object Exchange
    DataObjectExchange(DataObjectExchange),
    /// Device 3
    Device3(Device3),
    /// Integrity and Data Encryption (IDE)
    IntegrityAndDataEncryption(IntegrityAndDataEncryption),
    /// Physical Layer 64.0 GT/s
    PhysicalLayer64GTps(PhysicalLayer64GTps),
    /// Flit Logging
    FlitLogging(FlitLogging),
    /// Flit Performance Measurement
    FlitPerformanceMeasurement(FlitPerformanceMeasurement),
    /// Flit Error Injection
    FlitErrorInjection(FlitErrorInjection),
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
pub mod root_complex_internal_link_control;
pub use root_complex_internal_link_control::RootComplexInternalLinkControl;

// 0007h Root Complex Event Collector Endpoint Association
pub mod root_complex_event_collector_endpoint_association;
pub use root_complex_event_collector_endpoint_association::RootComplexEventCollectorEndpointAssociation;

// 0008h Multi-Function Virtual Channel (MFVC)
pub mod multifunction_virtual_channel;
pub use multifunction_virtual_channel::MultifunctionVirtualChannel;

// 000Ah Root Complex Register Block (RCRB) Header
pub mod root_complex_register_block_header;
pub use root_complex_register_block_header::RootComplexRegisterBlockHeader;

// 000Bh Vendor-Specific Extended Capability (VSEC)
pub mod vendor_specific_extended_capability;
pub use vendor_specific_extended_capability::VendorSpecificExtendedCapability;

// 000Ch Configuration Access Correlation (CAC)
// defined by the Trusted Configuration Space (TCS) for PCI Express ECN, which is no longer supported
pub mod configuration_access_correlation;
pub use configuration_access_correlation::ConfigurationAccessCorrelation;

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

// 0011h Multi-Root I/O Virtualization (MR-IOV)
// defined in the Multi-Root I/O Virtualization and Sharing Specification
pub mod multi_root_io_virtualization {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MultiRootIoVirtualization;
}
pub use multi_root_io_virtualization::MultiRootIoVirtualization;

// 0012h Multicast
pub mod multicast;
pub use multicast::Multicast;

// 0013h Page Request Interface (PRI)
pub mod page_request_interface;
pub use page_request_interface::PageRequestInterface;

// 0014h Reserved for AMD
pub mod reserved_for_amd {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ReservedForAmd;
}
pub use reserved_for_amd::ReservedForAmd;

// 0015h Resizable BAR
pub mod resizable_bar;
pub use resizable_bar::ResizableBar;

// 0016h Dynamic Power Allocation (DPA)
pub mod dynamic_power_allocation;
pub use dynamic_power_allocation::DynamicPowerAllocation;

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
pub mod protocol_multiplexing {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ProtocolMultiplexing;
}
pub use protocol_multiplexing::ProtocolMultiplexing;

// 001Bh Process Address Space ID (PASID)
pub mod process_address_space_id;
pub use process_address_space_id::ProcessAddressSpaceId;

// 001Ch LN Requester (LNR)
pub mod ln_requester {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LnRequester;
}
pub use ln_requester::LnRequester;

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
pub mod pci_express_over_m_phy {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PciExpressOverMphy;
}
pub use pci_express_over_m_phy::PciExpressOverMphy;

// 0021h FRS Queueing
pub mod frs_queueing {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FrsQueueing;
}
pub use frs_queueing::FrsQueueing;

// 0022h Readiness Time Reporting
pub mod readiness_time_reporting {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ReadinessTimeReporting;
}
pub use readiness_time_reporting::ReadinessTimeReporting;

// 0023h Designated Vendor-Specific Extended Capability
pub mod designated_vendor_specific_extended_capability {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DesignatedVendorSpecificExtendedCapability;
}
pub use designated_vendor_specific_extended_capability::DesignatedVendorSpecificExtendedCapability;

// 0024h VF Resizable BAR
pub mod vf_resizable_bar {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct VfResizableBar;
}
pub use vf_resizable_bar::VfResizableBar;

// 0025h Data Link Feature
pub mod data_link_feature {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DataLinkFeature;
}
pub use data_link_feature::DataLinkFeature;

// 0026h Physical Layer 16.0 GT/s
pub mod physical_layer_16_gtps {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PhysicalLayer16GTps;
}
pub use physical_layer_16_gtps::PhysicalLayer16GTps;

// 0027h Lane Margining at the Receiver
pub mod lane_margining_at_the_receiver {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LaneMarginingAtTheReceiver;
}
pub use lane_margining_at_the_receiver::LaneMarginingAtTheReceiver;

// 0028h Hierarchy ID
pub mod hierarchy_id {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HierarchyId;
}
pub use hierarchy_id::HierarchyId;

// 0029h Native PCIe Enclosure Management (NPEM)
pub mod native_pcie_enclosure_management {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NativePcieEnclosureManagement;
}
pub use native_pcie_enclosure_management::NativePcieEnclosureManagement;

// 002Ah Physical Layer 32.0 GT/s
pub mod physical_layer_32_gtps {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PhysicalLayer32GTps;
}
pub use physical_layer_32_gtps::PhysicalLayer32GTps;

// 002Bh Alternate Protocol
pub mod alternate_protocol {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AlternateProtocol;
}
pub use alternate_protocol::AlternateProtocol;

// 002Ch System Firmware Intermediary (SFI)
pub mod system_firmware_intermediary {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SystemFirmwareIntermediary;
}
pub use system_firmware_intermediary::SystemFirmwareIntermediary;

// 002Dh Shadow Functions
pub mod shadow_functions {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ShadowFunctions;
}
pub use shadow_functions::ShadowFunctions;

// 002Eh Data Object Exchange
pub mod data_object_exchange {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DataObjectExchange;
}
pub use data_object_exchange::DataObjectExchange;

// 002Fh Device 3
pub mod device_3 {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Device3;
}
pub use device_3::Device3;

// 0030h Integrity and Data Encryption (IDE)
pub mod integrity_and_data_encryption {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct IntegrityAndDataEncryption;
}
pub use integrity_and_data_encryption::IntegrityAndDataEncryption;

// 0031h Physical Layer 64.0 GT/s
pub mod physical_layer_64_gtps {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PhysicalLayer64GTps;
}
pub use physical_layer_64_gtps::PhysicalLayer64GTps;

// 0032h Flit Logging
pub mod flit_logging {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FlitLogging;
}
pub use flit_logging::FlitLogging;

// 0033h Flit Performance Measurement
pub mod flit_performance_measurement {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FlitPerformanceMeasurement;
}
pub use flit_performance_measurement::FlitPerformanceMeasurement;

// 0034h Flit Error Injection
pub mod flit_error_injection {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FlitErrorInjection;
}
pub use flit_error_injection::FlitErrorInjection;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    const DATA: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/device/8086:2030/config"
    ));

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
        let ecaps = ExtendedCapabilities::new(DATA[ECS_OFFSET..].try_into().unwrap());
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
        let result = ecaps
            .clone()
            .map(|ecap| ecap.map(|ecap| (ecap.offset, ecap.id())))
            .collect::<Vec<_>>();
        assert_eq!(sample, result);
    }
}
