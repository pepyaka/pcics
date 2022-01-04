//! PCI Express Capability
//!
//! PCI Express defines a Capability structure in PCI 3.0 compatible Configuration Space (first 256
//! bytes). This structure allows identification of a PCI Express device Function and indicates
//! support for new PCI Express features. The PCI Express Capability structure is required for PCI
//! Express device Functions. The Capability structure is a mechanism for enabling PCI software
//! transparent features requiring support on legacy operating systems.  In addition to identifying
//! a PCI Express device Function, the PCI Express Capability structure is used to provide access
//! to PCI Express specific Control/Status registers and related Power Management enhancements.



use core::ops::Range;
use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};



/// PCI Express Capability Structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciExpress {
    pub capabilities: Capabilities,
    pub device: Device,
    pub link: Option<Link>,
    pub slot: Option<Slot>,
    pub root: Option<Root>,
    pub device_2: Option<Device2>,
    pub link_2: Option<Link2>,
    pub slot_2: Option<Slot2>,
}
impl<'a> TryRead<'a, Endian> for PciExpress {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let capabilities: Capabilities = bytes.read_with::<u16>(offset, endian)?.into();
        let device = bytes.read_with::<Device>(offset, endian)?;
        let link = bytes.read_with::<LinkOption>(offset, endian)?.0;
        let slot = bytes.read_with::<SlotOption>(offset, endian)?.0;
        let root = bytes.read_with::<RootOption>(offset, endian)?.0;
        // Since PCI Express® Base Specification Revision 2
        let (device_2, link_2, slot_2) =
            if capabilities.version > 1 {
                (
                    bytes.read_with::<Device2Option>(offset, endian)?.0,
                    bytes.read_with::<Link2Option>(offset, endian)?.0,
                    // lspci.c allowed to read data shorter than PCi Express capability header
                    // bytes.read_with::<Slot2Option>(offset, endian)?.0,
                    bytes.read_with::<Slot2Option>(offset, endian).unwrap_or(Slot2Option(None)).0,
                )
            } else {
                (None, None, None)
            };
        Ok((PciExpress { capabilities, device, link, slot, root, device_2, link_2, slot_2 }, *offset))
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct CapabilitiesProto {
    version: B4,
    device_type: B4,
    slot_implemented: bool,
    interrupt_message_number: B5,
    tcs_routing_support: bool,
    rsvdp: B1,
}

/// The PCI Express Capabilities register identifies PCI Express device Function type and
/// associated capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    /// Indicates PCI-SIG defined PCI Express Capability structure version number
    pub version: u8,
    pub device_type: DeviceType,
    /// Indicates that the Link associated with this Port is connected to a slot
    pub slot_implemented: bool,
    /// This field indicates which MSI/MSI-X vector is used for the interrupt message generated in
    /// association with any of the status bits of this Capability structure 
    pub interrupt_message_number: u8,
    /// Indicate support for TCS Routing
    pub tcs_routing_support: bool,
}
impl From<CapabilitiesProto> for Capabilities {
    fn from(proto: CapabilitiesProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            version: proto.version(),
            device_type: proto.device_type().into(),
            slot_implemented: proto.slot_implemented(),
            interrupt_message_number: proto.interrupt_message_number(),
            tcs_routing_support: proto.tcs_routing_support(),
        }
    }
}
impl From<u16> for Capabilities {
    fn from(word: u16) -> Self { CapabilitiesProto::from(word).into() }
}

/// Indicates the specific type of this PCI Express Function
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    /// PCI Express Endpoint
    Endpoint,
    /// Legacy PCI Express Endpoint
    LegacyEndpoint,
    /// Root Port of PCI Express Root Complex
    RootPort,
    /// Upstream Port of PCI Express Switch
    UpstreamPort,
    /// Downstream Port of PCI Express Switch
    DownstreamPort,
    /// PCI Express to PCI/PCI-X Bridge
    PcieToPciBridge,
    /// PCI/PCI-X to PCI Express Bridge
    PciToPcieBridge,
    /// Root Complex Integrated Endpoint
    RootComplexIntegratedEndpoint,
    /// Root Complex Event Collector 
    RootComplexEventCollector,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for DeviceType {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::Endpoint,
            0b0001 => Self::LegacyEndpoint,
            0b0100 => Self::RootPort,
            0b0101 => Self::UpstreamPort,
            0b0110 => Self::DownstreamPort,
            0b0111 => Self::PcieToPciBridge,
            0b1000 => Self::PciToPcieBridge,
            0b1001 => Self::RootComplexIntegratedEndpoint,
            0b1010 => Self::RootComplexEventCollector,
                 v => Self::Reserved(v),
        }
    }
}

/// The Device Capabilities, Device Status, and Device Control registers are required for all PCI
/// Express device Functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub capabilities: DeviceCapabilities,
    pub control: DeviceControl,
    pub status: DeviceStatus,
}
impl<'a> TryRead<'a, Endian> for Device {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let result = Self {
            capabilities: bytes.read_with::<u32>(offset, endian)?.into(),
            control: bytes.read_with::<u16>(offset, endian)?.into(),
            status: bytes.read_with::<u16>(offset, endian)?.into(),
        };
        Ok((result, *offset))
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct DeviceCapabilitiesProto {
    max_payload_size_supported: B3,
    phantom_functions_supported: B2,
    extended_tag_field_supported: bool,
    endpoint_l0s_acceptable_latency: B3,
    endpoint_l1_acceptable_latency: B3,
    attention_button_present: bool,
    attention_indicator_present: bool,
    power_indicator_present: bool,
    role_based_error_reporting: bool,
    rsvdp: B2,
    captured_slot_power_limit_value: u8,
    captured_slot_power_limit_scale: B2,
    function_level_reset_capability: bool,
    rsvdp_2: B3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCapabilities {
    /// Max_Payload_Size Supported
    pub max_payload_size_supported: MaxSize,
    pub phantom_functions_supported: PhantomFunctionsSupported,
    pub extended_tag_field_supported: ExtendedTagFieldSupported,
    pub endpoint_l0s_acceptable_latency: EndpointL0sAcceptableLatency,
    pub endpoint_l1_acceptable_latency: EndpointL1AcceptableLatency,
    /// Attention Button is implemented on the adapter and electrically controlled by the component
    /// on the adapter
    pub attention_button_present: bool,
    /// Attention Indicator is implemented on the adapter and electrically controlled by the
    /// component on the adapter
    pub attention_indicator_present: bool,
    /// Power Indicator is implemented on the adapter and electrically controlled by the component
    /// on the adapter.
    pub power_indicator_present: bool,
    /// Role-Based Error Reporting 
    ///
    /// Function implements the functionality originally defined in the Error Reporting ECN for PCI
    /// Express Base Specification
    pub role_based_error_reporting: bool,
    pub captured_slot_power_limit: SlotPowerLimit,
    /// Function supports the optional Function Level Reset mechanism
    pub function_level_reset_capability: bool,
}
impl From<DeviceCapabilitiesProto> for DeviceCapabilities {
    fn from(proto: DeviceCapabilitiesProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            max_payload_size_supported: proto.max_payload_size_supported().into(),
            phantom_functions_supported: proto.phantom_functions_supported().into(),
            extended_tag_field_supported: proto.extended_tag_field_supported().into(),
            endpoint_l0s_acceptable_latency: proto.endpoint_l0s_acceptable_latency().into(),
            endpoint_l1_acceptable_latency: proto.endpoint_l1_acceptable_latency().into(),
            attention_button_present: proto.attention_button_present(),
            attention_indicator_present: proto.attention_indicator_present(),
            power_indicator_present: proto.power_indicator_present(),
            role_based_error_reporting: proto.role_based_error_reporting(),
            captured_slot_power_limit: SlotPowerLimit::new(
                proto.captured_slot_power_limit_value(),
                proto.captured_slot_power_limit_scale(),
            ),
            function_level_reset_capability: proto.function_level_reset_capability(),
        }
    }
}
impl From<u32> for DeviceCapabilities {
    fn from(dword: u32) -> Self { DeviceCapabilitiesProto::from(dword).into() }
}


/// Max_Payload_Size Supported / Max_Payload_Size / Max_Read_Request_Size 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxSize {
    /// 128 bytes max size 
    B128,
    /// 256 bytes max size 
    B256,
    /// 512 bytes max size 
    B512,
    /// 1024 bytes max size 
    B1024,
    /// 2048 bytes max size 
    B2048,
    /// 4096 bytes max size 
    B4096,
    /// Reserved
    Reserved0,
    /// Reserved
    Reserved1,
}
impl From<u8> for MaxSize {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::B128,
            0b001 => Self::B256,
            0b010 => Self::B512,
            0b011 => Self::B1024,
            0b100 => Self::B2048,
            0b101 => Self::B4096,
            0b110 => Self::Reserved0,
            0b111 => Self::Reserved1,
                _ => unreachable!(),
        }
    }
}

/// Support for use of unclaimed Function Numbers to extend the number of outstanding transactions
/// allowed by logically combining unclaimed Function Numbers (called Phantom Functions) with the
/// Tag identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhantomFunctionsSupported {
    /// No Function Number bits are used
    NoBits,
    /// The most significant bit
    MostSignificantBit,
    /// The two most significant bits
    /// Functions
    TwoMostSignificantBits,
    /// All 3 bits
    AllBits,
}
impl From<u8> for PhantomFunctionsSupported {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NoBits,
            0b01 => Self::MostSignificantBit,
            0b10 => Self::TwoMostSignificantBits,
            0b11 => Self::AllBits,
               _ => unreachable!(),
        }
    }
}


/// Maximum supported size of the Tag field as a Requester
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendedTagFieldSupported {
    /// 5-bit Tag field supported
    Five,
    /// 8-bit Tag field supported
    Eight,
}
impl From<bool> for ExtendedTagFieldSupported {
    fn from(b: bool) -> Self {
        if b {
            Self::Eight
        } else {
            Self::Five
        }
    }
}

/// Acceptable total latency that an Endpoint can withstand due to the transition from L0s state to
/// the L0 state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointL0sAcceptableLatency {
    /// Maximum of 64 ns
    Max64ns,
    /// Maximum of 128 ns
    Max128ns,
    /// Maximum of 256 ns
    Max256ns,
    /// Maximum of 512 ns
    Max512ns,
    /// Maximum of 1 µs
    Max1us,
    /// Maximum of 2 µs
    Max2us,
    /// Maximum of 4 µs
    Max4us,
    /// No limit
    NoLimit,
}
impl From<u8> for EndpointL0sAcceptableLatency {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Max64ns,
            0b001 => Self::Max128ns,
            0b010 => Self::Max256ns,
            0b011 => Self::Max512ns,
            0b100 => Self::Max1us,
            0b101 => Self::Max2us,
            0b110 => Self::Max4us,
            0b111 => Self::NoLimit,
                _ => unreachable!(),
        }
    }
}

/// Aacceptable latency that an Endpoint can withstand due to the transition from L1 state to the
/// L0 state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointL1AcceptableLatency {
    /// Maximum of 1 µs
    Max1us,
    /// Maximum of 2 µs
    Max2us,
    /// Maximum of 4 µs
    Max4us,
    /// Maximum of 8 µs
    Max8us,
    /// Maximum of 16 µs
    Max16us,
    /// Maximum of 32 µs
    Max32us,
    /// Maximum of 64 µs
    Max64us,
    /// No limit
    NoLimit,
}
impl From<u8> for EndpointL1AcceptableLatency {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Max1us,
            0b001 => Self::Max2us,
            0b010 => Self::Max4us,
            0b011 => Self::Max8us,
            0b100 => Self::Max16us,
            0b101 => Self::Max32us,
            0b110 => Self::Max64us,
            0b111 => Self::NoLimit,
                _ => unreachable!(),
        }
    }
}


