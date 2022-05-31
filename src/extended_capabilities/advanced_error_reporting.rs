/*!
# Advanced Error Reporting Capability

The PCI Express Advanced Error Reporting Capability is an optional Extended Capability that may
be implemented by PCI Express device Functions supporting advanced error control and reporting.

## Struct diagram
[AdvancedErrorReporting]
- [UncorrectableError]
- [UncorrectableError]
- [UncorrectableError]
- [CorrectableError]
- [CorrectableError]
- [AdvancedErrorCapabilitiesAndControl]
- [HeaderLog]
- [RootErrorCommand]
- [RootErrorStatus]
- [ErrorSourceIdentification]
- [TlpPrefixLog]

## Examples

> ```text
> UESta:  DLP- SDES- TLP- FCP- CmpltTO- CmpltAbrt- UnxCmplt- RxOF- MalfTLP- ECRC- UnsupReq- ACSViol-
> UEMsk:  DLP- SDES- TLP- FCP- CmpltTO- CmpltAbrt- UnxCmplt+ RxOF- MalfTLP- ECRC- UnsupReq+ ACSViol+
> UESvrt: DLP+ SDES- TLP+ FCP- CmpltTO+ CmpltAbrt+ UnxCmplt- RxOF+ MalfTLP+ ECRC- UnsupReq- ACSViol-
> CESta:  RxErr- BadTLP- BadDLLP- Rollover- Timeout- AdvNonFatalErr-
> CEMsk:  RxErr+ BadTLP+ BadDLLP+ Rollover+ Timeout+ AdvNonFatalErr+
> AERCap: First Error Pointer: 00, ECRCGenCap- ECRCGenEn- ECRCChkCap- ECRCChkEn-
>         MultHdrRecCap- MultHdrRecEn- TLPPfxPres- HdrLogCap-
> HeaderLog: 00000000 00000000 00000000 00000000
> RootCmd: CERptEn- NFERptEn- FERptEn-
> RootSta: CERcvd- MultCERcvd- UERcvd- MultUERcvd-
>          FirstFatal- NonFatalMsg- FatalMsg- IntMsg 0
> ErrorSrc: ERR_COR: 0000 ERR_FATAL/NONFATAL: 0000
> ```

```rust
# use pcics::extended_capabilities::advanced_error_reporting::*;
use pretty_assertions::assert_eq;
let data = [
    0x01, 0x00, 0x01, 0x14, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x31, 0x00, 0x11, 0xd0, 0x06, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xc1, 0x31, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
let result = data[4..].try_into().unwrap();
let sample = AdvancedErrorReporting {
    uncorrectable_error_status: UncorrectableError {
        link_training_error: false,
        data_link_protocol_error_status: false,
        surprise_down_error_status: false,
        poisoned_tlp_received_status: false,
        flow_control_protocol_error_status: false,
        completion_timeout_status: false,
        completer_abort_status: false,
        unexpected_completion_status: false,
        receiver_overflow_status: false,
        malformed_tlp_status: false,
        ecrc_error_status: false,
        unsupported_request_error_status: false,
        acs_violation_status: false,
        uncorrectable_internal_error_status: false,
        mc_blocked_tlp_status: false,
        atomicop_egress_blocked_status: false,
        tlp_prefix_blocked_error_status: false,
        poisoned_tlp_egress_blocked_status: false,
    },
    uncorrectable_error_mask: UncorrectableError {
        link_training_error: false,
        data_link_protocol_error_status: false,
        surprise_down_error_status: false,
        poisoned_tlp_received_status: false,
        flow_control_protocol_error_status: false,
        completion_timeout_status: false,
        completer_abort_status: false,
        unexpected_completion_status: true,
        receiver_overflow_status: false,
        malformed_tlp_status: false,
        ecrc_error_status: false,
        unsupported_request_error_status: true,
        acs_violation_status: true,
        uncorrectable_internal_error_status: false,
        mc_blocked_tlp_status: false,
        atomicop_egress_blocked_status: false,
        tlp_prefix_blocked_error_status: false,
        poisoned_tlp_egress_blocked_status: false,
    },
    uncorrectable_error_severity: UncorrectableError {
        link_training_error: true,
        data_link_protocol_error_status: true,
        surprise_down_error_status: false,
        poisoned_tlp_received_status: true,
        flow_control_protocol_error_status: false,
        completion_timeout_status: true,
        completer_abort_status: true,
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
    },
    correctable_error_status: CorrectableError {
        receiver_error_status: false,
        bad_tlp_status: false,
        bad_dllp_status: false,
        replay_num_rollover_status: false,
        replay_timer_timeout_status: false,
        advisory_non_fatal_error_status: false,
        corrected_internal_error_status: false,
        header_log_overflow_status: false,
    },
    correctable_error_mask: CorrectableError {
        receiver_error_status: true,
        bad_tlp_status: true,
        bad_dllp_status: true,
        replay_num_rollover_status: true,
        replay_timer_timeout_status: true,
        advisory_non_fatal_error_status: true,
        corrected_internal_error_status: false,
        header_log_overflow_status: false,
    },
    advanced_error_capabilities_and_control: AdvancedErrorCapabilitiesAndControl {
        first_error_pointer: 0x00,
        ecrc_generation_capable: false,
        ecrc_generation_enable: false,
        ecrc_check_capable: false,
        ecrc_check_enable: false,
        multiple_header_recording_capable: false,
        multiple_header_recording_enable: false,
        tlp_prefix_log_present: false,
        completion_timeout_prefix_or_header_log_capable: false,
    },
    header_log: HeaderLog([0x00, 0x00, 0x00, 0x00]),
    root_error_command: Some(RootErrorCommand {
        correctable_error_reporting_enable: false,
        non_fatal_error_reporting_enable: false,
        fatal_error_reporting_enable: false,
    }),
    root_error_status: Some(RootErrorStatus {
        err_cor_received: false,
        multiple_err_cor_received: false,
        err_fatal_or_nonfatal_received: false,
        multiple_err_fatal_or_nonfatal_received: false,
        first_uncorrectable_fatal: false,
        non_fatal_error_messages_received: false,
        fatal_error_messages_received: false,
        advanced_error_interrupt_message_number: 0x00,
    }),
    error_source_identification: Some(ErrorSourceIdentification {
        err_cor_source_identification: 0x00,
        err_fatal_or_nonfatal_source_identification: 0x00,
    }),
    tlp_prefix_log: None,
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P10, P11, P2, P21, P4, P7, P9};
use snafu::Snafu;

/// Advanced Error Reporting Error
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum AdvancedErrorReportingError {
    #[snafu(display("can't read common registres"))]
    Common,
    #[snafu(display("can't read TLP Prefix Log"))]
    TlpPrefixLog,
}

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
    pub root_error_command: Option<RootErrorCommand>,
    pub root_error_status: Option<RootErrorStatus>,
    pub error_source_identification: Option<ErrorSourceIdentification>,
    pub tlp_prefix_log: Option<TlpPrefixLog>,
}
impl AdvancedErrorReporting {
    /// Common registers size (exists) on any device type
    pub const COMMON_SIZE: usize = 0x2c - 0x04;
    // Root Ports and Root Complex Event Collectors registres size
    pub const RP_AND_RCEC_SIZE: usize = 0x38 - 0x04;
    /// Full size with TLP Prefix Log register
    pub const FULL_SIZE: usize = 0x48 - 0x04;
    /// Header log register size (4 x u32)
    pub const HEADER_LOG_SIZE: usize = 4 * 4;
    /// TLP Prefix Log register size (4 x u32)
    pub const TLP_PREFIX_LOG_SIZE: usize = 4 * 4;
}
impl TryFrom<&[u8]> for AdvancedErrorReporting {
    type Error = AdvancedErrorReportingError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head:
                Le((
                    uncorrectable_error_status,
                    uncorrectable_error_mask,
                    uncorrectable_error_severity,
                    correctable_error_status,
                    correctable_error_mask,
                    advanced_error_capabilities_and_control,
                    header_log,
                )),
            tail: slice,
        } = P7(slice)
            .try_into()
            .map_err(|_| AdvancedErrorReportingError::Common)?;
        let (root_error_command, root_error_status, error_source_identification) =
            if let Ok(Seq {
                head:
                    Le((
                        root_error_command,
                        root_error_status,
                        err_cor_source_identification,
                        err_fatal_or_nonfatal_source_identification,
                    )),
                ..
            }) = P4(slice).try_into()
            {
                (
                    Some(From::<u32>::from(root_error_command)),
                    Some(From::<u32>::from(root_error_status)),
                    Some(ErrorSourceIdentification {
                        err_cor_source_identification,
                        err_fatal_or_nonfatal_source_identification,
                    }),
                )
            } else {
                (None, None, None)
            };
        let advanced_error_capabilities_and_control: AdvancedErrorCapabilitiesAndControl =
            From::<u32>::from(advanced_error_capabilities_and_control);
        let Le(header_log) = From::<[u8; Self::HEADER_LOG_SIZE]>::from(header_log);
        let tlp_prefix_log = if advanced_error_capabilities_and_control.tlp_prefix_log_present {
            let Seq {
                head: Le((_rp_and_rcec, tlp_prefix_log)),
                ..
            } = P2(slice)
                .try_into()
                .map_err(|_| AdvancedErrorReportingError::TlpPrefixLog)?;
            let _: [u8; Self::RP_AND_RCEC_SIZE] = _rp_and_rcec;
            let Le(tlp_prefix_log) = From::<[u8; Self::TLP_PREFIX_LOG_SIZE]>::from(tlp_prefix_log);
            Some(TlpPrefixLog(tlp_prefix_log))
        } else {
            None
        };
        Ok(Self {
            uncorrectable_error_status: From::<u32>::from(uncorrectable_error_status),
            uncorrectable_error_mask: From::<u32>::from(uncorrectable_error_mask),
            uncorrectable_error_severity: From::<u32>::from(uncorrectable_error_severity),
            correctable_error_status: From::<u32>::from(correctable_error_status),
            correctable_error_mask: From::<u32>::from(correctable_error_mask),
            advanced_error_capabilities_and_control,
            header_log: HeaderLog(header_log),
            root_error_command,
            root_error_status,
            error_source_identification,
            tlp_prefix_log,
        })
    }
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
impl From<u32> for UncorrectableError {
    fn from(dword: u32) -> Self {
        let Lsb((
            link_training_error,
            (),
            data_link_protocol_error_status,
            surprise_down_error_status,
            (),
            poisoned_tlp_received_status,
            flow_control_protocol_error_status,
            completion_timeout_status,
            completer_abort_status,
            unexpected_completion_status,
            receiver_overflow_status,
            malformed_tlp_status,
            ecrc_error_status,
            unsupported_request_error_status,
            acs_violation_status,
            uncorrectable_internal_error_status,
            mc_blocked_tlp_status,
            atomicop_egress_blocked_status,
            tlp_prefix_blocked_error_status,
            poisoned_tlp_egress_blocked_status,
            (),
        )) = P21::<_, 1, 3, 1, 1, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5>(dword).into();
        Self {
            link_training_error,
            data_link_protocol_error_status,
            surprise_down_error_status,
            poisoned_tlp_received_status,
            flow_control_protocol_error_status,
            completion_timeout_status,
            completer_abort_status,
            unexpected_completion_status,
            receiver_overflow_status,
            malformed_tlp_status,
            ecrc_error_status,
            unsupported_request_error_status,
            acs_violation_status,
            uncorrectable_internal_error_status,
            mc_blocked_tlp_status,
            atomicop_egress_blocked_status,
            tlp_prefix_blocked_error_status,
            poisoned_tlp_egress_blocked_status,
        }
    }
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
impl From<u32> for CorrectableError {
    fn from(dword: u32) -> Self {
        let Lsb((
            receiver_error_status,
            (),
            bad_tlp_status,
            bad_dllp_status,
            replay_num_rollover_status,
            (),
            replay_timer_timeout_status,
            advisory_non_fatal_error_status,
            corrected_internal_error_status,
            header_log_overflow_status,
            (),
        )) = P11::<_, 1, 5, 1, 1, 1, 3, 1, 1, 1, 1, 16>(dword).into();
        Self {
            receiver_error_status,
            bad_tlp_status,
            bad_dllp_status,
            replay_num_rollover_status,
            replay_timer_timeout_status,
            advisory_non_fatal_error_status,
            corrected_internal_error_status,
            header_log_overflow_status,
        }
    }
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

impl From<u32> for AdvancedErrorCapabilitiesAndControl {
    fn from(dword: u32) -> Self {
        let Lsb((
            first_error_pointer,
            ecrc_generation_capable,
            ecrc_generation_enable,
            ecrc_check_capable,
            ecrc_check_enable,
            multiple_header_recording_capable,
            multiple_header_recording_enable,
            tlp_prefix_log_present,
            completion_timeout_prefix_or_header_log_capable,
            (),
        )) = P10::<_, 5, 1, 1, 1, 1, 1, 1, 1, 1, 10>(dword).into();
        Self {
            first_error_pointer,
            ecrc_generation_capable,
            ecrc_generation_enable,
            ecrc_check_capable,
            ecrc_check_enable,
            multiple_header_recording_capable,
            multiple_header_recording_enable,
            tlp_prefix_log_present,
            completion_timeout_prefix_or_header_log_capable,
        }
    }
}

/// The Header Log register contains the header for the TLP corresponding to a detected error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderLog(pub [u32; 4]);

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

impl From<u32> for RootErrorCommand {
    fn from(dword: u32) -> Self {
        let Lsb((
            correctable_error_reporting_enable,
            non_fatal_error_reporting_enable,
            fatal_error_reporting_enable,
            (),
        )) = P4::<_, 1, 1, 1, 29>(dword).into();
        Self {
            correctable_error_reporting_enable,
            non_fatal_error_reporting_enable,
            fatal_error_reporting_enable,
        }
    }
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

impl From<u32> for RootErrorStatus {
    fn from(dword: u32) -> Self {
        let Lsb((
            err_cor_received,
            multiple_err_cor_received,
            err_fatal_or_nonfatal_received,
            multiple_err_fatal_or_nonfatal_received,
            first_uncorrectable_fatal,
            non_fatal_error_messages_received,
            fatal_error_messages_received,
            (),
            advanced_error_interrupt_message_number,
        )) = P9::<_, 1, 1, 1, 1, 1, 1, 1, 20, 5>(dword).into();
        Self {
            err_cor_received,
            multiple_err_cor_received,
            err_fatal_or_nonfatal_received,
            multiple_err_fatal_or_nonfatal_received,
            first_uncorrectable_fatal,
            non_fatal_error_messages_received,
            fatal_error_messages_received,
            advanced_error_interrupt_message_number,
        }
    }
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
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn uncorrectable_error_status() {
        // UESvrt: DLP+ SDES- TLP- FCP- CmpltTO- CmpltAbrt- UnxCmplt- RxOF+ MalfTLP+ ECRC- UnsupReq- ACSViol-
        let data = [0x11, 0x00, 0x06, 0x00];
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
        let data = [0x00, 0x20, 0x00, 0x00];
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
        let data = [0x00, 0x00, 0x00, 0x00];
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
        let data = [0x07, 0x00, 0x00, 0x00];
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
        let data = [0x00, 0x00, 0x00, 0x00];
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
