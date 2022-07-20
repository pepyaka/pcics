/*!
# Virtual Channel Capability

The Virtual Channel (VC) Capability is an optional Extended Capability required for devices
that have Ports (or for individual Functions) that support functionality beyond the default
Traffic Class (TC0) over the default Virtual Channel (VC0).

## Struct diagram
<pre>
<a href="struct.VirtualChannel.html">VirtualChannel</a>
├─ <a href="struct.PortVcCapability1.html">PortVcCapability1</a>
│  ├─ <a href="enum.ReferenceClock.html">ReferenceClock</a>
│  └─ <a href="struct.PortArbitrationTableEntrySize.html">PortArbitrationTableEntrySize</a>
├─ <a href="struct.PortVcCapability2.html">PortVcCapability2</a>
│  └─ <a href="struct.VcArbitrationCapability.html">VcArbitrationCapability</a>
├─ <a href="struct.PortVcControl.html">PortVcControl</a>
│  └─ <a href="enum.VcArbitrationSelect.html">VcArbitrationSelect</a>
└─ <a href="struct.PortVcStatus.html">PortVcStatus</a>
</pre>

## Examples
> ```text
> Caps:   LPEVC=0 RefClk=100ns PATEntryBits=1
> Arb:    Fixed- WRR32- WRR64- WRR128-
> Ctrl:   ArbSelect=Fixed
> Status: InProgress-
> VC0:    Caps:   PATOffset=00 MaxTimeSlots=1 RejSnoopTrans-
>         Arb:    Fixed- WRR32- WRR64- WRR128- TWRR128- WRR256-
>         Ctrl:   Enable+ ID=0 ArbSelect=Fixed TC/VC=ff
>         Status: NegoPending- InProgress-
> ```

```rust
# use pcics::extended_capabilities::virtual_channel::*;
let data = [
    0x02, 0x00, 0x01, 0x00, // Capability header
    0x00, 0x00, 0x00, 0x00, // Port VC Capability Register 1
    0x00, 0x00, 0x00, 0x00, // Port VC Capability Register 2
    0x00, 0x00,             // Port VC Control Register
    0x00, 0x00,             // Port VC Status Register
    0x00, 0x00, 0x00, 0x00, // VC Resource Capability Register (0)
    0xff, 0x00, 0x00, 0x80, // VC Resource Control Register (0)
    0x00, 0x00,             // RsvdP
    0x00, 0x00,             // VC Resource Status Register (0)
];

let result: VirtualChannel = data[4..].try_into().unwrap();

let mut sample_data = [0u8; 6 * 4];
sample_data[0x10] = 0xff;
sample_data[0x13] = 0x80;
let sample = {
    let mut vc: VirtualChannel = sample_data.as_slice().try_into().unwrap();
    vc.port_vc_capability_1 = PortVcCapability1 {
        extended_vc_count: 0,
        low_priority_extended_vc_count: 0,
        reference_clock: ReferenceClock::Rc100ns,
        port_arbitration_table_entry_size: 0.into(),
    };
    vc.port_vc_capability_2 = PortVcCapability2 {
        vc_arbitration_capability: VcArbitrationCapability {
            hardware_fixed_arbitration: false,
            wrr_32_phases: false,
            wrr_64_phases: false,
            wrr_128_phases: false,
            reserved: 0x0,
        },
        vc_arbitration_table_offset: 0,
    };
    vc.port_vc_control = PortVcControl {
        load_vc_arbitration_table: false,
        vc_arbitration_select: VcArbitrationSelect::HardwareFixedArbitration,
    };
    vc.port_vc_status = PortVcStatus {
        vc_arbitration_table_status: false,
    };
    vc
};

assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P13, P2, P3, P4, P7, P8};
use snafu::Snafu;

use super::ExtendedCapabilityDataError;

use core::slice;

use super::ECH_BYTES;

/// Numeral unit for VC Arbitration Table Offset and Port Arbitration Table Offset
const DQWORD: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualChannel<'a> {
    data: &'a [u8],
    /// Port VC Capability Register 1
    pub port_vc_capability_1: PortVcCapability1,
    /// Port VC Capability Register 2
    pub port_vc_capability_2: PortVcCapability2,
    /// Por VC Control Register
    pub port_vc_control: PortVcControl,
    /// Port VC Status Register
    pub port_vc_status: PortVcStatus,
}
impl<'a> VirtualChannel<'a> {
    pub fn extended_virtual_channels(&self) -> ExtendedVirtualChannels<'a> {
        let count = self.port_vc_capability_1.extended_vc_count;
        let start = 0x10 - ECH_BYTES;
        let data = &self.data[start..];
        ExtendedVirtualChannels::new(data, count)
    }
    pub fn vc_arbitration_table(&self) -> VcArbitrationTable<'a> {
        let offset = self.port_vc_capability_2.vc_arbitration_table_offset;
        let entries_number = self
            .port_vc_control
            .vc_arbitration_select
            .vc_arbitration_table_length();
        let start = offset as usize * DQWORD - ECH_BYTES;
        // VC Arbitration Table entry length is 4 bits, so there are 2 entries in one byte
        let end = start + entries_number / 2;
        let data = &self.data[start..end];
        VcArbitrationTable::new(data)
    }
    pub fn port_arbitration_table(
        &'a self,
        evc: &'a ExtendedVirtualChannel,
    ) -> PortArbitrationTable<'a> {
        let offset = evc.vc_resource_capability.port_arbitration_table_offset;
        let entry_size_bits = self
            .port_vc_capability_1
            .port_arbitration_table_entry_size
            .bits();
        let entries_number = evc
            .vc_resource_control
            .port_arbitration_select
            .port_arbitration_table_length();
        let start = offset as usize * DQWORD - ECH_BYTES;
        let end = start + entry_size_bits * entries_number / 8;
        let data = &self.data[start..end];
        PortArbitrationTable::new(data, entry_size_bits)
    }
}
impl<'a> TryFrom<&'a [u8]> for VirtualChannel<'a> {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((port_vc_capability_1, port_vc_capability_2, port_vc_control, port_vc_status)),
            ..
        } = P4(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Virtual Channel",
                size: 8,
            })?;
        Ok(Self {
            data: slice,
            port_vc_capability_1: From::<u32>::from(port_vc_capability_1),
            port_vc_capability_2: From::<u32>::from(port_vc_capability_2),
            port_vc_control: From::<u16>::from(port_vc_control),
            port_vc_status: From::<u16>::from(port_vc_status),
        })
    }
}

/// The Port VC Capability register 1 describes the configuration of the Virtual Channels
/// associated with a PCI Express Port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortVcCapability1 {
    /// Indicates the number of (extended) Virtual Channels in addition to the default VC supported
    /// by the device.
    pub extended_vc_count: u8,
    /// Indicates the number of (extended) Virtual Channels in addition to the default VC belonging
    /// to the low-priority VC (LPVC) group that has the lowest priority with respect to other VC
    /// resources in a strictpriority VC Arbitration.
    pub low_priority_extended_vc_count: u8,
    /// Reference Clock
    pub reference_clock: ReferenceClock,
    /// Indicates the size (in bits) of Port Arbitration table entry in the Function.
    pub port_arbitration_table_entry_size: PortArbitrationTableEntrySize,
}

impl From<u32> for PortVcCapability1 {
    fn from(dword: u32) -> Self {
        let Lsb((
            extended_vc_count,
            (),
            low_priority_extended_vc_count,
            (),
            reference_clock,
            port_arbitration_table_entry_size,
            (),
        )) = P7::<_, 3, 1, 3, 1, 2, 2, 20>(dword).into();
        Self {
            extended_vc_count,
            low_priority_extended_vc_count,
            reference_clock: From::<u8>::from(reference_clock),
            port_arbitration_table_entry_size: From::<u8>::from(port_arbitration_table_entry_size),
        }
    }
}

/// Indicates the reference clock for Virtual Channels that support time-based WRR Port
/// Arbitration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceClock {
    /// 100 ns reference clock
    Rc100ns,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for ReferenceClock {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Rc100ns,
            v => Self::Reserved(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortArbitrationTableEntrySize(u8);
impl PortArbitrationTableEntrySize {
    pub fn bits(&self) -> usize {
        1 << self.0
    }
}
impl From<u8> for PortArbitrationTableEntrySize {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

/// An iterator through 0 - 7 [Extended Virtual Channels](ExtendedVirtualChannel)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedVirtualChannels<'a> {
    data: &'a [u8],
    count: u8,
    offset: usize,
}
impl<'a> ExtendedVirtualChannels<'a> {
    pub fn new(data: &'a [u8], count: u8) -> Self {
        Self {
            data,
            // > 7.9.1 ...
            // > PCI Express device that supports only TC0 over VC0 does not require VC Extended
            // > Capability and associated registers.
            // Does it mean there is always one iteration?
            count: count + 1,
            offset: 0,
        }
    }
}
impl<'a> Iterator for ExtendedVirtualChannels<'a> {
    type Item = Result<ExtendedVirtualChannel, ExtendedVirtualChannelError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let slice = self
                .data
                .get(self.offset..self.offset + ExtendedVirtualChannel::SIZE)?;
            let result = slice
                .try_into()
                .map(|Seq { head, .. }| From::<[u8; ExtendedVirtualChannel::SIZE]>::from(head))
                .map_err(|_| ExtendedVirtualChannelError {
                    number: self.count,
                    offset: self.offset,
                });
            self.count -= 1;
            self.offset += ExtendedVirtualChannel::SIZE;
            Some(result)
        }
    }
}

/// Extended Virtual Channel Error
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub struct ExtendedVirtualChannelError {
    number: u8,
    offset: usize,
}

/// Virtual Channel resources
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedVirtualChannel {
    /// VC Resource Capability Register
    pub vc_resource_capability: VcResourceCapability,
    /// VC Resource Control Register
    pub vc_resource_control: VcResourceControl,
    /// VC Resource Status Register
    pub vc_resource_status: VcResourceStatus,
}
impl ExtendedVirtualChannel {
    /// Register size (12 bytes)
    pub const SIZE: usize = 4 + 4 + 2 + 2;
}
impl From<[u8; Self::SIZE]> for ExtendedVirtualChannel {
    fn from(data: [u8; Self::SIZE]) -> Self {
        let Le((vc_resource_capability, vc_resource_control, r, vc_resource_status)) =
            P4(data).into();
        let _: u16 = r;
        Self {
            vc_resource_capability: From::<u32>::from(vc_resource_capability),
            vc_resource_control: From::<u32>::from(vc_resource_control),
            vc_resource_status: From::<u16>::from(vc_resource_status),
        }
    }
}

/// The VC Resource Capability register describes the capabilities and configuration of a
/// particular Virtual Channel resource
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcResourceCapability {
    /// Port Arbitration Capability
    pub port_arbitration_capability: PortArbitrationCapability,
    /// Advanced Packet Switching
    pub advanced_packet_switching: bool,
    /// When Set, any transaction for which the No Snoop attribute is applicable but is not Set
    /// within the TLP header is permitted to be rejected as an Unsupported Request.
    pub reject_snoop_transactions: bool,
    /// Indicates the maximum number of time slots (minus one) that the VC resource is capable of
    /// supporting when it is configured for time-based WRR Port Arbitration.
    pub maximum_time_slots: u8,
    /// Indicates the location of the Port Arbitration Table associated with the VC resource.
    pub port_arbitration_table_offset: u8,
}

impl From<u32> for VcResourceCapability {
    fn from(dword: u32) -> Self {
        let Lsb((
            hardware_fixed_arbitration,
            wrr_32_phases,
            wrr_64_phases,
            wrr_128_phases,
            time_based_wrr_128_phases,
            wrr_256_phases,
            reserved,
            (),
            advanced_packet_switching,
            reject_snoop_transactions,
            maximum_time_slots,
            (),
            port_arbitration_table_offset,
        )) = P13::<_, 1, 1, 1, 1, 1, 1, 2, 6, 1, 1, 7, 1, 8>(dword).into();
        Self {
            port_arbitration_capability: PortArbitrationCapability {
                hardware_fixed_arbitration,
                wrr_32_phases,
                wrr_64_phases,
                wrr_128_phases,
                time_based_wrr_128_phases,
                wrr_256_phases,
                reserved,
            },
            advanced_packet_switching,
            reject_snoop_transactions,
            maximum_time_slots,
            port_arbitration_table_offset,
        }
    }
}

/// Indicates types of Port Arbitration supported by the VC resource
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortArbitrationCapability {
    /// Non-configurable hardware-fixed arbitration scheme, e.g., Round Robin (RR)
    pub hardware_fixed_arbitration: bool,
    /// Weighted Round Robin (WRR) arbitration with 32 phases
    pub wrr_32_phases: bool,
    /// WRR arbitration with 64 phases
    pub wrr_64_phases: bool,
    /// WRR arbitration with 128 phases
    pub wrr_128_phases: bool,
    /// Time-based WRR with 128 phases
    pub time_based_wrr_128_phases: bool,
    /// WRR arbitration with 256 phases
    pub wrr_256_phases: bool,
    /// Bits 6-7 Reserved
    pub reserved: u8,
}

#[derive(Debug, Clone)]
pub struct PortArbitrationTable<'a> {
    data: slice::Iter<'a, u8>,
    entry_size_bits: usize,
    shift: usize,
    byte: u8,
}
impl<'a> PortArbitrationTable<'a> {
    pub fn new(data: &'a [u8], entry_size_bits: usize) -> Self {
        Self {
            data: data.iter(),
            entry_size_bits,
            shift: 0,
            byte: 0,
        }
    }
}
impl<'a> Iterator for PortArbitrationTable<'a> {
    type Item = PortArbitrationTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.entry_size_bits == 8 {
            self.data.next().map(|&v| PortArbitrationTableEntry(v))
        } else {
            if self.shift == 0 {
                self.byte = *self.data.next()?;
            }
            let init_mask = !(u8::MAX << self.entry_size_bits);
            let mask = init_mask << self.shift;
            let result = (self.byte & mask) >> self.shift;
            self.shift = (self.shift + self.entry_size_bits) % 8;
            Some(PortArbitrationTableEntry(result))
        }
    }
}
impl<'a> PartialEq for PortArbitrationTable<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data.clone().eq(other.data.clone())
            && self.entry_size_bits == other.entry_size_bits
            && self.shift == other.shift
    }
}
impl<'a> Eq for PortArbitrationTable<'a> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortArbitrationTableEntry(u8);

/// VC Resource Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcResourceControl {
    /// TC/VC Map
    pub tc_or_vc_map: u8,
    /// Load Port Arbitration Table
    pub load_port_arbitration_table: bool,
    /// Port Arbitration Select
    pub port_arbitration_select: PortArbitrationSelect,
    /// VC ID
    pub vc_id: u8,
    /// VC Enable
    pub vc_enable: bool,
}

impl From<u32> for VcResourceControl {
    fn from(dword: u32) -> Self {
        let Lsb((
            tc_or_vc_map,
            (),
            load_port_arbitration_table,
            port_arbitration_select,
            (),
            vc_id,
            (),
            vc_enable,
        )) = P8::<_, 8, 8, 1, 3, 4, 3, 4, 1>(dword).into();
        Self {
            tc_or_vc_map,
            load_port_arbitration_table,
            port_arbitration_select: From::<u8>::from(port_arbitration_select),
            vc_id,
            vc_enable,
        }
    }
}

/// Corresponding to one of the filed in the (Port Arbitration
/// Capability)[PortArbitrationCapability]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortArbitrationSelect {
    /// Non-configurable hardware-fixed arbitration scheme, e.g., Round Robin (RR)
    HardwareFixedArbitration,
    /// Weighted Round Robin (WRR) arbitration with 32 phases
    Wrr32phases,
    /// WRR arbitration with 64 phases
    Wrr64phases,
    /// WRR arbitration with 128 phases
    Wrr128phases,
    /// Time-based WRR with 128 phases
    TimeBasedWrr128phases,
    /// WRR arbitration with 256 phases
    Wrr256phases,
    /// Reserved
    Reserved(u8),
}
impl PortArbitrationSelect {
    /// Port Arbitration Table Length (in Number of Entries)
    pub fn port_arbitration_table_length(&self) -> usize {
        match self {
            Self::HardwareFixedArbitration => 0,
            Self::Wrr32phases => 32,
            Self::Wrr64phases => 64,
            Self::Wrr128phases => 128,
            Self::TimeBasedWrr128phases => 128,
            Self::Wrr256phases => 256,
            Self::Reserved(_) => 0,
        }
    }
}
impl From<u8> for PortArbitrationSelect {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::HardwareFixedArbitration,
            0b001 => Self::Wrr32phases,
            0b010 => Self::Wrr64phases,
            0b011 => Self::Wrr128phases,
            0b100 => Self::TimeBasedWrr128phases,
            0b101 => Self::Wrr256phases,
            v => Self::Reserved(v),
        }
    }
}
impl From<PortArbitrationSelect> for u8 {
    fn from(data: PortArbitrationSelect) -> Self {
        match data {
            PortArbitrationSelect::HardwareFixedArbitration => 0,
            PortArbitrationSelect::Wrr32phases => 1,
            PortArbitrationSelect::Wrr64phases => 2,
            PortArbitrationSelect::Wrr128phases => 3,
            PortArbitrationSelect::TimeBasedWrr128phases => 4,
            PortArbitrationSelect::Wrr256phases => 5,
            PortArbitrationSelect::Reserved(n) => n,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcResourceStatus {
    /// Port Arbitration Table Status
    pub port_arbitration_table_status: bool,
    /// VC Negotiation Pending
    pub vc_negotiation_pending: bool,
}

impl From<u16> for VcResourceStatus {
    fn from(word: u16) -> Self {
        let Lsb((port_arbitration_table_status, vc_negotiation_pending, ())) =
            P3::<_, 1, 1, 14>(word).into();
        Self {
            port_arbitration_table_status,
            vc_negotiation_pending,
        }
    }
}

/// The Port VC Capability register 2 provides further information about the configuration of the
/// Virtual Channels associated with a PCI Express Port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortVcCapability2 {
    /// VC Arbitration Capability
    pub vc_arbitration_capability: VcArbitrationCapability,
    /// This field contains the zero-based offset of the table in DQWORDS (16 bytes) from the base
    /// address of the Virtual Channel Capability structure.
    pub vc_arbitration_table_offset: u8,
}

impl From<u32> for PortVcCapability2 {
    fn from(dword: u32) -> Self {
        let Lsb((
            hardware_fixed_arbitration,
            wrr_32_phases,
            wrr_64_phases,
            wrr_128_phases,
            reserved,
            (),
            vc_arbitration_table_offset,
        )) = P7::<_, 1, 1, 1, 1, 4, 16, 8>(dword).into();
        Self {
            vc_arbitration_capability: VcArbitrationCapability {
                hardware_fixed_arbitration,
                wrr_32_phases,
                wrr_64_phases,
                wrr_128_phases,
                reserved,
            },
            vc_arbitration_table_offset,
        }
    }
}

/// Indicates the types of VC Arbitration supported by the Function for the LPVC group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcArbitrationCapability {
    /// Hardware fixed arbitration scheme, e.g., Round Robin
    pub hardware_fixed_arbitration: bool,
    /// Weighted Round Robin (WRR) arbitration with 32 phases
    pub wrr_32_phases: bool,
    /// WRR arbitration with 64 phases
    pub wrr_64_phases: bool,
    /// WRR arbitration with 128 phases
    pub wrr_128_phases: bool,
    /// Bits 4-7 reserved
    pub reserved: u8,
}

/// The VC Arbitration Table is a read-write register array that is used to store the arbitration
/// table for VC Arbitration
#[derive(Debug, Clone)]
pub struct VcArbitrationTable<'a> {
    data: slice::Iter<'a, u8>,
    vate: Option<VcArbitrationTableEntry>,
}
impl<'a> VcArbitrationTable<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: data.iter(),
            vate: None,
        }
    }
}
impl<'a> Iterator for VcArbitrationTable<'a> {
    type Item = VcArbitrationTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vate) = self.vate.take() {
            Some(vate)
        } else {
            let VatEntryPair([curr, next]) = (*self.data.next()?).into();
            self.vate = Some(next);
            Some(curr)
        }
    }
}
impl<'a> PartialEq for VcArbitrationTable<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data.clone().eq(other.data.clone()) && self.vate == other.vate
    }
}
impl<'a> Eq for VcArbitrationTable<'a> {}

/// The VC Arbitration Table is a register array with fixed-size entries of 4 bits.
/// Each 4-bit table entry corresponds to a phase within a WRR arbitration period.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcArbitrationTableEntry {
    /// Indicating that the corresponding phase within the WRR arbitration period is assigned to
    /// the Virtual Channel indicated by the VC ID
    pub vc_id: u8,
}

/// One byte size VAT Entries handler
struct VatEntryPair([VcArbitrationTableEntry; 2]);

impl From<u8> for VatEntryPair {
    /// One byte contains two [VcArbitrationTableEntry]
    fn from(byte: u8) -> Self {
        Self([
            VcArbitrationTableEntry {
                vc_id: byte & 0b111,
            },
            VcArbitrationTableEntry {
                vc_id: (byte >> 4) & 0b111,
            },
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortVcControl {
    /// Load VC Arbitration Table
    pub load_vc_arbitration_table: bool,
    /// VC Arbitration Select
    pub vc_arbitration_select: VcArbitrationSelect,
}

impl From<u16> for PortVcControl {
    fn from(word: u16) -> Self {
        let Lsb((load_vc_arbitration_table, vc_arbitration_select, ())) =
            P3::<_, 1, 3, 12>(word).into();
        Self {
            load_vc_arbitration_table,
            vc_arbitration_select: From::<u8>::from(vc_arbitration_select),
        }
    }
}

/// The values of this field are corresponding to one of the field in the
/// [VcArbitrationCapability].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VcArbitrationSelect {
    /// Hardware fixed arbitration scheme, e.g., Round Robin
    HardwareFixedArbitration,
    /// Weighted Round Robin (WRR) arbitration with 32 phases
    Wrr32phases,
    /// WRR arbitration with 64 phases
    Wrr64phases,
    /// WRR arbitration with 128 phases
    Wrr128phases,
    /// Reserved
    Reserved(u8),
}
impl VcArbitrationSelect {
    /// VC Arbitration Table Length (in # of Entries)
    pub fn vc_arbitration_table_length(&self) -> usize {
        match self {
            Self::HardwareFixedArbitration => 0,
            Self::Wrr32phases => 32,
            Self::Wrr64phases => 64,
            Self::Wrr128phases => 128,
            Self::Reserved(_) => 0,
        }
    }
}
impl From<u8> for VcArbitrationSelect {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::HardwareFixedArbitration,
            0b001 => Self::Wrr32phases,
            0b010 => Self::Wrr64phases,
            0b011 => Self::Wrr128phases,
            v => Self::Reserved(v),
        }
    }
}

/// The Port VC Status register provides status of the configuration of Virtual Channels associated
/// with a Port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortVcStatus {
    /// VC Arbitration Table Status
    pub vc_arbitration_table_status: bool,
}

impl From<u16> for PortVcStatus {
    fn from(word: u16) -> Self {
        let Lsb((vc_arbitration_table_status, ())) = P2::<_, 1, 15>(word).into();
        Self {
            vc_arbitration_table_status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn port_vc_capability_1() {
        let data = 0b1010_1010_1010;
        let result = PortVcCapability1::from(data);
        let sample = PortVcCapability1 {
            extended_vc_count: 2,
            low_priority_extended_vc_count: 2,
            reference_clock: ReferenceClock::Reserved(2),
            port_arbitration_table_entry_size: PortArbitrationTableEntrySize(2),
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn port_vc_capability_2() {
        let data = 0b1111_0000000000000000_1010_1010;
        let result = PortVcCapability2::from(data);
        let sample = PortVcCapability2 {
            vc_arbitration_capability: VcArbitrationCapability {
                hardware_fixed_arbitration: false,
                wrr_32_phases: true,
                wrr_64_phases: false,
                wrr_128_phases: true,
                reserved: 0b1010,
            },
            vc_arbitration_table_offset: 0xf,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn port_vc_control() {
        let data = 0b101;
        let result = PortVcControl::from(data);
        let sample = PortVcControl {
            load_vc_arbitration_table: true,
            vc_arbitration_select: VcArbitrationSelect::Wrr64phases,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn port_vc_status() {
        let data = 0b1;
        let result = PortVcStatus::from(data);
        let sample = PortVcStatus {
            vc_arbitration_table_status: true,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn vc_resource_capability() {
        let data = 0b10101010_0_1010101_1_1_000000_10101010;
        let result = VcResourceCapability::from(data);
        let sample = VcResourceCapability {
            port_arbitration_capability: PortArbitrationCapability {
                hardware_fixed_arbitration: false,
                wrr_32_phases: true,
                wrr_64_phases: false,
                wrr_128_phases: true,
                time_based_wrr_128_phases: false,
                wrr_256_phases: true,
                reserved: 0b10,
            },
            advanced_packet_switching: true,
            reject_snoop_transactions: true,
            maximum_time_slots: 85,
            port_arbitration_table_offset: 0xAA,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn vc_resource_control() {
        let data = 0b1_0000_101_0000_101_1_00000000_10101010;
        let result = VcResourceControl::from(data);
        let sample = VcResourceControl {
            tc_or_vc_map: 0xAA,
            load_port_arbitration_table: true,
            port_arbitration_select: PortArbitrationSelect::Wrr256phases,
            vc_id: 5,
            vc_enable: true,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn vc_resource_status() {
        let data = 0b11;
        let result = VcResourceStatus::from(data);
        let sample = VcResourceStatus {
            port_arbitration_table_status: true,
            vc_negotiation_pending: true,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn vc_arbitration_table() {
        let data = [0x10, 0x23];
        let result = VcArbitrationTable::new(&data).collect::<Vec<_>>();
        let sample = vec![
            VcArbitrationTableEntry { vc_id: 0 },
            VcArbitrationTableEntry { vc_id: 1 },
            VcArbitrationTableEntry { vc_id: 3 },
            VcArbitrationTableEntry { vc_id: 2 },
        ];
        assert_eq!(sample, result);
    }

    #[test]
    fn port_arbitration_table_entry_size() {
        use PortArbitrationTableEntry as E;
        let data = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
            0x89, 0xab, 0xcd, 0xef,
        ];

        let result = PortArbitrationTable::new(&data[..4], 1).collect::<Vec<_>>();
        let sample = vec![
            E(1),
            E(0),
            E(0),
            E(0),
            E(0),
            E(0),
            E(0),
            E(0),
            E(1),
            E(1),
            E(0),
            E(0),
            E(0),
            E(1),
            E(0),
            E(0),
            E(1),
            E(0),
            E(1),
            E(0),
            E(0),
            E(0),
            E(1),
            E(0),
            E(1),
            E(1),
            E(1),
            E(0),
            E(0),
            E(1),
            E(1),
            E(0),
        ];
        assert_eq!(sample, result, "Entry size: 1 bit");

        let result = PortArbitrationTable::new(&data[..8], 2).collect::<Vec<_>>();
        let sample = vec![
            E(1),
            E(0),
            E(0),
            E(0),
            E(3),
            E(0),
            E(2),
            E(0),
            E(1),
            E(1),
            E(0),
            E(1),
            E(3),
            E(1),
            E(2),
            E(1),
            E(1),
            E(2),
            E(0),
            E(2),
            E(3),
            E(2),
            E(2),
            E(2),
            E(1),
            E(3),
            E(0),
            E(3),
            E(3),
            E(3),
            E(2),
            E(3),
        ];
        assert_eq!(sample, result, "Entry size: 2 bit");

        let result = PortArbitrationTable::new(&data[..16], 4).collect::<Vec<_>>();
        let sample = vec![
            E(0x1),
            E(0x0),
            E(0x3),
            E(0x2),
            E(0x5),
            E(0x4),
            E(0x7),
            E(0x6),
            E(0x9),
            E(0x8),
            E(0xb),
            E(0xa),
            E(0xd),
            E(0xc),
            E(0xf),
            E(0xe),
            E(0x1),
            E(0x0),
            E(0x3),
            E(0x2),
            E(0x5),
            E(0x4),
            E(0x7),
            E(0x6),
            E(0x9),
            E(0x8),
            E(0xb),
            E(0xa),
            E(0xd),
            E(0xc),
            E(0xf),
            E(0xe),
        ];
        assert_eq!(sample, result, "Entry size: 4 bit");

        let result = PortArbitrationTable::new(&data, 8).collect::<Vec<_>>();
        let sample = vec![
            E(0x01),
            E(0x23),
            E(0x45),
            E(0x67),
            E(0x89),
            E(0xab),
            E(0xcd),
            E(0xef),
            E(0x01),
            E(0x23),
            E(0x45),
            E(0x67),
            E(0x89),
            E(0xab),
            E(0xcd),
            E(0xef),
            E(0x01),
            E(0x23),
            E(0x45),
            E(0x67),
            E(0x89),
            E(0xab),
            E(0xcd),
            E(0xef),
            E(0x01),
            E(0x23),
            E(0x45),
            E(0x67),
            E(0x89),
            E(0xab),
            E(0xcd),
            E(0xef),
        ];
        assert_eq!(sample, result, "Entry size: 8 bit");
    }

    #[test]
    fn extended_virtual_channels() {
        let data = [
            // Caps:   PATOffset=00 MaxTimeSlots=1 RejSnoopTrans-
            // Arb:    Fixed- WRR32- WRR64- WRR128- TWRR128- WRR256-
            // Ctrl:   Enable+ ID=0 ArbSelect=Fixed TC/VC=ff
            // Status: NegoPending- InProgress-
            0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
            // Caps:   PATOffset=02 MaxTimeSlots=1 RejSnoopTrans-
            // Arb:    Fixed+ WRR32+ WRR64- WRR128- TWRR128- WRR256-
            // Ctrl:   Enable+ ID=0 ArbSelect=Fixed TC/VC=ff
            // Status: NegoPending- InProgress-
            // Port Arbitration Table <?>
            0x03, 0x00, 0x00, 0x02, 0xff, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
        ];
        let result = ExtendedVirtualChannels::new(&data, 2).collect::<Vec<_>>();
        let sample = vec![
            Ok(ExtendedVirtualChannel {
                vc_resource_capability: VcResourceCapability {
                    port_arbitration_capability: PortArbitrationCapability {
                        hardware_fixed_arbitration: false,
                        wrr_32_phases: false,
                        wrr_64_phases: false,
                        wrr_128_phases: false,
                        time_based_wrr_128_phases: false,
                        wrr_256_phases: false,
                        reserved: 0b00,
                    },
                    advanced_packet_switching: false,
                    reject_snoop_transactions: false,
                    maximum_time_slots: 1 - 1,
                    port_arbitration_table_offset: 0x00,
                },
                vc_resource_control: VcResourceControl {
                    tc_or_vc_map: 0xff,
                    load_port_arbitration_table: false,
                    port_arbitration_select: PortArbitrationSelect::HardwareFixedArbitration,
                    vc_id: 0,
                    vc_enable: true,
                },
                vc_resource_status: VcResourceStatus {
                    port_arbitration_table_status: false,
                    vc_negotiation_pending: false,
                },
            }),
            Ok(ExtendedVirtualChannel {
                vc_resource_capability: VcResourceCapability {
                    port_arbitration_capability: PortArbitrationCapability {
                        hardware_fixed_arbitration: true,
                        wrr_32_phases: true,
                        wrr_64_phases: false,
                        wrr_128_phases: false,
                        time_based_wrr_128_phases: false,
                        wrr_256_phases: false,
                        reserved: 0b00,
                    },
                    advanced_packet_switching: false,
                    reject_snoop_transactions: false,
                    maximum_time_slots: 1 - 1,
                    port_arbitration_table_offset: 0x02,
                },
                vc_resource_control: VcResourceControl {
                    tc_or_vc_map: 0xff,
                    load_port_arbitration_table: false,
                    port_arbitration_select: PortArbitrationSelect::HardwareFixedArbitration,
                    vc_id: 0,
                    vc_enable: true,
                },
                vc_resource_status: VcResourceStatus {
                    port_arbitration_table_status: false,
                    vc_negotiation_pending: false,
                },
            }),
        ];
        assert_eq!(sample, result);
    }
}