/// Slot Power Limit (Captured)
/// Specifies the upper limit on power available/supplied to the adapter
#[derive(Debug, Clone, PartialEq)]
pub struct SlotPowerLimit {
    /// Slot Power Limit Value
    pub value: u8,
    /// Slot Power Limit Scale
    pub scale: f32,
}
impl SlotPowerLimit {
    pub fn new(value: u8, scale: u8) -> Self {
        match scale {
            0b00 => SlotPowerLimit { value, scale: 1.0 },
            0b01 => SlotPowerLimit { value, scale: 0.1 },
            0b10 => SlotPowerLimit { value, scale: 0.01 },
            0b11 => SlotPowerLimit { value, scale: 0.001 },
               _ => unreachable!(),
        }
    }
}
impl Eq for SlotPowerLimit {}
impl From<SlotPowerLimit> for f32 {
    fn from(cspl: SlotPowerLimit) -> Self {
        cspl.scale * cspl.value as f32
    }
}
impl<'a> From<&'a SlotPowerLimit> for f32 {
    fn from(cspl: &SlotPowerLimit) -> Self {
        cspl.scale * cspl.value as f32
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DeviceControlProto {
    correctable_error_reporting_enable: bool,
    non_fatal_error_reporting_enable: bool,
    fatal_error_reporting_enable: bool,
    unsupported_request_reporting_enable: bool,
    enable_relaxed_ordering: bool,
    max_payload_size: B3,
    extended_tag_field_enable: bool,
    phantom_functions_enable: bool,
    aux_power_pm_enable: bool,
    enable_no_snoop: bool,
    max_read_request_size: B3,
    bcre_or_flreset: bool,
}

/// The Device Control register controls PCI Express device specific parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceControl {
    /// Correctable Error Reporting Enable
    pub correctable_error_reporting_enable: bool,
    /// Non-Fatal Error Reporting Enable
    pub non_fatal_error_reporting_enable: bool,
    /// Fatal Error Reporting Enable
    pub fatal_error_reporting_enable: bool,
    /// Unsupported Request Reporting Enable
    pub unsupported_request_reporting_enable: bool,
    /// Enable Relaxed Ordering
    pub enable_relaxed_ordering: bool,
    /// Max_Payload_Size
    pub max_payload_size: MaxSize,
    /// Extended Tag Field Enable
    pub extended_tag_field_enable: bool,
    /// Phantom Functions Enable
    pub phantom_functions_enable: bool,
    /// Aux Power PM Enable
    pub aux_power_pm_enable: bool,
    /// Enable No Snoop
    pub enable_no_snoop: bool,
    /// Max_Read_Request_Size
    pub max_read_request_size: MaxSize,
    /// Bridge Configuration Retry Enable / Initiate Function Level Reset
    ///
    /// - PCI Express to PCI/PCI-X pub Bridges: Bridge Configuration Retry Enable
    /// - Endpoints with Function Level Reset Capability set to pub 1b: Initiate Function Level Reset
    /// - All pub others: Reserved
    pub bcre_or_flreset: bool,
}
impl From<DeviceControlProto> for DeviceControl {
    fn from(proto: DeviceControlProto) -> Self {
        Self {
            correctable_error_reporting_enable: proto.correctable_error_reporting_enable(),
            non_fatal_error_reporting_enable: proto.non_fatal_error_reporting_enable(),
            fatal_error_reporting_enable: proto.fatal_error_reporting_enable(),
            unsupported_request_reporting_enable: proto.unsupported_request_reporting_enable(),
            enable_relaxed_ordering: proto.enable_relaxed_ordering(),
            max_payload_size: proto.max_payload_size().into(),
            extended_tag_field_enable: proto.extended_tag_field_enable(),
            phantom_functions_enable: proto.phantom_functions_enable(),
            aux_power_pm_enable: proto.aux_power_pm_enable(),
            enable_no_snoop: proto.enable_no_snoop(),
            max_read_request_size: proto.max_read_request_size().into(),
            bcre_or_flreset: proto.bcre_or_flreset(),
        }
    }
}
impl From<u16> for DeviceControl {
    fn from(word: u16) -> Self { DeviceControlProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DeviceStatusProto {
    correctable_error_detected: bool,
    non_fatal_error_detected: bool,
    fatal_error_detected: bool,
    unsupported_request_detected: bool,
    aux_power_detected: bool,
    transactions_pending: bool,
    rsvdz: B10,
}

/// Provides information about PCI Express device (Function) specific parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStatus {
    /// Correctable Error Detected
    pub correctable_error_detected: bool,
    /// Non-Fatal Error Detected
    pub non_fatal_error_detected: bool,
    /// Fatal Error Detected
    pub fatal_error_detected: bool,
    /// Unsupported Request Detected
    pub unsupported_request_detected: bool,
    /// AUX Power Detected
    pub aux_power_detected: bool,
    /// Transactions Pending
    ///
    /// - pub Endpoints: indicates that the Function has issued NonPosted Requests that have not been
    ///   completed
    /// - Root and Switch pub Ports: indicates that a Port has issued Non-Posted Requests on its own
    ///   behalf (using the Port’s own Requester ID) which have not been completed
    pub transactions_pending: bool,
}
impl From<DeviceStatusProto> for DeviceStatus {
    fn from(proto: DeviceStatusProto) -> Self {
        let _ = proto.rsvdz();
        Self {
            correctable_error_detected: proto.correctable_error_detected(),
            non_fatal_error_detected: proto.non_fatal_error_detected(),
            fatal_error_detected: proto.fatal_error_detected(),
            unsupported_request_detected: proto.unsupported_request_detected(),
            aux_power_detected: proto.aux_power_detected(),
            transactions_pending: proto.transactions_pending(),
        }
    }
}
impl From<u16> for DeviceStatus {
    fn from(word: u16) -> Self { DeviceStatusProto::from(word).into() }
}


/// The Link Capabilities, Link Status, and Link Control registers are required for all Root Ports,
/// Switch Ports, Bridges, and Endpoints that are not Root Complex Integrated Endpoints
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub capabilities: LinkCapabilities,
    pub control: LinkControl,
    pub status: LinkStatus,
}
impl Link {
    pub fn new(capabilities: u32, control: u16, status: u16) -> Self {
        Self {
            capabilities: capabilities.into(),
            control: control.into(),
            status: status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LinkOption(Option<Link>);
impl<'a> TryRead<'a, Endian> for LinkOption {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
            let capabilities = bytes.read_with::<u32>(offset, endian)?;
            let control = bytes.read_with::<u16>(offset, endian)?;
            let status = bytes.read_with::<u16>(offset, endian)?;
        if (0, 0, 0) == (capabilities, control, status) {
            Ok((Self(None), *offset))
        } else {
            Ok((Self(Some(Link::new(capabilities, control, status))), *offset))
        }
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct LinkCapabilitiesProto {
    max_link_speed: B4,
    maximum_link_width: B6,
    active_state_power_management_support: B2,
    l0s_exit_latency: B3,
    l1_exit_latency: B3,
    clock_power_management: bool,
    surprise_down_error_reporting_capable: bool,
    data_link_layer_link_active_reporting_capable: bool,
    link_bandwidth_notification_capability: bool,
    aspm_optionality_compliance: bool,
    rsvdp: B1,
    port_number: u8,
}
impl From<LinkCapabilities> for LinkCapabilitiesProto {
    fn from(data: LinkCapabilities) -> Self {
        Self::new()
            .with_max_link_speed(data.max_link_speed.into())
            .with_maximum_link_width(data.maximum_link_width.into())
            .with_active_state_power_management_support(data.active_state_power_management_support as u8)
            .with_l0s_exit_latency(data.l0s_exit_latency as u8)
            .with_l1_exit_latency(data.l1_exit_latency as u8)
            .with_clock_power_management(data.clock_power_management)
            .with_surprise_down_error_reporting_capable(data.surprise_down_error_reporting_capable)
            .with_data_link_layer_link_active_reporting_capable(data.data_link_layer_link_active_reporting_capable)
            .with_link_bandwidth_notification_capability(data.link_bandwidth_notification_capability)
            .with_aspm_optionality_compliance(data.aspm_optionality_compliance)
            .with_rsvdp(0)
            .with_port_number(data.port_number)
    }
}

/// The Link Capabilities register identifies PCI Express Link specific capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkCapabilities {
    pub max_link_speed: LinkSpeed,
    pub maximum_link_width: LinkWidth,
    /// Active State Power Management (ASPM) Support
    pub active_state_power_management_support: ActiveStatePowerManagement,
    pub l0s_exit_latency: L0sExitLatency,
    pub l1_exit_latency: L1ExitLatency,
    /// Clock Power Management
    pub clock_power_management: bool,
    /// Surprise Down Error Reporting Capable
    pub surprise_down_error_reporting_capable: bool,
    /// Data Link Layer Link Active Reporting Capable
    pub data_link_layer_link_active_reporting_capable: bool,
    /// Link Bandwidth Notification Capability
    pub link_bandwidth_notification_capability: bool,
    /// ASPM Optionality Compliance
    pub aspm_optionality_compliance: bool,
    /// Port Number
    pub port_number: u8,
}
impl From<LinkCapabilitiesProto> for LinkCapabilities {
    fn from(proto: LinkCapabilitiesProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            max_link_speed: proto.max_link_speed().into(),
            maximum_link_width: proto.maximum_link_width().into(),
            active_state_power_management_support: proto.active_state_power_management_support().into(),
            l0s_exit_latency: proto.l0s_exit_latency().into(),
            l1_exit_latency: proto.l1_exit_latency().into(),
            clock_power_management: proto.clock_power_management(),
            surprise_down_error_reporting_capable: proto.surprise_down_error_reporting_capable(),
            data_link_layer_link_active_reporting_capable: proto.data_link_layer_link_active_reporting_capable(),
            link_bandwidth_notification_capability: proto.link_bandwidth_notification_capability(),
            aspm_optionality_compliance: proto.aspm_optionality_compliance(),
            port_number: proto.port_number(),
        }
    }
}
impl From<u32> for LinkCapabilities {
    fn from(dword: u32) -> Self { LinkCapabilitiesProto::from(dword).into() }
}
impl From<LinkCapabilities> for u32 {
    fn from(lc: LinkCapabilities) -> Self { LinkCapabilitiesProto::from(lc).into() }
}


/// Max/Current/Target Link Speed
/// Speeds should be taken from [SupportedLinkSpeedsVector]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkSpeed {
    /// 2.5 GT/s
    Rate2GTps,
    /// 5.0 GT/s
    Rate5GTps,
    /// 8.0 GT/s
    Rate8GTps,
    /// 16.0 GT/s
    Rate16GTps,
    /// 32.0 GT/s
    Rate32GTps,
    /// 64.0 GT/s
    Rate64GTps,
    /// Supported Link Speeds Vector field bit 6 (Reserved)
    RateRsvdp,
    /// All other encodings are Reserved
    Reserved(u8),
}
impl From<u8> for LinkSpeed {
    fn from(byte: u8) -> Self {
        match byte {
            0b0001 => Self::Rate2GTps,
            0b0010 => Self::Rate5GTps,
            0b0011 => Self::Rate8GTps,
            0b0100 => Self::Rate16GTps,
            0b0101 => Self::Rate32GTps,
            0b0110 => Self::Rate64GTps,
            0b0111 => Self::RateRsvdp,
                 v => Self::Reserved(v),
        }
    }
}
impl From<LinkSpeed> for u8 {
    fn from(data: LinkSpeed) -> Self {
        match data {
            LinkSpeed::Rate2GTps   => 1,
            LinkSpeed::Rate5GTps   => 2,
            LinkSpeed::Rate8GTps   => 3,
            LinkSpeed::Rate16GTps  => 4,
            LinkSpeed::Rate32GTps  => 5,
            LinkSpeed::Rate64GTps  => 6,
            LinkSpeed::RateRsvdp   => 7,
            LinkSpeed::Reserved(v) => v,
        }
    }
}


/// Maximum/Negotiated Link Width
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkWidth {
    Reserved(u8),
    X1,
    X2,
    X4,
    X8,
    X12,
    X16,
    X32,
}
impl LinkWidth {
    pub fn value(&self) -> usize {
        match self {
            Self::X1  => 1,
            Self::X2  => 2,
            Self::X4  => 4,
            Self::X8  => 8,
            Self::X12 => 12,
            Self::X16 => 16,
            Self::X32 => 32,
            Self::Reserved(_) => 0,
        }
    }
}
impl From<u8> for LinkWidth {
    fn from(byte: u8) -> Self {
        match byte {
            0b00_0001 => Self::X1,
            0b00_0010 => Self::X2,
            0b00_0100 => Self::X4,
            0b00_1000 => Self::X8,
            0b00_1100 => Self::X12,
            0b01_0000 => Self::X16,
            0b10_0000 => Self::X32,
                    v => Self::Reserved(v),
        }
    }
}
impl From<LinkWidth> for u8 {
    fn from(data: LinkWidth) -> Self {
        match data {
            LinkWidth::X1  => 0,
            LinkWidth::X2  => 1,
            LinkWidth::X4  => 2,
            LinkWidth::X8  => 3,
            LinkWidth::X12 => 4,
            LinkWidth::X16 => 5,
            LinkWidth::X32 => 6,
            LinkWidth::Reserved(v) => v,
        }
    }
}


/// Active State Power Management (ASPM) Support/Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveStatePowerManagement {
    NoAspm,
    L0s,
    L1,
    L0sAndL1,
}
impl From<u8> for ActiveStatePowerManagement {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NoAspm,
            0b01 => Self::L0s,
            0b10 => Self::L1,
            0b11 => Self::L0sAndL1,
               _ => unreachable!(),
        }
    }
}


