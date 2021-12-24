//! Advanced Error Reporting Capability
//!
//! The PCI Express Advanced Error Reporting Capability is an optional Extended Capability that may
//! be implemented by PCI Express device Functions supporting advanced error control and reporting.

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancedErrorReporting {
    /// The Uncorrectable Error Status register indicates error detection status of individual errors
    /// on a PCI Express device Function.
    pub uncorrectable_error_status: UncorrectableError,
    /// The Uncorrectable Error Mask register controls reporting of individual errors by the device
    /// Function to the PCI Express Root Complex via a PCI Express error Message.
    pub uncorrectable_error_mask: UncorrectableError,
    /// The Uncorrectable Error Severity register controls whether an individual error is reported as a
    /// Nonfatal or Fatal error.
    pub uncorrectable_error_severity: UncorrectableError,
    /// The Correctable Error Status register reports error status of individual correctable error
    /// sources on a PCI Express device Function.
    pub correctable_error_status: CorrectableError,
    /// The Correctable Error Mask register controls reporting of individual correctable errors by this
    /// Function to the PCI Express Root Complex via a PCI Express error Message.
    pub correctable_error_mask: CorrectableError,
    pub advanced_error_capabilities_and_control: AdvancedErrorCapabilitiesAndControl,
    pub header_log: HeaderLog,
    pub root_error_command: RootErrorCommand,
    pub root_error_status: RootErrorStatus,
    pub error_source_identification: ErrorSourceIdentification,
    pub tlp_prefix_log: TlpPrefixLog,
}
impl<'a> TryRead<'a, Endian> for AdvancedErrorReporting {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let aer = AdvancedErrorReporting {
            uncorrectable_error_status: bytes.read_with::<u32>(offset, endian)?.into(),
            uncorrectable_error_mask: bytes.read_with::<u32>(offset, endian)?.into(),
            uncorrectable_error_severity: bytes.read_with::<u32>(offset, endian)?.into(),
            correctable_error_status: bytes.read_with::<u32>(offset, endian)?.into(),
            correctable_error_mask: bytes.read_with::<u32>(offset, endian)?.into(),
            advanced_error_capabilities_and_control: bytes.read_with::<u32>(offset, endian)?.into(),
            header_log: HeaderLog([
                bytes.read_with::<u32>(offset, endian)?,
                bytes.read_with::<u32>(offset, endian)?,
                bytes.read_with::<u32>(offset, endian)?,
                bytes.read_with::<u32>(offset, endian)?,
            ]),
            root_error_command: bytes.read_with::<u32>(offset, endian)?.into(),
            root_error_status: bytes.read_with::<u32>(offset, endian)?.into(),
            error_source_identification: ErrorSourceIdentification {
                err_cor_source_identification: bytes.read_with::<u16>(offset, endian)?,
                err_fatal_or_nonfatal_source_identification: bytes.read_with::<u16>(offset, endian)?,
            },
            tlp_prefix_log: TlpPrefixLog([
                bytes.read_with::<u32>(offset, endian)?,
                bytes.read_with::<u32>(offset, endian)?,
                bytes.read_with::<u32>(offset, endian)?,
                bytes.read_with::<u32>(offset, endian)?,
            ]),
        };
        Ok((aer, *offset))
    }
}



#[bitfield(bits = 32)]
#[repr(u32)]
pub struct UncorrectableErrorProto {
    link_training_error: bool,
    rsvdz: B3,
    data_link_protocol_error_status: bool,
    surprise_down_error_status: bool,
    rsvdz_2: B6,
    poisoned_tlp_received_status: bool,
    flow_control_protocol_error_status: bool,
    completion_timeout_status: bool,
    completer_abort_status: bool,
    unexpected_completion_status: bool,
    receiver_overflow_status: bool,
    malformed_tlp_status: bool,
    ecrc_error_status: bool,
    unsupported_request_error_status: bool,
    acs_violation_status: bool,
    uncorrectable_internal_error_status: bool,
    mc_blocked_tlp_status: bool,
    atomicop_egress_blocked_status: bool,
    tlp_prefix_blocked_error_status: bool,
    poisoned_tlp_egress_blocked_status: bool,
    rsvdz_3: B5,
}

