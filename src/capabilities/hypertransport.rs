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

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

use crate::capabilities::CAP_HEADER_LEN;

/// For the primary and secondary interface capability blocks, there are 3 bits (15:13) used in the
/// encoding. For all other HyperTransport capability blocks, 5 bits (15:11) are used.
const HT_CAP_TYPE_MASK: u16 = 0xf800;


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
impl<'a> TryRead<'a, Endian> for Hypertransport {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let word = bytes.read_with::<u16>(&mut 0, endian)?;
        let capability_type = (word & HT_CAP_TYPE_MASK) >> 11;
        let offset = &mut 0;
        let ht = match capability_type {
            0b00000..=0b00011 => {
                let data = bytes.read_with::<SlaveOrPrimaryInterface>(offset, endian)?;
                Self::SlaveOrPrimaryInterface(data)
            },
            0b00100..=0b00111 => {
                let data = bytes.read_with::<HostOrSecondaryInterface>(offset, endian)?;
                Self::HostOrSecondaryInterface(data)
            },
            0b01000           => {
                // let data = bytes.read_with::<Switch>(offset, endian)?;
                Self::Switch(Switch {})
            },
            0b01001           => {
                // let data = bytes.read_with::<ReservedHost>(offset, endian)?;
                Self::ReservedHost(ReservedHost {})
            },
            0b10000           => {
                // let data = bytes.read_with::<InterruptDiscoveryAndConfiguration>(offset, endian)?;
                Self::InterruptDiscoveryAndConfiguration(InterruptDiscoveryAndConfiguration {})
            },
            0b10001           => {
                let data = bytes.read_with::<u8>(offset, endian)?.into();
                Self::RevisionId(data)
            },
            0b10010           => {
                // let data = bytes.read_with::<UnitIdClumping>(offset, endian)?;
                Self::UnitIdClumping(UnitIdClumping {})
            },
            0b10011           => {
                // let data = bytes.read_with::<ExtendedConfigurationSpaceAccess>(offset, endian)?;
                Self::ExtendedConfigurationSpaceAccess(ExtendedConfigurationSpaceAccess {})
            },
            0b10100           => {
                // let data = bytes.read_with::<AddressMapping>(offset, endian)?;
                Self::AddressMapping(AddressMapping {})
            },
            0b10101           => {
                let data = bytes.read_with::<MsiMapping>(offset, endian)?;
                Self::MsiMapping(data)
            },
            0b10110           => {
                // let data = bytes.read_with::<DirectRoute>(offset, endian)?;
                Self::DirectRoute(DirectRoute {})
            },
            0b10111           => {
                // let data = bytes.read_with::<VCSet>(offset, endian)?;
                Self::VCSet(VCSet {})
            },
            0b11000           => {
                // let data = bytes.read_with::<RetryMode>(offset, endian)?;
                Self::RetryMode(RetryMode {})
            },
            0b11001           => {
                // let data = bytes.read_with::<X86Encoding>(offset, endian)?;
                Self::X86Encoding(X86Encoding {})
            },
            0b11010           => {
                // let data = bytes.read_with::<Gen3>(offset, endian)?;
                Self::Gen3(Gen3 {})
            },
            0b11011           => {
                // let data = bytes.read_with::<FunctionLevelExtension>(offset, endian)?;
                Self::FunctionLevelExtension(FunctionLevelExtension {})
            },
            0b11100           => {
                // let data = bytes.read_with::<PowerManagement>(offset, endian)?;
                Self::PowerManagement(PowerManagement {})
            },
            0b11101           => {
                // let data = bytes.read_with::<HighNodeCount>(offset, endian)?;
                Self::HighNodeCount(HighNodeCount {})
            },
            v => Self::Reserved(v as u8),
        };
        Ok((ht, *offset))
    }
}
impl<'a> From<&'a Hypertransport> for u8 {
    fn from(ht: &'a Hypertransport) -> Self {
        match ht {
            Hypertransport::SlaveOrPrimaryInterface(_)            => 0b00000,
            Hypertransport::HostOrSecondaryInterface(_)           => 0b00100,
            Hypertransport::Switch(_)                             => 0b01000,
            Hypertransport::ReservedHost(_)                       => 0b01001,
            Hypertransport::InterruptDiscoveryAndConfiguration(_) => 0b10000,
            Hypertransport::RevisionId(_)                         => 0b10001,
            Hypertransport::UnitIdClumping(_)                     => 0b10010,
            Hypertransport::ExtendedConfigurationSpaceAccess(_)   => 0b10011,
            Hypertransport::AddressMapping(_)                     => 0b10100,
            Hypertransport::MsiMapping(_)                         => 0b10101,
            Hypertransport::DirectRoute(_)                        => 0b10110,
            Hypertransport::VCSet(_)                              => 0b10111,
            Hypertransport::RetryMode(_)                          => 0b11000,
            Hypertransport::X86Encoding(_)                        => 0b11001,
            Hypertransport::Gen3(_)                               => 0b11010,
            Hypertransport::FunctionLevelExtension(_)             => 0b11011,
            Hypertransport::PowerManagement(_)                    => 0b11100,
            Hypertransport::HighNodeCount(_)                      => 0b11101,
            Hypertransport::Reserved(v)                           => *v,
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
    pub fn link_freq_0(&self, link_freq_ext: bool) -> LinkFrequency {
        LinkFrequency::new(link_freq_ext, self.link_freq_0)
    }
    pub fn link_freq_1(&self, link_freq_ext: bool) -> LinkFrequency {
        LinkFrequency::new(link_freq_ext, self.link_freq_1)
    }
}
impl<'a> TryRead<'a, Endian> for SlaveOrPrimaryInterface {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let link_freq_err_proto_0: LinkFreqErrProto =
            bytes.read_with::<u8>(&mut (0x0d - CAP_HEADER_LEN), endian)?.into();
        let link_freq_err_proto_1: LinkFreqErrProto =
            bytes.read_with::<u8>(&mut (0x11 - CAP_HEADER_LEN), endian)?.into();
        let sopi = SlaveOrPrimaryInterface {
            command: bytes.read_with::<u16>(offset, endian)?.into(),
            link_control_0: bytes.read_with::<u16>(offset, endian)?.into(),
            link_config_0: bytes.read_with::<u16>(offset, endian)?.into(),
            link_control_1: bytes.read_with::<u16>(offset, endian)?.into(),
            link_config_1: bytes.read_with::<u16>(offset, endian)?.into(),
            revision_id: bytes.read_with::<u8>(offset, endian)?.into(),
            link_freq_0: {
                *offset += 1; // link_freq_err_proto_0 skipped
                link_freq_err_proto_0.link_freq()
            },
            link_error_0: link_freq_err_proto_0.into(),
            link_freq_cap_0: bytes.read_with::<u16>(offset, endian)?.into(),
            feature: (bytes.read_with::<u8>(offset, endian)? as u16).into(),
            link_freq_1: {
                *offset += 1; // link_freq_err_proto_1 skipped
                link_freq_err_proto_1.link_freq()
            },
            link_error_1: link_freq_err_proto_1.into(),
            link_freq_cap_1: bytes.read_with::<u16>(offset, endian)?.into(),
            enumeration_scratchpad: bytes.read_with::<u16>(offset, endian)?,
            error_handling: bytes.read_with::<u16>(offset, endian)?.into(),
            mem_base_upper: bytes.read_with::<u8>(offset, endian)?,
            mem_limit_upper: bytes.read_with::<u8>(offset, endian)?,
            bus_number: bytes.read_with::<u8>(offset, endian)?,
        };
        Ok((sopi, *offset))
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SlaveOrPrimaryCommandProto {
    base_unitid: B5,
    unit_count: B5,
    master_host: bool,
    default_direction: bool,
    drop_on_uninitialized_link: bool,
    capability_type: B3,
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
impl From<SlaveOrPrimaryCommandProto> for SlaveOrPrimaryCommand {
    fn from(proto: SlaveOrPrimaryCommandProto) -> Self {
        let _ = proto.capability_type();
        Self {
            base_unitid: proto.base_unitid(),
            unit_count: proto.unit_count(),
            master_host: proto.master_host(),
            default_direction: proto.default_direction(),
            drop_on_uninitialized_link: proto.drop_on_uninitialized_link(),
        }
    }
}
impl From<u16> for SlaveOrPrimaryCommand {
    fn from(word: u16) -> Self { SlaveOrPrimaryCommandProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LinkControlProto {
    source_id_enable: bool,
    crc_flood_enable: bool,
    crc_start_test: bool,
    crc_force_error: bool,
    link_failure: bool,
    initialization_complete: bool,
    end_of_chain: bool,
    transmitter_off: bool,
    crc_error: B4,
    isochronous_flow_control_enable: bool,
    ldtstop_tristate_enable: bool,
    extended_ctl_time: bool,
    enable_64_bit_addressing: bool,
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
impl From<LinkControlProto> for LinkControl {
    fn from(proto: LinkControlProto) -> Self {
       Self {
           source_id_enable: proto.source_id_enable(),
           crc_flood_enable: proto.crc_flood_enable(),
           crc_start_test: proto.crc_start_test(),
           crc_force_error: proto.crc_force_error(),
           link_failure: proto.link_failure(),
           initialization_complete: proto.initialization_complete(),
           end_of_chain: proto.end_of_chain(),
           transmitter_off: proto.transmitter_off(),
           crc_error: proto.crc_error(),
           isochronous_flow_control_enable: proto.isochronous_flow_control_enable(),
           ldtstop_tristate_enable: proto.ldtstop_tristate_enable(),
           extended_ctl_time: proto.extended_ctl_time(),
           enable_64_bit_addressing: proto.enable_64_bit_addressing(),
        }
    }
}
impl From<u16> for LinkControl {
    fn from(word: u16) -> Self { LinkControlProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LinkConfigurationProto {
    max_link_width_in: B3,
    doubleword_flow_control_in: bool,
    max_link_width_out: B3,
    doubleword_flow_control_out: bool,
    link_width_in: B3,
    doubleword_flow_control_in_enable: bool,
    link_width_out: B3,
    doubleword_flow_control_out_enable: bool,
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
impl From<LinkConfigurationProto> for LinkConfiguration {
    fn from(proto: LinkConfigurationProto) -> Self {
       Self {
           max_link_width_in: proto.max_link_width_in().into(),
           doubleword_flow_control_in: proto.doubleword_flow_control_in(),
           max_link_width_out: proto.max_link_width_out().into(),
           doubleword_flow_control_out: proto.doubleword_flow_control_out(),
           link_width_in: proto.link_width_in().into(),
           doubleword_flow_control_in_enable: proto.doubleword_flow_control_in_enable(),
           link_width_out: proto.link_width_out().into(),
           doubleword_flow_control_out_enable: proto.doubleword_flow_control_out_enable(),
        }
    }
}
impl From<u16> for LinkConfiguration {
    fn from(word: u16) -> Self { LinkConfigurationProto::from(word).into() }
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


#[bitfield(bits = 8)]
#[repr(u8)]
pub struct RevisionIdProto {
    minor: B5,
    major: B3,
}
impl<'a> From<&'a RevisionId> for RevisionIdProto {
    fn from(data: &'a RevisionId) -> Self {
        RevisionIdProto::new()
            .with_minor(data.minor)
            .with_major(data.major)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionId {
    pub minor: u8,
    pub major: u8,
}
impl From<RevisionIdProto> for RevisionId {
    fn from(proto: RevisionIdProto) -> Self {
        Self {
            minor: proto.minor(),
            major: proto.major(),
        }
    }
}
impl From<u8> for RevisionId {
    fn from(byte: u8) -> Self { RevisionIdProto::from(byte).into() }
}
impl<'a> From<&'a RevisionId> for u8 {
    fn from(data: &'a RevisionId) -> Self {
        RevisionIdProto::from(data).into()
    }
}


#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkFreqErrProto {
    link_freq: B4,
    protocol_error: bool,
    overflow_error: bool,
    end_of_chain_error: bool,
    ctl_timeout: bool,
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
            (true, 0b0001)  => Self::Rate2800MHz,
            (true, 0b0010)  => Self::Rate3000MHz,
            (true, 0b0011)  => Self::Rate3200MHz,
            (b, v) => Self::Reserved((v & 0b1111) | ((b as u8) << 4)),
        }
    }
}
impl Default for LinkFrequency {
    fn default() -> Self { Self::Rate200MHz }
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
impl From<LinkFreqErrProto> for LinkError {
    fn from(proto: LinkFreqErrProto) -> Self {
        Self {
            protocol_error: proto.protocol_error(),
            overflow_error: proto.overflow_error(),
            end_of_chain_error: proto.end_of_chain_error(),
            ctl_timeout: proto.ctl_timeout(),
        }
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct LinkFrequencyCapabilityProto {
    supports_200mhz: bool,
    supports_300mhz: bool,
    supports_400mhz: bool,
    supports_500mhz: bool,
    supports_600mhz: bool,
    supports_800mhz: bool,
    supports_1000mhz: bool,
    supports_1200mhz: bool,
    supports_1400mhz: bool,
    supports_1600mhz: bool,
    supports_1800mhz: bool,
    supports_2000mhz: bool,
    supports_2200mhz: bool,
    supports_2400mhz: bool,
    supports_2600mhz: bool,
    supports_vendor_specific: bool,
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
impl From<LinkFrequencyCapabilityProto> for LinkFrequencyCapability {
    fn from(proto: LinkFrequencyCapabilityProto) -> Self {
        Self {
            supports_200mhz: proto.supports_200mhz(),
            supports_300mhz: proto.supports_300mhz(),
            supports_400mhz: proto.supports_400mhz(),
            supports_500mhz: proto.supports_500mhz(),
            supports_600mhz: proto.supports_600mhz(),
            supports_800mhz: proto.supports_800mhz(),
            supports_1000mhz: proto.supports_1000mhz(),
            supports_1200mhz: proto.supports_1200mhz(),
            supports_1400mhz: proto.supports_1400mhz(),
            supports_1600mhz: proto.supports_1600mhz(),
            supports_1800mhz: proto.supports_1800mhz(),
            supports_2000mhz: proto.supports_2000mhz(),
            supports_2200mhz: proto.supports_2200mhz(),
            supports_2400mhz: proto.supports_2400mhz(),
            supports_2600mhz: proto.supports_2600mhz(),
            supports_vendor_specific: proto.supports_vendor_specific(),
        }
    }
}
impl From<u16> for LinkFrequencyCapability {
    fn from(word: u16) -> Self { LinkFrequencyCapabilityProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct FeatureCapabilityProto {
    isochronous_flow_control_mode: bool,
    ldtstop: bool,
    crc_test_mode: bool,
    extended_ctl_time_required: bool,
    qword_addressing: bool,
    unitid_reorder_disable: bool,
    source_identification_extension: bool,
    rsvdp: B1,
    extended_register_set: bool,
    upstream_configuration_enable: bool,
    rsvdp_2: B6,
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
impl From<FeatureCapabilityProto> for FeatureCapability {
    fn from(proto: FeatureCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            isochronous_flow_control_mode: proto.isochronous_flow_control_mode(),
            ldtstop: proto.ldtstop(),
            crc_test_mode: proto.crc_test_mode(),
            extended_ctl_time_required: proto.extended_ctl_time_required(),
            qword_addressing: proto.qword_addressing(),
            unitid_reorder_disable: proto.unitid_reorder_disable(),
            source_identification_extension: proto.source_identification_extension(),
            extended_register_set: proto.extended_register_set(),
            upstream_configuration_enable: proto.upstream_configuration_enable(),
        }
    }
}
impl From<u16> for FeatureCapability {
    fn from(word: u16) -> Self { FeatureCapabilityProto::from(word).into() }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct ErrorHandlingProto {
    protocol_error_flood_enable: bool,
    overflow_error_flood_enable: bool,
    protocol_error_fatal_enable: bool,
    overflow_error_fatal_enable: bool,
    end_of_chain_error_fatal_enable: bool,
    response_error_fatal_enable: bool,
    crc_error_fatal_enable: bool,
    system_error_fatal_enable: bool,
    chain_fail: bool,
    response_error: bool,
    protocol_error_nonfatal_enable: bool,
    overflow_error_nonfatal_enable: bool,
    end_of_chain_error_nonfatal_enable: bool,
    response_error_nonfatal_enable: bool,
    crc_error_nonfatal_enable: bool,
    system_error_nonfatal_enable: bool,
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
impl From<ErrorHandlingProto> for ErrorHandling {
    fn from(proto: ErrorHandlingProto) -> Self {
        Self {
            protocol_error_flood_enable: proto.protocol_error_flood_enable(),
            overflow_error_flood_enable: proto.overflow_error_flood_enable(),
            protocol_error_fatal_enable: proto.protocol_error_fatal_enable(),
            overflow_error_fatal_enable: proto.overflow_error_fatal_enable(),
            end_of_chain_error_fatal_enable: proto.end_of_chain_error_fatal_enable(),
            response_error_fatal_enable: proto.response_error_fatal_enable(),
            crc_error_fatal_enable: proto.crc_error_fatal_enable(),
            system_error_fatal_enable: proto.system_error_fatal_enable(),
            chain_fail: proto.chain_fail(),
            response_error: proto.response_error(),
            protocol_error_nonfatal_enable: proto.protocol_error_nonfatal_enable(),
            overflow_error_nonfatal_enable: proto.overflow_error_nonfatal_enable(),
            end_of_chain_error_nonfatal_enable: proto.end_of_chain_error_nonfatal_enable(),
            response_error_nonfatal_enable: proto.response_error_nonfatal_enable(),
            crc_error_nonfatal_enable: proto.crc_error_nonfatal_enable(),
            system_error_nonfatal_enable: proto.system_error_nonfatal_enable(),
        }
    }
}
impl From<u16> for ErrorHandling {
    fn from(word: u16) -> Self { ErrorHandlingProto::from(word).into() }
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
    pub fn link_freq(&self, link_freq_ext: bool) -> LinkFrequency {
        LinkFrequency::new(link_freq_ext, self.link_freq)
    }
}
impl<'a> TryRead<'a, Endian> for HostOrSecondaryInterface {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let link_freq_err_proto: LinkFreqErrProto =
            bytes.read_with::<u8>(&mut (0x09 - CAP_HEADER_LEN), endian)?.into();
        let hosi = HostOrSecondaryInterface {
            command: bytes.read_with::<u16>(offset, endian)?.into(),
            link_control: bytes.read_with::<u16>(offset, endian)?.into(),
            link_config: bytes.read_with::<u16>(offset, endian)?.into(),
            revision_id: bytes.read_with::<u8>(offset, endian)?.into(),
            link_freq: link_freq_err_proto.link_freq(),
            link_error: link_freq_err_proto.into(),
            link_freq_cap: {
                *offset += 1; // link_freq_err_proto skipped
                bytes.read_with::<u16>(offset, endian)?.into()
            },
            feature: bytes.read_with::<u16>(offset, endian)?.into(),
            enumeration_scratchpad: {
                let _reserved = bytes.read_with::<u16>(offset, endian)?;
                bytes.read_with::<u16>(offset, endian)?
            },
            error_handling: bytes.read_with::<u16>(offset, endian)?.into(),
            mem_base_upper: bytes.read_with::<u8>(offset, endian)?,
            mem_limit_upper: bytes.read_with::<u8>(offset, endian)?,
        };
        Ok((hosi, *offset))
    }
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct HostOrSecondaryCommandProto {
    warm_reset: bool,
    double_ended: bool,
    device_number: B5,
    chain_side: bool,
    host_hide: bool,
    rsvdp: B1,
    act_as_slave: bool,
    host_inbound_end_of_chain_error: bool,
    drop_on_uninitialized_link: bool,
    capability_type: B3,
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
impl From<HostOrSecondaryCommandProto> for HostOrSecondaryCommand {
    fn from(proto: HostOrSecondaryCommandProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.capability_type();
        Self {
            warm_reset: proto.warm_reset(),
            double_ended: proto.double_ended(),
            device_number: proto.device_number(),
            chain_side: proto.chain_side(),
            host_hide: proto.host_hide(),
            act_as_slave: proto.act_as_slave(),
            host_inbound_end_of_chain_error: proto.host_inbound_end_of_chain_error(),
            drop_on_uninitialized_link: proto.drop_on_uninitialized_link(),
        }
    }
}
impl From<u16> for HostOrSecondaryCommand {
    fn from(word: u16) -> Self { HostOrSecondaryCommandProto::from(word).into() }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservedHost {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterruptDiscoveryAndConfiguration {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitIdClumping {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedConfigurationSpaceAccess {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressMapping {
}


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct MsiMappingProto {
    enabled: bool,
    fixed: bool,
    rsvdp: B9,
    capability_type: B5,
}

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
    pub fn base_address(&self) -> u64 {
        let h = (self.base_address_upper as u64) << 32;
        let l = (self.base_address_lower as u64) & !0xfffff;
        h | l
    }
}
impl<'a> TryRead<'a, Endian> for MsiMapping {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let msi_mapping_proto: MsiMappingProto =
            bytes.read_with::<u16>(offset, endian)?.into();
        let _ = msi_mapping_proto.rsvdp();
        let _ = msi_mapping_proto.capability_type();
        let msim = MsiMapping {
            enabled: msi_mapping_proto.enabled(),
            fixed: msi_mapping_proto.fixed(),
            base_address_lower: bytes.read_with::<u32>(offset, endian)?,
            base_address_upper: bytes.read_with::<u32>(offset, endian)?,
        };
        Ok((msim, *offset))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectRoute {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VCSet {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryMode {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X86Encoding {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gen3 {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionLevelExtension {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerManagement {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighNodeCount {
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

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
        let result = data[2..].read_with::<Hypertransport>(&mut 0, LE).unwrap();
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
            revision_id: RevisionId { minor: 0, major: 3, },
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
        assert_eq!(Hypertransport::SlaveOrPrimaryInterface(sample), result);
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
        let result = data[2..].read_with::<Hypertransport>(&mut 0, LE).unwrap();
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
            revision_id: RevisionId { minor: 0, major: 3, },
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
        assert_eq!(Hypertransport::HostOrSecondaryInterface(sample), result);
    }

    #[test]
    fn revision_id() {
        let data = [
            0x08, 0x00, 0x25, 0b10001000, // +00h
        ];
        let result = data[2..].read_with::<Hypertransport>(&mut 0, LE).unwrap();
        let sample = RevisionId {
            major: 1,
            minor: 5,
        };
        assert_eq!(Hypertransport::RevisionId(sample), result);
    }

    #[test]
    fn msi_mapping() {
        // MSI Mapping Enable+ Fixed+
        let data = [
            0x08, 0xb0, 0x03, 0xa8, // +00h
            0x00, 0x00, 0x00, 0x00, // +04h
            0x34, 0x17, 0xda, 0x11, // +08h
        ];
        let result = data[2..].read_with::<Hypertransport>(&mut 0, LE).unwrap();
        let sample = MsiMapping {
            enabled: true,
            fixed: true,
            base_address_lower: 0,
            base_address_upper: 0x11da1734,
        };
        assert_eq!(Hypertransport::MsiMapping(sample), result);
        
        match result {
            Hypertransport::MsiMapping(msim) =>
                assert_eq!(0x11da173400000000, msim.base_address()),
            _ => unreachable!(),
        }
    }
}
