/*!
# Downstream Port Containment (DPC)

The automatic disabling of the Link below a Downstream Port following an uncorrectable error,
which prevents TLPs subsequent to the error from propagating Upstream or Downstream.

## Struct diagram
[DownstreamPortContainment]
- [DpcCapability]
- [DpcControl]
  - [DpcTrigger]
- [DpcStatus]
  - [DpcTriggerReason]
- [RpExtensions]
  - [RpPio] x 4
  - [HeaderLog]
  - [TlpPrefixLog]

## Examples

> ```text
> DpcCap: INT Msg #0, RPExt+ PoisonedTLP+ SwTrigger+ RP PIO Log 7, DL_ActiveErr+
> DpcCtl: Trigger:0 Cmpl- INT- ErrCor- PoisonedTLP- SwTrigger- DL_ActiveErr-
> DpcSta: Trigger- Reason:00 INT- RPBusy- TriggerExt:00 RP PIO ErrPtr:1f
> Source: 0000
> ```

```rust
# use pcics::extended_capabilities::downstream_port_containment::*;
let data = [
    0x1d, 0x00, 0x01, 0x00, // Header
    0xe0, 0x17, 0x00, 0x00, // DPC Capability & DPC Control
    0x00, 0x1f, 0x00, 0x00, // DPC Status & DPC Error Source ID
    0x01, 0x00, 0x00, 0x00, // RP PIO Status
    0x07, 0x07, 0x07, 0x00, // RP PIO Mask
    0x00, 0x00, 0x00, 0x00, // RP PIO Severity
    0x00, 0x00, 0x00, 0x00, // RP PIO SysError
    0x00, 0x00, 0x00, 0x00, // RP PIO Exception
    // RP PIO Header Log
    0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
    // RP PIO ImpSpec Log
    0x42, 0x00, 0x00, 0x00,
    // RP PIO TLP Prefix Log
    0x11, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00,
    0x33, 0x00, 0x00, 0x00,
];

let result = data[4..].try_into().unwrap();

let sample = DownstreamPortContainment {
    dpc_capability: DpcCapability {
        dpc_interrupt_message_number: 0,
        rp_extensions_for_dpc: true,
        poisoned_tlp_egress_blocking_supported: true,
        dpc_software_triggering_supported: true,
        rp_pio_log_size: 7,
        dl_active_err_cor_signaling_supported: true,
    },
    dpc_control: DpcControl {
        dpc_trigger_enable: DpcTrigger::Disabled,
        dpc_completion_control: false,
        dpc_interrupt_enable: false,
        dpc_err_cor_enable: false,
        poisoned_tlp_egress_blocking_enable: false,
        dpc_software_trigger: false,
        dl_active_err_cor_enable: false,
    },
    dpc_status: DpcStatus {
        dpc_trigger_status: false,
        dpc_trigger_reason: DpcTriggerReason::UnmaskedUncorrectableError,
        dpc_interrupt_status: false,
        dpc_rp_busy: false,
        rp_pio_first_error_pointer: 0x1f,
    },
    dpc_error_source_id: 0,
    rp_extensions: Some(RpExtensions {
        rp_pio_status: RpPio {
            cfg_ur_cpl: true,
            cfg_ca_cpl: false,
            cfg_cto: false,
            io_ur_cpl: false,
            io_ca_cpl: false,
            io_cto: false,
            mem_ur_cpl: false,
            mem_ca_cpl: false,
            mem_cto: false,
        },
        rp_pio_mask: RpPio {
            cfg_ur_cpl: true,
            cfg_ca_cpl: true,
            cfg_cto: true,
            io_ur_cpl: true,
            io_ca_cpl: true,
            io_cto: true,
            mem_ur_cpl: true,
            mem_ca_cpl: true,
            mem_cto: true,
        },
        rp_pio_severity: RpPio {
            cfg_ur_cpl: false,
            cfg_ca_cpl: false,
            cfg_cto: false,
            io_ur_cpl: false,
            io_ca_cpl: false,
            io_cto: false,
            mem_ur_cpl: false,
            mem_ca_cpl: false,
            mem_cto: false,
        },
        rp_pio_syserr: RpPio {
            cfg_ur_cpl: false,
            cfg_ca_cpl: false,
            cfg_cto: false,
            io_ur_cpl: false,
            io_ca_cpl: false,
            io_cto: false,
            mem_ur_cpl: false,
            mem_ca_cpl: false,
            mem_cto: false,
        },
        rp_pio_exception: RpPio {
            cfg_ur_cpl: false,
            cfg_ca_cpl: false,
            cfg_cto: false,
            io_ur_cpl: false,
            io_ca_cpl: false,
            io_cto: false,
            mem_ur_cpl: false,
            mem_ca_cpl: false,
            mem_cto: false,
        },
        rp_pio_header_log: HeaderLog([0x01, 0x02, 0x03, 0x04]),
        rp_pio_impspec_log: Some(0x42),
        rp_pio_tlp_prefix_log: Some(TlpPrefixLog([0x11, 0x22, 0x00, 0x00])),
    }),
};

assert_eq!(sample, result);
```
*/

