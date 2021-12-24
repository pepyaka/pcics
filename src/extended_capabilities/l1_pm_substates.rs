//! L1 PM Substates
//!
//! The PCI Express L1 PM Substates Capability is an optional Extended Capability, that is required
//! if L1 PM Substates is implemented at a Port. For a Multi-Function Device associated with an
//! Upstream Port implementing L1 PM Substates, this Extended Capability Structure must be
//! implemented only in Function 0, and must control the Upstream Port’s Link behavior on behalf of
//! all the Functions of the device.



use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

use super::latency_tolerance_reporting::MaxLatency;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1PmSubstates {
    /// L1 PM Substates Capabilities
    pub l1_pm_substates_capabilities: L1PmSubstatesCapabilities,
    /// L1 PM Substates Control 1
    pub l1_pm_substates_control_1: L1PmSubstatesControl1,
    /// L1 PM Substates Control 2
    pub l1_pm_substates_control_2: L1PmSubstatesControl2,
}
impl<'a> TryRead<'a, Endian> for L1PmSubstates {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let l1pms = L1PmSubstates {
            l1_pm_substates_capabilities: bytes.read_with::<u32>(offset, endian)?.into(),
            l1_pm_substates_control_1: bytes.read_with::<u32>(offset, endian)?.into(),
            l1_pm_substates_control_2: bytes.read_with::<u32>(offset, endian)?.into(),
        };
        Ok((l1pms, *offset))
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct L1PmSubstatesCapabilitiesProto {
    pci_pm_l1_2_supported: bool,
    pci_pm_l1_1_supported: bool,
    aspm_l1_2_supported: bool,
    aspm_l1_1_supported: bool,
    l1_pm_substates_supported: bool,
    rsvdp: B3,
    port_common_mode_restore_time: u8,
    port_t_power_on_scale: B2,
    rsvdp_2: B1,
    port_t_power_on_value: B5,
    rsvdp_3: B8,
}
impl From<L1PmSubstatesCapabilities> for L1PmSubstatesCapabilitiesProto {
    fn from(data: L1PmSubstatesCapabilities) -> Self {
        Self::new()
            .with_pci_pm_l1_2_supported(data.pci_pm_l1_2_supported)
            .with_pci_pm_l1_1_supported(data.pci_pm_l1_1_supported)
            .with_aspm_l1_2_supported(data.aspm_l1_2_supported)
            .with_aspm_l1_1_supported(data.aspm_l1_1_supported)
            .with_l1_pm_substates_supported(data.l1_pm_substates_supported)
            .with_rsvdp(0)
            .with_port_common_mode_restore_time(data.port_common_mode_restore_time)
            .with_port_t_power_on_scale(data.port_t_power_on.scale.into())
            .with_rsvdp_2(0)
            .with_port_t_power_on_value(data.port_t_power_on.value)
            .with_rsvdp_3(0)
    }
}

/// L1 PM Substates Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1PmSubstatesCapabilities {
    /// PCI-PM L1.2 Supported
    pub pci_pm_l1_2_supported: bool,
    /// PCI-PM L1.1 Supported
    pub pci_pm_l1_1_supported: bool,
    /// ASPM L1.2 Supported
    pub aspm_l1_2_supported: bool,
    /// ASPM L1.1 Supported
    pub aspm_l1_1_supported: bool,
    /// L1 PM Substates Supported
    pub l1_pm_substates_supported: bool,
    /// Port Common_Mode_Restore_Time
    pub port_common_mode_restore_time: u8,
    pub port_t_power_on: PortTPowerOn,
}
impl From<L1PmSubstatesCapabilitiesProto> for L1PmSubstatesCapabilities {
    fn from(proto: L1PmSubstatesCapabilitiesProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        let _ = proto.rsvdp_3();
        Self {
            pci_pm_l1_2_supported: proto.pci_pm_l1_2_supported(),
            pci_pm_l1_1_supported: proto.pci_pm_l1_1_supported(),
            aspm_l1_2_supported: proto.aspm_l1_2_supported(),
            aspm_l1_1_supported: proto.aspm_l1_1_supported(),
            l1_pm_substates_supported: proto.l1_pm_substates_supported(),
            port_common_mode_restore_time: proto.port_common_mode_restore_time(),
            port_t_power_on: PortTPowerOn {
                scale: proto.port_t_power_on_scale().into(),
                value: proto.port_t_power_on_value(),
            },
        }
    }
}
impl From<u32> for L1PmSubstatesCapabilities {
    fn from(dword: u32) -> Self { L1PmSubstatesCapabilitiesProto::from(dword).into() }
}
impl From<L1PmSubstatesCapabilities> for u32 {
    fn from(data: L1PmSubstatesCapabilities) -> Self {
        L1PmSubstatesCapabilitiesProto::from(data).into()
    }
}


