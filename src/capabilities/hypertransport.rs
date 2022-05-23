//! HyperTransport Link Capability
//!
//! This Capability structure provides control and status for devices that implement HyperTransport
//! Technology links. For details, refer to the HyperTransport I/O Link Specification
//!
//! Capability types:
//! - [x] [Slave or Primary Interface](SlaveOrPrimaryInterface)
//! - [x] [Host or Secondary Interface](HostOrSecondaryInterface)
//! - [ ] Switch
//! - [ ] Reserved-Host
//! - [ ] Interrupt Discovery and Configuration
//! - [x] [Revision ID](RevisionId)
//! - [ ] UnitID Clumping
//! - [ ] Extended Configuration Space Access
//! - [ ] Address Mapping
//! - [x] [MSI Mapping](MsiMapping)
//! - [ ] DirectRoute
//! - [ ] VCSet
//! - [ ] Retry Mode
//! - [ ] X86 Encoding (Reserved)
//! - [ ] Gen3
//! - [ ] Function-Level Extension
//! - [ ] Power Management
//! - [ ] High Node Count

use heterob::{bit_numbering::Lsb, endianness::Le, P10, P11, P13, P16, P17, P2, P3, P5, P6, P8};
use snafu::Snafu;

/// HyperTransport errors
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum HypertransportError {
    CapabilityType,
    SlaveOrPrimaryInterface,
    HostOrSecondaryInterface,
    Switch,
    ReservedHost,
    InterruptDiscoveryAndConfiguration,
    RevisionId,
    UnitIdClumping,
    ExtendedConfigurationSpaceAccess,
    AddressMapping,
    MsiMapping,
    DirectRoute,
    VCSet,
    X86Encoding,
    Gen3,
    FunctionLevelExtension,
    PowerManagement,
    HighNodeCount,
}

/// The layout of the capabilities block is determined by the value in the Capability Type field in
/// the Command register
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hypertransport {
    /// Slave or Primary Interface
    SlaveOrPrimaryInterface(SlaveOrPrimaryInterface),
    /// Host or Secondary Interface
    HostOrSecondaryInterface(HostOrSecondaryInterface),
    /// Switch
    Switch(Switch),
    /// Reserved-Host
    ReservedHost(ReservedHost),
    /// Interrupt Discovery and Configuration
    InterruptDiscoveryAndConfiguration(InterruptDiscoveryAndConfiguration),
    /// Revision ID
    RevisionId(RevisionId),
    /// UnitID Clumping
    UnitIdClumping(UnitIdClumping),
    /// Extended Configuration Space Access
    ExtendedConfigurationSpaceAccess(ExtendedConfigurationSpaceAccess),
    /// Address Mapping
    AddressMapping(AddressMapping),
    /// MSI Mapping
    MsiMapping(MsiMapping),
    /// DirectRoute
    DirectRoute(DirectRoute),
    /// VCSet
    VCSet(VCSet),
    /// Retry Mode
    RetryMode(RetryMode),
    /// X86 Encoding (Reserved)
    X86Encoding(X86Encoding),
    /// Gen3
    Gen3(Gen3),
    /// Function-Level Extension
    FunctionLevelExtension(FunctionLevelExtension),
    /// Power Management
    PowerManagement(PowerManagement),
    /// High Node Count
    HighNodeCount(HighNodeCount),
    /// Reserved
    Reserved(u8),
}
impl<'a> TryFrom<&'a [u8]> for Hypertransport {
    type Error = HypertransportError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let cmd = slice
            .get(..2)
            .and_then(|slice| <[u8; 2]>::try_from(slice).ok())
            .map(u16::from_le_bytes)
            .ok_or(HypertransportError::CapabilityType)?;
        // For the primary and secondary interface capability blocks,
        // there are 3 bits (15:13) used in the encoding.
        // For all other HyperTransport capability blocks, 5 bits (15:11) are used.
        let Lsb(((), capability_type)) = P2::<u16, 11, 5>(cmd).into();
        let _: u8 = capability_type;
        Ok(match capability_type {
            0b00000..=0b00011 => slice
                .get(..SlaveOrPrimaryInterface::SIZE)
                .and_then(|slice| <[u8; SlaveOrPrimaryInterface::SIZE]>::try_from(slice).ok())
                .map(|data| Self::SlaveOrPrimaryInterface(data.into()))
                .ok_or(HypertransportError::SlaveOrPrimaryInterface)?,
            0b00100..=0b00111 => slice
                .get(..HostOrSecondaryInterface::SIZE)
                .and_then(|slice| <[u8; HostOrSecondaryInterface::SIZE]>::try_from(slice).ok())
                .map(|data| Self::HostOrSecondaryInterface(data.into()))
                .ok_or(HypertransportError::HostOrSecondaryInterface)?,
            0b01000 => Self::Switch(Switch {}),
            0b01001 => Self::ReservedHost(ReservedHost {}),
            0b10000 => {
                Self::InterruptDiscoveryAndConfiguration(InterruptDiscoveryAndConfiguration {})
            }
            0b10001 => slice
                .get(0)
                .map(|&data| Self::RevisionId(data.into()))
                .ok_or(HypertransportError::RevisionId)?,
            0b10010 => Self::UnitIdClumping(UnitIdClumping {}),
            0b10011 => Self::ExtendedConfigurationSpaceAccess(ExtendedConfigurationSpaceAccess {}),
            0b10100 => Self::AddressMapping(AddressMapping {}),
            0b10101 => slice
                .get(..MsiMapping::SIZE)
                .and_then(|slice| <[u8; MsiMapping::SIZE]>::try_from(slice).ok())
                .map(|data| Self::MsiMapping(data.into()))
                .ok_or(HypertransportError::MsiMapping)?,
            0b10110 => Self::DirectRoute(DirectRoute {}),
            0b10111 => Self::VCSet(VCSet {}),
            0b11000 => Self::RetryMode(RetryMode {}),
            0b11001 => Self::X86Encoding(X86Encoding {}),
            0b11010 => Self::Gen3(Gen3 {}),
            0b11011 => Self::FunctionLevelExtension(FunctionLevelExtension {}),
            0b11100 => Self::PowerManagement(PowerManagement {}),
            0b11101 => Self::HighNodeCount(HighNodeCount {}),
            v => Self::Reserved(v as u8),
        })
    }
}