use heterob::{
    bit_numbering::Lsb,
    endianness::{Le, LeBytesTryInto},
    Seq, P12, P4, P6, P7, P8,
};
use snafu::Snafu;

pub use super::advanced_error_reporting::{HeaderLog, TlpPrefixLog};

/// Downstream Port Containment Error
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum DownstreamPortContainmentError {
    #[snafu(display("can't read manadatory registers (8 bytes)"))]
    Mandatory,
    #[snafu(display("can't read RP Extensions registers (36 bytes)"))]
    RpExtensions,
    #[snafu(display("can't read RP PIO ImpSpec Log Register (4 bytes)"))]
    RpPioImpspecLog,
    #[snafu(display("can't read RP PIO TLP Prefix Log Register word#: {number}"))]
    RpPioTlpPrefixLog { number: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownstreamPortContainment {
    /// DPC Capability
    pub dpc_capability: DpcCapability,
    /// DPC Control
    pub dpc_control: DpcControl,
    /// DPC Status
    pub dpc_status: DpcStatus,
    /// DPC Error Source ID
    pub dpc_error_source_id: u16,
    /// Available if [DpcCapability::rp_extensions_for_dpc] is set
    ///
    /// Avalaible if Root Port supports a defined set of DPC Extensions that
    /// are specific to Root Ports. Switch Downstream Ports must not Set this bit.
    pub rp_extensions: Option<RpExtensions>,
}
impl TryFrom<&[u8]> for DownstreamPortContainment {
    type Error = DownstreamPortContainmentError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((dpc_capability, dpc_control, dpc_status, dpc_error_source_id)),
            tail,
        } = P4(slice)
            .try_into()
            .map_err(|_| DownstreamPortContainmentError::Mandatory)?;
        let dpc_capability: DpcCapability = From::<u16>::from(dpc_capability);
        let rp_extensions = if dpc_capability.rp_extensions_for_dpc {
            let Seq {
                head:
                    Le((
                        rp_pio_status,
                        rp_pio_mask,
                        rp_pio_severity,
                        rp_pio_syserr,
                        rp_pio_exception,
                        rp_pio_header_log,
                    )),
                tail,
            } = P6(tail)
                .try_into()
                .map_err(|_| DownstreamPortContainmentError::RpExtensions)?;
            let Le(rp_pio_header_log) = From::<[u8; 4 * 4]>::from(rp_pio_header_log);
            let mut slice = tail;
            let rp_pio_impspec_log = if dpc_capability.rp_pio_log_size >= 5 {
                let Seq {
                    head: rp_pio_impspec_log,
                    tail,
                } = slice
                    .le_bytes_try_into()
                    .map_err(|_| DownstreamPortContainmentError::RpPioImpspecLog)?;
                slice = tail;
                Some(rp_pio_impspec_log)
            } else {
                None
            };

            let rp_pio_tlp_prefix_log = if dpc_capability.rp_pio_log_size > 5 {
                let mut rp_pio_tlp_prefix_log = [0u32; 4];
                let min_rp_pio_log_size = ((dpc_capability.rp_pio_log_size - 5) as usize).min(4);
                for (i, entry) in rp_pio_tlp_prefix_log.iter_mut().enumerate().take(min_rp_pio_log_size) {
                    let Seq { head, tail } = slice.le_bytes_try_into().map_err(|_| {
                        DownstreamPortContainmentError::RpPioTlpPrefixLog { number: i }
                    })?;
                    slice = tail;
                    *entry = u32::from_le_bytes(head);
                }
                Some(TlpPrefixLog(rp_pio_tlp_prefix_log))
            } else {
                None
            };
            Some(RpExtensions {
                rp_pio_status: From::<u32>::from(rp_pio_status),
                rp_pio_mask: From::<u32>::from(rp_pio_mask),
                rp_pio_severity: From::<u32>::from(rp_pio_severity),
                rp_pio_syserr: From::<u32>::from(rp_pio_syserr),
                rp_pio_exception: From::<u32>::from(rp_pio_exception),
                rp_pio_header_log: HeaderLog(rp_pio_header_log),
                rp_pio_impspec_log,
                rp_pio_tlp_prefix_log,
            })
        } else {
            None
        };
        Ok(Self {
            dpc_capability,
            dpc_control: From::<u16>::from(dpc_control),
            dpc_status: From::<u16>::from(dpc_status),
            dpc_error_source_id,
            rp_extensions,
        })
    }
}

