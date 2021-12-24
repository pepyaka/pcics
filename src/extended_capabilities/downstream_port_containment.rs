//! Downstream Port Containment (DPC)
//!
//! The automatic disabling of the Link below a Downstream Port following an uncorrectable error,
//! which prevents TLPs subsequent to the error from propagating Upstream or Downstream.

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


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
    pub rp_pio_header_log: [u8; 16],
    /// RP PIO ImpSpec Log
    pub rp_pio_impspec_log: u32,
    /// RP PIO TLP Prefix Log
    pub rp_pio_tlp_prefix_log: [u8; 16],
}
impl<'a> TryRead<'a, Endian> for DownstreamPortContainment {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let dpc = DownstreamPortContainment {
            dpc_capability: bytes.read_with::<u16>(offset, endian)?.into(),
            dpc_control: bytes.read_with::<u16>(offset, endian)?.into(),
            dpc_status: bytes.read_with::<u16>(offset, endian)?.into(),
            dpc_error_source_id: bytes.read_with::<u16>(offset, endian)?,
            rp_pio_status: bytes.read_with::<u32>(offset, endian)?.into(),
            rp_pio_mask: bytes.read_with::<u32>(offset, endian)?.into(),
            rp_pio_severity: bytes.read_with::<u32>(offset, endian)?.into(),
            rp_pio_syserr: bytes.read_with::<u32>(offset, endian)?.into(),
            rp_pio_exception: bytes.read_with::<u32>(offset, endian)?.into(),
            rp_pio_header_log: bytes.read_with::<&[u8]>(offset, Bytes::Len(16))?
                .try_into().unwrap(),
            rp_pio_impspec_log: bytes.read_with::<u32>(offset, endian)?,
            rp_pio_tlp_prefix_log: bytes.read_with::<&[u8]>(offset, Bytes::Len(16))?
                .try_into().unwrap(),
        };
        Ok((dpc, *offset))
    }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DpcCapabilityProto {
    dpc_interrupt_message_number: B5,
    rp_extensions_for_dpc: bool,
    poisoned_tlp_egress_blocking_supported: bool,
    dpc_software_triggering_supported: bool,
    rp_pio_log_size: B4,
    dl_active_err_cor_signaling_supported: bool,
    rsvdp: B3,
}
impl<'a> From<&'a DpcCapability> for DpcCapabilityProto {
    fn from(data: &'a DpcCapability) -> Self {
        Self::new()
            .with_dpc_interrupt_message_number(data.dpc_interrupt_message_number)
            .with_rp_extensions_for_dpc(data.rp_extensions_for_dpc)
            .with_poisoned_tlp_egress_blocking_supported(data.poisoned_tlp_egress_blocking_supported)
            .with_dpc_software_triggering_supported(data.dpc_software_triggering_supported)
            .with_rp_pio_log_size(data.rp_pio_log_size)
            .with_dl_active_err_cor_signaling_supported(data.dl_active_err_cor_signaling_supported)
            .with_rsvdp(0)
    }
}

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
impl From<DpcCapabilityProto> for DpcCapability {
    fn from(proto: DpcCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            dpc_interrupt_message_number: proto.dpc_interrupt_message_number(),
            rp_extensions_for_dpc: proto.rp_extensions_for_dpc(),
            poisoned_tlp_egress_blocking_supported: proto.poisoned_tlp_egress_blocking_supported(),
            dpc_software_triggering_supported: proto.dpc_software_triggering_supported(),
            rp_pio_log_size: proto.rp_pio_log_size(),
            dl_active_err_cor_signaling_supported: proto.dl_active_err_cor_signaling_supported(),
        }
    }
}
impl From<u16> for DpcCapability {
    fn from(word: u16) -> Self { DpcCapabilityProto::from(word).into() }
}
impl<'a> From<&'a DpcCapability> for u16 {
    fn from(data: &'a DpcCapability) -> Self { DpcCapabilityProto::from(data).into() }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DpcControlProto {
    dpc_trigger_enable: B2,
    dpc_completion_control: bool,
    dpc_interrupt_enable: bool,
    dpc_err_cor_enable: bool,
    poisoned_tlp_egress_blocking_enable: bool,
    dpc_software_trigger: bool,
    dl_active_err_cor_enable: bool,
    rsvdp: B8,
}
impl<'a> From<&'a DpcControl> for DpcControlProto {
    fn from(data: &'a DpcControl) -> Self {
        Self::new()
            .with_dpc_trigger_enable(data.dpc_trigger_enable as u8)
            .with_dpc_completion_control(data.dpc_completion_control)
            .with_dpc_interrupt_enable(data.dpc_interrupt_enable)
            .with_dpc_err_cor_enable(data.dpc_err_cor_enable)
            .with_poisoned_tlp_egress_blocking_enable(data.poisoned_tlp_egress_blocking_enable)
            .with_dpc_software_trigger(data.dpc_software_trigger)
            .with_dl_active_err_cor_enable(data.dl_active_err_cor_enable)
            .with_rsvdp(0)
    }
}

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
impl From<DpcControlProto> for DpcControl {
    fn from(proto: DpcControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            dpc_trigger_enable: proto.dpc_trigger_enable().into(),
            dpc_completion_control: proto.dpc_completion_control(),
            dpc_interrupt_enable: proto.dpc_interrupt_enable(),
            dpc_err_cor_enable: proto.dpc_err_cor_enable(),
            poisoned_tlp_egress_blocking_enable: proto.poisoned_tlp_egress_blocking_enable(),
            dpc_software_trigger: proto.dpc_software_trigger(),
            dl_active_err_cor_enable: proto.dl_active_err_cor_enable(),
        }
    }
}
impl From<u16> for DpcControl {
    fn from(word: u16) -> Self { DpcControlProto::from(word).into() }
}
impl<'a> From<&'a DpcControl> for u16 {
    fn from(data: &'a DpcControl) -> Self { DpcControlProto::from(data).into() }
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

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DpcStatusProto {
    dpc_trigger_status: bool,
    dpc_trigger_reason: B2,
    dpc_interrupt_status: bool,
    dpc_rp_busy: bool,
    dpc_trigger_reason_extension: B2,
    rsvdz: B1,
    rp_pio_first_error_pointer: B5,
    rsvdz_2: B3,
}
impl<'a> From<&'a DpcStatus> for DpcStatusProto {
    fn from(data: &'a DpcStatus) -> Self {
        Self::new()
            .with_dpc_trigger_status(data.dpc_trigger_status)
            .with_dpc_trigger_reason(data.dpc_trigger_reason.value())
            .with_dpc_interrupt_status(data.dpc_interrupt_status)
            .with_dpc_rp_busy(data.dpc_rp_busy)
            .with_dpc_trigger_reason_extension(data.dpc_trigger_reason.extension_value())
            .with_rsvdz(0)
            .with_rp_pio_first_error_pointer(data.rp_pio_first_error_pointer)
            .with_rsvdz_2(0)
    }
}

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
impl From<DpcStatusProto> for DpcStatus {
    fn from(proto: DpcStatusProto) -> Self {
        let _ = proto.rsvdz();
        let _ = proto.rsvdz_2();
        Self {
            dpc_trigger_status: proto.dpc_trigger_status(),
            dpc_trigger_reason: DpcTriggerReason::new(
                proto.dpc_trigger_reason(),
                proto.dpc_trigger_reason_extension(),
            ),
            dpc_interrupt_status: proto.dpc_interrupt_status(),
            dpc_rp_busy: proto.dpc_rp_busy(),
            rp_pio_first_error_pointer: proto.rp_pio_first_error_pointer(),
        }
    }
}
impl From<u16> for DpcStatus {
    fn from(word: u16) -> Self { DpcStatusProto::from(word).into() }
}
impl<'a> From<&'a DpcStatus> for u16 {
    fn from(data: &'a DpcStatus) -> Self { DpcStatusProto::from(data).into() }
}

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
            (_, v) => Self::Reserved(v)
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

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct RpPioProto {
    cfg_ur_cpl: bool,
    cfg_ca_cpl: bool,
    cfg_cto: bool,
    rsvdp: B5,
    io_ur_cpl: bool,
    io_ca_cpl: bool,
    io_cto: bool,
    rsvdp_2: B5,
    mem_ur_cpl: bool,
    mem_ca_cpl: bool,
    mem_cto: bool,
    rsvdp_3: B13,
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
impl From<RpPioProto> for RpPio {
    fn from(proto: RpPioProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        let _ = proto.rsvdp_3();
        Self {
            cfg_ur_cpl: proto.cfg_ur_cpl(),
            cfg_ca_cpl: proto.cfg_ca_cpl(),
            cfg_cto: proto.cfg_cto(),
            io_ur_cpl: proto.io_ur_cpl(),
            io_ca_cpl: proto.io_ca_cpl(),
            io_cto: proto.io_cto(),
            mem_ur_cpl: proto.mem_ur_cpl(),
            mem_ca_cpl: proto.mem_ca_cpl(),
            mem_cto: proto.mem_cto(),
        }
    }
}
impl From<u32> for RpPio {
    fn from(dword: u32) -> Self { RpPioProto::from(dword).into() }
}
