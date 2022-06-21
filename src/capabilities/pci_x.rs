/*!
# PCI-X

PCI-X, short for Peripheral Component Interconnect eXtended, is a computer bus
and expansion card standard that enhances the 32-bit PCI local bus for higher
bandwidth demanded mostly by servers and workstations.

## Struct diagram

PCI-X
<pre>
<a href="struct.PciX.html">PciX</a>
├─ <a href="struct.Command.html">Command</a>
│  ├─ <a href="struct.MaximumByteCount.html">MaximumByteCount</a>
│  └─ <a href="struct.MaximumOutstandingSplitTransactions.html">MaximumOutstandingSplitTransactions</a>
├─ <a href="struct.Status.html">Status</a>
│  ├─ <a href="enum.DeviceComplexity.html">DeviceComplexity</a>
│  ├─ <a href="struct.MaximumByteCount.html">MaximumByteCount</a>
│  ├─ <a href="struct.MaximumOutstandingSplitTransactions.html">MaximumOutstandingSplitTransactions</a>
│  └─ <a href="struct.MaximumCumulativeReadSize.html">MaximumCumulativeReadSize</a>
└─ <a href="enum.Ecc.html">Ecc</a>
   └─ <a href="struct.EccControlAndStatus.html">EccControlAndStatus</a>
      ├─ <a href="enum.EccErrorPhase.html">EccErrorPhase</a>
      └─ <a href="struct.Syndrome.html">Syndrome</a>
</pre>

PCI-X Bridge
<pre>
<a href="struct.PciXBridge.html">PciXBridge</a>
├─ <a href="struct.SecondaryStatus.html">SecondaryStatus</a>
│  ├─ <a href="enum.SecondaryBusMode.html">SecondaryBusMode</a>
│  ├─ <a href="enum.ErrorProtection.html">ErrorProtection</a>
│  └─ <a href="enum.SecondaryBusFrequency.html">SecondaryBusFrequency</a>
├─ <a href="struct.BridgeStatus.html">BridgeStatus</a>
├─ <a href="struct.SplitTransactionControl.html">2 x SplitTransactionControl</a>
└─ <a href="enum.Ecc.html">Ecc</a>
   └─ <a href="struct.EccControlAndStatus.html">EccControlAndStatus</a>
      ├─ <a href="enum.EccErrorPhase.html">EccErrorPhase</a>
      └─ <a href="struct.Syndrome.html">Syndrome</a>
</pre>

## Examples

PCI-X Simple
```rust
# use pcics::capabilities::pci_x::*;
let data = [
    0x07, 0x00, // Header
    0x24, 0b0010_0000, // Command
    0x21, 0x04, 0xAA, 0xAA, // Status
    0xAA, 0xAA, 0xAA, 0xAA, // ECC Control and Status
    0x01, 0x00, 0x00, 0x00, // ECC First Address
    0x02, 0x00, 0x00, 0x00, // ECC Second Address
    0x03, 0x00, 0x00, 0x00, // ECC Attribute
];
let result = data[2..].try_into().unwrap();
let sample = PciX {
    command: Command {
        uncorrectable_data_error_recovery_enable: false,
        enable_relaxed_ordering: false,
        maximum_memory_read_byte_count: MaximumByteCount(1),
        maximum_outstanding_split_transactions: MaximumOutstandingSplitTransactions(2),
    },
    status: Status {
        function_number: 1,
        device_number: 4,
        bus_number: 4,
        device_64_bit: false,
        pci_x_133_capable: true,
        slit_completion_discarded: false,
        unexpected_split_completion: true,
        device_complexity: DeviceComplexity::Simple,
        designed_maximum_memory_read_byte_count: MaximumByteCount(1),
        designed_maximum_outstanding_split_transactions:
            MaximumOutstandingSplitTransactions(5),
        designed_maximum_cumulative_read_size: MaximumCumulativeReadSize(2),
        received_split_completion_error_message: true,
        pci_x_266_capable: false,
        pci_x_533_capable: true,
    },
    ecc: Ecc::Mode1OrMode2 {
        control_and_status: EccControlAndStatus {
            select_secondary_ecc_registers: false,
            error_present_in_other_ecc_register_bank: true,
            additional_correctable_ecc_error: false,
            additional_uncorrectable_ecc_error: true,
            ecc_error_phase: EccErrorPhase::Second32bits,
            ecc_error_corrected: true,
            syndrome: Syndrome {
                e0: false,
                e1: true,
                e2: false,
                e3: true,
                e4: false,
                e5: true,
                e6: false,
                e7: true,
            },
            error_first_command: 0xA,
            error_second_command: 0xA,
            error_upper_attributes: 0xA,
            ecc_control_update_enable: false,
            disable_single_bit_error_correction: false,
            ecc_mode: true,
        },
        first_address: 0x01,
        second_address: 0x02,
        attribute: 0x03,
    },
};
assert_eq!(sample, result);
```

PCI-X Bridge
```rust
# use pcics::capabilities::pci_x::*;
let data = [
    0x07, 0x00, // Header
    0x55, 0x05, // Secondary Status
    0x55, 0x55, 0x55, 0x55, // Bridge Status
    0x00, 0x11, 0x22, 0x33, // Upstream Split Transaction
    0x44, 0x55, 0x66, 0x77, // Downstream Split Transaction
];
let result = data[2..].try_into().unwrap();
let sample = PciXBridge {
    secondary_status: SecondaryStatus {
        device_64_bit: true,
        pci_x_133_capable: false,
        slit_completion_discarded: true,
        unexpected_split_completion: false,
        split_completion_overrun: true,
        split_request_delayed: false,
        secondary_bus_mode: SecondaryBusMode::Mode1,
        error_protection: ErrorProtection::Ecc,
        secondary_bus_frequency: SecondaryBusFrequency::Freq66MHz,
        pci_x_266_capable: false,
        pci_x_533_capable: false,
    },
    bridge_status: BridgeStatus {
        function_number: 5,
        device_number: 10,
        bus_number: 0x55,
        device_64_bit: true,
        pci_x_133_capable: false,
        slit_completion_discarded: true,
        unexpected_split_completion: false,
        split_completion_overrun: true,
        split_request_delayed: false,
        device_id_messaging_capable: false,
        pci_x_266_capable: true,
        pci_x_533_capable: false,
    },
    upstream_split_transaction_control: SplitTransactionControl {
        split_transaction_capacity: 0x1100,
        split_transaction_commitment_limit: 0x3322,
    },
    downstream_split_transaction_control: SplitTransactionControl {
        split_transaction_capacity: 0x5544,
        split_transaction_commitment_limit: 0x7766,
    },
    ecc: Ecc::None,
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P11, P13, P14, P2, P4, P5, P6, P8};
use snafu::Snafu;

/// PCI-X Errors
#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum PciXError {
    #[snafu(display("command and status registers are unreadable"))]
    CommandAndStatus,
    #[snafu(display("Mode 2 ECC registres unreadable"))]
    EccMode2Only,
    #[snafu(display("Mode 1/2 ECC registres unreadable"))]
    EccMode1OrMode2,
}

/// Type 00h Configuration Space header PCI-X Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciX {
    pub command: Command,
    pub status: Status,
    pub ecc: Ecc,
}

impl TryFrom<&[u8]> for PciX {
    type Error = PciXError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((command, status)),
            tail: slice,
        } = P2(slice)
            .try_into()
            .map_err(|_| PciXError::CommandAndStatus)?;
        let Lsb(((), pci_x_capabilities_list_item_version)) = P2::<u16, 12, 2>(command).into();
        let ecc = match pci_x_capabilities_list_item_version {
            0b00u8 => Ecc::None,
            0b01 => {
                let Seq {
                    head: Le((control_and_status, first_address, second_address, attribute)),
                    ..
                } = P4(slice).try_into().map_err(|_| PciXError::EccMode2Only)?;
                Ecc::Mode2Only {
                    control_and_status: From::<u32>::from(control_and_status),
                    first_address,
                    second_address,
                    attribute,
                }
            }
            0b10 => {
                let Seq {
                    head: Le((control_and_status, first_address, second_address, attribute)),
                    ..
                } = P4(slice)
                    .try_into()
                    .map_err(|_| PciXError::EccMode1OrMode2)?;
                Ecc::Mode1OrMode2 {
                    control_and_status: From::<u32>::from(control_and_status),
                    first_address,
                    second_address,
                    attribute,
                }
            }
            0b11 => Ecc::Reserved,
            _ => unreachable!(),
        };
        Ok(Self {
            ecc,
            command: From::<u16>::from(command),
            status: From::<u32>::from(status),
        })
    }
}

/// Controls various modes and features of the PCI-X device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// Enable the device to attempt to recover from uncorrectable data errors
    pub uncorrectable_data_error_recovery_enable: bool,
    /// Permitted to set the Relaxed Ordering bit in the Requester Attributes
    /// of transactions it initiates that do not require strong write ordering
    pub enable_relaxed_ordering: bool,
    pub maximum_memory_read_byte_count: MaximumByteCount,
    pub maximum_outstanding_split_transactions: MaximumOutstandingSplitTransactions,
}

impl From<u16> for Command {
    fn from(word: u16) -> Self {
        let Lsb((
            uncorrectable_data_error_recovery_enable,
            enable_relaxed_ordering,
            maximum_memory_read_byte_count,
            maximum_outstanding_split_transactions,
            (),
        )) = P5::<u16, 1, 1, 2, 3, 9>(word).into();
        Self {
            uncorrectable_data_error_recovery_enable,
            enable_relaxed_ordering,
            maximum_memory_read_byte_count: MaximumByteCount(maximum_memory_read_byte_count),
            maximum_outstanding_split_transactions: MaximumOutstandingSplitTransactions(
                maximum_outstanding_split_transactions,
            ),
        }
    }
}

/// Maximum byte count the device uses (device-function is designed to use)
/// when initiating a Sequence with one of the burst memory read commands
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximumByteCount(pub u8);
impl MaximumByteCount {
    pub fn value(&self) -> usize {
        1 << (self.0 + 9)
    }
}

/// Indicates a number that is greater than or equal to the maximum number of
/// Split Transactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximumOutstandingSplitTransactions(pub u8);
impl MaximumOutstandingSplitTransactions {
    pub fn value(&self) -> u8 {
        match self.0 {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 8,
            5 => 12,
            6 => 16,
            7 => 32,
            _ => unreachable!(),
        }
    }
}

/// Capabilities and current operating mode of the device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    /// Function Number
    pub function_number: u8,
    /// Device Number
    pub device_number: u8,
    /// Bus Number
    pub bus_number: u8,
    /// 64-bit Device
    pub device_64_bit: bool,
    /// Device is capable of 133 MHz operation in PCI-X mode
    pub pci_x_133_capable: bool,
    /// Split Completion Discarded
    pub slit_completion_discarded: bool,
    /// Unexpected Split Completion with this device’s Requester ID is received
    pub unexpected_split_completion: bool,
    pub device_complexity: DeviceComplexity,
    /// Designed Maximum Memory Read Byte Count
    pub designed_maximum_memory_read_byte_count: MaximumByteCount,
    /// Designed Maximum Outstanding Split Transactions
    pub designed_maximum_outstanding_split_transactions: MaximumOutstandingSplitTransactions,
    /// Designed Maximum Cumulative Read Size
    pub designed_maximum_cumulative_read_size: MaximumCumulativeReadSize,
    /// Received Split Completion Error Message
    pub received_split_completion_error_message: bool,
    /// Indicates whether the device is capable of PCI-X 266 operation
    pub pci_x_266_capable: bool,
    /// Indicates whether the device is capable of PCI-X 533 operation
    pub pci_x_533_capable: bool,
}

impl From<u32> for Status {
    fn from(dword: u32) -> Self {
        let Lsb((
            function_number,
            device_number,
            bus_number,
            device_64_bit,
            pci_x_133_capable,
            slit_completion_discarded,
            unexpected_split_completion,
            device_complexity,
            designed_maximum_memory_read_byte_count,
            designed_maximum_outstanding_split_transactions,
            designed_maximum_cumulative_read_size,
            received_split_completion_error_message,
            pci_x_266_capable,
            pci_x_533_capable,
        )) = P14::<u32, 3, 5, 8, 1, 1, 1, 1, 1, 2, 3, 3, 1, 1, 1>(dword).into();
        Self {
            function_number,
            device_number,
            bus_number,
            device_64_bit,
            pci_x_133_capable,
            slit_completion_discarded,
            unexpected_split_completion,
            device_complexity: From::<bool>::from(device_complexity),
            designed_maximum_memory_read_byte_count: MaximumByteCount(
                designed_maximum_memory_read_byte_count,
            ),
            designed_maximum_outstanding_split_transactions: MaximumOutstandingSplitTransactions(
                designed_maximum_outstanding_split_transactions,
            ),
            designed_maximum_cumulative_read_size: MaximumCumulativeReadSize(
                designed_maximum_cumulative_read_size,
            ),
            received_split_completion_error_message,
            pci_x_266_capable,
            pci_x_533_capable,
        }
    }
}
/// Indicates whether this device is a simple device or a bridge device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceComplexity {
    Simple,
    Bridge,
}
impl From<bool> for DeviceComplexity {
    fn from(b: bool) -> Self {
        if b {
            Self::Bridge
        } else {
            Self::Simple
        }
    }
}

/// Indicates a number that is greater than or equal to the maximum cumulative
/// size of all burst memory read transactions the devicefunction is designed to
/// have outstanding at one time as a requester
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximumCumulativeReadSize(pub u8);
impl MaximumCumulativeReadSize {
    pub fn adqs(&self) -> usize {
        1 << (self.0 + 3)
    }
    pub fn bytes(&self) -> usize {
        1 << self.0
    }
}

/// Indicate the format of the PCI-X Capabilities List item, and whether the
/// device supports ECC in Mode 1
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ecc {
    None,
    Mode2Only {
        control_and_status: EccControlAndStatus,
        first_address: u32,
        second_address: u32,
        attribute: u32,
    },
    Mode1OrMode2 {
        control_and_status: EccControlAndStatus,
        first_address: u32,
        second_address: u32,
        attribute: u32,
    },
    Reserved,
}

/// Provides information about detected ECC errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EccControlAndStatus {
    /// Select Secondary ECC Registers
    pub select_secondary_ecc_registers: bool,
    /// Error Present in Other ECC Register Bank
    pub error_present_in_other_ecc_register_bank: bool,
    /// Additional Correctable ECC Error
    pub additional_correctable_ecc_error: bool,
    /// Additional Uncorrectable ECC Error
    pub additional_uncorrectable_ecc_error: bool,
    pub ecc_error_phase: EccErrorPhase,
    /// ECC Error Corrected
    pub ecc_error_corrected: bool,
    pub syndrome: Syndrome,
    /// Error First (or only) Command
    pub error_first_command: u8,
    /// Error Second Command
    pub error_second_command: u8,
    /// Error Upper Attributes
    pub error_upper_attributes: u8,
    /// ECC Control Update Enable
    pub ecc_control_update_enable: bool,
    /// Disable Single-Bit-Error Correction
    pub disable_single_bit_error_correction: bool,
    /// ECC Mode
    pub ecc_mode: bool,
}
impl From<u32> for EccControlAndStatus {
    fn from(dword: u32) -> Self {
        let Lsb((
            select_secondary_ecc_registers,
            error_present_in_other_ecc_register_bank,
            additional_correctable_ecc_error,
            additional_uncorrectable_ecc_error,
            ecc_error_phase,
            ecc_error_corrected,
            syndrome,
            error_first_command,
            error_second_command,
            error_upper_attributes,
            ecc_control_update_enable,
            (),
            disable_single_bit_error_correction,
            ecc_mode,
        )) = P14::<_, 1, 1, 1, 1, 3, 1, 8, 4, 4, 4, 1, 1, 1, 1>(dword).into();
        Self {
            select_secondary_ecc_registers,
            error_present_in_other_ecc_register_bank,
            additional_correctable_ecc_error,
            additional_uncorrectable_ecc_error,
            ecc_error_phase: From::<u8>::from(ecc_error_phase),
            ecc_error_corrected,
            syndrome: From::<u8>::from(syndrome),
            error_first_command,
            error_second_command,
            error_upper_attributes,
            ecc_control_update_enable,
            disable_single_bit_error_correction,
            ecc_mode,
        }
    }
}

/// ECC Error Phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EccErrorPhase {
    /// No error
    NoError,
    /// First 32 bits of address
    First32bits,
    /// Second 32 bits of address
    Second32bits,
    /// Attribute phase
    AttributePhase,
    /// 32- or 16-bit data phase
    Phase32or16bit,
    /// 64-bit data phase
    Phase64bit,
    /// Reserved
    Reserved,
}
impl From<u8> for EccErrorPhase {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::NoError,
            1 => Self::First32bits,
            2 => Self::Second32bits,
            3 => Self::AttributePhase,
            4 => Self::Phase32or16bit,
            5 => Self::Phase64bit,
            6 | 7 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

/// The syndrome indicates information about the bit or bits that are in error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syndrome {
    /// E0
    pub e0: bool,
    /// E1
    pub e1: bool,
    /// E2
    pub e2: bool,
    /// E3
    pub e3: bool,
    /// E4
    pub e4: bool,
    /// E5
    pub e5: bool,
    /// E6
    pub e6: bool,
    /// E7 for 64-bit data, 0b for 32-bit data, or E16/Chk for 16-bit data
    pub e7: bool,
}

impl From<u8> for Syndrome {
    fn from(byte: u8) -> Self {
        let Lsb((e0, e1, e2, e3, e4, e5, e6, e7)) = P8::<_, 1, 1, 1, 1, 1, 1, 1, 1>(byte).into();
        Self {
            e0,
            e1,
            e2,
            e3,
            e4,
            e5,
            e6,
            e7,
        }
    }
}

/// PCI-X Bridge Errors
#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum PciXBridgeError {
    #[snafu(display("statuses and split trx control registers are unreadable"))]
    Mandatory,
    #[snafu(display("Mode 2 ECC registres unreadable"))]
    BridgeEccMode2Only,
    #[snafu(display("Mode 1/2 ECC registres unreadable"))]
    BridgeEccMode1OrMode2,
}

/// Type 01h Configuration Space header (Bridge) PCI-X Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciXBridge {
    pub secondary_status: SecondaryStatus,
    pub bridge_status: BridgeStatus,
    pub upstream_split_transaction_control: SplitTransactionControl,
    pub downstream_split_transaction_control: SplitTransactionControl,
    pub ecc: Ecc,
}

impl TryFrom<&[u8]> for PciXBridge {
    type Error = PciXBridgeError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head:
                Le((
                    secondary_status,
                    bridge_status,
                    up_split_trx_capacity,
                    up_split_trx_commitment_limit,
                    down_split_trx_capacity,
                    down_split_trx_commitment_limit,
                )),
            tail: slice,
        } = P6(slice)
            .try_into()
            .map_err(|_| PciXBridgeError::Mandatory)?;
        let Lsb(((), pci_x_capabilities_list_item_version)) =
            P2::<u16, 12, 2>(secondary_status).into();
        let ecc = match pci_x_capabilities_list_item_version {
            0b00u8 => Ecc::None,
            0b01 => {
                let Seq {
                    head: Le((control_and_status, first_address, second_address, attribute)),
                    ..
                } = P4(slice)
                    .try_into()
                    .map_err(|_| PciXBridgeError::BridgeEccMode2Only)?;
                Ecc::Mode2Only {
                    control_and_status: From::<u32>::from(control_and_status),
                    first_address,
                    second_address,
                    attribute,
                }
            }
            0b10 => {
                let Seq {
                    head: Le((control_and_status, first_address, second_address, attribute)),
                    ..
                } = P4(slice)
                    .try_into()
                    .map_err(|_| PciXBridgeError::BridgeEccMode1OrMode2)?;
                Ecc::Mode1OrMode2 {
                    control_and_status: From::<u32>::from(control_and_status),
                    first_address,
                    second_address,
                    attribute,
                }
            }
            0b11 => Ecc::Reserved,
            _ => unreachable!(),
        };
        Ok(Self {
            secondary_status: From::<u16>::from(secondary_status),
            bridge_status: From::<u32>::from(bridge_status),
            upstream_split_transaction_control: SplitTransactionControl {
                split_transaction_capacity: up_split_trx_capacity,
                split_transaction_commitment_limit: up_split_trx_commitment_limit,
            },
            downstream_split_transaction_control: SplitTransactionControl {
                split_transaction_capacity: down_split_trx_capacity,
                split_transaction_commitment_limit: down_split_trx_commitment_limit,
            },
            ecc,
        })
    }
}

/// Reports status information about the secondary bus
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondaryStatus {
    /// 64-bit Device
    pub device_64_bit: bool,
    /// 133 MHz Capable
    pub pci_x_133_capable: bool,
    /// Split Completion Discarded
    pub slit_completion_discarded: bool,
    /// Unexpected Split Completion
    pub unexpected_split_completion: bool,
    /// Split Completion Overrun
    pub split_completion_overrun: bool,
    /// Split Request Delayed
    pub split_request_delayed: bool,
    pub secondary_bus_mode: SecondaryBusMode,
    pub error_protection: ErrorProtection,
    pub secondary_bus_frequency: SecondaryBusFrequency,
    /// PCI-X 266 Capable
    pub pci_x_266_capable: bool,
    /// PCI-X 533 Capable
    pub pci_x_533_capable: bool,
}

impl SecondaryStatus {
    pub fn secondary_bus_mode_and_frequency(&self) -> u8 {
        use ErrorProtection as Ep;
        use SecondaryBusFrequency as Sbf;
        use SecondaryBusMode as Sbm;
        match (
            self.secondary_bus_mode,
            self.error_protection,
            self.secondary_bus_frequency,
        ) {
            (Sbm::Conventional, Ep::Parity, Sbf::NotAvailable) => 0x0,
            (Sbm::Mode1, Ep::Parity, Sbf::Freq66MHz) => 0x1,
            (Sbm::Mode1, Ep::Parity, Sbf::Freq100MHz) => 0x2,
            (Sbm::Mode1, Ep::Parity, Sbf::Freq133MHz) => 0x3,
            (Sbm::Mode1, Ep::Ecc, Sbf::Reserved) => 0x4,
            (Sbm::Mode1, Ep::Ecc, Sbf::Freq66MHz) => 0x5,
            (Sbm::Mode1, Ep::Ecc, Sbf::Freq100MHz) => 0x6,
            (Sbm::Mode1, Ep::Ecc, Sbf::Freq133MHz) => 0x7,
            (Sbm::PciX266, Ep::Ecc, Sbf::Reserved) => 0x8,
            (Sbm::PciX266, Ep::Ecc, Sbf::Freq66MHz) => 0x9,
            (Sbm::PciX266, Ep::Ecc, Sbf::Freq100MHz) => 0xA,
            (Sbm::PciX266, Ep::Ecc, Sbf::Freq133MHz) => 0xB,
            (Sbm::PciX533, Ep::Ecc, Sbf::Reserved) => 0xC,
            (Sbm::PciX533, Ep::Ecc, Sbf::Freq66MHz) => 0xD,
            (Sbm::PciX533, Ep::Ecc, Sbf::Freq100MHz) => 0xE,
            (Sbm::PciX533, Ep::Ecc, Sbf::Freq133MHz) => 0xF,
            _ => unreachable!(),
        }
    }
}

impl From<u16> for SecondaryStatus {
    fn from(word: u16) -> Self {
        let Lsb((
            device_64_bit,
            pci_x_133_capable,
            slit_completion_discarded,
            unexpected_split_completion,
            split_completion_overrun,
            split_request_delayed,
            secondary_bus_mode_and_frequency,
            (),
            (),
            pci_x_266_capable,
            pci_x_533_capable,
        )) = P11::<u16, 1, 1, 1, 1, 1, 1, 4, 2, 2, 1, 1>(word).into();
        Self {
            device_64_bit,
            pci_x_133_capable,
            slit_completion_discarded,
            unexpected_split_completion,
            split_completion_overrun,
            split_request_delayed,
            secondary_bus_mode: From::<u8>::from(secondary_bus_mode_and_frequency),
            error_protection: From::<u8>::from(secondary_bus_mode_and_frequency),
            secondary_bus_frequency: From::<u8>::from(secondary_bus_mode_and_frequency),
            pci_x_266_capable,
            pci_x_533_capable,
        }
    }
}

/// Secondary Bus Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryBusMode {
    /// Conventional
    Conventional,
    /// PCI-X Mode 1
    Mode1,
    /// PCI-X 266 (Mode 2)
    PciX266,
    /// PCI-X 533 (Mode 2)
    PciX533,
}

impl From<u8> for SecondaryBusMode {
    fn from(byte: u8) -> Self {
        match byte {
            0x0 => Self::Conventional,
            0x1..=0x7 => Self::Mode1,
            0x8..=0xB => Self::PciX266,
            0xC..=0xF => Self::PciX533,
            _ => unreachable!(),
        }
    }
}

/// Error Protection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorProtection {
    /// Parity
    Parity,
    /// ECC
    Ecc,
}

impl From<u8> for ErrorProtection {
    fn from(byte: u8) -> Self {
        match byte {
            0x0..=0x3 => Self::Parity,
            0x4..=0xF => Self::Ecc,
            _ => unreachable!(),
        }
    }
}

/// Secondary Bus Max Clock Freq and Min Clock Period
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryBusFrequency {
    /// N/A
    NotAvailable,
    /// Max Clock Freq 66 MHz, Min Clock Period 15 ns
    Freq66MHz,
    /// Max Clock Freq 100 MHz, Min Clock Period 10 ns
    Freq100MHz,
    /// Max Clock Freq 133 MHz, Min Clock Period 7.5 ns
    Freq133MHz,
    /// Reserved
    Reserved,
}

impl From<u8> for SecondaryBusFrequency {
    fn from(byte: u8) -> Self {
        match byte {
            0x0 => Self::NotAvailable,
            0x1 | 0x5 | 0x9 | 0xD => Self::Freq66MHz,
            0x2 | 0x6 | 0xA | 0xE => Self::Freq100MHz,
            0x3 | 0x7 | 0xB | 0xF => Self::Freq133MHz,
            0x4 | 0x8 | 0xC => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

/// Identifies the capabilities and current operating mode of the bridge on its
/// primary bus as listed in the following table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStatus {
    /// Function Number
    pub function_number: u8,
    /// Device Number
    pub device_number: u8,
    /// Bus Number
    pub bus_number: u8,
    /// 64-bit Device
    pub device_64_bit: bool,
    /// Bridge’s primary interface is capable of 133 MHz operation in PCI-X mode
    pub pci_x_133_capable: bool,
    /// Split Completion Discarded
    pub slit_completion_discarded: bool,
    /// Unexpected Split Completion with this device’s Requester ID is received
    pub unexpected_split_completion: bool,
    /// Split Completion Overrun
    pub split_completion_overrun: bool,
    /// Split Request Delayed
    pub split_request_delayed: bool,
    /// Device ID Messaging Capable
    pub device_id_messaging_capable: bool,
    /// Indicates whether the bridge is capable of PCI-X 266 operation
    pub pci_x_266_capable: bool,
    /// Indicates whether the bridge is capable of PCI-X 533 operation
    pub pci_x_533_capable: bool,
}

impl From<u32> for BridgeStatus {
    fn from(dword: u32) -> Self {
        let Lsb((
            function_number,
            device_number,
            bus_number,
            device_64_bit,
            pci_x_133_capable,
            slit_completion_discarded,
            unexpected_split_completion,
            split_completion_overrun,
            split_request_delayed,
            (),
            device_id_messaging_capable,
            pci_x_266_capable,
            pci_x_533_capable,
        )) = P13::<u32, 3, 5, 8, 1, 1, 1, 1, 1, 1, 7, 1, 1, 1>(dword).into();
        Self {
            function_number,
            device_number,
            bus_number,
            device_64_bit,
            pci_x_133_capable,
            slit_completion_discarded,
            unexpected_split_completion,
            split_completion_overrun,
            split_request_delayed,
            device_id_messaging_capable,
            pci_x_266_capable,
            pci_x_533_capable,
        }
    }
}

/// Controls behavior of the bridge buffers for forwarding Split Transactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitTransactionControl {
    /// Split Transaction Capacity
    pub split_transaction_capacity: u16,
    /// Split Transaction Commitment Limit
    pub split_transaction_commitment_limit: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn secondary_bus_mode_and_frequency() {
        for word in 0..256 {
            let ss: SecondaryStatus = (word << 6).into();
            assert_eq!((word & 0b1111) as u8, ss.secondary_bus_mode_and_frequency());
        }
    }
}