impl<'a> From<&'a Hypertransport> for u8 {
    fn from(ht: &'a Hypertransport) -> Self {
        match ht {
            Hypertransport::SlaveOrPrimaryInterface(_) => 0b00000,
            Hypertransport::HostOrSecondaryInterface(_) => 0b00100,
            Hypertransport::Switch(_) => 0b01000,
            Hypertransport::ReservedHost(_) => 0b01001,
            Hypertransport::InterruptDiscoveryAndConfiguration(_) => 0b10000,
            Hypertransport::RevisionId(_) => 0b10001,
            Hypertransport::UnitIdClumping(_) => 0b10010,
            Hypertransport::ExtendedConfigurationSpaceAccess(_) => 0b10011,
            Hypertransport::AddressMapping(_) => 0b10100,
            Hypertransport::MsiMapping(_) => 0b10101,
            Hypertransport::DirectRoute(_) => 0b10110,
            Hypertransport::VCSet(_) => 0b10111,
            Hypertransport::RetryMode(_) => 0b11000,
            Hypertransport::X86Encoding(_) => 0b11001,
            Hypertransport::Gen3(_) => 0b11010,
            Hypertransport::FunctionLevelExtension(_) => 0b11011,
            Hypertransport::PowerManagement(_) => 0b11100,
            Hypertransport::HighNodeCount(_) => 0b11101,
            Hypertransport::Reserved(v) => *v,
        }
    }
}