/// L0s Exit Latency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L0sExitLatency {
    /// Less than 64 ns
    Lt64ns,
    /// 64 ns to less than 128 ns
    Ge64nsAndLt128ns,
    /// 128 ns to less than 256 ns
    Ge128nsAndLt256ns,
    /// 256 ns to less than 512 ns
    Ge256nsAndLt512ns,
    /// 512 ns to less than 1 µs
    Ge512nsAndLt1us,
    /// 1 µs to less than 2 µs
    Ge1usAndLt2us,
    /// 2 µs-4 µs
    Ge2usAndLt4us,
    /// More than 4 µs
    Gt4ns,
}
impl From<u8> for L0sExitLatency {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Lt64ns,
            0b001 => Self::Ge64nsAndLt128ns,
            0b010 => Self::Ge128nsAndLt256ns,
            0b011 => Self::Ge256nsAndLt512ns,
            0b100 => Self::Ge512nsAndLt1us,
            0b101 => Self::Ge1usAndLt2us,
            0b110 => Self::Ge2usAndLt4us,
            0b111 => Self::Gt4ns,
                _ => unreachable!(),
        }
    }
}


/// L1 Exit Latency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L1ExitLatency {
    /// Less than 1 µs
    Lt1us,
    /// 1 µs to less than 2 µs
    Ge1usAndLt2us,
    /// 2 µs to less than 4 µs
    Ge2usAndLt4us,
    /// 4 µs to less than 8 µs
    Ge4usAndLt8us,
    /// 8 µs to less than 16 µs
    Ge8usAndLt16us,
    /// 16 µs to less than 32 µs
    Ge16usAndLt32us,
    /// 32 µs-64 µs
    Ge32usAndLt64us,
    /// More than 64 µs
    Gt64ns,
}
impl From<u8> for L1ExitLatency {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Lt1us,
            0b001 => Self::Ge1usAndLt2us,
            0b010 => Self::Ge2usAndLt4us,
            0b011 => Self::Ge4usAndLt8us,
            0b100 => Self::Ge8usAndLt16us,
            0b101 => Self::Ge16usAndLt32us,
            0b110 => Self::Ge32usAndLt64us,
            0b111 => Self::Gt64ns,
                _ => unreachable!(),
        }
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LinkControlProto {
    active_state_power_management_control: B2,
    rsvdp: B1,
    read_completion_boundary: bool,
    link_disable: bool,
    retrain_link: bool,
    common_clock_configuration: bool,
    extended_synch: bool,
    enable_clock_power_management: bool,
    hardware_autonomous_width_disable: bool,
    link_bandwidth_management_interrupt_enable: bool,
    link_autonomous_bandwidth_interrupt_enable: bool,
    rsvdp_2: B4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkControl {
    /// Active State Power Management (ASPM) Control
    pub active_state_power_management_control: ActiveStatePowerManagement,
    /// Read Completion Boundary (RCB)
    pub read_completion_boundary: ReadCompletionBoundary,
    /// Link Disable
    pub link_disable: bool,
    /// Retrain Link
    pub retrain_link: bool,
    /// Common Clock Configuration
    pub common_clock_configuration: bool,
    /// Extended Synch
    pub extended_synch: bool,
    /// Enable Clock Power Management
    pub enable_clock_power_management: bool,
    /// Hardware Autonomous Width Disable
    pub hardware_autonomous_width_disable: bool,
    /// Link Bandwidth Management Interrupt Enable
    pub link_bandwidth_management_interrupt_enable: bool,
    /// Link Autonomous Bandwidth Interrupt Enable
    pub link_autonomous_bandwidth_interrupt_enable: bool,
}
impl From<LinkControlProto> for LinkControl {
    fn from(proto: LinkControlProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            active_state_power_management_control: proto.active_state_power_management_control().into(),
            read_completion_boundary: proto.read_completion_boundary().into(),
            link_disable: proto.link_disable(),
            retrain_link: proto.retrain_link(),
            common_clock_configuration: proto.common_clock_configuration(),
            extended_synch: proto.extended_synch(),
            enable_clock_power_management: proto.enable_clock_power_management(),
            hardware_autonomous_width_disable: proto.hardware_autonomous_width_disable(),
            link_bandwidth_management_interrupt_enable: proto.link_bandwidth_management_interrupt_enable(),
            link_autonomous_bandwidth_interrupt_enable: proto.link_autonomous_bandwidth_interrupt_enable(),
        }
    }
}
impl From<u16> for LinkControl {
    fn from(word: u16) -> Self { LinkControlProto::from(word).into() }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadCompletionBoundary {
    B64 = 64,
    B128 = 128,
}
impl From<bool> for ReadCompletionBoundary {
    fn from(b: bool) -> Self { if b { Self::B128 } else { Self::B64 } }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LinkStatusProto {
    current_link_speed: B4,
    negotiated_link_width: B6,
    link_training_error: bool,
    link_training: bool,
    slot_clock_configuration: bool,
    data_link_layer_link_active: bool,
    link_bandwidth_management_status: bool,
    link_autonomous_bandwidth_status: bool,
}

/// The Link Status register provides information about PCI Express Link specific parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkStatus {
    /// Current Link Speed
    pub current_link_speed: LinkSpeed,
    /// Negotiated Link Width
    pub negotiated_link_width: LinkWidth,
    /// Undefined
    pub link_training_error: bool,
    /// Link Training
    pub link_training: bool,
    /// Slot Clock Configuration
    pub slot_clock_configuration: bool,
    /// Data Link Layer Link Active
    pub data_link_layer_link_active: bool,
    /// Link Bandwidth Management Status
    pub link_bandwidth_management_status: bool,
    /// Link Autonomous Bandwidth Status
    pub link_autonomous_bandwidth_status: bool,
}
impl From<LinkStatusProto> for LinkStatus {
    fn from(proto: LinkStatusProto) -> Self {
        Self {
            current_link_speed: proto.current_link_speed().into(),
            negotiated_link_width: proto.negotiated_link_width().into(),
            link_training_error: proto.link_training_error(),
            link_training: proto.link_training(),
            slot_clock_configuration: proto.slot_clock_configuration(),
            data_link_layer_link_active: proto.data_link_layer_link_active(),
            link_bandwidth_management_status: proto.link_bandwidth_management_status(),
            link_autonomous_bandwidth_status: proto.link_autonomous_bandwidth_status(),
        }
    }
}
impl From<u16> for LinkStatus {
    fn from(word: u16) -> Self { LinkStatusProto::from(word).into() }
}


/// Slot Capabilities, Slot Status, and Slot Control registers are required for Switch Downstream
/// and Root Ports if a slot is implemented on the Port (indicated by the Slot Implemented bit in
/// the PCI Express Capabilities register)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot {
    pub capabilities: SlotCapabilities,
    pub control: SlotControl,
    pub status: SlotStatus,
}
impl Slot {
    pub fn new(capabilities: u32, control: u16, status: u16) -> Self {
        Self {
            capabilities: capabilities.into(),
            control: control.into(),
            status: status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SlotOption(Option<Slot>);
impl<'a> TryRead<'a, Endian> for SlotOption {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
            let capabilities = bytes.read_with::<u32>(offset, endian)?;
            let control = bytes.read_with::<u16>(offset, endian)?;
            let status = bytes.read_with::<u16>(offset, endian)?;
        if (0, 0, 0) == (capabilities, control, status) {
            Ok((Self(None), *offset))
        } else {
            Ok((Self(Some(Slot::new(capabilities, control, status))), *offset))
        }
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct SlotCapabilitiesProto {
    attention_button_present: bool,
    power_controller_present: bool,
    mrl_sensor_present: bool,
    attention_indicator_present: bool,
    power_indicator_present: bool,
    hot_plug_surprise: bool,
    hot_plug_capable: bool,
    slot_power_limit_value: u8,
    slot_power_limit_scale: B2,
    electromechanical_interlock_present: bool,
    no_command_completed_support: bool,
    physical_slot_number: B13,
}

/// The Slot Capabilities register identifies PCI Express slot specific capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotCapabilities {
    /// Attention Button Present
    pub attention_button_present: bool,
    /// Power Controller Present
    pub power_controller_present: bool,
    /// MRL Sensor Present
    pub mrl_sensor_present: bool,
    /// Attention Indicator Present
    pub attention_indicator_present: bool,
    /// Power Indicator Present
    pub power_indicator_present: bool,
    /// Hot-Plug Surprise
    pub hot_plug_surprise: bool,
    /// Hot-Plug Capable
    pub hot_plug_capable: bool,
    /// Slot Power Limit
    pub slot_power_limit: SlotPowerLimit,
    /// Electromechanical Interlock Present
    pub electromechanical_interlock_present: bool,
    /// No Command Completed Support
    pub no_command_completed_support: bool,
    /// Physical Slot Number
    pub physical_slot_number: u16,
}
impl From<SlotCapabilitiesProto> for SlotCapabilities {
    fn from(proto: SlotCapabilitiesProto) -> Self {
        Self {
            attention_button_present: proto.attention_button_present(),
            power_controller_present: proto.power_controller_present(),
            mrl_sensor_present: proto.mrl_sensor_present(),
            attention_indicator_present: proto.attention_indicator_present(),
            power_indicator_present: proto.power_indicator_present(),
            hot_plug_surprise: proto.hot_plug_surprise(),
            hot_plug_capable: proto.hot_plug_capable(),
            slot_power_limit: SlotPowerLimit::new(
                proto.slot_power_limit_value(),
                proto.slot_power_limit_scale(),
            ),
            electromechanical_interlock_present: proto.electromechanical_interlock_present(),
            no_command_completed_support: proto.no_command_completed_support(),
            physical_slot_number: proto.physical_slot_number(),
        }
    }
}
impl From<u32> for SlotCapabilities {
    fn from(dword: u32) -> Self { SlotCapabilitiesProto::from(dword).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SlotControlProto {
    attention_button_pressed_enable: bool,
    power_fault_detected_enable: bool,
    mrl_sensor_changed_enable: bool,
    presence_detect_changed_enable: bool,
    command_completed_interrupt_enable: bool,
    hot_plug_interrupt_enable: bool,
    attention_indicator_control: B2,
    power_indicator_control: B2,
    power_controller_control: bool,
    electromechanical_interlock_control: bool,
    data_link_layer_state_changed_enable: bool,
    rsvdp: B3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotControl {
    /// Attention Button Pressed Enable
    pub attention_button_pressed_enable: bool,
    /// Power Fault Detected Enable
    pub power_fault_detected_enable: bool,
    /// MRL Sensor Changed Enable
    pub mrl_sensor_changed_enable: bool,
    /// Presence Detect Changed Enable
    pub presence_detect_changed_enable: bool,
    /// Command Completed Interrupt Enable
    pub command_completed_interrupt_enable: bool,
    /// Hot-Plug Interrupt Enable
    pub hot_plug_interrupt_enable: bool,
    /// Attention Indicator Control
    pub attention_indicator_control: IndicatorControl,
    /// Power Indicator Control
    pub power_indicator_control: IndicatorControl,
    /// Power Controller Control
    pub power_controller_control: bool,
    /// Electromechanical Interlock Control
    pub electromechanical_interlock_control: bool,
    /// Data Link Layer State Changed Enable
    pub data_link_layer_state_changed_enable: bool,
}
impl From<SlotControlProto> for SlotControl {
    fn from(proto: SlotControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            attention_button_pressed_enable: proto.attention_button_pressed_enable(),
            power_fault_detected_enable: proto.power_fault_detected_enable(),
            mrl_sensor_changed_enable: proto.mrl_sensor_changed_enable(),
            presence_detect_changed_enable: proto.presence_detect_changed_enable(),
            command_completed_interrupt_enable: proto.command_completed_interrupt_enable(),
            hot_plug_interrupt_enable: proto.hot_plug_interrupt_enable(),
            attention_indicator_control: proto.attention_indicator_control().into(),
            power_indicator_control: proto.power_indicator_control().into(),
            power_controller_control: proto.power_controller_control(),
            electromechanical_interlock_control: proto.electromechanical_interlock_control(),
            data_link_layer_state_changed_enable: proto.data_link_layer_state_changed_enable(),
        }
    }
}
impl From<u16> for SlotControl {
    fn from(word: u16) -> Self { SlotControlProto::from(word).into() }
}


/// Attention/Power Indicator Control 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndicatorControl {
    Reserved,
    On,
    Blink,
    Off,
}
impl From<u8> for IndicatorControl {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Reserved,
            0b01 => Self::On,
            0b10 => Self::Blink,
            0b11 => Self::Off,
               _ => unreachable!(),
        }
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SlotStatusProto {
    attention_button_pressed: bool,
    power_fault_detected: bool,
    mrl_sensor_changed: bool,
    presence_detect_changed: bool,
    command_completed: bool,
    mrl_sensor_state: bool,
    presence_detect_state: bool,
    electromechanical_interlock_status: bool,
    data_link_layer_state_changed: bool,
    rsvdz: B7,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotStatus {
    /// Attention Button Pressed
    pub attention_button_pressed: bool,
    /// Power Fault Detected
    pub power_fault_detected: bool,
    /// MRL Sensor Changed
    pub mrl_sensor_changed: bool,
    /// Presence Detect Changed
    pub presence_detect_changed: bool,
    /// Command Completed
    pub command_completed: bool,
    /// MRL Sensor State
    pub mrl_sensor_state: bool,
    /// Presence Detect State
    pub presence_detect_state: bool,
    /// Electromechanical Interlock Status
    pub electromechanical_interlock_status: bool,
    /// Data Link Layer State Changed
    pub data_link_layer_state_changed: bool,
}
impl From<SlotStatusProto> for SlotStatus {
    fn from(proto: SlotStatusProto) -> Self {
        let _ = proto.rsvdz();
        Self {
            attention_button_pressed: proto.attention_button_pressed(),
            power_fault_detected: proto.power_fault_detected(),
            mrl_sensor_changed: proto.mrl_sensor_changed(),
            presence_detect_changed: proto.presence_detect_changed(),
            command_completed: proto.command_completed(),
            mrl_sensor_state: proto.mrl_sensor_state(),
            presence_detect_state: proto.presence_detect_state(),
            electromechanical_interlock_status: proto.electromechanical_interlock_status(),
            data_link_layer_state_changed: proto.data_link_layer_state_changed(),
        }
    }
}
impl From<u16> for SlotStatus {
    fn from(word: u16) -> Self { SlotStatusProto::from(word).into() }
}


/// Root Ports and Root Complex Event Collectors must implement the Root Capabilities, Root Status,
/// and Root Control registers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Root {
    pub control: RootControl,
    pub capabilities: RootCapabilities,
    pub status: RootStatus,
}
impl Root {
    pub fn new(capabilities: u16, control: u16, status: u32) -> Self {
        Self {
            capabilities: capabilities.into(),
            control: control.into(),
            status: status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RootOption(Option<Root>);
impl<'a> TryRead<'a, Endian> for RootOption {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        // Root structure has different filed order
        let control = bytes.read_with::<u16>(offset, endian)?;
        let capabilities = bytes.read_with::<u16>(offset, endian)?;
        let status = bytes.read_with::<u32>(offset, endian)?;
        if (0, 0, 0) == (capabilities, control, status) {
            Ok((Self(None), *offset))
        } else {
            Ok((Self(Some(Root::new(capabilities, control, status))), *offset))
        }
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct RootControlProto {
    system_error_on_correctable_error_enable: bool,
    system_error_on_non_fatal_error_enable: bool,
    system_error_on_fatal_error_enable: bool,
    pme_interrupt_enable: bool,
    crs_software_visibility_enable: bool,
    rsvdp: B11,
}

/// The Root Control register controls PCI Express Root Complex specific parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootControl {
    /// System Error on Correctable Error Enable
    pub system_error_on_correctable_error_enable: bool,
    /// System Error on Non-Fatal Error Enable
    pub system_error_on_non_fatal_error_enable: bool,
    /// System Error on Fatal Error Enable
    pub system_error_on_fatal_error_enable: bool,
    /// PME Interrupt Enable
    pub pme_interrupt_enable: bool,
    /// CRS Software Visibility Enable
    pub crs_software_visibility_enable: bool,
}
impl From<RootControlProto> for RootControl {
    fn from(proto: RootControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            system_error_on_correctable_error_enable: proto.system_error_on_correctable_error_enable(),
            system_error_on_non_fatal_error_enable: proto.system_error_on_non_fatal_error_enable(),
            system_error_on_fatal_error_enable: proto.system_error_on_fatal_error_enable(),
            pme_interrupt_enable: proto.pme_interrupt_enable(),
            crs_software_visibility_enable: proto.crs_software_visibility_enable(),
        }
    }
}
impl From<u16> for RootControl {
    fn from(word: u16) -> Self { RootControlProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct RootCapabilitiesProto {
    crs_software_visibility: bool,
    rsvdp: B15,
}

/// The Root Capabilities register identifies PCI Express Root Port specific capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootCapabilities {
    /// CRS Software Visibility
    pub crs_software_visibility: bool,
}
impl From<RootCapabilitiesProto> for RootCapabilities {
    fn from(proto: RootCapabilitiesProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            crs_software_visibility: proto.crs_software_visibility(),
        }
    }
}
impl From<u16> for RootCapabilities {
    fn from(word: u16) -> Self { RootCapabilitiesProto::from(word).into() }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct RootStatusProto {
    pme_requester_id: u16,
    pme_status: bool,
    pme_pending: bool,
    rsvdz: B14,
}

/// The Root Status register provides information about PCI Express device specific parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootStatus {
    /// PME Requester ID
    pub pme_requester_id: u16,
    /// PME Status
    pub pme_status: bool,
    /// PME Pending
    pub pme_pending: bool,
}
impl From<RootStatusProto> for RootStatus {
    fn from(proto: RootStatusProto) -> Self {
        let _ = proto.rsvdz();
        Self {
            pme_requester_id: proto.pme_requester_id(),
            pme_status: proto.pme_status(),
            pme_pending: proto.pme_pending(),
        }
    }
}
impl From<u32> for RootStatus {
    fn from(dword: u32) -> Self { RootStatusProto::from(dword).into() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device2 {
    pub capabilities: DeviceCapabilities2,
    pub control: DeviceControl2,
    pub status: DeviceStatus2,
}
impl Device2 {
    pub fn new(capabilities: u32, control: u16, status: u16) -> Self {
        Self {
            capabilities: capabilities.into(),
            control: control.into(),
            status: status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Device2Option(Option<Device2>);
impl<'a> TryRead<'a, Endian> for Device2Option {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let capabilities = bytes.read_with::<u32>(offset, endian)?;
        let control = bytes.read_with::<u16>(offset, endian)?;
        let status = bytes.read_with::<u16>(offset, endian)?;
        if (0, 0, 0) == (capabilities, control, status) {
            Ok((Self(None), *offset))
        } else {
            Ok((Self(Some(Device2::new(capabilities, control, status))), *offset))
        }
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct DeviceCapabilities2Proto {
    completion_timeout_ranges_supported: B4,
    completion_timeout_disable_supported: bool,
    ari_forwarding_supported: bool,
    atomic_op_routing_supported: bool,
    u32_atomicop_completer_supported: bool,
    u64_atomicop_completer_supported: bool,
    u128_cas_completer_supported: bool,
    no_ro_enabled_pr_pr_passing: bool,
    ltr_mechanism_supported: bool,
    tph_completer_supported: B2,
    ln_system_cls: B2,
    support_10bit_tag_completer: bool,
    support_10bit_tag_requester: bool,
    obff_supported: B2,
    extended_fmt_field_supported: bool,
    end_end_tlp_prefix_supported: bool,
    max_end_end_tlp_prefixes: B2,
    emergency_power_reduction_supported: B2,
    emergency_power_reduction_initialization_required: bool,
    rsvdp: B4,
    frs_supported: bool,
}
impl<'a> From<&'a DeviceCapabilities2> for DeviceCapabilities2Proto {
    fn from(data: &DeviceCapabilities2) -> Self {
        Self::new()
            .with_completion_timeout_ranges_supported(data.completion_timeout_ranges_supported.clone().into())
            .with_completion_timeout_disable_supported(data.completion_timeout_disable_supported)
            .with_ari_forwarding_supported(data.ari_forwarding_supported)
            .with_atomic_op_routing_supported(data.atomic_op_routing_supported)
            .with_u32_atomicop_completer_supported(data.u32_atomicop_completer_supported)
            .with_u64_atomicop_completer_supported(data.u64_atomicop_completer_supported)
            .with_u128_cas_completer_supported(data.u128_cas_completer_supported)
            .with_no_ro_enabled_pr_pr_passing(data.no_ro_enabled_pr_pr_passing)
            .with_ltr_mechanism_supported(data.ltr_mechanism_supported)
            .with_tph_completer_supported(data.tph_completer_supported.clone().into())
            .with_ln_system_cls(data.ln_system_cls.clone().into())
            .with_support_10bit_tag_completer(data.support_10bit_tag_completer)
            .with_support_10bit_tag_requester(data.support_10bit_tag_requester)
            .with_obff_supported(data.obff_supported.clone().into())
            .with_extended_fmt_field_supported(data.extended_fmt_field_supported)
            .with_end_end_tlp_prefix_supported(data.end_end_tlp_prefix_supported)
            .with_max_end_end_tlp_prefixes(data.max_end_end_tlp_prefixes.into())
            .with_emergency_power_reduction_supported(data.emergency_power_reduction_supported.clone().into())
            .with_emergency_power_reduction_initialization_required(
                data.emergency_power_reduction_initialization_required
            )
            .with_rsvdp(0)
            .with_frs_supported(data.frs_supported)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCapabilities2  {
    /// Completion Timeout Ranges Supported
    pub completion_timeout_ranges_supported: CompletionTimeoutRanges,
    /// Completion Timeout Disable Supported
    pub completion_timeout_disable_supported: bool,
    /// ARI Forwarding Supported
    pub ari_forwarding_supported: bool,
    /// AtomicOp Routing Supported
    pub atomic_op_routing_supported: bool,
    /// 32-bit AtomicOp Completer Supported
    pub u32_atomicop_completer_supported: bool,
    /// 64-bit AtomicOp Completer Supported
    pub u64_atomicop_completer_supported: bool,
    /// 128-bit CAS Completer Supported
    pub u128_cas_completer_supported: bool,
    /// No RO-enabled PR-PR Passing
    pub no_ro_enabled_pr_pr_passing: bool,
    /// LTR Mechanism Supported
    pub ltr_mechanism_supported: bool,
    /// TPH Completer Supported
    pub tph_completer_supported: TphCompleter,
    /// LN System CLS
    pub ln_system_cls: LnSystemCls,
    /// 10-Bit Tag Completer Supported
    pub support_10bit_tag_completer: bool,
    /// 10-Bit Tag Requester Supported
    pub support_10bit_tag_requester: bool,
    /// OBFF Supported
    pub obff_supported: Obff,
    /// Extended Fmt Field Supported
    pub extended_fmt_field_supported: bool,
    /// End-End TLP Prefix Supported
    pub end_end_tlp_prefix_supported: bool,
    /// Max End-End TLP Prefixes
    pub max_end_end_tlp_prefixes: MaxEndEndTlpPrefixes,
    /// Emergency Power Reduction Supported
    pub emergency_power_reduction_supported: EmergencyPowerReduction,
    /// Emergency Power Reduction Initialization Required
    pub emergency_power_reduction_initialization_required: bool,
    /// FRS Supported
    pub frs_supported: bool,
}
impl From<DeviceCapabilities2Proto> for DeviceCapabilities2 {
    fn from(proto: DeviceCapabilities2Proto) -> Self {
        let _ = proto.rsvdp();
        Self {
            completion_timeout_ranges_supported: proto.completion_timeout_ranges_supported().into(),
            completion_timeout_disable_supported: proto.completion_timeout_disable_supported(),
            ari_forwarding_supported: proto.ari_forwarding_supported(),
            atomic_op_routing_supported: proto.atomic_op_routing_supported(),
            u32_atomicop_completer_supported: proto.u32_atomicop_completer_supported(),
            u64_atomicop_completer_supported: proto.u64_atomicop_completer_supported(),
            u128_cas_completer_supported: proto.u128_cas_completer_supported(),
            no_ro_enabled_pr_pr_passing: proto.no_ro_enabled_pr_pr_passing(),
            ltr_mechanism_supported: proto.ltr_mechanism_supported(),
            tph_completer_supported: proto.tph_completer_supported().into(),
            ln_system_cls: proto.ln_system_cls().into(),
            support_10bit_tag_completer: proto.support_10bit_tag_completer(),
            support_10bit_tag_requester: proto.support_10bit_tag_requester(),
            obff_supported: proto.obff_supported().into(),
            extended_fmt_field_supported: proto.extended_fmt_field_supported(),
            end_end_tlp_prefix_supported: proto.end_end_tlp_prefix_supported(),
            max_end_end_tlp_prefixes: proto.max_end_end_tlp_prefixes().into(),
            emergency_power_reduction_supported: proto.emergency_power_reduction_supported().into(),
            emergency_power_reduction_initialization_required:
                proto.emergency_power_reduction_initialization_required(),
            frs_supported: proto.frs_supported(),
        }
    }
}
impl From<u32> for DeviceCapabilities2 {
    fn from(dword: u32) -> Self { DeviceCapabilities2Proto::from(dword).into() }
}
impl<'a> From<&'a DeviceCapabilities2> for u32 {
    fn from(data: &DeviceCapabilities2) -> Self { DeviceCapabilities2Proto::from(data).into() }
}


/// Indicates device Function support for the optional Completion Timeout programmability mechanism
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionTimeoutRanges {
    /// Completion Timeout programming not supported – the Function must implement a timeout value
    /// in the range 50 µs to 50 ms.
    NotSupported,
    /// Range A
    RangeA,
    /// Range B
    RangeB,
    /// Ranges A and B
    RangesAB,
    /// Ranges B and C
    RangesBC,
    /// Ranges A, B, and C
    RangesABC,
    /// Ranges B, C, and D
    RangesBCD,
    /// Ranges A, B, C, and D 
    RangesABCD,
    /// Reserved
    Reserved(u8),
}
impl CompletionTimeoutRanges {
    pub const A: Range<f64> = 50e-6 .. 10e-3;
    pub const B: Range<f64> = 10e-3 .. 250e-3;
    pub const C: Range<f64> = 250e-6 .. 4.0;
    pub const D: Range<f64> = 4.0 .. 64.0;
}
impl From<u8> for CompletionTimeoutRanges {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::NotSupported,
            0b0001 => Self::RangeA,
            0b0010 => Self::RangeB,
            0b0011 => Self::RangesAB,
            0b0110 => Self::RangesBC,
            0b0111 => Self::RangesABC,
            0b1110 => Self::RangesBCD,
            0b1111 => Self::RangesABCD,
                 v => Self::Reserved(v),
        }
    }
}
impl From<CompletionTimeoutRanges> for u8 {
    fn from(data: CompletionTimeoutRanges) -> Self {
        match data {
            CompletionTimeoutRanges::NotSupported => 0b0000,
            CompletionTimeoutRanges::RangeA       => 0b0001,
            CompletionTimeoutRanges::RangeB       => 0b0010,
            CompletionTimeoutRanges::RangesAB     => 0b0011,
            CompletionTimeoutRanges::RangesBC     => 0b0110,
            CompletionTimeoutRanges::RangesABC    => 0b0111,
            CompletionTimeoutRanges::RangesBCD    => 0b1110,
            CompletionTimeoutRanges::RangesABCD   => 0b1111,
            CompletionTimeoutRanges::Reserved(v)  => v,
        }
    }
}

/// Value indicates Completer support for TPH or Extended TPH
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TphCompleter {
    /// TPH and Extended TPH Completer not supported.
    NotSupported,
    /// TPH Completer supported; Extended TPH Completer not supported.
    Tph,
    /// Reserved.
    Reserved,
    /// Both TPH and Extended TPH Completer supported.
    TphAndExtendedTph,
}
impl From<u8> for TphCompleter {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NotSupported,      
            0b01 => Self::Tph,               
            0b10 => Self::Reserved,          
            0b11 => Self::TphAndExtendedTph, 
               _ => unreachable!(),
        }
    }
}
impl From<TphCompleter> for u8 {
    fn from(data: TphCompleter) -> Self {
        match data {
            TphCompleter::NotSupported      => 0b00,
            TphCompleter::Tph               => 0b01,
            TphCompleter::Reserved          => 0b10,
            TphCompleter::TphAndExtendedTph => 0b11,
        }
    }
}

/// Indicates if the Root Port or RCRB supports LN protocol as an LN Completer, and if so, what
/// cacheline size is in effect
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LnSystemCls {
    /// LN Completer either not supported or not in effect
    NotSupported,
    /// LN Completer with 64-byte cachelines in effect
    Cachelines64Byte,
    /// LN Completer with 128-byte cachelines in effect
    Cachelines128Byte,
    /// Reserved
    Reserved,
}
impl From<u8> for LnSystemCls {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NotSupported,
            0b01 => Self::Cachelines64Byte,
            0b10 => Self::Cachelines128Byte,
            0b11 => Self::Reserved,
               _ => unreachable!(),
        }
    }
}
impl From<LnSystemCls> for u8 {
    fn from(data: LnSystemCls) -> Self {
        match data {
            LnSystemCls::NotSupported      => 0b00,
            LnSystemCls::Cachelines64Byte  => 0b01,
            LnSystemCls::Cachelines128Byte => 0b10,
            LnSystemCls::Reserved          => 0b11,
        }
    }
}

/// Indicates if OBFF is supported and, if so, what signaling mechanism is used
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Obff {
    /// OBFF Not Supported
    NotSupported,
    /// OBFF supported using Message signaling only
    Message,
    /// OBFF supported using WAKE# signaling only
    Wake,
    /// OBFF supported using WAKE# and Message signaling 
    WakeAndMessage,
}
impl From<u8> for Obff {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NotSupported,
            0b01 => Self::Message,
            0b10 => Self::Wake,
            0b11 => Self::WakeAndMessage,
               _ => unreachable!(),
        }
    }
}
impl From<Obff> for u8 {
    fn from(data: Obff) -> Self {
        match data {
            Obff::NotSupported   => 0b00,
            Obff::Message        => 0b01,
            Obff::Wake           => 0b10,
            Obff::WakeAndMessage => 0b11,
        }
    }
}

/// Indicates the maximum number of End-End TLP Prefixes supported by this Function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxEndEndTlpPrefixes {
    /// 4 End-End TLP Prefixes
    Max4,
    /// 1 End-End TLP Prefix
    Max1,
    /// 2 End-End TLP Prefixes
    Max2,
    /// 3 End-End TLP Prefixes
    Max3,
}
impl From<u8> for MaxEndEndTlpPrefixes {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Max4,
            0b01 => Self::Max1,
            0b10 => Self::Max2,
            0b11 => Self::Max3,
               _ => unreachable!(),
        }
    }
}
impl From<MaxEndEndTlpPrefixes> for u8 {
    fn from(data: MaxEndEndTlpPrefixes) -> Self {
        match data {
            MaxEndEndTlpPrefixes::Max4 => 0b00,
            MaxEndEndTlpPrefixes::Max1 => 0b01,
            MaxEndEndTlpPrefixes::Max2 => 0b10,
            MaxEndEndTlpPrefixes::Max3 => 0b11,
        }
    }
}

/// Indicates support level of the optional Emergency Power Reduction State feature
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmergencyPowerReduction {
    /// Emergency Power Reduction State not supported
    NotSupported,
    /// Emergency Power Reduction State is supported and is triggered by Device Specific
    /// mechanism(s)
    DeviceSpecific,
    /// Emergency Power Reduction State is supported and is triggered either by the mechanism
    /// defined in the corresponding Form Factor specification or by Device Specific mechanism(s)
    FormFactorOrDeviceSpecific,
    /// Reserved
    Reserved,
}
impl From<u8> for EmergencyPowerReduction {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NotSupported,
            0b01 => Self::DeviceSpecific,
            0b10 => Self::FormFactorOrDeviceSpecific,
            0b11 => Self::Reserved,
               _ => unreachable!(),
        }
    }
}
impl From<EmergencyPowerReduction> for u8 {
    fn from(data: EmergencyPowerReduction) -> Self {
        match data {
             EmergencyPowerReduction::NotSupported               => 0b00,
             EmergencyPowerReduction::DeviceSpecific             => 0b01,
             EmergencyPowerReduction::FormFactorOrDeviceSpecific => 0b10,
             EmergencyPowerReduction::Reserved                   => 0b11,
        }
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DeviceControl2Proto {
    completion_timeout_value: B4,
    completion_timeout_disable: bool,
    ari_forwarding_enable: bool,
    atomic_op_requester_enable: bool,
    atomic_op_egress_blocking: bool,
    ido_request_enable: bool,
    ido_completion_enable: bool,
    ltr_mechanism_enable: bool,
    emergency_power_reduction_request: bool,
    enable_10bit_tag_requester: bool,
    obff_enable: B2,
    end_end_tlp_prefix_blocking: bool,
}
impl<'a> From<&'a DeviceControl2> for DeviceControl2Proto {
    fn from(data: &DeviceControl2) -> Self {
        Self::new()
            .with_completion_timeout_value(data.completion_timeout_value.clone().into())
            .with_completion_timeout_disable(data.completion_timeout_disable)
            .with_ari_forwarding_enable(data.ari_forwarding_enable)
            .with_atomic_op_requester_enable(data.atomic_op_requester_enable)
            .with_atomic_op_egress_blocking(data.atomic_op_egress_blocking)
            .with_ido_request_enable(data.ido_request_enable)
            .with_ido_completion_enable(data.ido_completion_enable)
            .with_ltr_mechanism_enable(data.ltr_mechanism_enable)
            .with_emergency_power_reduction_request(data.emergency_power_reduction_request)
            .with_enable_10bit_tag_requester(data.enable_10bit_tag_requester)
            .with_obff_enable(data.obff_enable.clone().into())
            .with_end_end_tlp_prefix_blocking(data.end_end_tlp_prefix_blocking.clone().into())
    }
}

/// Device Control 2 Register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceControl2  {
    /// Completion Timeout Value
    pub completion_timeout_value: CompletionTimeoutValue,
    /// Completion Timeout Disable
    pub completion_timeout_disable: bool,
    /// ARI Forwarding Enable
    pub ari_forwarding_enable: bool,
    /// AtomicOp Requester Enable
    pub atomic_op_requester_enable: bool,
    /// AtomicOp Egress Blocking
    pub atomic_op_egress_blocking: bool,
    /// IDO Request Enable
    pub ido_request_enable: bool,
    /// IDO Completion Enable
    pub ido_completion_enable: bool,
    /// LTR Mechanism Enable
    pub ltr_mechanism_enable: bool,
    /// Emergency Power Reduction Request
    pub emergency_power_reduction_request: bool,
    /// 10-Bit Tag Requester Enable
    pub enable_10bit_tag_requester: bool,
    /// OBFF Enable
    pub obff_enable: ObffEnable,
    /// End-End TLP Prefix Blocking
    pub end_end_tlp_prefix_blocking: EndEndTlpPrefixBlocking,
}
impl From<DeviceControl2Proto> for DeviceControl2 {
    fn from(proto: DeviceControl2Proto) -> Self {
        Self {
            completion_timeout_value: proto.completion_timeout_value().into(),
            completion_timeout_disable: proto.completion_timeout_disable(),
            ari_forwarding_enable: proto.ari_forwarding_enable(),
            atomic_op_requester_enable: proto.atomic_op_requester_enable(),
            atomic_op_egress_blocking: proto.atomic_op_egress_blocking(),
            ido_request_enable: proto.ido_request_enable(),
            ido_completion_enable: proto.ido_completion_enable(),
            ltr_mechanism_enable: proto.ltr_mechanism_enable(),
            emergency_power_reduction_request: proto.emergency_power_reduction_request(),
            enable_10bit_tag_requester: proto.enable_10bit_tag_requester(),
            obff_enable: proto.obff_enable().into(),
            end_end_tlp_prefix_blocking: proto.end_end_tlp_prefix_blocking().into(),
        }
    }
}
impl From<u16> for DeviceControl2 {
    fn from(word: u16) -> Self { DeviceControl2Proto::from(word).into() }
}
impl<'a> From<&'a DeviceControl2> for u16 {
    fn from(data: &DeviceControl2) -> Self { DeviceControl2Proto::from(data).into() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionTimeoutValue {
    /// Default range: 50 µs to 50 ms
    DefaultRange50usTo50ms,
    /// 50 µs to 100 µs
    RangeA50usTo100us,
    /// 1 ms to 10 ms
    RangeA1msTo10ms,
    /// 16 ms to 55 ms
    RangeB16msTo55mss,
    /// 65 ms to 210 ms
    RangeB65msTo210ms,
    /// 260 ms to 900 ms
    RangeC260msTo900ms,
    /// 1 s to 3.5 s
    RangeC1000msTo3500ms,
    /// 4 s to 13 s
    RangeD4sTo13s,
    /// 17 s to 64 s
    RangeD17sTo64s,
    Reserved(u8),
}
impl From<u8> for CompletionTimeoutValue {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::DefaultRange50usTo50ms,
            0b0001 => Self::RangeA50usTo100us,
            0b0010 => Self::RangeA1msTo10ms,
            0b0101 => Self::RangeB16msTo55mss,
            0b0110 => Self::RangeB65msTo210ms,
            0b1001 => Self::RangeC260msTo900ms,
            0b1010 => Self::RangeC1000msTo3500ms,
            0b1101 => Self::RangeD4sTo13s,
            0b1110 => Self::RangeD17sTo64s,
            v => Self::Reserved(v),
        }
    }
}
impl From<CompletionTimeoutValue> for u8 {
    fn from(data: CompletionTimeoutValue) -> Self {
        match data {
            CompletionTimeoutValue::DefaultRange50usTo50ms => 0b0000,
            CompletionTimeoutValue::RangeA50usTo100us      => 0b0001,
            CompletionTimeoutValue::RangeA1msTo10ms        => 0b0010,
            CompletionTimeoutValue::RangeB16msTo55mss      => 0b0101,
            CompletionTimeoutValue::RangeB65msTo210ms      => 0b0110,
            CompletionTimeoutValue::RangeC260msTo900ms     => 0b1001,
            CompletionTimeoutValue::RangeC1000msTo3500ms   => 0b1010,
            CompletionTimeoutValue::RangeD4sTo13s          => 0b1101,
            CompletionTimeoutValue::RangeD17sTo64s         => 0b1110,
            CompletionTimeoutValue::Reserved(v)            => v,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObffEnable {
    /// Disabled
    Disabled,
    /// Enabled using Message signaling [Variation A]
    MessageSignalingA,
    /// Enabled using Message signaling [Variation B]
    MessageSignalingB,
    /// Enabled using WAKE# signaling
    WakeSignaling,
}
impl From<u8> for ObffEnable {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Disabled,
            0b01 => Self::MessageSignalingA,
            0b10 => Self::MessageSignalingB,
            0b11 => Self::WakeSignaling,
               _ => unreachable!(),
        }
    }
}
impl From<ObffEnable> for u8 {
    fn from(data: ObffEnable) -> Self {
        match data {
            ObffEnable::Disabled          => 0b00,
            ObffEnable::MessageSignalingA => 0b01,
            ObffEnable::MessageSignalingB => 0b10,
            ObffEnable::WakeSignaling     => 0b11,
        }
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DeviceStatus2Proto {
    rsvdz: B16,
}

/// Device Status 2 Register is a placeholder
/// There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStatus2  {
}
impl From<DeviceStatus2Proto> for DeviceStatus2 {
    fn from(proto: DeviceStatus2Proto) -> Self {
        let _ = proto.rsvdz();
        Self {
        }
    }
}
impl From<u16> for DeviceStatus2 {
    fn from(word: u16) -> Self { DeviceStatus2Proto::from(word).into() }
}

/// Controls whether the routing function is permitted to forward TLPs containing an End-End TLP
/// Prefix
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndEndTlpPrefixBlocking {
    /// – Function is permitted to send TLPs with End-End TLP Prefixes
    ForwardingEnabled,
    /// Function is not permitted to send TLPs with End-End TLP Prefixes
    ForwardingBlocked,
}
impl From<bool> for EndEndTlpPrefixBlocking {
    fn from(b: bool) -> Self {
        if b {
            Self::ForwardingBlocked
        } else {
            Self::ForwardingEnabled
        }
    }
}
impl From<EndEndTlpPrefixBlocking> for bool {
    fn from(data: EndEndTlpPrefixBlocking) -> Self {
        match data {
            EndEndTlpPrefixBlocking::ForwardingEnabled => false,
            EndEndTlpPrefixBlocking::ForwardingBlocked => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link2 {
    pub capabilities: LinkCapabilities2,
    pub control: LinkControl2,
    pub status: LinkStatus2,
}
impl Link2 {
    pub fn new(capabilities: u32, control: u16, status: u16) -> Self {
        Self {
            capabilities: capabilities.into(),
            control: control.into(),
            status: status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Link2Option(Option<Link2>);
impl<'a> TryRead<'a, Endian> for Link2Option {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
            let capabilities = bytes.read_with::<u32>(offset, endian)?;
            let control = bytes.read_with::<u16>(offset, endian)?;
            let status = bytes.read_with::<u16>(offset, endian)?;
        if (0, 0, 0) == (capabilities, control, status) {
            Ok((Self(None), *offset))
        } else {
            Ok((Self(Some(Link2::new(capabilities, control, status))), *offset))
        }
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct LinkCapabilities2Proto {
    rsvdp: B1,
    supported_link_speeds_vector: SupportedLinkSpeedsVectorProto,
    crosslink_supported: bool,
    lower_skp_os_generation_supported_speeds_vector: SupportedLinkSpeedsVectorProto,
    lower_skp_os_reception_supported_speeds_vector: SupportedLinkSpeedsVectorProto,
    retimer_presence_detect_supported: bool,
    two_retimers_presence_detect_supported: bool,
    rsvdp_2: B6,
    drs_supported: bool,
}
impl From<LinkCapabilities2> for LinkCapabilities2Proto {
    fn from(data: LinkCapabilities2) -> Self {
        Self::new()
            .with_rsvdp(0)
            .with_supported_link_speeds_vector(data.supported_link_speeds_vector.into())
            .with_crosslink_supported(data.crosslink_supported)
            .with_lower_skp_os_generation_supported_speeds_vector(data.lower_skp_os_generation_supported_speeds_vector.into())
            .with_lower_skp_os_reception_supported_speeds_vector(data.lower_skp_os_reception_supported_speeds_vector.into())
            .with_retimer_presence_detect_supported(data.retimer_presence_detect_supported)
            .with_two_retimers_presence_detect_supported(data.two_retimers_presence_detect_supported)
            .with_rsvdp_2(0)
            .with_drs_supported(data.drs_supported)
    }
}

/// Link Capabilities 2 Register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkCapabilities2  {
    /// Supported Link Speeds Vector
    pub supported_link_speeds_vector: SupportedLinkSpeedsVector,
    /// Crosslink Supported
    pub crosslink_supported: bool,
    /// Lower SKP OS Generation Supported Speeds Vector
    pub lower_skp_os_generation_supported_speeds_vector: SupportedLinkSpeedsVector,
    /// Lower SKP OS Reception Supported Speeds Vector –
    pub lower_skp_os_reception_supported_speeds_vector: SupportedLinkSpeedsVector,
    /// Retimer Presence Detect Supported
    pub retimer_presence_detect_supported: bool,
    /// Two Retimers Presence Detect Supported
    pub two_retimers_presence_detect_supported: bool,
    /// DRS Supported
    pub drs_supported: bool,
}
impl From<LinkCapabilities2Proto> for LinkCapabilities2 {
    fn from(proto: LinkCapabilities2Proto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            supported_link_speeds_vector: proto.supported_link_speeds_vector().into(),
            crosslink_supported: proto.crosslink_supported(),
            lower_skp_os_generation_supported_speeds_vector:
                proto.lower_skp_os_generation_supported_speeds_vector().into(),
            lower_skp_os_reception_supported_speeds_vector:
                proto.lower_skp_os_reception_supported_speeds_vector().into(),
            retimer_presence_detect_supported: proto.retimer_presence_detect_supported(),
            two_retimers_presence_detect_supported: proto.two_retimers_presence_detect_supported(),
            drs_supported: proto.drs_supported(),
        }
    }
}
impl From<u32> for LinkCapabilities2 {
    fn from(dword: u32) -> Self { LinkCapabilities2Proto::from(dword).into() }
}
impl From<LinkCapabilities2> for u32 {
    fn from(data: LinkCapabilities2) -> Self { LinkCapabilities2Proto::from(data).into() }
}

#[bitfield(bits = 7)]
#[derive(BitfieldSpecifier)]
pub struct SupportedLinkSpeedsVectorProto {
    speed_2_5_gtps: bool,
    speed_5_0_gtps: bool,
    speed_8_0_gtps: bool,
    speed_16_0_gtps: bool,
    speed_32_0_gtps: bool,
    speed_64_0_gtps: bool,
    rsvdp: B1,
}
impl From<SupportedLinkSpeedsVector> for SupportedLinkSpeedsVectorProto {
    fn from(data: SupportedLinkSpeedsVector) -> Self {
        Self::new()
            .with_speed_2_5_gtps(data.speed_2_5_gtps)
            .with_speed_5_0_gtps(data.speed_5_0_gtps)
            .with_speed_8_0_gtps(data.speed_8_0_gtps)
            .with_speed_16_0_gtps(data.speed_16_0_gtps)
            .with_speed_32_0_gtps(data.speed_32_0_gtps)
            .with_speed_64_0_gtps(data.speed_64_0_gtps)
            .with_rsvdp(0)
    }
}

/// Indicates the supported Link speed(s) of the associated Port
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedLinkSpeedsVector  {
    /// 2.5 GT/s
    pub speed_2_5_gtps: bool,
    /// 5.0 GT/s
    pub speed_5_0_gtps: bool,
    /// 5.0 GT/s
    pub speed_8_0_gtps: bool,
    /// 16.0 GT/s
    pub speed_16_0_gtps: bool,
    /// 32.0 GT/s
    pub speed_32_0_gtps: bool,
    /// 64.0 GT/s
    pub speed_64_0_gtps: bool,
}
impl From<SupportedLinkSpeedsVectorProto> for SupportedLinkSpeedsVector {
    fn from(proto: SupportedLinkSpeedsVectorProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            speed_2_5_gtps: proto.speed_2_5_gtps(),
            speed_5_0_gtps: proto.speed_5_0_gtps(),
            speed_8_0_gtps: proto.speed_8_0_gtps(),
            speed_16_0_gtps: proto.speed_16_0_gtps(),
            speed_32_0_gtps: proto.speed_32_0_gtps(),
            speed_64_0_gtps: proto.speed_64_0_gtps(),
        }
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LinkControl2Proto {
    target_link_speed: B4,
    enter_compliance: bool,
    hardware_autonomous_speed_disable: bool,
    selectable_de_emphasis: bool,
    transmit_margin: B3,
    enter_modified_compliance: bool,
    compliance_sos: bool,
    compliance_preset_or_de_emphasis: B4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkControl2  {
    /// Target Link Speed
    pub target_link_speed: LinkSpeed,
    /// Enter Compliance
    pub enter_compliance: bool,
    /// Hardware Autonomous Speed Disable
    pub hardware_autonomous_speed_disable: bool,
    pub selectable_de_emphasis: DeEmphasis,
    /// Transmit Margin
    pub transmit_margin: TransmitMargin,
    /// Enter Modified Compliance
    pub enter_modified_compliance: bool,
    /// Compliance SOS
    pub compliance_sos: bool,
    /// Compliance Preset/De-emphasis
    pub compliance_preset_or_de_emphasis: CompliancePresetOrDeEmphasis,
}
impl From<LinkControl2Proto> for LinkControl2 {
    fn from(proto: LinkControl2Proto) -> Self {
        Self {
            target_link_speed: proto.target_link_speed().into(),
            enter_compliance: proto.enter_compliance(),
            hardware_autonomous_speed_disable: proto.hardware_autonomous_speed_disable(),
            selectable_de_emphasis: proto.selectable_de_emphasis().into(),
            transmit_margin: proto.transmit_margin().into(),
            enter_modified_compliance: proto.enter_modified_compliance(),
            compliance_sos: proto.compliance_sos(),
            compliance_preset_or_de_emphasis: proto.compliance_preset_or_de_emphasis().into(),
        }
    }
}
impl From<u16> for LinkControl2 {
    fn from(word: u16) -> Self { LinkControl2Proto::from(word).into() }
}

/// Selectable De-emphasis
///
/// Used to control the transmit deemphasis of the link in specific situations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeEmphasis {
    Minus3_5dB,
    Minus6dB,
}
impl From<bool> for DeEmphasis {
    fn from(b: bool) -> Self {
        if b {
            Self::Minus3_5dB
        } else {
            Self::Minus6dB
        }
    }
}

/// Controls the value of the nondeemphasized voltage level at the Transmitter pins
///
/// 0b000 - Normal operating range
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransmitMargin(pub u8);
impl From<u8> for TransmitMargin {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

/// Compliance Preset/De-emphasis
///
/// - **8.0 GT/s Data Rate:** This field sets the Transmitter Preset in *Polling.Compliance* The
///   encodings are defined by Transmitter Preset. Results are undefined if a reserved preset
///   encoding is used when entering *Polling.Compliance* in this way
/// - **5.0 GT/s Data Rate:** This field sets the de-emphasis level in *Polling.Compliance* state
///   if the entry occurred due to the Enter Compliance bit being 1b. 
/// - **2.5 GT/s Data Rate:** The setting of this field has no effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompliancePresetOrDeEmphasis(pub u8);
impl From<u8> for CompliancePresetOrDeEmphasis {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LinkStatus2Proto {
    current_de_emphasis_level: bool,
    equalization_complete: bool,
    equalization_phase_1_successful: bool,
    equalization_phase_2_successful: bool,
    equalization_phase_3_successful: bool,
    link_equalization_request: bool,
    retimer_presence_detected: bool,
    two_retimers_presence_detected: bool,
    crosslink_resolution: B2,
    rsvdz: B2,
    downstream_component_presence: B3,
    drs_message_received: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkStatus2  {
    /// Current De-emphasis Level
    pub current_de_emphasis_level: DeEmphasis,
    /// Equalization Complete
    pub equalization_complete: bool,
    /// Equalization Phase 1 Successful
    pub equalization_phase_1_successful: bool,
    /// Equalization Phase 2 Successful
    pub equalization_phase_2_successful: bool,
    /// Equalization Phase 3 Successful
    pub equalization_phase_3_successful: bool,
    /// Link Equalization Request
    pub link_equalization_request: bool,
    /// Retimer Presence Detected
    pub retimer_presence_detected: bool,
    /// Two Retimers Presence Detected
    pub two_retimers_presence_detected: bool,
    /// Crosslink Resolution
    pub crosslink_resolution: CrosslinkResolution,
    /// Downstream Component Presence
    pub downstream_component_presence: DownstreamComponentPresence,
    /// DRS Message Received
    pub drs_message_received: bool,
}
impl From<LinkStatus2Proto> for LinkStatus2 {
    fn from(proto: LinkStatus2Proto) -> Self {
        let _ = proto.rsvdz();
        Self {
            current_de_emphasis_level: proto.current_de_emphasis_level().into(),
            equalization_complete: proto.equalization_complete(),
            equalization_phase_1_successful: proto.equalization_phase_1_successful(),
            equalization_phase_2_successful: proto.equalization_phase_2_successful(),
            equalization_phase_3_successful: proto.equalization_phase_3_successful(),
            link_equalization_request: proto.link_equalization_request(),
            retimer_presence_detected: proto.retimer_presence_detected(),
            two_retimers_presence_detected: proto.two_retimers_presence_detected(),
            crosslink_resolution: proto.crosslink_resolution().into(),
            downstream_component_presence: proto.downstream_component_presence().into(),
            drs_message_received: proto.drs_message_received(),
        }
    }
}
impl From<u16> for LinkStatus2 {
    fn from(word: u16) -> Self { LinkStatus2Proto::from(word).into() }
}


/// Indicates the state of the Crosslink negotiation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrosslinkResolution {
    /// Crosslink Resolution is not supported
    NotSupported,
    /// Crosslink negotiation resolved as an Upstream Port
    UpstreamPort,
    /// Crosslink negotiation resolved as a Downstream Port
    DownstreamPort,
    /// Crosslink negotiation is not completed
    NotCompleted,
}
impl From<u8> for CrosslinkResolution {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::NotSupported,
            0b01 => Self::UpstreamPort,
            0b10 => Self::DownstreamPort,
            0b11 => Self::NotCompleted,
            _ => unreachable!(),
        }
    }
}


/// Indicates the presence and DRS status for the Downstream Component, if any, connected to the
/// Link
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownstreamComponentPresence {
    /// Link Down – Presence Not Determined
    DownNotDetermined,
    /// Link Down – Component Not Present
    DownNotPresent,
    /// Link Down – Component Present
    DownPresent,
    /// Link Up – Component Present
    UpPresent,
    /// Link Up – Component Present and DRS Received
    UpPresentAndDrsReceived,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for DownstreamComponentPresence {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::DownNotDetermined,
            0b001 => Self::DownNotPresent,
            0b010 => Self::DownPresent,
            0b100 => Self::UpPresent,
            0b101 => Self::UpPresentAndDrsReceived,
            v => Self::Reserved(v),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot2 {
    pub capabilities: SlotCapabilities2,
    pub control: SlotControl2,
    pub status: SlotStatus2,
}
impl Slot2 {
    pub fn new(capabilities: u32, control: u16, status: u16) -> Self {
        Self {
            capabilities: capabilities.into(),
            control: control.into(),
            status: status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Slot2Option(Option<Slot2>);
impl<'a> TryRead<'a, Endian> for Slot2Option {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
            let capabilities = bytes.read_with::<u32>(offset, endian)?;
            let control = bytes.read_with::<u16>(offset, endian)?;
            let status = bytes.read_with::<u16>(offset, endian)?;
        if (0, 0, 0) == (capabilities, control, status) {
            Ok((Self(None), *offset))
        } else {
            Ok((Self(Some(Slot2::new(capabilities, control, status))), *offset))
        }
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct SlotCapabilities2Proto {
    rsvdp: B32,
}

/// Slot Capabilities 2 Register
///
/// This section is a placeholder. There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotCapabilities2  {
}
impl From<SlotCapabilities2Proto> for SlotCapabilities2 {
    fn from(proto: SlotCapabilities2Proto) -> Self {
        let _ = proto.rsvdp();
        Self {
        }
    }
}
impl From<u32> for SlotCapabilities2 {
    fn from(dword: u32) -> Self { SlotCapabilities2Proto::from(dword).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SlotControl2Proto {
    rsvdp: B16,
}

/// Slot Control 2 Register
///
/// This section is a placeholder. There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotControl2  {
}
impl From<SlotControl2Proto> for SlotControl2 {
    fn from(proto: SlotControl2Proto) -> Self {
        let _ = proto.rsvdp();
        Self {
        }
    }
}
impl From<u16> for SlotControl2 {
    fn from(word: u16) -> Self { SlotControl2Proto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SlotStatus2Proto {
    rsvdz: B16,
}

/// Slot Status 2 Register
///
/// This section is a placeholder. There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotStatus2  {
}
impl From<SlotStatus2Proto> for SlotStatus2 {
    fn from(proto: SlotStatus2Proto) -> Self {
        let _ = proto.rsvdz();
        Self {
        }
    }
}
impl From<u16> for SlotStatus2 {
    fn from(word: u16) -> Self { SlotStatus2Proto::from(word).into() }
}


/// Transmitter Preset
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransmitterPreset {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    Reserved(u8),
}
impl From<u8> for TransmitterPreset {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::P0,
            0b0001 => Self::P1,
            0b0010 => Self::P2,
            0b0011 => Self::P3,
            0b0100 => Self::P4,
            0b0101 => Self::P5,
            0b0110 => Self::P6,
            0b0111 => Self::P7,
            0b1000 => Self::P8,
            0b1001 => Self::P9,
            0b1010 => Self::P10,
            v => Self::Reserved(v),
        }
    }
}

/// Receiver Preset Hint
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiverPresetHint {
    Minus6dB = -6,
    Minus7dB = -7,
    Minus8dB = -8,
    Minus9dB = -9,
    Minus10dB = -10,
    Minus11dB = -11,
    Minus12dB = -12,
    Reserved = 0,
}
impl From<u8> for ReceiverPresetHint {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Minus6dB,
            0b001 => Self::Minus7dB,
            0b010 => Self::Minus8dB,
            0b011 => Self::Minus9dB,
            0b100 => Self::Minus10dB,
            0b101 => Self::Minus11dB,
            0b110 => Self::Minus12dB,
                _ => Self::Reserved,
        }
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use byte::BytesExt;
    use super::*;

    #[test]
    fn endpoint() {
        // Capabilities: [a0] Express (v2) Endpoint, MSI 00
        //         DevCap: MaxPayload 512 bytes, PhantFunc 0, Latency L0s <512ns, L1 <64us
        //                 ExtTag- AttnBtn- AttnInd- PwrInd- RBE+ FLReset+ SlotPowerLimit 0.000W
        //         DevCtl: CorrErr- NonFatalErr+ FatalErr+ UnsupReq+
        //                 RlxdOrd+ ExtTag- PhantFunc- AuxPwr- NoSnoop- FLReset-
        //                 MaxPayload 256 bytes, MaxReadReq 512 bytes
        //         DevSta: CorrErr+ NonFatalErr- FatalErr- UnsupReq+ AuxPwr- TransPend-
        //         LnkCap: Port #0, Speed 8GT/s, Width x4, ASPM L0s L1, Exit Latency L0s <2us, L1 <16us
        //                 ClockPM- Surprise- LLActRep- BwNot- ASPMOptComp+
        //         LnkCtl: ASPM Disabled; RCB 64 bytes Disabled- CommClk+
        //                 ExtSynch- ClockPM- AutWidDis- BWInt- AutBWInt-
        //         LnkSta: Speed 8GT/s (ok), Width x4 (ok)
        //                 TrErr- Train- SlotClk+ DLActive- BWMgmt- ABWMgmt-
        //         DevCap2: Completion Timeout: Range ABCD, TimeoutDis+, NROPrPrP-, LTR+
        //                  10BitTagComp-, 10BitTagReq-, OBFF Not Supported, ExtFmt-, EETLPPrefix-
        //                  EmergencyPowerReduction Not Supported, EmergencyPowerReductionInit-
        //                  FRS-, TPHComp-, ExtTPHComp-
        //                  AtomicOpsCap: 32bit- 64bit- 128bitCAS-
        //         DevCtl2: Completion Timeout: 65ms to 210ms, TimeoutDis-, LTR-, OBFF Disabled
        //                  AtomicOpsCtl: ReqEn-
        //         LnkCtl2: Target Link Speed: 2.5GT/s, EnterCompliance- SpeedDis-
        //                  Transmit Margin: Normal Operating Range, EnterModifiedCompliance- ComplianceSOS-
        //                  Compliance De-emphasis: -6dB
        //         LnkSta2: Current De-emphasis Level: -3.5dB, EqualizationComplete+, EqualizationPhase1+
        //                  EqualizationPhase2+, EqualizationPhase3+, LinkEqualizationRequest-
        let data = [
            0x10, 0xe0, 0x02, 0x00, 0xc2, 0x8c, 0x00, 0x10, 0x3e, 0x20, 0x09, 0x00, 0x43, 0x5c, 0x42, 0x00,
            0x40, 0x00, 0x43, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x1f, 0x08, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let result: PciExpress = data[2..].read_with(&mut 0, LE).unwrap();

        let sample = PciExpress {
            capabilities: Capabilities {
                version: 2,
                device_type: DeviceType::Endpoint,
                slot_implemented: false,
                interrupt_message_number: 0,
                tcs_routing_support: false,
            },
            device: Device {
                capabilities: DeviceCapabilities {
                    max_payload_size_supported: MaxSize::B512,
                    phantom_functions_supported: PhantomFunctionsSupported::NoBits,
                    extended_tag_field_supported: ExtendedTagFieldSupported::Five,
                    endpoint_l0s_acceptable_latency: EndpointL0sAcceptableLatency::Max512ns,
                    endpoint_l1_acceptable_latency: EndpointL1AcceptableLatency::Max64us,
                    attention_button_present: false,
                    attention_indicator_present: false,
                    power_indicator_present: false,
                    role_based_error_reporting: true,
                    captured_slot_power_limit: SlotPowerLimit { value: 0, scale: 1.0 },
                    function_level_reset_capability: true,
                },
                control: DeviceControl {
                    correctable_error_reporting_enable: false,
                    non_fatal_error_reporting_enable: true,
                    fatal_error_reporting_enable: true,
                    unsupported_request_reporting_enable: true,
                    enable_relaxed_ordering: true,
                    max_payload_size: MaxSize::B256,
                    extended_tag_field_enable: false,
                    phantom_functions_enable: false,
                    aux_power_pm_enable: false,
                    enable_no_snoop: false,
                    max_read_request_size: MaxSize::B512,
                    bcre_or_flreset: false,
                },
                status: DeviceStatus {
                    correctable_error_detected: true,
                    non_fatal_error_detected: false,
                    fatal_error_detected: false,
                    unsupported_request_detected: true,
                    aux_power_detected: false,
                    transactions_pending: false,
                },
            },
            link: Some(Link {
                capabilities: LinkCapabilities {
                    max_link_speed: LinkSpeed::Rate8GTps,
                    maximum_link_width: LinkWidth::X4,
                    active_state_power_management_support: ActiveStatePowerManagement::L0sAndL1,
                    l0s_exit_latency: L0sExitLatency::Ge1usAndLt2us,
                    l1_exit_latency: L1ExitLatency::Ge8usAndLt16us,
                    clock_power_management: false,
                    surprise_down_error_reporting_capable: false,
                    data_link_layer_link_active_reporting_capable: false,
                    link_bandwidth_notification_capability: false,
                    aspm_optionality_compliance: true,
                    port_number: 0,
                },
                control: LinkControl {
                    active_state_power_management_control: ActiveStatePowerManagement::NoAspm,
                    read_completion_boundary: ReadCompletionBoundary::B64,
                    link_disable: false,
                    retrain_link: false,
                    common_clock_configuration: true,
                    extended_synch: false,
                    enable_clock_power_management: false,
                    hardware_autonomous_width_disable: false,
                    link_bandwidth_management_interrupt_enable: false,
                    link_autonomous_bandwidth_interrupt_enable: false,
                },
                status: LinkStatus {
                    current_link_speed: LinkSpeed::Rate8GTps,
                    negotiated_link_width: LinkWidth::X4,
                    link_training_error: false,
                    link_training: false,
                    slot_clock_configuration: true,
                    data_link_layer_link_active: false,
                    link_bandwidth_management_status: false,
                    link_autonomous_bandwidth_status: false,
                },
            }),
            slot: None,
            root: None,
            device_2: Some(Device2 {
                capabilities: DeviceCapabilities2 {
                    ln_system_cls: LnSystemCls::NotSupported,
                    support_10bit_tag_completer: false,
                    support_10bit_tag_requester: false,
                    obff_supported: Obff::NotSupported,
                    emergency_power_reduction_supported: EmergencyPowerReduction::NotSupported,
                    emergency_power_reduction_initialization_required: false,
                    frs_supported: false,
                    completion_timeout_ranges_supported: CompletionTimeoutRanges::RangesABCD,
                    completion_timeout_disable_supported: true,
                    ari_forwarding_supported: false,
                    atomic_op_routing_supported: false,
                    u32_atomicop_completer_supported: false,
                    u64_atomicop_completer_supported: false,
                    u128_cas_completer_supported: false,
                    no_ro_enabled_pr_pr_passing: false,
                    ltr_mechanism_supported: true,
                    tph_completer_supported: TphCompleter::NotSupported,
                    extended_fmt_field_supported: false,
                    end_end_tlp_prefix_supported: false,
                    max_end_end_tlp_prefixes: MaxEndEndTlpPrefixes::Max4,
                },
                control: DeviceControl2 {
                    emergency_power_reduction_request: false,
                    enable_10bit_tag_requester: false,
                    completion_timeout_value: CompletionTimeoutValue::RangeB65msTo210ms,
                    completion_timeout_disable: false,
                    ari_forwarding_enable: false,
                    atomic_op_requester_enable: false,
                    atomic_op_egress_blocking: false,
                    ido_request_enable: false,
                    ido_completion_enable: false,
                    ltr_mechanism_enable: false,
                    obff_enable: ObffEnable::Disabled,
                    end_end_tlp_prefix_blocking: EndEndTlpPrefixBlocking::ForwardingEnabled,
                },
                status: DeviceStatus2 {},
            }),
            link_2: Some(Link2 {
                capabilities: LinkCapabilities2 {
                    lower_skp_os_generation_supported_speeds_vector: SupportedLinkSpeedsVector {
                        speed_2_5_gtps: false,
                        speed_5_0_gtps: false,
                        speed_8_0_gtps: false,
                        speed_16_0_gtps: false,
                        speed_32_0_gtps: false,
                        speed_64_0_gtps: false,
                    },
                    lower_skp_os_reception_supported_speeds_vector: SupportedLinkSpeedsVector {
                        speed_2_5_gtps: false,
                        speed_5_0_gtps: false,
                        speed_8_0_gtps: false,
                        speed_16_0_gtps: false,
                        speed_32_0_gtps: false,
                        speed_64_0_gtps: false,
                    },
                    retimer_presence_detect_supported: false,
                    two_retimers_presence_detect_supported: false,
                    drs_supported: false,
                    supported_link_speeds_vector: SupportedLinkSpeedsVector {
                        speed_2_5_gtps: true,
                        speed_5_0_gtps: true,
                        speed_8_0_gtps: true,
                        speed_16_0_gtps: false,
                        speed_32_0_gtps: false,
                        speed_64_0_gtps: false,
                    },
                    crosslink_supported: false,
                },
                control: LinkControl2 {
                    target_link_speed: LinkSpeed::Rate2GTps,
                    enter_compliance: false,
                    hardware_autonomous_speed_disable: false,
                    selectable_de_emphasis: DeEmphasis::Minus6dB,
                    transmit_margin: TransmitMargin(0),
                    enter_modified_compliance: false,
                    compliance_sos: false,
                    compliance_preset_or_de_emphasis: CompliancePresetOrDeEmphasis(0),
                },
                status: LinkStatus2 {
                    current_de_emphasis_level: DeEmphasis::Minus3_5dB,
                    equalization_complete: true,
                    equalization_phase_1_successful: true,
                    equalization_phase_2_successful: true,
                    equalization_phase_3_successful: true,
                    link_equalization_request: false,
                    retimer_presence_detected: false,
                    two_retimers_presence_detected: false,
                    crosslink_resolution: CrosslinkResolution::NotSupported,
                    downstream_component_presence: DownstreamComponentPresence::DownNotDetermined,
                    drs_message_received: false,
                },
            }),
            slot_2: None,
        };
        assert_eq!(sample, result);
    }
}