/// Root Ports that support RP Extensions for DPC  
/// Switch Downstream Ports must not has this structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpExtensions {
    /// RP PIO Status
    pub rp_pio_status: RpPio,
    /// RP PIO Mask
    pub rp_pio_mask: RpPio,
    /// RP PIO Severity
    pub rp_pio_severity: RpPio,
    /// RP PIO SysErro
    pub rp_pio_syserr: RpPio,
    /// RP PIO Exception
    pub rp_pio_exception: RpPio,
    /// RP PIO Header Log
    pub rp_pio_header_log: HeaderLog,
    /// RP PIO ImpSpec Log  
    /// Space is allocated for this register if the value of the RP PIO Log
    /// Size field is 5 or greater
    pub rp_pio_impspec_log: Option<u32>,
    /// RP PIO TLP Prefix Log  
    /// The allocated size in DWORDs of the RP PIO TLP Prefix Log register is
    /// the RP PIO Log Size minus 5 if the RP PIO Log Size is 9 or less, or 4 if the RP
    /// PIO Log Size is greater than 9.
    pub rp_pio_tlp_prefix_log: Option<TlpPrefixLog>,
}

/// DPC Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpcCapability {
    /// DPC Interrupt Message Number
    pub dpc_interrupt_message_number: u8,
    /// RP Extensions for DPC
    pub rp_extensions_for_dpc: bool,
    /// Poisoned TLP Egress Blocking Supported
    pub poisoned_tlp_egress_blocking_supported: bool,
    /// DPC Software Triggering Supported
    pub dpc_software_triggering_supported: bool,
    /// RP PIO Log Size
    pub rp_pio_log_size: u8,
    /// DL_Active ERR_COR Signaling Supported
    pub dl_active_err_cor_signaling_supported: bool,
}

impl From<u16> for DpcCapability {
    fn from(word: u16) -> Self {
        let Lsb((
            dpc_interrupt_message_number,
            rp_extensions_for_dpc,
            poisoned_tlp_egress_blocking_supported,
            dpc_software_triggering_supported,
            rp_pio_log_size,
            dl_active_err_cor_signaling_supported,
            (),
        )) = P7::<_, 5, 1, 1, 1, 4, 1, 3>(word).into();
        Self {
            dpc_interrupt_message_number,
            rp_extensions_for_dpc,
            poisoned_tlp_egress_blocking_supported,
            dpc_software_triggering_supported,
            rp_pio_log_size,
            dl_active_err_cor_signaling_supported,
        }
    }
}

/// DPC Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpcControl {
    /// DPC Trigger Enable
    pub dpc_trigger_enable: DpcTrigger,
    /// DPC Completion Control
    pub dpc_completion_control: bool,
    /// DPC Interrupt Enable
    pub dpc_interrupt_enable: bool,
    /// DPC ERR_COR Enable
    pub dpc_err_cor_enable: bool,
    /// Poisoned TLP Egress Blocking Enable
    pub poisoned_tlp_egress_blocking_enable: bool,
    /// DPC Software Trigger
    pub dpc_software_trigger: bool,
    /// DL_Active ERR_COR Enable
    pub dl_active_err_cor_enable: bool,
}

impl From<u16> for DpcControl {
    fn from(word: u16) -> Self {
        let Lsb((
            dpc_trigger_enable,
            dpc_completion_control,
            dpc_interrupt_enable,
            dpc_err_cor_enable,
            poisoned_tlp_egress_blocking_enable,
            dpc_software_trigger,
            dl_active_err_cor_enable,
            (),
        )) = P8::<_, 2, 1, 1, 1, 1, 1, 1, 8>(word).into();
        Self {
            dpc_trigger_enable: From::<u8>::from(dpc_trigger_enable),
            dpc_completion_control,
            dpc_interrupt_enable,
            dpc_err_cor_enable,
            poisoned_tlp_egress_blocking_enable,
            dpc_software_trigger,
            dl_active_err_cor_enable,
        }
    }
}