/// Slave/Primary Interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlaveOrPrimaryInterface {
    /// Command
    pub command: SlaveOrPrimaryCommand,
    /// Link Control 0
    pub link_control_0: LinkControl,
    /// Link Config 0
    pub link_config_0: LinkConfiguration,
    /// Link Control 1
    pub link_control_1: LinkControl,
    /// Link Config 1
    pub link_config_1: LinkConfiguration,
    /// Revision ID
    pub revision_id: RevisionId,
    /// Link Freq 0
    pub link_freq_0: u8,
    /// Link Error 0
    pub link_error_0: LinkError,
    /// LinkFreqCap0
    pub link_freq_cap_0: LinkFrequencyCapability,
    /// Feature
    pub feature: FeatureCapability,
    /// Link Freq 1
    pub link_freq_1: u8,
    /// Link Error 1
    pub link_error_1: LinkError,
    /// LinkFreqCap1
    pub link_freq_cap_1: LinkFrequencyCapability,
    /// Enumeration Scratchpad
    pub enumeration_scratchpad: u16,
    /// Error Handling
    pub error_handling: ErrorHandling,
    /// Mem Base Upper
    pub mem_base_upper: u8,
    /// Mem Limit Upper
    pub mem_limit_upper: u8,
    /// Bus Number
    pub bus_number: u8,
}
impl SlaveOrPrimaryInterface {
    pub const SIZE: usize = 26;
    pub fn link_freq_0(&self, link_freq_ext: bool) -> LinkFrequency {
        LinkFrequency::new(link_freq_ext, self.link_freq_0)
    }
    pub fn link_freq_1(&self, link_freq_ext: bool) -> LinkFrequency {
        LinkFrequency::new(link_freq_ext, self.link_freq_1)
    }
}
impl<'a> From<[u8; Self::SIZE]> for SlaveOrPrimaryInterface {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((
            command,
            link_control_0,
            link_config_0,
            link_control_1,
            link_config_1,
            revision_id,
            link_freq_err_0,
            link_freq_cap_0,
            feature,
            link_freq_err_1,
            link_freq_cap_1,
            enumeration_scratchpad,
            error_handling,
            mem_base_upper,
            mem_limit_upper,
            bus_number,
            rsvd,
        )) = P17(bytes).into();
        let _: (u8, u8) = (feature, rsvd);
        let Lsb((link_freq_0, protocol_error, overflow_error, end_of_chain_error, ctl_timeout)) =
            P5::<u8, 4, 1, 1, 1, 1>(link_freq_err_0).into();
        let link_error_0 = LinkError {
            protocol_error,
            overflow_error,
            end_of_chain_error,
            ctl_timeout,
        };
        let Lsb((link_freq_1, protocol_error, overflow_error, end_of_chain_error, ctl_timeout)) =
            P5::<u8, 4, 1, 1, 1, 1>(link_freq_err_1).into();
        let link_error_1 = LinkError {
            protocol_error,
            overflow_error,
            end_of_chain_error,
            ctl_timeout,
        };
        Self {
            command: From::<u16>::from(command),
            link_control_0: From::<u16>::from(link_control_0),
            link_config_0: From::<u16>::from(link_config_0),
            link_control_1: From::<u16>::from(link_control_1),
            link_config_1: From::<u16>::from(link_config_1),
            revision_id: From::<u8>::from(revision_id),
            link_freq_0: From::<u8>::from(link_freq_0),
            link_error_0,
            link_freq_cap_0: From::<u16>::from(link_freq_cap_0),
            feature: (feature as u16).into(),
            link_freq_1: From::<u8>::from(link_freq_1),
            link_error_1,
            link_freq_cap_1: From::<u16>::from(link_freq_cap_1),
            enumeration_scratchpad: From::<u16>::from(enumeration_scratchpad),
            error_handling: From::<u16>::from(error_handling),
            mem_base_upper,
            mem_limit_upper,
            bus_number,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlaveOrPrimaryCommand {
    /// Base UnitID
    pub base_unitid: u8,
    /// Unit Count
    pub unit_count: u8,
    /// Master Host
    pub master_host: bool,
    /// Default Direction
    pub default_direction: bool,
    /// Drop on Uninitialized Link
    pub drop_on_uninitialized_link: bool,
}

impl From<u16> for SlaveOrPrimaryCommand {
    fn from(word: u16) -> Self {
        let Lsb((
            base_unitid,
            unit_count,
            master_host,
            default_direction,
            drop_on_uninitialized_link,
            (),
        )) = P6::<_, 5, 5, 1, 1, 1, 3>(word).into();
        Self {
            base_unitid,
            unit_count,
            master_host,
            default_direction,
            drop_on_uninitialized_link,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkControl {
    /// Source ID Enable
    pub source_id_enable: bool,
    /// CRC Flood Enable
    pub crc_flood_enable: bool,
    /// CRC Start Test
    pub crc_start_test: bool,
    /// CRC Force Error
    pub crc_force_error: bool,
    /// Link Failure
    pub link_failure: bool,
    /// Initialization Complete
    pub initialization_complete: bool,
    /// End of Chain
    pub end_of_chain: bool,
    /// Transmitter Off
    pub transmitter_off: bool,
    /// CRC Error
    pub crc_error: u8,
    /// Isochronous Flow Control Enable
    pub isochronous_flow_control_enable: bool,
    /// LDTSTOP# Tristate Enable
    pub ldtstop_tristate_enable: bool,
    /// Extended CTL Time
    pub extended_ctl_time: bool,
    /// 64 Bit Addressing Enable
    pub enable_64_bit_addressing: bool,
}

impl From<u16> for LinkControl {
    fn from(word: u16) -> Self {
        let Lsb((
            source_id_enable,
            crc_flood_enable,
            crc_start_test,
            crc_force_error,
            link_failure,
            initialization_complete,
            end_of_chain,
            transmitter_off,
            crc_error,
            isochronous_flow_control_enable,
            ldtstop_tristate_enable,
            extended_ctl_time,
            enable_64_bit_addressing,
        )) = P13::<_, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1>(word).into();
        Self {
            source_id_enable,
            crc_flood_enable,
            crc_start_test,
            crc_force_error,
            link_failure,
            initialization_complete,
            end_of_chain,
            transmitter_off,
            crc_error,
            isochronous_flow_control_enable,
            ldtstop_tristate_enable,
            extended_ctl_time,
            enable_64_bit_addressing,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkConfiguration {
    /// Max Link Width In
    pub max_link_width_in: LinkWidth,
    /// Doubleword Flow Control In
    pub doubleword_flow_control_in: bool,
    /// Max Link Width Out
    pub max_link_width_out: LinkWidth,
    /// Doubleword Flow Control Out
    pub doubleword_flow_control_out: bool,
    /// Link Width In
    pub link_width_in: LinkWidth,
    /// Doubleword Flow Control In Enable
    pub doubleword_flow_control_in_enable: bool,
    /// Link Width Out
    pub link_width_out: LinkWidth,
    /// Doubleword Flow Control Out Enable
    pub doubleword_flow_control_out_enable: bool,
}

impl From<u16> for LinkConfiguration {
    fn from(word: u16) -> Self {
        let Lsb((
            max_link_width_in,
            doubleword_flow_control_in,
            max_link_width_out,
            doubleword_flow_control_out,
            link_width_in,
            doubleword_flow_control_in_enable,
            link_width_out,
            doubleword_flow_control_out_enable,
        )) = P8::<_, 3, 1, 3, 1, 3, 1, 3, 1>(word).into();
        Self {
            max_link_width_in: From::<u8>::from(max_link_width_in),
            doubleword_flow_control_in,
            max_link_width_out: From::<u8>::from(max_link_width_out),
            doubleword_flow_control_out,
            link_width_in: From::<u8>::from(link_width_in),
            doubleword_flow_control_in_enable,
            link_width_out: From::<u8>::from(link_width_out),
            doubleword_flow_control_out_enable,
        }
    }
}

/// Indicate the physical width of the incoming side of the HyperTransport link implemented by this
/// device. Unganged links indicate a maximum width of 8 bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkWidth {
    /// 8 bits
    Width8bits,
    /// 16 bits
    Width16bits,
    /// 32 bits
    Width32bits,
    /// 2 bits
    Width2bits,
    /// 4 bits
    Width4bits,
    /// Link physically not connected
    NotConnected,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for LinkWidth {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Width8bits,
            0b001 => Self::Width16bits,
            0b011 => Self::Width32bits,
            0b100 => Self::Width2bits,
            0b101 => Self::Width4bits,
            0b111 => Self::NotConnected,
            v => Self::Reserved(v),
        }
    }
}
impl Default for LinkWidth {
    fn default() -> Self {
        Self::Width8bits
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionId {
    pub minor: u8,
    pub major: u8,
}
impl RevisionId {
    pub const SIZE: usize = 1;
}
impl From<u8> for RevisionId {
    fn from(byte: u8) -> Self {
        let Lsb((minor, major)) = P2::<_, 5, 3>(byte).into();
        Self { minor, major }
    }
}
impl<'a> From<&'a RevisionId> for u8 {
    fn from(data: &'a RevisionId) -> Self {
        (data.major << 5) | (data.minor & 0b11111)
    }
}

/// The Link Frequency register specifies the operating frequency of the link’s transmitter
/// clock—the data rate is twice this value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkFrequency {
    Rate200MHz,
    Rate300MHz,
    Rate400MHz,
    Rate500MHz,
    Rate600MHz,
    Rate800MHz,
    Rate1000MHz,
    Rate1200MHz,
    Rate1400MHz,
    Rate1600MHz,
    Rate1800MHz,
    Rate2000MHz,
    Rate2200MHz,
    Rate2400MHz,
    Rate2600MHz,
    VendorSpecific,
    Rate2800MHz,
    Rate3000MHz,
    Rate3200MHz,
    Reserved(u8),
}
impl LinkFrequency {
    /// The encoding of this field, combined with another bit from the Link Frequency Extension
    /// register defined in [Gen3] capability.
    pub fn new(link_freq_ext: bool, link_freq: u8) -> Self {
        match (link_freq_ext, link_freq) {
            (false, 0b0000) => Self::Rate200MHz,
            (false, 0b0001) => Self::Rate300MHz,
            (false, 0b0010) => Self::Rate400MHz,
            (false, 0b0011) => Self::Rate500MHz,
            (false, 0b0100) => Self::Rate600MHz,
            (false, 0b0101) => Self::Rate800MHz,
            (false, 0b0110) => Self::Rate1000MHz,
            (false, 0b0111) => Self::Rate1200MHz,
            (false, 0b1000) => Self::Rate1400MHz,
            (false, 0b1001) => Self::Rate1600MHz,
            (false, 0b1010) => Self::Rate1800MHz,
            (false, 0b1011) => Self::Rate2000MHz,
            (false, 0b1100) => Self::Rate2200MHz,
            (false, 0b1101) => Self::Rate2400MHz,
            (false, 0b1110) => Self::Rate2600MHz,
            (false, 0b1111) => Self::VendorSpecific,
            (true, 0b0001) => Self::Rate2800MHz,
            (true, 0b0010) => Self::Rate3000MHz,
            (true, 0b0011) => Self::Rate3200MHz,
            (b, v) => Self::Reserved((v & 0b1111) | ((b as u8) << 4)),
        }
    }
}
impl Default for LinkFrequency {
    fn default() -> Self {
        Self::Rate200MHz
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkError {
    /// Protocol Error
    pub protocol_error: bool,
    /// Overflow Error
    pub overflow_error: bool,
    /// End Of Chain Error
    pub end_of_chain_error: bool,
    /// CTL Timeout
    pub ctl_timeout: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkFrequencyCapability {
    pub supports_200mhz: bool,
    pub supports_300mhz: bool,
    pub supports_400mhz: bool,
    pub supports_500mhz: bool,
    pub supports_600mhz: bool,
    pub supports_800mhz: bool,
    pub supports_1000mhz: bool,
    pub supports_1200mhz: bool,
    pub supports_1400mhz: bool,
    pub supports_1600mhz: bool,
    pub supports_1800mhz: bool,
    pub supports_2000mhz: bool,
    pub supports_2200mhz: bool,
    pub supports_2400mhz: bool,
    pub supports_2600mhz: bool,
    pub supports_vendor_specific: bool,
}

impl From<u16> for LinkFrequencyCapability {
    fn from(word: u16) -> Self {
        let Lsb((
            supports_200mhz,
            supports_300mhz,
            supports_400mhz,
            supports_500mhz,
            supports_600mhz,
            supports_800mhz,
            supports_1000mhz,
            supports_1200mhz,
            supports_1400mhz,
            supports_1600mhz,
            supports_1800mhz,
            supports_2000mhz,
            supports_2200mhz,
            supports_2400mhz,
            supports_2600mhz,
            supports_vendor_specific,
        )) = P16::<_, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1>(word).into();
        Self {
            supports_200mhz,
            supports_300mhz,
            supports_400mhz,
            supports_500mhz,
            supports_600mhz,
            supports_800mhz,
            supports_1000mhz,
            supports_1200mhz,
            supports_1400mhz,
            supports_1600mhz,
            supports_1800mhz,
            supports_2000mhz,
            supports_2200mhz,
            supports_2400mhz,
            supports_2600mhz,
            supports_vendor_specific,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureCapability {
    /// Isochronous Flow Control Mode
    pub isochronous_flow_control_mode: bool,
    /// LDTSTOP#
    pub ldtstop: bool,
    /// CRC Test Mode
    pub crc_test_mode: bool,
    /// Extended CTL Time Required
    pub extended_ctl_time_required: bool,
    /// 64 Bit Addressing
    pub qword_addressing: bool,
    /// UnitID Reorder Disable
    pub unitid_reorder_disable: bool,
    /// Source Identification Extension
    pub source_identification_extension: bool,
    /// Extended Register Set
    pub extended_register_set: bool,
    /// Upstream Configuration Enable
    pub upstream_configuration_enable: bool,
}

impl From<u16> for FeatureCapability {
    fn from(word: u16) -> Self {
        let Lsb((
            isochronous_flow_control_mode,
            ldtstop,
            crc_test_mode,
            extended_ctl_time_required,
            qword_addressing,
            unitid_reorder_disable,
            source_identification_extension,
            (),
            extended_register_set,
            upstream_configuration_enable,
            (),
        )) = P11::<_, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6>(word).into();
        Self {
            isochronous_flow_control_mode,
            ldtstop,
            crc_test_mode,
            extended_ctl_time_required,
            qword_addressing,
            unitid_reorder_disable,
            source_identification_extension,
            extended_register_set,
            upstream_configuration_enable,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorHandling {
    /// Protocol Error Flood Enable
    pub protocol_error_flood_enable: bool,
    /// Overflow Error Flood Enable
    pub overflow_error_flood_enable: bool,
    /// Protocol Error Fatal Enable
    pub protocol_error_fatal_enable: bool,
    /// Overflow Error Fatal Enable
    pub overflow_error_fatal_enable: bool,
    /// End of Chain Error Fatal Enable
    pub end_of_chain_error_fatal_enable: bool,
    /// Response Error Fatal Enable
    pub response_error_fatal_enable: bool,
    /// CRC Error Fatal Enable
    pub crc_error_fatal_enable: bool,
    /// System Error Fatal Enable
    pub system_error_fatal_enable: bool,
    /// Chain Fail
    pub chain_fail: bool,
    /// Response Error
    pub response_error: bool,
    /// Protocol Error Nonfatal Enable
    pub protocol_error_nonfatal_enable: bool,
    /// Overflow Error Nonfatal Enable
    pub overflow_error_nonfatal_enable: bool,
    /// End of Chain Error Nonfatal Enable
    pub end_of_chain_error_nonfatal_enable: bool,
    /// Response Error Nonfatal Enable
    pub response_error_nonfatal_enable: bool,
    /// CRC Error Nonfatal Enable
    pub crc_error_nonfatal_enable: bool,
    /// System Error Nonfatal Enable
    pub system_error_nonfatal_enable: bool,
}

impl From<u16> for ErrorHandling {
    fn from(word: u16) -> Self {
        let Lsb((
            protocol_error_flood_enable,
            overflow_error_flood_enable,
            protocol_error_fatal_enable,
            overflow_error_fatal_enable,
            end_of_chain_error_fatal_enable,
            response_error_fatal_enable,
            crc_error_fatal_enable,
            system_error_fatal_enable,
            chain_fail,
            response_error,
            protocol_error_nonfatal_enable,
            overflow_error_nonfatal_enable,
            end_of_chain_error_nonfatal_enable,
            response_error_nonfatal_enable,
            crc_error_nonfatal_enable,
            system_error_nonfatal_enable,
        )) = P16::<_, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1>(word).into();
        Self {
            protocol_error_flood_enable,
            overflow_error_flood_enable,
            protocol_error_fatal_enable,
            overflow_error_fatal_enable,
            end_of_chain_error_fatal_enable,
            response_error_fatal_enable,
            crc_error_fatal_enable,
            system_error_fatal_enable,
            chain_fail,
            response_error,
            protocol_error_nonfatal_enable,
            overflow_error_nonfatal_enable,
            end_of_chain_error_nonfatal_enable,
            response_error_nonfatal_enable,
            crc_error_nonfatal_enable,
            system_error_nonfatal_enable,
        }
    }
}

/// Host/Secondary Interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostOrSecondaryInterface {
    /// Command
    pub command: HostOrSecondaryCommand,
    /// Link Control
    pub link_control: LinkControl,
    /// Link Config
    pub link_config: LinkConfiguration,
    /// Revision ID
    pub revision_id: RevisionId,
    /// Link Freq
    pub link_freq: u8,
    /// Link Error
    pub link_error: LinkError,
    /// LinkFreqCap
    pub link_freq_cap: LinkFrequencyCapability,
    /// Feature
    pub feature: FeatureCapability,
    /// Enumeration Scratchpad
    pub enumeration_scratchpad: u16,
    /// Error Handling
    pub error_handling: ErrorHandling,
    /// Mem Base Upper
    pub mem_base_upper: u8,
    /// Mem Limit Upper
    pub mem_limit_upper: u8,
}

impl HostOrSecondaryInterface {
    pub const SIZE: usize = 22;
    pub fn link_freq(&self, link_freq_ext: bool) -> LinkFrequency {
        LinkFrequency::new(link_freq_ext, self.link_freq)
    }
}
impl<'a> From<[u8; Self::SIZE]> for HostOrSecondaryInterface {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((
            command,
            link_control,
            link_config,
            revision_id,
            link_freq_err,
            link_freq_cap,
            feature,
            rsvd_0,
            enumeration_scratchpad,
            error_handling,
            mem_base_upper,
            mem_limit_upper,
            rsvd_1,
        )) = P13(bytes).into();
        let _: (u16, u16) = (rsvd_0, rsvd_1);
        let Lsb((link_freq, protocol_error, overflow_error, end_of_chain_error, ctl_timeout)) =
            P5::<u8, 4, 1, 1, 1, 1>(link_freq_err).into();
        let link_error = LinkError {
            protocol_error,
            overflow_error,
            end_of_chain_error,
            ctl_timeout,
        };
        Self {
            command: From::<u16>::from(command),
            link_control: From::<u16>::from(link_control),
            link_config: From::<u16>::from(link_config),
            revision_id: From::<u8>::from(revision_id),
            link_freq: From::<u8>::from(link_freq),
            link_error,
            link_freq_cap: From::<u16>::from(link_freq_cap),
            feature: From::<u16>::from(feature),
            enumeration_scratchpad: From::<u16>::from(enumeration_scratchpad),
            error_handling: From::<u16>::from(error_handling),
            mem_base_upper,
            mem_limit_upper,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostOrSecondaryCommand {
    /// Warm Reset
    pub warm_reset: bool,
    /// Double-Ended
    pub double_ended: bool,
    /// Device Number
    pub device_number: u8,
    /// Chain Side
    pub chain_side: bool,
    /// Host Hide
    pub host_hide: bool,
    /// Act as Slave
    pub act_as_slave: bool,
    /// Host Inbound End of Chain Error
    pub host_inbound_end_of_chain_error: bool,
    /// Drop on Uninitialized Link
    pub drop_on_uninitialized_link: bool,
}

impl From<u16> for HostOrSecondaryCommand {
    fn from(word: u16) -> Self {
        let Lsb((
            warm_reset,
            double_ended,
            device_number,
            chain_side,
            host_hide,
            (),
            act_as_slave,
            host_inbound_end_of_chain_error,
            drop_on_uninitialized_link,
            (),
        )) = P10::<_, 1, 1, 5, 1, 1, 1, 1, 1, 1, 3>(word).into();
        Self {
            warm_reset,
            double_ended,
            device_number,
            chain_side,
            host_hide,
            act_as_slave,
            host_inbound_end_of_chain_error,
            drop_on_uninitialized_link,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservedHost {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterruptDiscoveryAndConfiguration {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitIdClumping {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedConfigurationSpaceAccess {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressMapping {}

/// MSI Mapping Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsiMapping {
    /// Indicating if the mapping is active
    pub enabled: bool,
    /// Indicating if the next two doublewords for programming address are present in the
    /// capability
    pub fixed: bool,
    /// Holds the lower portion of the base address where the mapping of MSIs takes place. It is
    /// set to FEEh upon warm reset
    pub base_address_lower: u32,
    /// Holds the upper portion of the base address for MSI mapping. It is cleared upon warm reset
    pub base_address_upper: u32,
}
impl MsiMapping {
    pub const SIZE: usize = 2 + 4 + 4;
    pub fn base_address(&self) -> u64 {
        let h = (self.base_address_upper as u64) << 32;
        let l = (self.base_address_lower as u64) & !0xfffff;
        h | l
    }
}
impl From<[u8; Self::SIZE]> for MsiMapping {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((cmd, base_address_lower, base_address_upper)) = P3(bytes).into();
        let Lsb((enabled, fixed, ())) = P3::<u16, 1, 1, 14>(cmd).into();
        Self {
            enabled,
            fixed,
            base_address_lower,
            base_address_upper,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectRoute {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VCSet {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryMode {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X86Encoding {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gen3 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionLevelExtension {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerManagement {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighNodeCount {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn slave_or_primary_interface() {
        // Command: BaseUnitID=0 UnitCnt=12 MastHost- DefDir- DUL-
        // Link Control 0: CFlE- CST- CFE- <LkFail- Init+ EOC- TXO- <CRCErr=0 IsocEn- LSEn- ExtCTL- 64b-
        // Link Config 0: MLWI=16bit DwFcIn- MLWO=16bit DwFcOut- LWI=16bit DwFcInEn- LWO=16bit DwFcOutEn-
        // Link Control 1: CFlE- CST- CFE- <LkFail+ Init- EOC+ TXO+ <CRCErr=0 IsocEn- LSEn- ExtCTL- 64b-
        // Link Config 1: MLWI=8bit DwFcIn- MLWO=8bit DwFcOut- LWI=8bit DwFcInEn- LWO=8bit DwFcOutEn-
        // Revision ID: 3.00
        // Link Frequency 0: [c]
        // Link Error 0: <Prot- <Ovfl- <EOC- CTLTm-
        // Link Frequency Capability 0: 200MHz+ 300MHz- 400MHz+ 500MHz- 600MHz+ 800MHz+ 1.0GHz+ 1.2GHz- 1.4GHz- 1.6GHz- Vend-
        // Feature Capability: IsocFC- LDTSTOP+ CRCTM- ECTLT- 64bA- UIDRD-
        // Link Frequency 1: 200MHz
        // Link Error 1: <Prot- <Ovfl- <EOC- CTLTm-
        // Link Frequency Capability 1: 200MHz- 300MHz- 400MHz- 500MHz- 600MHz- 800MHz- 1.0GHz- 1.2GHz- 1.4GHz- 1.6GHz- Vend-
        // Error Handling: PFlE- OFlE- PFE- OFE- EOCFE- RFE- CRCFE- SERRFE- CF- RE- PNFE- ONFE- EOCNFE- RNFE- CRCNFE- SERRNFE-
        // Prefetchable memory behind bridge Upper: 00-00
        // Bus Number: 00
        let data = [
            0x08, 0x54, 0x80, 0x01, // +00h
            0x20, 0x00, 0x11, 0x11, // +04h
            0xd0, 0x00, 0x00, 0x00, // +08h
            0x60, 0x0c, 0x75, 0x1e, // +0Ch
            0x02, 0x00, 0x00, 0x00, // +10h
            0x00, 0x00, 0x00, 0x00, // +14h
            0x00, 0x00, 0x00, 0x00, // +18h
        ];
        let data: [u8; SlaveOrPrimaryInterface::SIZE] = data[2..].try_into().unwrap();
        let result = data.into();
        let sample = SlaveOrPrimaryInterface {
            command: SlaveOrPrimaryCommand {
                base_unitid: 0,
                unit_count: 12,
                master_host: false,
                default_direction: false,
                drop_on_uninitialized_link: false,
            },
            link_control_0: LinkControl {
                source_id_enable: false,
                crc_flood_enable: false,
                crc_start_test: false,
                crc_force_error: false,
                link_failure: false,
                initialization_complete: true,
                end_of_chain: false,
                transmitter_off: false,
                crc_error: 0,
                isochronous_flow_control_enable: false,
                ldtstop_tristate_enable: false,
                extended_ctl_time: false,
                enable_64_bit_addressing: false,
            },
            link_config_0: LinkConfiguration {
                max_link_width_in: LinkWidth::Width16bits,
                doubleword_flow_control_in: false,
                max_link_width_out: LinkWidth::Width16bits,
                doubleword_flow_control_out: false,
                link_width_in: LinkWidth::Width16bits,
                doubleword_flow_control_in_enable: false,
                link_width_out: LinkWidth::Width16bits,
                doubleword_flow_control_out_enable: false,
            },
            link_control_1: LinkControl {
                source_id_enable: false,
                crc_flood_enable: false,
                crc_start_test: false,
                crc_force_error: false,
                link_failure: true,
                initialization_complete: false,
                end_of_chain: true,
                transmitter_off: true,
                crc_error: 0,
                isochronous_flow_control_enable: false,
                ldtstop_tristate_enable: false,
                extended_ctl_time: false,
                enable_64_bit_addressing: false,
            },
            link_config_1: LinkConfiguration {
                max_link_width_in: LinkWidth::Width8bits,
                doubleword_flow_control_in: false,
                max_link_width_out: LinkWidth::Width8bits,
                doubleword_flow_control_out: false,
                link_width_in: LinkWidth::Width8bits,
                doubleword_flow_control_in_enable: false,
                link_width_out: LinkWidth::Width8bits,
                doubleword_flow_control_out_enable: false,
            },
            revision_id: RevisionId { minor: 0, major: 3 },
            link_freq_0: 0b1100,
            link_error_0: LinkError {
                protocol_error: false,
                overflow_error: false,
                end_of_chain_error: false,
                ctl_timeout: false,
            },
            link_freq_cap_0: LinkFrequencyCapability {
                supports_200mhz: true,
                supports_300mhz: false,
                supports_400mhz: true,
                supports_500mhz: false,
                supports_600mhz: true,
                supports_800mhz: true,
                supports_1000mhz: true,
                supports_1200mhz: false,
                supports_1400mhz: false,
                supports_1600mhz: true,
                supports_1800mhz: true,
                supports_2000mhz: true,
                supports_2200mhz: true,
                supports_2400mhz: false,
                supports_2600mhz: false,
                supports_vendor_specific: false,
            },
            feature: FeatureCapability {
                isochronous_flow_control_mode: false,
                ldtstop: true,
                crc_test_mode: false,
                extended_ctl_time_required: false,
                qword_addressing: false,
                unitid_reorder_disable: false,
                source_identification_extension: false,
                extended_register_set: false,
                upstream_configuration_enable: false,
            },
            link_freq_1: 0, // 0 -> 200MHz (default)
            link_error_1: LinkError {
                protocol_error: false,
                overflow_error: false,
                end_of_chain_error: false,
                ctl_timeout: false,
            },
            link_freq_cap_1: LinkFrequencyCapability {
                supports_200mhz: false,
                supports_300mhz: false,
                supports_400mhz: false,
                supports_500mhz: false,
                supports_600mhz: false,
                supports_800mhz: false,
                supports_1000mhz: false,
                supports_1200mhz: false,
                supports_1400mhz: false,
                supports_1600mhz: false,
                supports_1800mhz: false,
                supports_2000mhz: false,
                supports_2200mhz: false,
                supports_2400mhz: false,
                supports_2600mhz: false,
                supports_vendor_specific: false,
            },
            enumeration_scratchpad: 0,
            error_handling: ErrorHandling {
                protocol_error_flood_enable: false,
                overflow_error_flood_enable: false,
                protocol_error_fatal_enable: false,
                overflow_error_fatal_enable: false,
                end_of_chain_error_fatal_enable: false,
                response_error_fatal_enable: false,
                crc_error_fatal_enable: false,
                system_error_fatal_enable: false,
                chain_fail: false,
                response_error: false,
                protocol_error_nonfatal_enable: false,
                overflow_error_nonfatal_enable: false,
                end_of_chain_error_nonfatal_enable: false,
                response_error_nonfatal_enable: false,
                crc_error_nonfatal_enable: false,
                system_error_nonfatal_enable: false,
            },
            mem_base_upper: 0,
            mem_limit_upper: 0,
            bus_number: 0,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn host_or_secondary_interface() {
        // Command: WarmRst+ DblEnd- DevNum=0 ChainSide- HostHide+ Slave- <EOCErr- DUL-
        // Link Control: CFlE- CST- CFE- <LkFail- Init+ EOC- TXO- <CRCErr=0 IsocEn- LSEn+ ExtCTL- 64b+
        // Link Config: MLWI=16bit DwFcIn- MLWO=16bit DwFcOut- LWI=16bit DwFcInEn- LWO=16bit DwFcOutEn-
        // Revision ID: 3.00
        // Link Frequency: [c]
        // Link Error: <Prot- <Ovfl- <EOC- CTLTm-
        // Link Frequency Capability: 200MHz+ 300MHz- 400MHz+ 500MHz- 600MHz+ 800MHz+ 1.0GHz+ 1.2GHz+ 1.4GHz+ 1.6GHz+ Vend+
        // Feature Capability: IsocFC+ LDTSTOP+ CRCTM- ECTLT- 64bA+ UIDRD- ExtRS- UCnfE-
        let data = [
            0x08, 0x00, 0x01, 0x21, // +00h
            0x20, 0xa0, 0x11, 0x11, // +04h
            0x60, 0x0c, 0xf5, 0xff, // +08h
            0x13, 0x00, 0x00, 0x00, // +0Ch
            0xee, 0x02, 0x84, 0x80, // +10h
            0x00, 0x00, 0x01, 0x00, // +14h
        ];
        let data: [u8; HostOrSecondaryInterface::SIZE] = data[2..].try_into().unwrap();
        let result = data.into();
        let sample = HostOrSecondaryInterface {
            command: HostOrSecondaryCommand {
                warm_reset: true,
                double_ended: false,
                device_number: 0,
                chain_side: false,
                host_hide: true,
                act_as_slave: false,
                host_inbound_end_of_chain_error: false,
                drop_on_uninitialized_link: false,
            },
            link_control: LinkControl {
                source_id_enable: false,
                crc_flood_enable: false,
                crc_start_test: false,
                crc_force_error: false,
                link_failure: false,
                initialization_complete: true,
                end_of_chain: false,
                transmitter_off: false,
                crc_error: 0,
                isochronous_flow_control_enable: false,
                ldtstop_tristate_enable: true,
                extended_ctl_time: false,
                enable_64_bit_addressing: true,
            },
            link_config: LinkConfiguration {
                max_link_width_in: LinkWidth::Width16bits,
                doubleword_flow_control_in: false,
                max_link_width_out: LinkWidth::Width16bits,
                doubleword_flow_control_out: false,
                link_width_in: LinkWidth::Width16bits,
                doubleword_flow_control_in_enable: false,
                link_width_out: LinkWidth::Width16bits,
                doubleword_flow_control_out_enable: false,
            },
            revision_id: RevisionId { minor: 0, major: 3 },
            link_freq: 0b1100,
            link_error: LinkError {
                protocol_error: false,
                overflow_error: false,
                end_of_chain_error: false,
                ctl_timeout: false,
            },
            // There is an error in lspci.c:
            // lfcap defined as u8 (should be u16)
            // therefore 1400MHz - VendorSpecific always not set
            link_freq_cap: LinkFrequencyCapability {
                supports_200mhz: true,
                supports_300mhz: false,
                supports_400mhz: true,
                supports_500mhz: false,
                supports_600mhz: true,
                supports_800mhz: true,
                supports_1000mhz: true,
                supports_1200mhz: true,
                supports_1400mhz: true,
                supports_1600mhz: true,
                supports_1800mhz: true,
                supports_2000mhz: true,
                supports_2200mhz: true,
                supports_2400mhz: true,
                supports_2600mhz: true,
                supports_vendor_specific: true,
            },
            feature: FeatureCapability {
                isochronous_flow_control_mode: true,
                ldtstop: true,
                crc_test_mode: false,
                extended_ctl_time_required: false,
                qword_addressing: true,
                unitid_reorder_disable: false,
                source_identification_extension: false,
                extended_register_set: false,
                upstream_configuration_enable: false,
            },
            enumeration_scratchpad: 0x02ee,
            error_handling: ErrorHandling {
                protocol_error_flood_enable: false,
                overflow_error_flood_enable: false,
                protocol_error_fatal_enable: true,
                overflow_error_fatal_enable: false,
                end_of_chain_error_fatal_enable: false,
                response_error_fatal_enable: false,
                crc_error_fatal_enable: false,
                system_error_fatal_enable: true,
                chain_fail: false,
                response_error: false,
                protocol_error_nonfatal_enable: false,
                overflow_error_nonfatal_enable: false,
                end_of_chain_error_nonfatal_enable: false,
                response_error_nonfatal_enable: false,
                crc_error_nonfatal_enable: false,
                system_error_nonfatal_enable: true,
            },
            mem_base_upper: 0,
            mem_limit_upper: 0,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn revision_id() {
        let data = 0b00100101;
        let result = data.into();
        let sample = RevisionId { major: 1, minor: 5 };
        assert_eq!(sample, result);
    }

    #[test]
    fn msi_mapping() {
        // MSI Mapping Enable+ Fixed+
        let data = [
            0x08, 0x00, 0x03, 0xa8, // +00h
            0x00, 0x00, 0x00, 0x00, // +04h
            0x34, 0x17, 0xda, 0x11, // +08h
        ];
        let data: [u8; MsiMapping::SIZE] = data[2..].try_into().unwrap();
        let result = data.into();
        let sample = MsiMapping {
            enabled: true,
            fixed: true,
            base_address_lower: 0,
            base_address_upper: 0x11da1734,
        };
        assert_eq!(sample, result);

        assert_eq!(0x11da173400000000, result.base_address());
    }
}
