/*!
PCI Express Capability

PCI Express defines a Capability structure in PCI 3.0 compatible Configuration Space (first 256
bytes). This structure allows identification of a PCI Express device Function and indicates
support for new PCI Express features. The PCI Express Capability structure is required for PCI
Express device Functions. The Capability structure is a mechanism for enabling PCI software
transparent features requiring support on legacy operating systems.  In addition to identifying
a PCI Express device Function, the PCI Express Capability structure is used to provide access
to PCI Express specific Control/Status registers and related Power Management enhancements.

Register implementation depends on Device/Port Type. Required implementation described at table:

| Device/Port Type                      | Device | Link | Slot[^1] | Root | Device2 | Link2 | Slot2 |
| :------------------------------------ | :----: | :--: | :------: | :--: | :-----: | :---: | :---: |
| PCI Express Endpoint                  | +      | +    | -        | -    | v2      | v2    | -     |
| Legacy PCI Express Endpoint           | +      | +    | -        | -    | v2      | v2    | -     |
| Root Complex Integrated Endpoint      | +      | -    | -        | -    | v2      | -     | -     |
| Root Complex Event Collector          | +      | -    | -        | +    | v2      | -     | -     |
| Root Port of PCI Express Root Complex | +      | +    | +        | +    | v2      | v2    | v2    |
| Upstream Port of PCI Express Switch   | +      | +    | -        | -    | v2      | v2    |  -    |
| Downstream Port of PCI Express Switch | +      | +    | +        | -    | v2      | v2    | v2    |
| PCI Express to PCI/PCI-X Bridge       | +      | +    | -        | -    | v2      | v2    | -     |
| PCI/PCI-X to PCI Express Bridge       | +      | +    | +        | -    | v2      | v2    | v2    |

[^1]: Switch Downstream and Root Ports are permitted to implement these registers, even when they
are not required.  PCI/PCI-X to PCI Express Bridges (Reverse Bridges) also permitted to implement
these registers.

*/


use core::ops::Range;

use heterob::{P6,P3, endianness::Le, bit_numbering::LsbInto, P12, P14, P7, P8, P10, P2, P4, P21, P9};
use thiserror::Error;


#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PciExpressError {
    #[error("can't read required bytes from slice")]
    RequiredBytesSlice,
    #[error("can't read root bytes from slice")]
    RootBytesSlice,
}

/// PCI Express Capability Structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciExpress {
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
    pub device: Device,
    pub device_2: Option<Device2>,
}
impl PciExpress {
    pub const SIZE: usize = 0x3c - super::Capability::HEADER_SIZE;
}
impl<'a> TryFrom<&'a [u8]> for PciExpress {
    type Error = PciExpressError;
    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        // The PCI Express Capabilities, Device Capabilities, Device Status, and Device Control
        // registers are required for all PCI Express device Functions
        let (start, end) = (0, 26);
        let required_bytes = slice.get(start..end)
            .and_then(|slice| <[u8; 26]>::try_from(slice).ok())
            .ok_or(PciExpressError::RequiredBytesSlice)?;
        let Le((caps,
                dev_caps, dev_ctrl, dev_st,
                link_caps, link_ctrl, link_st,
                slot_caps, slot_ctrl, slot_st,
            )) = P10(required_bytes).into();
        let (
            version, device_type, slot_implemented,
            interrupt_message_number, tcs_routing_support, ()
        ) = P6::<u16, 4, 4, 1, 5, 1, 1>(caps).lsb_into();
        let device = Device::new(dev_caps, dev_ctrl, dev_st);
        let link = Link::new(link_caps, link_ctrl, link_st);
        let slot = Slot::new(slot_caps, slot_ctrl, slot_st);

        // Root Capabilities, Root Status, and Root Control
        let (start, end) = (end, end + 8);
        let root = slice.get(start..end)
            .and_then(|slice| {
                let bytes = <[u8; 8]>::try_from(slice).ok()?;
                let Le((root_ctrl, root_caps, root_st)) = P3(bytes).into();
                Some(Root::new(root_ctrl, root_caps, root_st))
            });