/// Enables DPC and controls the conditions that cause DPC to be triggered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpcTrigger {
    Disabled,
    /// Enabled and is triggered when the Downstream Port detects an unmasked uncorrectable error
    /// or when the Downstream Port receives an ERR_FATAL Message
    ErrFatalMessage,
    /// Enabled and is triggered when the Downstream Port detects an unmasked uncorrectable error
    /// or when the Downstream Port receives an ERR_NONFATAL or ERR_FATAL Message
    ErrNonFatalMessage,
    Reserved,
}
impl From<u8> for DpcTrigger {
    fn from(data: u8) -> Self {
        match data {
            0b00 => Self::Disabled,
            0b01 => Self::ErrFatalMessage,
            0b10 => Self::ErrNonFatalMessage,
            _ => Self::Reserved,
        }
    }
}

/// DPC Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpcStatus {
    /// DPC Trigger Status
    pub dpc_trigger_status: bool,
    /// DPC Trigger Reason
    pub dpc_trigger_reason: DpcTriggerReason,
    /// DPC Interrupt Status
    pub dpc_interrupt_status: bool,
    /// DPC RP Busy
    pub dpc_rp_busy: bool,
    /// RP PIO First Error Pointer
    pub rp_pio_first_error_pointer: u8,
}

impl From<u16> for DpcStatus {
    fn from(word: u16) -> Self {
        let Lsb((
            dpc_trigger_status,
            dpc_trigger_reason,
            dpc_interrupt_status,
            dpc_rp_busy,
            dpc_trigger_reason_extension,
            (),
            rp_pio_first_error_pointer,
            (),
        )) = P8::<_, 1, 2, 1, 1, 2, 1, 5, 3>(word).into();
        Self {
            dpc_trigger_status,
            dpc_trigger_reason: DpcTriggerReason::new(
                dpc_trigger_reason,
                dpc_trigger_reason_extension,
            ),
            dpc_interrupt_status,
            dpc_rp_busy,
            rp_pio_first_error_pointer,
        }
    }
}

/// Indicates why DPC has been triggered
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DpcTriggerReason {
    /// unmasked uncorrectable error
    UnmaskedUncorrectableError,
    /// receiving an ERR_NONFATAL
    ReceivingAnErrNonFatal,
    /// receiving an ERR_FATAL
    ReceivingAnErrFatal,
    /// RP PIO error
    RpPioError,
    /// DPC Software Trigger bit
    DpcSoftwareTriggerBit,
    Reserved(u8),
}

impl DpcTriggerReason {
    pub fn new(reason: u8, reason_extension: u8) -> Self {
        match (reason, reason_extension) {
            (0b00, _) => Self::UnmaskedUncorrectableError,
            (0b01, _) => Self::ReceivingAnErrNonFatal,
            (0b10, _) => Self::ReceivingAnErrFatal,
            (_, 0b00) => Self::RpPioError,
            (_, 0b01) => Self::DpcSoftwareTriggerBit,
            (_, v) => Self::Reserved(v),
        }
    }
    pub fn value(&self) -> u8 {
        match self {
            Self::UnmaskedUncorrectableError => 0b00,
            Self::ReceivingAnErrNonFatal => 0b01,
            Self::ReceivingAnErrFatal => 0b10,
            _ => 0b11,
        }
    }
    pub fn extension_value(&self) -> u8 {
        match self {
            Self::RpPioError => 0b00,
            Self::DpcSoftwareTriggerBit => 0b01,
            Self::Reserved(v) => *v,
            _ => 0b00,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpPio {
    /// Configuration Request received UR Completion
    pub cfg_ur_cpl: bool,
    /// Configuration Request received CA Completion
    pub cfg_ca_cpl: bool,
    /// Configuration Request Completion Timeout
    pub cfg_cto: bool,
    /// I/O Request received UR Completion
    pub io_ur_cpl: bool,
    /// I/O Request received CA Completion
    pub io_ca_cpl: bool,
    /// I/O Request Completion Timeout
    pub io_cto: bool,
    /// Memory Request received UR Completion
    pub mem_ur_cpl: bool,
    /// Memory Request received CA Completion
    pub mem_ca_cpl: bool,
    /// Memory Request Completion Timeout
    pub mem_cto: bool,
}

impl From<u32> for RpPio {
    fn from(dword: u32) -> Self {
        let Lsb((
            cfg_ur_cpl,
            cfg_ca_cpl,
            cfg_cto,
            (),
            io_ur_cpl,
            io_ca_cpl,
            io_cto,
            (),
            mem_ur_cpl,
            mem_ca_cpl,
            mem_cto,
            (),
        )) = P12::<_, 1, 1, 1, 5, 1, 1, 1, 5, 1, 1, 1, 13>(dword).into();
        Self {
            cfg_ur_cpl,
            cfg_ca_cpl,
            cfg_cto,
            io_ur_cpl,
            io_ca_cpl,
            io_cto,
            mem_ur_cpl,
            mem_ca_cpl,
            mem_cto,
        }
    }
}