/// Sets the time (in μs) that this Port requires the port on the opposite side of Link to wait in
/// L1.2.Exit after sampling CLKREQ# asserted before actively driving the interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortTPowerOn {
    pub value: u8,
    pub scale: PortTPowerOnScale,
}
impl PortTPowerOn {
    pub fn value(&self) -> Option<usize> {
        match self.scale {
            PortTPowerOnScale::Reserved => None,
            _ => Some(self.value as usize * (self.scale.clone() as usize)),
        }
    }
}

/// Specifies the scale used for the Port T_POWER_ON Value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortTPowerOnScale {
    /// 2 µs
    Time2us = 2,
    /// 10 µs
    Time10us = 10,
    /// 100 µs
    Time100us = 100,
    /// Reserved
    Reserved = 0,
}
impl From<u8> for PortTPowerOnScale {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Time2us,
            0b01 => Self::Time10us,
            0b10 => Self::Time100us,
            _ => Self::Reserved,
        }
    }
}
impl From<PortTPowerOnScale> for u8 {
    fn from(data: PortTPowerOnScale) -> Self {
        match data {
            PortTPowerOnScale::Time2us => 0,
            PortTPowerOnScale::Time10us => 1,
            PortTPowerOnScale::Time100us => 2,
            PortTPowerOnScale::Reserved => 3,
        }
    }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct L1PmSubstatesControl1Proto {
    pci_pm_l1_2_enable: bool,
    pci_pm_l1_1_enable: bool,
    aspm_l1_2_enable: bool,
    aspm_l1_1_enable: bool,
    rsvdp: B4,
    common_mode_restore_time: u8,
    ltr_l1_2_threshold_value: B10,
    rsvdp_2: B3,
    ltr_l1_2_threshold_scale: B3,
}
impl From<L1PmSubstatesControl1> for L1PmSubstatesControl1Proto {
    fn from(data: L1PmSubstatesControl1) -> Self {
        Self::new()
            .with_pci_pm_l1_2_enable(data.pci_pm_l1_2_enable)
            .with_pci_pm_l1_1_enable(data.pci_pm_l1_1_enable)
            .with_aspm_l1_2_enable(data.aspm_l1_2_enable)
            .with_aspm_l1_1_enable(data.aspm_l1_1_enable)
            .with_rsvdp(0)
            .with_common_mode_restore_time(data.common_mode_restore_time)
            .with_ltr_l1_2_threshold_value(data.ltr_l1_2_threshold.value)
            .with_rsvdp_2(0)
            .with_ltr_l1_2_threshold_scale(data.ltr_l1_2_threshold.scale)
    }
}

/// L1 PM Substates Control 1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1PmSubstatesControl1 {
    /// PCI-PM L1.2 Enable
    pub pci_pm_l1_2_enable: bool,
    /// PCI-PM L1.1 Enable
    pub pci_pm_l1_1_enable: bool,
    /// ASPM L1.2 Enable
    pub aspm_l1_2_enable: bool,
    /// ASPM L1.1 Enable
    pub aspm_l1_1_enable: bool,
    /// Value of T(COMMONMODE) (in µs), which must be used by the Downstream Port for timing the
    /// re-establishment of common mode
    pub common_mode_restore_time: u8,
    /// Indicates the LTR threshold used to determine if entry into L1 results in L1.1 (if enabled)
    /// or L1.2 (if enabled).
    pub ltr_l1_2_threshold: MaxLatency,
}
impl From<L1PmSubstatesControl1Proto> for L1PmSubstatesControl1 {
    fn from(proto: L1PmSubstatesControl1Proto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            pci_pm_l1_2_enable: proto.pci_pm_l1_2_enable(),
            pci_pm_l1_1_enable: proto.pci_pm_l1_1_enable(),
            aspm_l1_2_enable: proto.aspm_l1_2_enable(),
            aspm_l1_1_enable: proto.aspm_l1_1_enable(),
            common_mode_restore_time: proto.common_mode_restore_time(),
            ltr_l1_2_threshold: MaxLatency {
                value: proto.ltr_l1_2_threshold_value(),
                scale: proto.ltr_l1_2_threshold_scale(),
            }
        }
    }
}
impl From<u32> for L1PmSubstatesControl1 {
    fn from(dword: u32) -> Self { L1PmSubstatesControl1Proto::from(dword).into() }
}
impl From<L1PmSubstatesControl1> for u32 {
    fn from(data: L1PmSubstatesControl1) -> Self {
        L1PmSubstatesControl1Proto::from(data).into()
    }
}