        let (device_2, link_2, slot_2) =
            if version > 1 {
                // Device Capabilities 2, Device Status 2, and Device Control 2
                let (start, end) = (end, end + 8);
                let device_2 = slice.get(start..end)
                    .and_then(|slice| {
                        let bytes = <[u8; 8]>::try_from(slice).ok()?;
                        let Le((dev_ctrl, dev_caps, dev_st)) = P3(bytes).into();
                        Some(Device2::new(dev_ctrl, dev_caps, dev_st))
                    });
                // Link Capabilities 2, Link Status 2, and Link Control 2
                let (start, end) = (end, end + 8);
                let link_2 = slice.get(start..end)
                    .and_then(|slice| {
                        let bytes = <[u8; 8]>::try_from(slice).ok()?;
                        let Le((link_caps_2, link_ctrl_2, link_st_2)) = P3(bytes).into();
                        Some(Link2::new(link_caps_2, link_ctrl_2, link_st_2))
                    });
                // Slot Capabilities 2, Slot Status 2, and Slot Control 2
                let (start, end) = (end, end + 8);
                let slot_2 = slice.get(start..end)
                    .and_then(|slice| {
                        let bytes = <[u8; 8]>::try_from(slice).ok()?;
                        let Le((slot_caps_2, slot_ctrl_2, slot_st_2)) = P3(bytes).into();
                        Some(Slot2::new(slot_caps_2, slot_ctrl_2, slot_st_2))
                    });
                (device_2, link_2, slot_2)
            } else {
                (None, None, None)
            };

        let device_type_args = (device_type, link, slot, root, link_2, slot_2);
        let device_type = DeviceType::try_from(device_type_args)?;
        Ok(Self {
            version, device_type, slot_implemented, interrupt_message_number,
            tcs_routing_support, device, device_2,
        })
    }
}

type DeviceTypeArgs = (u8, Link, Slot, Option<Root>, Option<Link2>, Option<Slot2>);

/// Indicates the specific type of this PCI Express Function
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    /// PCI Express Endpoint
    Endpoint {
        link: Link,
        link_2: Option<Link2>,
    },
    /// Legacy PCI Express Endpoint
    LegacyEndpoint {
        link: Link,
        link_2: Option<Link2>,
    },
    /// Root Complex Integrated Endpoint
    RootComplexIntegratedEndpoint,
    /// Root Complex Event Collector
    RootComplexEventCollector {
        root: Root,
    },
    /// Root Port of PCI Express Root Complex
    RootPort {
        link: Link,
        link_2: Option<Link2>,
        slot: Slot,
        slot_2: Option<Slot2>,
        root: Root,
    },
    /// Upstream Port of PCI Express Switch
    UpstreamPort {
        link: Link,
        link_2: Option<Link2>,
    },
    /// Downstream Port of PCI Express Switch
    DownstreamPort {
        link: Link,
        link_2: Option<Link2>,
        slot: Slot,
        slot_2: Option<Slot2>,
    },
    /// PCI Express to PCI/PCI-X Bridge
    PcieToPciBridge {
        link: Link,
        link_2: Option<Link2>,
    },
    /// PCI/PCI-X to PCI Express Bridge
    PciToPcieBridge {
        link: Link,
        link_2: Option<Link2>,
        slot: Slot,
        slot_2: Option<Slot2>,
    },
    /// Reserved
    Reserved {
        id: u8,
        link: Link,
        link_2: Option<Link2>,
        slot: Slot,
        slot_2: Option<Slot2>,
        root: Option<Root>,
    },
}
impl DeviceType {
    pub fn is_endpoint(&self) -> bool {
        matches!(self,
            Self::Endpoint { .. } | Self::LegacyEndpoint { .. } | Self::RootComplexIntegratedEndpoint
        )
    }
    pub fn is_root(&self) -> bool {
        matches!(self,
            DeviceType::RootPort { .. } | DeviceType::RootComplexEventCollector { .. }
        )
    }
    pub fn is_downstream_port(&self) -> bool {
        matches!(self,
            DeviceType::RootPort { .. } |
            DeviceType::DownstreamPort { .. } |
            DeviceType::PciToPcieBridge { .. }
        )
    }
}
impl TryFrom<DeviceTypeArgs> for DeviceType {
    type Error = PciExpressError;

