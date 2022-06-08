use displaydoc::Display as DisplayDoc;
use heterob::{bit_numbering::Lsb, P13};

/// The Status register is used to record status information for PCI bus related events.
///
/// Devices may not need to implement all bits, depending on device functionality. Reserved bits
/// should be read-only and return zero when read.
/// There are three types of Status Register:
/// 1. Primary (identical for all device types)
/// 2. Secondary PCI-to-PCI Bridge
/// 3. Secondary CardBus
///
/// Status type selected by generic constant [char] 'P', 'B' or 'C'
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status<const T: char> {
    pub reserved: u8,
    pub interrupt_status: bool,
    pub capabilities_list: bool,
    pub is_66mhz_capable: bool,
    pub user_definable_features: bool,
    pub fast_back_to_back_capable: bool,
    pub master_data_parity_error: bool,
    pub devsel_timing: DevselTiming,
    pub signaled_target_abort: bool,
    pub received_target_abort: bool,
    pub received_master_abort: bool,
    /// Primary device status: Signaled System Error
    /// Secondary Bridge device status: Received System Error
    /// Secondary CardBus device status: bridge has detected SERR# asserted on the CardBus
    pub system_error: bool,
    pub detected_parity_error: bool,
}

impl<const T: char> From<u16> for Status<T> {
    fn from(word: u16) -> Self {
        let Lsb((
            reserved,
            interrupt_status,
            capabilities_list,
            is_66mhz_capable,
            user_definable_features,
            fast_back_to_back_capable,
            master_data_parity_error,
            devsel_timing,
            signaled_target_abort,
            received_target_abort,
            received_master_abort,
            system_error,
            detected_parity_error,
        )) = P13::<_, 3, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1>(word).into();
        Self {
            reserved,
            interrupt_status,
            capabilities_list,
            is_66mhz_capable,
            user_definable_features,
            fast_back_to_back_capable,
            master_data_parity_error,
            devsel_timing: From::<u8>::from(devsel_timing),
            signaled_target_abort,
            received_target_abort,
            received_master_abort,
            system_error,
            detected_parity_error,
        }
    }
}

#[derive(DisplayDoc, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevselTiming {
    /// fast
    Fast,
    /// medium
    Medium,
    /// slow
    Slow,
    /// undefined
    Undefined,
}
impl From<u8> for DevselTiming {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Fast,
            0b01 => Self::Medium,
            0b10 => Self::Slow,
            0b11 => Self::Undefined,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_word() {
        let result: Status<'P'> = 0xAAAA.into();
        let sample = Status {
            reserved: 0b010,
            interrupt_status: true,
            capabilities_list: false,
            is_66mhz_capable: true,
            user_definable_features: false,
            fast_back_to_back_capable: true,
            master_data_parity_error: false,
            devsel_timing: DevselTiming::Medium,
            signaled_target_abort: true,
            received_target_abort: false,
            received_master_abort: true,
            system_error: false,
            detected_parity_error: true,
        };
        assert_eq!(sample, result);
    }
}