#[bitfield(bits = 32)]
#[repr(u32)]
pub struct L1PmSubstatesControl2Proto {
    t_power_on_scale: B2,
    rsvdp: B1,
    t_power_on_value: B5,
    rsvdp_2: B24,
}
impl From<L1PmSubstatesControl2> for L1PmSubstatesControl2Proto {
    fn from(data: L1PmSubstatesControl2) -> Self {
        Self::new()
            .with_t_power_on_scale(data.t_power_on.scale.into())
            .with_rsvdp(0)
            .with_t_power_on_value(data.t_power_on.value)
            .with_rsvdp_2(0)
    }
}

/// L1 PM Substates Control 2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1PmSubstatesControl2 {
    /// T_POWER_ON
    pub t_power_on: PortTPowerOn
}
impl From<L1PmSubstatesControl2Proto> for L1PmSubstatesControl2 {
    fn from(proto: L1PmSubstatesControl2Proto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            t_power_on: PortTPowerOn {
                scale: proto.t_power_on_scale().into(),
                value: proto.t_power_on_value(),
            }
        }
    }
}
impl From<u32> for L1PmSubstatesControl2 {
    fn from(dword: u32) -> Self { L1PmSubstatesControl2Proto::from(dword).into() }
}
impl From<L1PmSubstatesControl2> for u32 {
    fn from(data: L1PmSubstatesControl2) -> Self {
        L1PmSubstatesControl2Proto::from(data).into()
    }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::extended_capabilities::ECH_BYTES;

    use super::*;

    // Capabilities: [258 v1] L1 PM Substates
    //         L1SubCap: PCI-PM_L1.2+ PCI-PM_L1.1+ ASPM_L1.2+ ASPM_L1.1+ L1_PM_Substates+
    //                   PortCommonModeRestoreTime=255us PortTPowerOnTime=10us
    //         L1SubCtl1: PCI-PM_L1.2+ PCI-PM_L1.1+ ASPM_L1.2- ASPM_L1.1-
    //                    T_CommonMode=0us LTR1.2_Threshold=51200ns
    //         L1SubCtl2: T_PwrOn=44us
    const DATA: [u8; 16] = [
        0x1e,0x00,0x81,0x12,0x1f,0xff,0x28,0x00,0x03,0x00,0x32,0x40,0xb0,0x00,0x00,0x00
    ];
    const SAMPLE: L1PmSubstates = L1PmSubstates {
        l1_pm_substates_capabilities: L1PmSubstatesCapabilities {
            pci_pm_l1_2_supported: true,
            pci_pm_l1_1_supported: true,
            aspm_l1_2_supported: true,
            aspm_l1_1_supported: true,
            l1_pm_substates_supported: true,
            port_common_mode_restore_time: 255,
            port_t_power_on: PortTPowerOn { value: 5, scale: PortTPowerOnScale::Time2us },
        },
        l1_pm_substates_control_1: L1PmSubstatesControl1 {
            pci_pm_l1_2_enable: true,
            pci_pm_l1_1_enable: true,
            aspm_l1_2_enable: false,
            aspm_l1_1_enable: false,
            common_mode_restore_time: 0,
            ltr_l1_2_threshold: MaxLatency { value: 50, scale: 2 },
        },
        l1_pm_substates_control_2: L1PmSubstatesControl2 {
            t_power_on: PortTPowerOn { value: 22, scale: PortTPowerOnScale::Time2us },
        },
    };

    #[test]
    fn from_bytes_into_struct() {
        let result = DATA[ECH_BYTES..].read_with::<L1PmSubstates>(&mut 0, LE).unwrap();
        assert_eq!(SAMPLE, result);
    }

    #[test]
    fn from_capabilities_into_dword() {
        assert_eq!(
            u32::from_le_bytes([0x1f,0xff,0x28,0x00]),
            SAMPLE.l1_pm_substates_capabilities.into(),
            "Capabilities"
        );
    }

    #[test]
    fn from_control_1_into_dword() {
        assert_eq!(
            u32::from_le_bytes([0x03,0x00,0x32,0x40]),
            SAMPLE.l1_pm_substates_control_1.into(),
            "Control 1"
        );
    }

    #[test]
    fn from_control_2_into_dword() {
        assert_eq!(
            u32::from_le_bytes([0xb0,0x00,0x00,0x00]),
            SAMPLE.l1_pm_substates_control_2.into(),
            "Control 2"
        );
    }
}