    fn try_from(args: DeviceTypeArgs) -> Result<Self, Self::Error> {
        let (device_type, link, slot, root, link_2, slot_2) = args;
        match (device_type, root) {
            (0b0000, _) => Ok(Self::Endpoint { link, link_2, }),
            (0b0001, _) => Ok(Self::LegacyEndpoint { link, link_2, }),
            (0b1001, _) => Ok(Self::RootComplexIntegratedEndpoint),
            (0b1010, Some(root)) => Ok(Self::RootComplexEventCollector { root, }),
            (0b1010, None) => Err(PciExpressError::RootBytesSlice),
            (0b0100, Some(root)) => Ok(Self::RootPort { link, link_2, slot, slot_2, root, }),
            (0b0100, None) => Err(PciExpressError::RootBytesSlice),
            (0b0101, _) => Ok(Self::UpstreamPort { link, link_2, }),
            (0b0110, _) => Ok(Self::DownstreamPort { link, link_2, slot, slot_2, }),
            (0b0111, _) => Ok(Self::PcieToPciBridge { link, link_2, }),
            (0b1000, _) => Ok(Self::PciToPcieBridge { link, link_2, slot, slot_2, }),
            (id, Some(root)) =>
                Ok(Self::Reserved { id, link, link_2, slot, slot_2, root: Some(root), }),
            (id, None) =>
                Ok(Self::Reserved { id, link, link_2, slot, slot_2, root: None, }),

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
impl Device {
    pub fn new(capabilities: u32, control: u16, status: u16) -> Self {
        Self {
            capabilities: capabilities.into(),
            control: control.into(),
            status: status.into(),
        }
    }
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
impl From<u32> for DeviceCapabilities {
    fn from(dword: u32) -> Self {
        let (
            mpss, phfs, etfs, l0s_lat, l1_lat,
            attention_button_present,
            attention_indicator_present,
            power_indicator_present,
            role_based_error_reporting,
            (),
            cspl_value, cspl_scale,
            function_level_reset_capability,
            (),
        ) = P14::<_, 3,2,1,3,3,1,1,1,1,2,8,2,1,3>(dword).lsb_into();
        Self {
            max_payload_size_supported: From::<u8>::from(mpss),
            phantom_functions_supported: From::<u8>::from(phfs),
            extended_tag_field_supported: From::<bool>::from(etfs),
            endpoint_l0s_acceptable_latency: From::<u8>::from(l0s_lat),
            endpoint_l1_acceptable_latency: From::<u8>::from(l1_lat),
            attention_button_present,
            attention_indicator_present,
            power_indicator_present,
            role_based_error_reporting,
            captured_slot_power_limit: SlotPowerLimit::new(cspl_value, cspl_scale),
            function_level_reset_capability,
        }
    }
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
impl From<u16> for DeviceControl {
    fn from(word: u16) -> Self {
        let (
            correctable_error_reporting_enable,
            non_fatal_error_reporting_enable,
            fatal_error_reporting_enable,
            unsupported_request_reporting_enable,
            enable_relaxed_ordering,
            mps,
            extended_tag_field_enable,
            phantom_functions_enable,
            aux_power_pm_enable,
            enable_no_snoop,
            mrrs,
            bcre_or_flreset,
        ) = P12::<_, 1,1,1,1,1,3,1,1,1,1,3,1>(word).lsb_into();
        Self {
            correctable_error_reporting_enable,
            non_fatal_error_reporting_enable,
            fatal_error_reporting_enable,
            unsupported_request_reporting_enable,
            enable_relaxed_ordering,
            max_payload_size: From::<u8>::from(mps),
            extended_tag_field_enable,
            phantom_functions_enable,
            aux_power_pm_enable,
            enable_no_snoop,
            max_read_request_size: From::<u8>::from(mrrs),
            bcre_or_flreset,
        }
    }
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
impl From<u16> for DeviceStatus {
    fn from(word: u16) -> Self {
        let (
            correctable_error_detected,
            non_fatal_error_detected,
            fatal_error_detected,
            unsupported_request_detected,
            aux_power_detected,
            transactions_pending,
            (),
        ) = P7::<_, 1,1,1,1,1,1,10>(word).lsb_into();
        Self {
            correctable_error_detected,
            non_fatal_error_detected,
            fatal_error_detected,
            unsupported_request_detected,
            aux_power_detected,
            transactions_pending,
        }
    }
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
impl From<u32> for LinkCapabilities {
    fn from(dword: u32) -> Self {
        let (
            mls, mlw, aspms, l0s_lat, l1_lat,
            clock_power_management,
            surprise_down_error_reporting_capable,
            data_link_layer_link_active_reporting_capable,
            link_bandwidth_notification_capability,
            aspm_optionality_compliance,
            (),
            port_number,
        ) = P12::<_, 4,6,2,3,3,1,1,1,1,1,1,8>(dword).lsb_into();
        Self {
            max_link_speed: From::<u8>::from(mls),
            maximum_link_width: From::<u8>::from(mlw),
            active_state_power_management_support: From::<u8>::from(aspms),
            l0s_exit_latency: From::<u8>::from(l0s_lat),
            l1_exit_latency: From::<u8>::from(l1_lat),
            clock_power_management,
            surprise_down_error_reporting_capable,
            data_link_layer_link_active_reporting_capable,
            link_bandwidth_notification_capability,
            aspm_optionality_compliance,
            port_number,
        }
    }
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
            LinkWidth::X1  => 1,
            LinkWidth::X2  => 2,
            LinkWidth::X4  => 3,
            LinkWidth::X8  => 4,
            LinkWidth::X12 => 5,
            LinkWidth::X16 => 6,
            LinkWidth::X32 => 7,
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
impl From<u16> for LinkControl {
    fn from(word: u16) -> Self {
        let (
            aspmc, (), rcb, link_disable, retrain_link, common_clock_configuration, extended_synch,
            enable_clock_power_management, hardware_autonomous_width_disable,
            link_bandwidth_management_interrupt_enable, link_autonomous_bandwidth_interrupt_enable,
            (),
        ) = P12::<_, 2,1,1,1,1,1,1,1,1,1,1,4>(word).lsb_into();
        Self {
            active_state_power_management_control: From::<u8>::from(aspmc),
            read_completion_boundary: From::<bool>::from(rcb),
            link_disable,
            retrain_link,
            common_clock_configuration,
            extended_synch,
            enable_clock_power_management,
            hardware_autonomous_width_disable,
            link_bandwidth_management_interrupt_enable,
            link_autonomous_bandwidth_interrupt_enable,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadCompletionBoundary {
    B64 = 64,
    B128 = 128,
}
impl From<bool> for ReadCompletionBoundary {
    fn from(b: bool) -> Self { if b { Self::B128 } else { Self::B64 } }
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
impl From<u16> for LinkStatus {
    fn from(word: u16) -> Self {
        let (
            cls, nlw, link_training_error, link_training, slot_clock_configuration,
            data_link_layer_link_active, link_bandwidth_management_status,
            link_autonomous_bandwidth_status,
        ) = P8::<_, 4,6,1,1,1,1,1,1>(word).lsb_into();
        Self {
            current_link_speed: From::<u8>::from(cls),
            negotiated_link_width: From::<u8>::from(nlw),
            link_training_error,
            link_training,
            slot_clock_configuration,
            data_link_layer_link_active,
            link_bandwidth_management_status,
            link_autonomous_bandwidth_status,
        }
    }
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
impl From<u32> for SlotCapabilities {
    fn from(dword: u32) -> Self {
        let (
            attention_button_present, power_controller_present, mrl_sensor_present,
            attention_indicator_present, power_indicator_present, hot_plug_surprise,
            hot_plug_capable, spl_value, spl_scale, electromechanical_interlock_present,
            no_command_completed_support, physical_slot_number,
        ) = P12::<_, 1,1,1,1,1,1,1,8,2,1,1,13>(dword).lsb_into();
        Self {
            attention_button_present,
            power_controller_present,
            mrl_sensor_present,
            attention_indicator_present,
            power_indicator_present,
            hot_plug_surprise,
            hot_plug_capable,
            slot_power_limit: SlotPowerLimit::new(spl_value, spl_scale),
            electromechanical_interlock_present,
            no_command_completed_support,
            physical_slot_number,
        }
    }
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
impl From<u16> for SlotControl {
    fn from(word: u16) -> Self {
        let (
            attention_button_pressed_enable, power_fault_detected_enable,
            mrl_sensor_changed_enable, presence_detect_changed_enable,
            command_completed_interrupt_enable, hot_plug_interrupt_enable, aic, pic,
            power_controller_control, electromechanical_interlock_control,
            data_link_layer_state_changed_enable, (),
        ) = P12::<_, 1,1,1,1,1,1,2,2,1,1,1,3>(word).lsb_into();
        Self {
            attention_button_pressed_enable,
            power_fault_detected_enable,
            mrl_sensor_changed_enable,
            presence_detect_changed_enable,
            command_completed_interrupt_enable,
            hot_plug_interrupt_enable,
            attention_indicator_control: From::<u8>::from(aic),
            power_indicator_control: From::<u8>::from(pic),
            power_controller_control,
            electromechanical_interlock_control,
            data_link_layer_state_changed_enable,
        }
    }
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
impl From<u16> for SlotStatus {
    fn from(word: u16) -> Self {
        let (
            attention_button_pressed, power_fault_detected, mrl_sensor_changed,
            presence_detect_changed, command_completed, mrl_sensor_state, presence_detect_state,
            electromechanical_interlock_status, data_link_layer_state_changed, (),
        ) = P10::<_, 1,1,1,1,1,1,1,1,1,7>(word).lsb_into();
        Self {
            attention_button_pressed,
            power_fault_detected,
            mrl_sensor_changed,
            presence_detect_changed,
            command_completed,
            mrl_sensor_state,
            presence_detect_state,
            electromechanical_interlock_status,
            data_link_layer_state_changed,
        }
    }
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
    pub fn new(control: u16, capabilities: u16, status: u32) -> Self {
        Self {
            control: control.into(),
            capabilities: capabilities.into(),
            status: status.into(),
        }
    }
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
impl From<u16> for RootControl {
    fn from(word: u16) -> Self {
        let (
            system_error_on_correctable_error_enable, system_error_on_non_fatal_error_enable,
            system_error_on_fatal_error_enable, pme_interrupt_enable,
            crs_software_visibility_enable, (),
        ) = P6::<_, 1,1,1,1,1,11>(word).lsb_into();
        Self {
            system_error_on_correctable_error_enable,
            system_error_on_non_fatal_error_enable,
            system_error_on_fatal_error_enable,
            pme_interrupt_enable,
            crs_software_visibility_enable,
        }
    }
}


/// The Root Capabilities register identifies PCI Express Root Port specific capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootCapabilities {
    /// CRS Software Visibility
    pub crs_software_visibility: bool,
}
impl From<u16> for RootCapabilities {
    fn from(word: u16) -> Self {
        let (
            crs_software_visibility, (),
        ) = P2::<_, 1,15>(word).lsb_into();
        Self {
            crs_software_visibility,
        }
    }
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
impl From<u32> for RootStatus {
    fn from(dword: u32) -> Self {
        let (
            pme_requester_id, pme_status, pme_pending, (),
        ) = P4::<_, 16,1,1,14>(dword).lsb_into();
        Self {
            pme_requester_id,
            pme_status,
            pme_pending,
        }
    }
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
impl From<u32> for DeviceCapabilities2 {
    fn from(dword: u32) -> Self {
        let (
            ctrs, completion_timeout_disable_supported, ari_forwarding_supported,
            atomic_op_routing_supported, u32_atomicop_completer_supported,
            u64_atomicop_completer_supported, u128_cas_completer_supported,
            no_ro_enabled_pr_pr_passing, ltr_mechanism_supported, tcs, lsc,
            support_10bit_tag_completer, support_10bit_tag_requester, os,
            extended_fmt_field_supported, end_end_tlp_prefix_supported, meetp, eprs,
            emergency_power_reduction_initialization_required, (), frs_supported,
        ) = P21::<_, 4,1,1,1,1,1,1,1,1,2,2,1,1,2,1,1,2,2,1,4,1>(dword).lsb_into();
        Self {
            completion_timeout_ranges_supported: From::<u8>::from(ctrs),
            completion_timeout_disable_supported,
            ari_forwarding_supported,
            atomic_op_routing_supported,
            u32_atomicop_completer_supported,
            u64_atomicop_completer_supported,
            u128_cas_completer_supported,
            no_ro_enabled_pr_pr_passing,
            ltr_mechanism_supported,
            tph_completer_supported: From::<u8>::from(tcs),
            ln_system_cls: From::<u8>::from(lsc),
            support_10bit_tag_completer,
            support_10bit_tag_requester,
            obff_supported: From::<u8>::from(os),
            extended_fmt_field_supported,
            end_end_tlp_prefix_supported,
            max_end_end_tlp_prefixes: From::<u8>::from(meetp),
            emergency_power_reduction_supported: From::<u8>::from(eprs),
            emergency_power_reduction_initialization_required,
            frs_supported,
        }
    }
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
impl From<u16> for DeviceControl2 {
    fn from(word: u16) -> Self {
        let (
            ctv, completion_timeout_disable, ari_forwarding_enable, atomic_op_requester_enable,
            atomic_op_egress_blocking, ido_request_enable, ido_completion_enable,
            ltr_mechanism_enable, emergency_power_reduction_request, enable_10bit_tag_requester,
            oe, eetpd,
        ) = P12::<_, 4,1,1,1,1,1,1,1,1,1,2,1>(word).lsb_into();
        Self {
            completion_timeout_value: From::<u8>::from(ctv),
            completion_timeout_disable,
            ari_forwarding_enable,
            atomic_op_requester_enable,
            atomic_op_egress_blocking,
            ido_request_enable,
            ido_completion_enable,
            ltr_mechanism_enable,
            emergency_power_reduction_request,
            enable_10bit_tag_requester,
            obff_enable: From::<u8>::from(oe),
            end_end_tlp_prefix_blocking: From::<bool>::from(eetpd),
        }
    }
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


/// Device Status 2 Register is a placeholder
/// There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceStatus2  {
}
impl From<u16> for DeviceStatus2 {
    fn from(_word: u16) -> Self {
        Self {
        }
    }
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
impl From<u32> for LinkCapabilities2 {
    fn from(dword: u32) -> Self {
        let (
            (), slsv, crosslink_supported, lsog, lsor, retimer_presence_detect_supported,
            two_retimers_presence_detect_supported, (), drs_supported,
        ) = P9::<_, 1,7,1,7,7,1,1,6,1>(dword).lsb_into();
        Self {
            supported_link_speeds_vector: From::<u8>::from(slsv),
            crosslink_supported,
            lower_skp_os_generation_supported_speeds_vector: From::<u8>::from(lsog),
            lower_skp_os_reception_supported_speeds_vector: From::<u8>::from(lsor),
            retimer_presence_detect_supported,
            two_retimers_presence_detect_supported,
            drs_supported,
        }
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
    /// Reserved
    pub reserved: bool,
}
impl From<u8> for SupportedLinkSpeedsVector {
    fn from(byte: u8) -> Self {
        let (
            speed_2_5_gtps, speed_5_0_gtps, speed_8_0_gtps, speed_16_0_gtps, speed_32_0_gtps,
            speed_64_0_gtps, reserved,
        ) = P7::<_, 1,1,1,1,1,1,1>(byte).lsb_into();
        Self {
            speed_2_5_gtps,
            speed_5_0_gtps,
            speed_8_0_gtps,
            speed_16_0_gtps,
            speed_32_0_gtps,
            speed_64_0_gtps,
            reserved,
        }
    }
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
impl From<u16> for LinkControl2 {
    fn from(word: u16) -> Self {
        let (
            tls, enter_compliance, hardware_autonomous_speed_disable, sde, tm,
            enter_modified_compliance, compliance_sos, cpode,
        ) = P8::<_, 4,1,1,1,3,1,1,4>(word).lsb_into();
        Self {
            target_link_speed: From::<u8>::from(tls),
            enter_compliance,
            hardware_autonomous_speed_disable,
            selectable_de_emphasis: From::<bool>::from(sde),
            transmit_margin: From::<u8>::from(tm),
            enter_modified_compliance,
            compliance_sos,
            compliance_preset_or_de_emphasis: From::<u8>::from(cpode),
        }
    }
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
impl From<u16> for LinkStatus2 {
    fn from(word: u16) -> Self {
        let (
            cdel, equalization_complete, equalization_phase_1_successful,
            equalization_phase_2_successful, equalization_phase_3_successful,
            link_equalization_request, retimer_presence_detected, two_retimers_presence_detected,
            cr, (), dcp, drs_message_received,
        ) = P12::<_, 1,1,1,1,1,1,1,1,2,2,3,1>(word).lsb_into();
        Self {
            current_de_emphasis_level: From::<bool>::from(cdel),
            equalization_complete,
            equalization_phase_1_successful,
            equalization_phase_2_successful,
            equalization_phase_3_successful,
            link_equalization_request,
            retimer_presence_detected,
            two_retimers_presence_detected,
            crosslink_resolution: From::<u8>::from(cr),
            downstream_component_presence: From::<u8>::from(dcp),
            drs_message_received,
        }
    }
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


/// Slot Capabilities 2 Register
///
/// This section is a placeholder. There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotCapabilities2  {
}
impl From<u32> for SlotCapabilities2 {
    fn from(_dword: u32) -> Self {
        Self {}
    }
}

/// Slot Control 2 Register
///
/// This section is a placeholder. There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotControl2  {
}
impl From<u16> for SlotControl2 {
    fn from(_word: u16) -> Self {
        Self {}
    }
}

/// Slot Status 2 Register
///
/// This section is a placeholder. There are no capabilities that require this register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotStatus2  {
}
impl From<u16> for SlotStatus2 {
    fn from(_word: u16) -> Self {
        Self {}
    }
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
        let result: PciExpress = data[2..].try_into().unwrap();

        let sample = PciExpress {
            version: 2,
            device_type: DeviceType::Endpoint {
                link: Link {
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
                },
                link_2: Some(Link2 {
                    capabilities: LinkCapabilities2 {
                        lower_skp_os_generation_supported_speeds_vector: SupportedLinkSpeedsVector {
                            speed_2_5_gtps: false,
                            speed_5_0_gtps: false,
                            speed_8_0_gtps: false,
                            speed_16_0_gtps: false,
                            speed_32_0_gtps: false,
                            speed_64_0_gtps: false,
                            reserved: false,
                        },
                        lower_skp_os_reception_supported_speeds_vector: SupportedLinkSpeedsVector {
                            speed_2_5_gtps: false,
                            speed_5_0_gtps: false,
                            speed_8_0_gtps: false,
                            speed_16_0_gtps: false,
                            speed_32_0_gtps: false,
                            speed_64_0_gtps: false,
                            reserved: false,
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
                            reserved: false,
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
            },
            slot_implemented: false,
            interrupt_message_number: 0,
            tcs_routing_support: false,
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
        };
        assert_eq!(sample, result);
    }
}