/// Uncorrectable Error Status, Uncorrectable Error Mask and Uncorrectable Error Severity has same
/// fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UncorrectableError {
    /// Indicate a Link Training Error (legacy)
    pub link_training_error: bool,
    /// Data Link Protocol Error Status
    pub data_link_protocol_error_status: bool,
    /// Surprise Down Error Status (Optional)
    pub surprise_down_error_status: bool,
    /// Poisoned TLP Received Status
    pub poisoned_tlp_received_status: bool,
    /// Flow Control Protocol Error Status (Optional)
    pub flow_control_protocol_error_status: bool,
    /// Completion Timeout Status134
    pub completion_timeout_status: bool,
    /// Completer Abort Status (Optional)
    pub completer_abort_status: bool,
    /// Unexpected Completion Status
    pub unexpected_completion_status: bool,
    /// Receiver Overflow Status (Optional)
    pub receiver_overflow_status: bool,
    /// Malformed TLP Status
    pub malformed_tlp_status: bool,
    /// ECRC Error Status (Optional)
    pub ecrc_error_status: bool,
    /// Unsupported Request Error Status
    pub unsupported_request_error_status: bool,
    /// ACS Violation Status (Optional)
    pub acs_violation_status: bool,
    /// Uncorrectable Internal Error Status (Optional)
    pub uncorrectable_internal_error_status: bool,
    /// MC Blocked TLP Status (Optional)
    pub mc_blocked_tlp_status: bool,
    /// AtomicOp Egress Blocked Status (Optional)
    pub atomicop_egress_blocked_status: bool,
    /// TLP Prefix Blocked Error Status (Optional)
    pub tlp_prefix_blocked_error_status: bool,
    /// Poisoned TLP Egress Blocked Status (Optional)
    pub poisoned_tlp_egress_blocked_status: bool,
}
impl From<UncorrectableErrorProto> for UncorrectableError {
    fn from(proto: UncorrectableErrorProto) -> Self {
        let _ = proto.rsvdz();
        let _ = proto.rsvdz_2();
        let _ = proto.rsvdz_3();
        Self {
            link_training_error: proto.link_training_error(),
            data_link_protocol_error_status: proto.data_link_protocol_error_status(),
            surprise_down_error_status: proto.surprise_down_error_status(),
            poisoned_tlp_received_status: proto.poisoned_tlp_received_status(),
            flow_control_protocol_error_status: proto.flow_control_protocol_error_status(),
            completion_timeout_status: proto.completion_timeout_status(),
            completer_abort_status: proto.completer_abort_status(),
            unexpected_completion_status: proto.unexpected_completion_status(),
            receiver_overflow_status: proto.receiver_overflow_status(),
            malformed_tlp_status: proto.malformed_tlp_status(),
            ecrc_error_status: proto.ecrc_error_status(),
            unsupported_request_error_status: proto.unsupported_request_error_status(),
            acs_violation_status: proto.acs_violation_status(),
            uncorrectable_internal_error_status: proto.uncorrectable_internal_error_status(),
            mc_blocked_tlp_status: proto.mc_blocked_tlp_status(),
            atomicop_egress_blocked_status: proto.atomicop_egress_blocked_status(),
            tlp_prefix_blocked_error_status: proto.tlp_prefix_blocked_error_status(),
            poisoned_tlp_egress_blocked_status: proto.poisoned_tlp_egress_blocked_status(),
        }
    }
}
impl From<u32> for UncorrectableError {
    fn from(dword: u32) -> Self { UncorrectableErrorProto::from(dword).into() }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct CorrectableErrorProto {
    receiver_error_status: bool,
    rsvdz: B5,
    bad_tlp_status: bool,
    bad_dllp_status: bool,
    replay_num_rollover_status: bool,
    rsvdz_2: B3,
    replay_timer_timeout_status: bool,
    advisory_non_fatal_error_status: bool,
    corrected_internal_error_status: bool,
    header_log_overflow_status: bool,
    rsvdz_3: B16,
}

/// Correctable Error Status and Correctable Error Mask has same fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorrectableError {
    /// Receiver Error Status
    pub receiver_error_status: bool,
    /// Bad TLP Status
    pub bad_tlp_status: bool,
    /// Bad DLLP Status
    pub bad_dllp_status: bool,
    /// REPLAY_NUM Rollover Status
    pub replay_num_rollover_status: bool,
    /// Replay Timer Timeout Status
    pub replay_timer_timeout_status: bool,
    /// Advisory Non-Fatal Error Status
    pub advisory_non_fatal_error_status: bool,
    /// Corrected Internal Error Status 
    pub corrected_internal_error_status: bool,
    /// Header Log Overflow Status 
    pub header_log_overflow_status: bool,
}
impl From<CorrectableErrorProto> for CorrectableError {
    fn from(proto: CorrectableErrorProto) -> Self {
        let _ = proto.rsvdz();
        let _ = proto.rsvdz_2();
        let _ = proto.rsvdz_3();
        Self {
            receiver_error_status: proto.receiver_error_status(),
            bad_tlp_status: proto.bad_tlp_status(),
            bad_dllp_status: proto.bad_dllp_status(),
            replay_num_rollover_status: proto.replay_num_rollover_status(),
            replay_timer_timeout_status: proto.replay_timer_timeout_status(),
            advisory_non_fatal_error_status: proto.advisory_non_fatal_error_status(),
            corrected_internal_error_status: proto.corrected_internal_error_status(),
            header_log_overflow_status: proto.header_log_overflow_status(),
        }
    }
}
impl From<u32> for CorrectableError {
    fn from(dword: u32) -> Self { CorrectableErrorProto::from(dword).into() }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct AdvancedErrorCapabilitiesAndControlProto {
    first_error_pointer: B5,
    ecrc_generation_capable: bool,
    ecrc_generation_enable: bool,
    ecrc_check_capable: bool,
    ecrc_check_enable: bool,
    multiple_header_recording_capable: bool,
    multiple_header_recording_enable: bool,
    tlp_prefix_log_present: bool,
    completion_timeout_prefix_or_header_log_capable: bool,
    rsvdp: B19,
}

/// Advanced Error Capabilities and Control Register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancedErrorCapabilitiesAndControl {
    /// First Error Pointer
    pub first_error_pointer: u8,
    /// ECRC Generation Capable
    pub ecrc_generation_capable: bool,
    /// ECRC Generation Enable
    pub ecrc_generation_enable: bool,
    /// ECRC Check Capable
    pub ecrc_check_capable: bool,
    /// ECRC Check Enable
    pub ecrc_check_enable: bool,
    /// Multiple Header Recording Capable
    pub multiple_header_recording_capable: bool,
    /// Multiple Header Recording Enable
    pub multiple_header_recording_enable: bool,
    /// TLP Prefix Log Present
    pub tlp_prefix_log_present: bool,
    /// Completion Timeout Prefix/Header Log Capable
    pub completion_timeout_prefix_or_header_log_capable: bool,
}
impl From<AdvancedErrorCapabilitiesAndControlProto> for AdvancedErrorCapabilitiesAndControl {
    fn from(proto: AdvancedErrorCapabilitiesAndControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            first_error_pointer: proto.first_error_pointer(),
            ecrc_generation_capable: proto.ecrc_generation_capable(),
            ecrc_generation_enable: proto.ecrc_generation_enable(),
            ecrc_check_capable: proto.ecrc_check_capable(),
            ecrc_check_enable: proto.ecrc_check_enable(),
            multiple_header_recording_capable: proto.multiple_header_recording_capable(),
            multiple_header_recording_enable: proto.multiple_header_recording_enable(),
            tlp_prefix_log_present: proto.tlp_prefix_log_present(),
            completion_timeout_prefix_or_header_log_capable: proto.completion_timeout_prefix_or_header_log_capable(),
        }
    }
}
impl From<u32> for AdvancedErrorCapabilitiesAndControl {
    fn from(dword: u32) -> Self { AdvancedErrorCapabilitiesAndControlProto::from(dword).into() }
}


