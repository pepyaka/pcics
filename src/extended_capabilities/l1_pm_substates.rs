/*!
# L1 PM Substates

The PCI Express L1 PM Substates Capability is an optional Extended Capability, that is required
if L1 PM Substates is implemented at a Port. For a Multi-Function Device associated with an
Upstream Port implementing L1 PM Substates, this Extended Capability Structure must be
implemented only in Function 0, and must control the Upstream Port’s Link behavior on behalf of
all the Functions of the device.

## Struct diagram
[L1PmSubstates]
- [L1PmSubstatesCapabilities]
  - [PortTPowerOn]
- [L1PmSubstatesControl1]
  - [MaxLatency]
- [L1PmSubstatesControl2]
  - [PortTPowerOn]

## Examples
> L1 PM Substates  
    L1SubCap: PCI-PM_L1.2+ PCI-PM_L1.1+ ASPM_L1.2+ ASPM_L1.1+ L1_PM_Substates+  
              PortCommonModeRestoreTime=40us PortTPowerOnTime=44us  
    L1SubCtl1: PCI-PM_L1.2+ PCI-PM_L1.1+ ASPM_L1.2+ ASPM_L1.1+  
               T_CommonMode=255us LTR1.2_Threshold=81920ns  
    L1SubCtl2: T_PwrOn=44us  
  
```rust
# use pcics::extended_capabilities::l1_pm_substates::*;
let data = [
    0x1e, 0x00, 0x01, 0x22,
    0x1f, 0x28, 0xb0, 0x00,
    0x0f, 0xff, 0x50, 0x40,
    0xb0, 0x00, 0x00, 0x00,
];
let result = data[4..].try_into().unwrap();
let sample = L1PmSubstates {
    l1_pm_substates_capabilities: L1PmSubstatesCapabilities {
        pci_pm_l1_2_supported: true,
        pci_pm_l1_1_supported: true,
        aspm_l1_2_supported: true,
        aspm_l1_1_supported: true,
        l1_pm_substates_supported: true,
        port_common_mode_restore_time: 40,
        port_t_power_on: PortTPowerOn { value: 22, scale: PortTPowerOnScale::Time2us },
    },
    l1_pm_substates_control_1: L1PmSubstatesControl1 {
        pci_pm_l1_2_enable: true,
        pci_pm_l1_1_enable: true,
        aspm_l1_2_enable: true,
        aspm_l1_1_enable: true,
        common_mode_restore_time: 255,
        ltr_l1_2_threshold: MaxLatency { value: 80, scale: 2 },
    },
    l1_pm_substates_control_2: L1PmSubstatesControl2 {
        t_power_on: PortTPowerOn { value: 22, scale: PortTPowerOnScale::Time2us },
    },
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P11, P3, P4, P9};

use super::ExtendedCapabilityDataError;

pub use super::latency_tolerance_reporting::MaxLatency;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1PmSubstates {
    /// L1 PM Substates Capabilities
    pub l1_pm_substates_capabilities: L1PmSubstatesCapabilities,
    /// L1 PM Substates Control 1
    pub l1_pm_substates_control_1: L1PmSubstatesControl1,
    /// L1 PM Substates Control 2
    pub l1_pm_substates_control_2: L1PmSubstatesControl2,
}
impl TryFrom<&[u8]> for L1PmSubstates {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head:
                Le((l1_pm_substates_capabilities, l1_pm_substates_control_1, l1_pm_substates_control_2)),
            ..
        } = P3(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "L1 PM Substates",
                size: 12,
            })?;
        let Lsb((
            pci_pm_l1_2_supported,
            pci_pm_l1_1_supported,
            aspm_l1_2_supported,
            aspm_l1_1_supported,
            l1_pm_substates_supported,
            (),
            port_common_mode_restore_time,
            port_t_power_on_scale,
            (),
            port_t_power_on_value,
            (),
        )) = P11::<u32, 1, 1, 1, 1, 1, 3, 8, 2, 1, 5, 8>(l1_pm_substates_capabilities).into();
        let Lsb((
            pci_pm_l1_2_enable,
            pci_pm_l1_1_enable,
            aspm_l1_2_enable,
            aspm_l1_1_enable,
            (),
            common_mode_restore_time,
            ltr_l1_2_threshold_value,
            (),
            ltr_l1_2_threshold_scale,
        )) = P9::<u32, 1, 1, 1, 1, 4, 8, 10, 3, 3>(l1_pm_substates_control_1).into();
        let Lsb((t_power_on_scale, (), t_power_on_value, ())) =
            P4::<u32, 2, 1, 5, 24>(l1_pm_substates_control_2).into();
        Ok(Self {
            l1_pm_substates_capabilities: L1PmSubstatesCapabilities {
                pci_pm_l1_2_supported,
                pci_pm_l1_1_supported,
                aspm_l1_2_supported,
                aspm_l1_1_supported,
                l1_pm_substates_supported,
                port_common_mode_restore_time,
                port_t_power_on: PortTPowerOn {
                    value: port_t_power_on_value,
                    scale: From::<u8>::from(port_t_power_on_scale),
                },
            },
            l1_pm_substates_control_1: L1PmSubstatesControl1 {
                pci_pm_l1_2_enable,
                pci_pm_l1_1_enable,
                aspm_l1_2_enable,
                aspm_l1_1_enable,
                common_mode_restore_time,
                ltr_l1_2_threshold: MaxLatency {
                    value: ltr_l1_2_threshold_value,
                    scale: ltr_l1_2_threshold_scale,
                },
            },
            l1_pm_substates_control_2: L1PmSubstatesControl2 {
                t_power_on: PortTPowerOn {
                    value: t_power_on_value,
                    scale: From::<u8>::from(t_power_on_scale),
                },
            },
        })
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
impl From<L1PmSubstatesCapabilities> for u32 {
    fn from(data: L1PmSubstatesCapabilities) -> Self {
        let b0 = u8::from(data.pci_pm_l1_2_supported)
            | u8::from(data.pci_pm_l1_1_supported) << 1
            | u8::from(data.aspm_l1_2_supported) << 2
            | u8::from(data.aspm_l1_1_supported) << 3
            | u8::from(data.l1_pm_substates_supported) << 4;
        let b2 = u8::from(data.port_t_power_on);
        u32::from_le_bytes([b0, data.port_common_mode_restore_time, b2, 0x00])
    }
}

/// Sets the time (in μs) that this Port requires the port on the opposite side
/// of Link to wait in L1.2
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
impl From<PortTPowerOn> for u8 {
    fn from(data: PortTPowerOn) -> Self {
        (data.value << 3) | u8::from(data.scale)
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
impl From<L1PmSubstatesControl1> for u32 {
    fn from(data: L1PmSubstatesControl1) -> Self {
        let b0 = u8::from(data.pci_pm_l1_2_enable)
            | u8::from(data.pci_pm_l1_1_enable) << 1
            | u8::from(data.aspm_l1_2_enable) << 2
            | u8::from(data.aspm_l1_1_enable) << 3;
        let [b2, b3] = u16::from(data.ltr_l1_2_threshold).to_le_bytes();
        u32::from_le_bytes([b0, data.common_mode_restore_time, b2, b3])
    }
}

/// L1 PM Substates Control 2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1PmSubstatesControl2 {
    /// T_POWER_ON
    pub t_power_on: PortTPowerOn,
}
impl From<L1PmSubstatesControl2> for u32 {
    fn from(data: L1PmSubstatesControl2) -> Self {
        u32::from_le_bytes([data.t_power_on.into(), 0x00, 0x00, 0x00])
    }
}

#[cfg(test)]
mod tests {
    use crate::extended_capabilities::ECH_BYTES;
    use pretty_assertions::assert_eq;

    use super::*;

    // Capabilities: [258 v1] L1 PM Substates
    //         L1SubCap: PCI-PM_L1.2+ PCI-PM_L1.1+ ASPM_L1.2+ ASPM_L1.1+ L1_PM_Substates+
    //                   PortCommonModeRestoreTime=255us PortTPowerOnTime=10us
    //         L1SubCtl1: PCI-PM_L1.2+ PCI-PM_L1.1+ ASPM_L1.2- ASPM_L1.1-
    //                    T_CommonMode=0us LTR1.2_Threshold=51200ns
    //         L1SubCtl2: T_PwrOn=44us
    const DATA: [u8; 16] = [
        0x1e, 0x00, 0x81, 0x12, 0x1f, 0xff, 0x28, 0x00, 0x03, 0x00, 0x32, 0x40, 0xb0, 0x00, 0x00,
        0x00,
    ];
    const SAMPLE: L1PmSubstates = L1PmSubstates {
        l1_pm_substates_capabilities: L1PmSubstatesCapabilities {
            pci_pm_l1_2_supported: true,
            pci_pm_l1_1_supported: true,
            aspm_l1_2_supported: true,
            aspm_l1_1_supported: true,
            l1_pm_substates_supported: true,
            port_common_mode_restore_time: 255,
            port_t_power_on: PortTPowerOn {
                value: 5,
                scale: PortTPowerOnScale::Time2us,
            },
        },
        l1_pm_substates_control_1: L1PmSubstatesControl1 {
            pci_pm_l1_2_enable: true,
            pci_pm_l1_1_enable: true,
            aspm_l1_2_enable: false,
            aspm_l1_1_enable: false,
            common_mode_restore_time: 0,
            ltr_l1_2_threshold: MaxLatency {
                value: 50,
                scale: 2,
            },
        },
        l1_pm_substates_control_2: L1PmSubstatesControl2 {
            t_power_on: PortTPowerOn {
                value: 22,
                scale: PortTPowerOnScale::Time2us,
            },
        },
    };

    #[test]
    fn from_bytes_into_struct() {
        let result = DATA[ECH_BYTES..].try_into().unwrap();
        assert_eq!(SAMPLE, result);
    }

    #[test]
    fn from_capabilities_into_dword() {
        assert_eq!(
            u32::from_le_bytes([0x1f, 0xff, 0x28, 0x00]),
            SAMPLE.l1_pm_substates_capabilities.into(),
            "Capabilities"
        );
    }

    #[test]
    fn from_control_1_into_dword() {
        assert_eq!(
            u32::from_le_bytes([0x03, 0x00, 0x32, 0x40]),
            SAMPLE.l1_pm_substates_control_1.into(),
            "Control 1"
        );
    }

    #[test]
    fn from_control_2_into_dword() {
        assert_eq!(
            u32::from_le_bytes([0xb0, 0x00, 0x00, 0x00]),
            SAMPLE.l1_pm_substates_control_2.into(),
            "Control 2"
        );
    }
}