/// The Header Log register contains the header for the TLP corresponding to a detected error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderLog(pub [u32; 4]);


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct RootErrorCommandProto {
    correctable_error_reporting_enable: bool,
    non_fatal_error_reporting_enable: bool,
    fatal_error_reporting_enable: bool,
    rsvdp: B29,
}
/// The Root Error Command register allows further control of Root Complex response to Correctable,
/// Non-Fatal, and Fatal error Messages than the basic Root Complex capability to generate system
/// errors in response to error Messages (either received or internally generated).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootErrorCommand {
    /// Correctable Error Reporting Enable
    pub correctable_error_reporting_enable: bool,
    /// Non-Fatal Error Reporting Enable
    pub non_fatal_error_reporting_enable: bool,
    /// Fatal Error Reporting Enable
    pub fatal_error_reporting_enable: bool,
}
impl From<RootErrorCommandProto> for RootErrorCommand {
    fn from(proto: RootErrorCommandProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            correctable_error_reporting_enable: proto.correctable_error_reporting_enable(),
            non_fatal_error_reporting_enable: proto.non_fatal_error_reporting_enable(),
            fatal_error_reporting_enable: proto.fatal_error_reporting_enable(),
        }
    }
}
impl From<u32> for RootErrorCommand {
    fn from(dword: u32) -> Self { RootErrorCommandProto::from(dword).into() }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct RootErrorStatusProto {
    err_cor_received: bool,
    multiple_err_cor_received: bool,
    err_fatal_or_nonfatal_received: bool,
    multiple_err_fatal_or_nonfatal_received: bool,
    first_uncorrectable_fatal: bool,
    non_fatal_error_messages_received: bool,
    fatal_error_messages_received: bool,
    rsvdz: B20,
    advanced_error_interrupt_message_number: B5,
}
/// The Root Error Status register reports status of error Messages (ERR_COR, ERR_NONFATAL, and
/// ERR_FATAL) received by the Root Port, and of errors detected by the Root Port itself (which are
/// treated conceptually as if the Root Port had sent an error Message to itself).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootErrorStatus {
    /// ERR_COR Received
    pub err_cor_received: bool,
    /// Multiple ERR_COR Received
    pub multiple_err_cor_received: bool,
    /// ERR_FATAL/NONFATAL Received
    pub err_fatal_or_nonfatal_received: bool,
    /// Multiple ERR_FATAL/NONFATAL Received
    pub multiple_err_fatal_or_nonfatal_received: bool,
    /// First Uncorrectable Fatal
    pub first_uncorrectable_fatal: bool,
    /// Non-Fatal Error Messages Received
    pub non_fatal_error_messages_received: bool,
    /// Fatal Error Messages Received
    pub fatal_error_messages_received: bool,
    /// Advanced Error Interrupt Message Number
    pub advanced_error_interrupt_message_number: u8,
}
impl From<RootErrorStatusProto> for RootErrorStatus {
    fn from(proto: RootErrorStatusProto) -> Self {
        let _ = proto.rsvdz();
        Self {
            err_cor_received: proto.err_cor_received(),
            multiple_err_cor_received: proto.multiple_err_cor_received(),
            err_fatal_or_nonfatal_received: proto.err_fatal_or_nonfatal_received(),
            multiple_err_fatal_or_nonfatal_received: proto.multiple_err_fatal_or_nonfatal_received(),
            first_uncorrectable_fatal: proto.first_uncorrectable_fatal(),
            non_fatal_error_messages_received: proto.non_fatal_error_messages_received(),
            fatal_error_messages_received: proto.fatal_error_messages_received(),
            advanced_error_interrupt_message_number: proto.advanced_error_interrupt_message_number(),
        }
    }
}
impl From<u32> for RootErrorStatus {
    fn from(dword: u32) -> Self { RootErrorStatusProto::from(dword).into() }
}


/// The Error Source Identification register identifies the source (Requester ID) of first
/// correctable and uncorrectable (Non-fatal/Fatal) errors reported in the Root Error Status
/// register. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorSourceIdentification {
    /// ERR_COR Source Identification
    pub err_cor_source_identification: u16,
    /// ERR_FATAL/NONFATAL Source Identification
    pub err_fatal_or_nonfatal_source_identification: u16,
}


/// The TLP Prefix Log register captures the End-End TLP Prefix(s) for the TLP corresponding to the
/// detected error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlpPrefixLog(pub [u32; 4]);


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn uncorrectable_error_status() {
        // UESvrt: DLP+ SDES- TLP- FCP- CmpltTO- CmpltAbrt- UnxCmplt- RxOF+ MalfTLP+ ECRC- UnsupReq- ACSViol-
        let data = [ 0x11, 0x00, 0x06, 0x00 ];
        let sample = UncorrectableError {
            link_training_error: true,
            data_link_protocol_error_status: true,
            surprise_down_error_status: false,
            poisoned_tlp_received_status: false,
            flow_control_protocol_error_status: false,
            completion_timeout_status: false,
            completer_abort_status: false,
            unexpected_completion_status: false,
            receiver_overflow_status: true,
            malformed_tlp_status: true,
            ecrc_error_status: false,
            unsupported_request_error_status: false,
            acs_violation_status: false,
            uncorrectable_internal_error_status: false,
            mc_blocked_tlp_status: false,
            atomicop_egress_blocked_status: false,
            tlp_prefix_blocked_error_status: false,
            poisoned_tlp_egress_blocked_status: false,
        };
        assert_eq!(sample, u32::from_le_bytes(data).into());
    }

    #[test]
    fn correctable_error_status() {
        // CEMsk:  RxErr- BadTLP- BadDLLP- Rollover- Timeout- AdvNonFatalErr+
        let data = [ 0x00, 0x20, 0x00, 0x00 ];
        let sample = CorrectableError {
            receiver_error_status: false,
            bad_tlp_status: false,
            bad_dllp_status: false,
            replay_num_rollover_status: false,
            replay_timer_timeout_status: false,
            advisory_non_fatal_error_status: true,
            corrected_internal_error_status: false,
            header_log_overflow_status: false,
        };
        assert_eq!(sample, u32::from_le_bytes(data).into());
    }

    #[test]
    fn advanced_error_capabilities_and_control() {
        // AERCap: First Error Pointer: 00, ECRCGenCap- ECRCGenEn- ECRCChkCap- ECRCChkEn-
        //         MultHdrRecCap- MultHdrRecEn- TLPPfxPres- HdrLogCap-
        let data = [ 0x00, 0x00, 0x00, 0x00 ];
        let sample = AdvancedErrorCapabilitiesAndControl {
            first_error_pointer: 0,
            ecrc_generation_capable: false,
            ecrc_generation_enable: false,
            ecrc_check_capable: false,
            ecrc_check_enable: false,
            multiple_header_recording_capable: false,
            multiple_header_recording_enable: false,
            tlp_prefix_log_present: false,
            completion_timeout_prefix_or_header_log_capable: false,
        };
        assert_eq!(sample, u32::from_le_bytes(data).into());
    }

    #[test]
    fn root_error_command() {
        // RootCmd: CERptEn+ NFERptEn+ FERptEn+
        let data = [ 0x07, 0x00, 0x00, 0x00 ];
        let sample = RootErrorCommand {
            correctable_error_reporting_enable: true,
            non_fatal_error_reporting_enable: true,
            fatal_error_reporting_enable: true,
        };
        assert_eq!(sample, u32::from_le_bytes(data).into());
    }

    #[test]
    fn root_error_status() {
        // RootSta: CERcvd- MultCERcvd- UERcvd- MultUERcvd-
        //          FirstFatal- NonFatalMsg- FatalMsg- IntMsg 0
        let data = [ 0x00, 0x00, 0x00, 0x00 ];
        let sample = RootErrorStatus {
            err_cor_received: false,
            multiple_err_cor_received: false,
            err_fatal_or_nonfatal_received: false,
            multiple_err_fatal_or_nonfatal_received: false,
            first_uncorrectable_fatal: false,
            non_fatal_error_messages_received: false,
            fatal_error_messages_received: false,
            advanced_error_interrupt_message_number: 0,
        };
        assert_eq!(sample, u32::from_le_bytes(data).into());
    }
}
