/*!
# Multi-Function Virtual Channel

The Multi-Function Virtual Channel (MFVC) Capability is an optional Extended
Capability that permits enhanced QoS management in a Multi-Function Device,
including TC/VC mapping, optional VC arbitration, and optional Function
arbitration for Upstream Requests.

## Struct diagram
<pre>
<a href="struct.MultifunctionVirtualChannel.html">MultifunctionVirtualChannel</a>
├─ <a href="struct.PortVcCapability1.html">PortVcCapability1</a>
│  ├─ <a href="../virtual_channel/enum.ReferenceClock.html">ReferenceClock</a>
│  └─ <a href="struct.FunctionArbitrationTableEntrySize.html">FunctionArbitrationTableEntrySize</a>
├─ <a href="../virtual_channel/struct.PortVcCapability2.html">PortVcCapability2</a>
│  └─ <a href="../virtual_channel/struct.VcArbitrationCapability.html">VcArbitrationCapability</a>
├─ <a href="../virtual_channel/struct.PortVcControl.html">PortVcControl</a>
│  └─ <a href="../virtual_channel/enum.VcArbitrationSelect.html">VcArbitrationSelect</a>
├─ <a href="../virtual_channel/struct.PortVcStatus.html">PortVcStatus</a>
├─ <a href="../virtual_channel/struct.VcArbitrationTable.html">VcArbitrationTable</a>
│  └─ <a href="../virtual_channel/struct.VcArbitrationTableEntry.html">VcArbitrationTableEntry (0 .. N)</a>
└─ <a href="struct.ExtendedVirtualChannels.html">ExtendedVirtualChannels</a>
   └─ <a href="struct.ExtendedVirtualChannel.html">ExtendedVirtualChannel (0 .. N)</a>
      ├─ <a href="struct.VcResourceCapability.html">VcResourceCapability</a>
      │  └─ <a href="struct.FunctionArbitrationCapability.html">FunctionArbitrationCapability</a>
      ├─ <a href="struct.VcResourceControl.html">VcResourceControl</a>
      │  └─ <a href="enum.FunctionArbitrationSelect.html">FunctionArbitrationSelect</a>
      ├─ <a href="struct.VcResourceStatus.html">VcResourceStatus</a>
      └─ <a href="struct.FunctionArbitrationTable.html">FunctionArbitrationTable</a>
         └─ <a href="struct.FunctionArbitrationTableEntry.html">FunctionArbitrationTableEntry (0 .. N)</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::multifunction_virtual_channel::*;
let data = [
    /* 00h */ 0x08, 0x00, 0x01, 0x00,             // Capability header
    /* 04h */ 0b0_000_0_001, 0b1_00, 0x00, 0x00,  // Port VC Capability Register 1
    /* 08h */ 0b1111, 0x00, 0x00, 0x00,           // Port VC Capability Register 2
    /* 0Ch */ 0b0010, 0x00,                       // Port VC Control Register
              0x00, 0x00,                         // Port VC Status Register
    /* 10h */ 0b111, 0x00, 0x00, 0x00,            // VC Resource Capability Register (0)
    /* 14h */ 0xff, 0x00, 0x04, 0x80,             // VC Resource Control Register (0)
    /* 18h */ 0x00, 0x00,                         // RsvdP
              0b10, 0x00,                         // VC Resource Status Register (0)
];

let result: MultifunctionVirtualChannel = data.as_slice().try_into().unwrap();

let mut evcs_sample_data = [0u8; 0x1C];
evcs_sample_data[..0x10].clone_from_slice(&data[..0x10]);
evcs_sample_data[0x10..].clone_from_slice({
    let evc = ExtendedVirtualChannel {
        vc_resource_capability: VcResourceCapability {
            function_arbitration_capability: FunctionArbitrationCapability {
                hardware_fixed_arbitration: true,
                wrr_32_phases: true,
                wrr_64_phases: true,
                wrr_128_phases: false,
                time_based_wrr_128_phases: false,
                wrr_256_phases: false,
                reserved: 0,
            },
            maximum_time_slots: 0,
            function_arbitration_table_offset: 0,
        },
        vc_resource_control: VcResourceControl {
            tc_to_vc_map: 0xff,
            load_function_arbitration_table: false,
            function_arbitration_select: FunctionArbitrationSelect::Wrr64phases,
            vc_id: 0,
            vc_enable: true,
        },
        vc_resource_status: VcResourceStatus {
            function_arbitration_table_status: false,
            vc_negotiation_pending: true,
        },
        function_arbitration_table: None,
    };
    Into::<[u8; ExtendedVirtualChannel::SIZE]>::into(evc).as_slice()
});

let sample = MultifunctionVirtualChannel {
    port_vc_capability_1: PortVcCapability1 {
        extended_vc_count: 1,
        low_priority_extended_vc_count: 0,
        reference_clock: ReferenceClock::Rc100ns,
        function_arbitration_table_entry_size: FunctionArbitrationTableEntrySize(0b01),
    },
    port_vc_capability_2: PortVcCapability2 {
        vc_arbitration_capability: VcArbitrationCapability {
            hardware_fixed_arbitration: true,
            wrr_32_phases: true,
            wrr_64_phases: true,
            wrr_128_phases: true,
            reserved: 0,
        },
        vc_arbitration_table_offset: 0,
    },
    port_vc_control: PortVcControl {
        load_vc_arbitration_table: false,
        vc_arbitration_select: VcArbitrationSelect::Wrr32phases,
    },
    port_vc_status: PortVcStatus {
        vc_arbitration_table_status: false,
    },
    vc_arbitration_table: None,
    extended_virtual_channels: ExtendedVirtualChannels::new(evcs_sample_data.as_slice(), 1, 2),
};

assert_eq!(sample, result);
```
*/

use core::slice;

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P11, P3, P4, P5, P7, P8};
use snafu::Snafu;

pub use super::virtual_channel::{
    PortVcCapability2, PortVcControl, PortVcStatus, ReferenceClock, VcArbitrationCapability,
    VcArbitrationSelect, VcArbitrationTable,
};

/// Numeral unit for VC Arbitration Table Offset and Function Arbitration Table Offset
const DQWORD: usize = 0x10;

#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum MultifunctionVirtualChannelError {
    #[snafu(display("capability 1, capability 2, control and status of Port VC are unreadable"))]
    PortVcData,
    #[snafu(display("VC Arbitration Table offset should be >= 2 * 10h"))]
    VcArbitrationTableOffset,
    #[snafu(display("VC Arbitration Table data ureadable"))]
    VcArbitrationTableData,
    #[snafu(display("VC Arbitration Table data too short: expected {expected}, real {real}"))]
    VcArbitrationTableLength { expected: usize, real: usize },
    #[snafu(display("at least one Function Arbitration Table should be readable"))]
    FunctionArbitrationTable0,
}

/// Multi-Function Virtual Channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultifunctionVirtualChannel<'a> {
    pub port_vc_capability_1: PortVcCapability1,
    /// Port VC Capability 2
    ///
    /// Provides further information about the configuration of the Virtual
    /// Channels associated with a PCI Express Port of the Multi-Function Device
    pub port_vc_capability_2: PortVcCapability2,
    pub port_vc_control: PortVcControl,
    pub port_vc_status: PortVcStatus,
    pub vc_arbitration_table: Option<VcArbitrationTable<'a>>,
    pub extended_virtual_channels: ExtendedVirtualChannels<'a>,
}

impl<'a> TryFrom<&'a [u8]> for MultifunctionVirtualChannel<'a> {
    type Error = MultifunctionVirtualChannelError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq {
            head:
                Le((
                    extended_capability_header,
                    port_vc_capability_1,
                    port_vc_capability_2,
                    port_vc_control,
                    port_vc_status,
                )),
            ..
        } = P5(slice)
            .try_into()
            .map_err(|_| MultifunctionVirtualChannelError::PortVcData)?;
        let _: u32 = extended_capability_header;
        let port_vc_capability_1 @ PortVcCapability1 {
            extended_vc_count,
            function_arbitration_table_entry_size,
            ..
        } = From::<u32>::from(port_vc_capability_1);
        let port_vc_capability_2: PortVcCapability2 = From::<u32>::from(port_vc_capability_2);
        let port_vc_control: PortVcControl = From::<u16>::from(port_vc_control);
        let port_vc_status: PortVcStatus = From::<u16>::from(port_vc_status);
        let vc_arbitration_table = if port_vc_capability_2.vc_arbitration_table_offset == 0 {
            None
        } else if port_vc_capability_2.vc_arbitration_table_offset < 2 {
            return Err(MultifunctionVirtualChannelError::VcArbitrationTableOffset);
        } else {
            let vat_offset = port_vc_capability_2.vc_arbitration_table_offset as usize * DQWORD;
            let vat_slice = slice
                .get(vat_offset..)
                .ok_or(MultifunctionVirtualChannelError::VcArbitrationTableData)?;
            let number_of_entries = port_vc_control
                .vc_arbitration_select
                .vc_arbitration_table_length();
            // VAT entries is fixed 4 bits long
            let length = number_of_entries * 4 / 8;
            let vat_data = vat_slice.get(..length).ok_or(
                MultifunctionVirtualChannelError::VcArbitrationTableLength {
                    expected: length,
                    real: vat_slice.len(),
                },
            )?;
            Some(VcArbitrationTable::new(vat_data))
        };
        Ok(Self {
            extended_virtual_channels: ExtendedVirtualChannels::new(
                slice,
                extended_vc_count,
                function_arbitration_table_entry_size.bits(),
            ),
            port_vc_capability_1,
            port_vc_capability_2,
            port_vc_control,
            port_vc_status,
            vc_arbitration_table,
        })
    }
}

/// Port VC Capability 1
///
/// Describes the configuration of the Virtual Channels associated with a PCI
/// Express Port of the Multi-Function Device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortVcCapability1 {
    /// Indicates the number of (extended) Virtual Channels in addition to the default VC supported
    /// by the device.
    pub extended_vc_count: u8,
    /// Indicates the number of (extended) Virtual Channels in addition to the default VC belonging
    /// to the low-priority VC (LPVC) group that has the lowest priority with respect to other VC
    /// resources in a strictpriority VC Arbitration.
    pub low_priority_extended_vc_count: u8,
    /// Indicates the reference clock for Virtual Channels that support time-based
    /// WRR Function Arbitration
    pub reference_clock: ReferenceClock,
    pub function_arbitration_table_entry_size: FunctionArbitrationTableEntrySize,
}

impl From<u32> for PortVcCapability1 {
    fn from(dword: u32) -> Self {
        let Lsb((
            extended_vc_count,
            (),
            low_priority_extended_vc_count,
            (),
            reference_clock,
            function_arbitration_table_entry_size,
            (),
        )) = P7::<_, 3, 1, 3, 1, 2, 2, 20>(dword).into();
        Self {
            extended_vc_count,
            low_priority_extended_vc_count,
            reference_clock: From::<u8>::from(reference_clock),
            function_arbitration_table_entry_size: From::<u8>::from(
                function_arbitration_table_entry_size,
            ),
        }
    }
}

/// Function Arbitration Table Entry Size
///
/// Indicates the size (in bits) of Function Arbitration table entry in the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionArbitrationTableEntrySize(pub u8);
impl FunctionArbitrationTableEntrySize {
    pub fn bits(&self) -> usize {
        1 << self.0
    }
}

impl From<u8> for FunctionArbitrationTableEntrySize {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

/// An iterator through 0 - 7 [Extended Virtual Channels](ExtendedVirtualChannel)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedVirtualChannels<'a> {
    data: &'a [u8],
    count: u8,
    fat_entry_bits: usize,
    offset: usize,
}
impl<'a> ExtendedVirtualChannels<'a> {
    pub fn new(data: &'a [u8], extended_vc_count: u8, fat_entry_bits: usize) -> Self {
        Self {
            data,
            // Default Extended Vc should always exists
            count: extended_vc_count + 1,
            fat_entry_bits,
            offset: 0x10,
        }
    }
}
impl<'a> Iterator for ExtendedVirtualChannels<'a> {
    type Item = ExtendedVirtualChannel<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let slice = self.data.get(self.offset..)?;
            let Seq { head, .. } = slice.try_into().ok()?;
            let mut evc: ExtendedVirtualChannel =
                From::<[u8; ExtendedVirtualChannel::SIZE]>::from(head);
            if let fat_offset @ 2.. = evc.vc_resource_capability.function_arbitration_table_offset {
                let offset = fat_offset as usize * 0x10;
                let number_of_entries = evc
                    .vc_resource_control
                    .function_arbitration_select
                    .function_arbitration_table_length();
                let length = self.fat_entry_bits * number_of_entries / 8;
                evc.function_arbitration_table = self
                    .data
                    .get(offset..offset + length)
                    .map(|data| FunctionArbitrationTable::new(data, self.fat_entry_bits))
            }
            self.count -= 1;
            self.offset += ExtendedVirtualChannel::SIZE;
            Some(evc)
        }
    }
}

/// Extended Virtual Channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedVirtualChannel<'a> {
    pub vc_resource_capability: VcResourceCapability,
    pub vc_resource_control: VcResourceControl,
    pub vc_resource_status: VcResourceStatus,
    pub function_arbitration_table: Option<FunctionArbitrationTable<'a>>,
}
impl<'a> ExtendedVirtualChannel<'a> {
    /// [VcResourceCapability] + [VcResourceControl] + RsvdP (1 Byte) + [VcResourceStatus]
    pub const SIZE: usize = 4 + 4 + 2 + 2;
}

impl<'a> From<[u8; ExtendedVirtualChannel::SIZE]> for ExtendedVirtualChannel<'a> {
    fn from(data: [u8; ExtendedVirtualChannel::SIZE]) -> Self {
        let Le((vc_resource_capability, vc_resource_control, r, vc_resource_status)) =
            P4(data).into();
        let _: u16 = r;
        Self {
            vc_resource_capability: From::<u32>::from(vc_resource_capability),
            vc_resource_control: From::<u32>::from(vc_resource_control),
            vc_resource_status: From::<u16>::from(vc_resource_status),
            function_arbitration_table: None,
        }
    }
}

impl<'a> From<ExtendedVirtualChannel<'a>> for [u8; ExtendedVirtualChannel::SIZE] {
    fn from(evc: ExtendedVirtualChannel<'a>) -> Self {
        let cap = Into::<u32>::into(evc.vc_resource_capability).to_le_bytes();
        let ctrl = Into::<u32>::into(evc.vc_resource_control).to_le_bytes();
        let st = Into::<u16>::into(evc.vc_resource_status).to_le_bytes();
        [
            cap[0], cap[1], cap[2], cap[3], ctrl[0], ctrl[1], ctrl[2], ctrl[3], 0, 0, st[0], st[1],
        ]
    }
}

/// The VC Resource Capability register describes the capabilities and configuration of a
/// particular Virtual Channel resource
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcResourceCapability {
    pub function_arbitration_capability: FunctionArbitrationCapability,
    /// Indicates the maximum number of time slots (minus 1) that the VC
    /// resource is capable of supporting when it is configured for time-based WRR
    /// Function Arbitration
    pub maximum_time_slots: u8,
    /// Indicates the location of the Function Arbitration Table associated with the VC resource.
    pub function_arbitration_table_offset: u8,
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
            maximum_time_slots,
            (),
            function_arbitration_table_offset,
        )) = P11::<_, 1, 1, 1, 1, 1, 1, 2, 8, 7, 1, 8>(dword).into();
        Self {
            function_arbitration_capability: FunctionArbitrationCapability {
                hardware_fixed_arbitration,
                wrr_32_phases,
                wrr_64_phases,
                wrr_128_phases,
                time_based_wrr_128_phases,
                wrr_256_phases,
                reserved,
            },
            maximum_time_slots,
            function_arbitration_table_offset,
        }
    }
}

impl From<VcResourceCapability> for u32 {
    fn from(cap: VcResourceCapability) -> Self {
        u32::from_le_bytes([
            cap.function_arbitration_capability.into(),
            0,
            cap.maximum_time_slots.into(),
            cap.function_arbitration_table_offset,
        ])
    }
}

/// Indicates types of Function Arbitration supported by the VC resource
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionArbitrationCapability {
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

impl From<FunctionArbitrationCapability> for u8 {
    fn from(fac: FunctionArbitrationCapability) -> Self {
        fac.hardware_fixed_arbitration as u8
            | (fac.wrr_32_phases as u8) << 1
            | (fac.wrr_64_phases as u8) << 2
            | (fac.wrr_128_phases as u8) << 3
            | (fac.time_based_wrr_128_phases as u8) << 4
            | (fac.wrr_256_phases as u8) << 5
            | fac.reserved << 6
    }
}

/// VC Resource Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcResourceControl {
    /// Indicates the TCs that are mapped to this VC resource
    pub tc_to_vc_map: u8,
    /// When Set, this bit updates the Function Arbitration logic from the
    /// Function Arbitration Table for the VC resource
    pub load_function_arbitration_table: bool,
    pub function_arbitration_select: FunctionArbitrationSelect,
    /// Assigns a VC ID to the VC resource
    pub vc_id: u8,
    /// Enables a Virtual Channel
    pub vc_enable: bool,
}

impl From<u32> for VcResourceControl {
    fn from(dword: u32) -> Self {
        let Lsb((
            tc_to_vc_map,
            (),
            load_function_arbitration_table,
            function_arbitration_select,
            (),
            vc_id,
            (),
            vc_enable,
        )) = P8::<_, 8, 8, 1, 3, 4, 3, 4, 1>(dword).into();
        Self {
            tc_to_vc_map,
            load_function_arbitration_table,
            function_arbitration_select: From::<u8>::from(function_arbitration_select),
            vc_id,
            vc_enable,
        }
    }
}

impl From<VcResourceControl> for u32 {
    fn from(ctrl: VcResourceControl) -> Self {
        u32::from_le_bytes([
            ctrl.tc_to_vc_map,
            0,
            (ctrl.load_function_arbitration_table as u8)
                | Into::<u8>::into(ctrl.function_arbitration_select) << 1,
            ctrl.vc_id | (ctrl.vc_enable as u8) << 7,
        ])
    }
}

/// Corresponding to one of the asserted fields in the [Function Arbitration
/// Capability](FunctionArbitrationCapability) field of the VC resource
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionArbitrationSelect {
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
impl FunctionArbitrationSelect {
    /// Function Arbitration Table Length (in Number of Entries)
    pub fn function_arbitration_table_length(&self) -> usize {
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
impl From<u8> for FunctionArbitrationSelect {
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
impl From<FunctionArbitrationSelect> for u8 {
    fn from(data: FunctionArbitrationSelect) -> Self {
        match data {
            FunctionArbitrationSelect::HardwareFixedArbitration => 0,
            FunctionArbitrationSelect::Wrr32phases => 1,
            FunctionArbitrationSelect::Wrr64phases => 2,
            FunctionArbitrationSelect::Wrr128phases => 3,
            FunctionArbitrationSelect::TimeBasedWrr128phases => 4,
            FunctionArbitrationSelect::Wrr256phases => 5,
            FunctionArbitrationSelect::Reserved(n) => n,
        }
    }
}

/// VC Resource Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcResourceStatus {
    /// Indicates the coherency status of the Function Arbitration Table
    /// associated with the VC resource
    pub function_arbitration_table_status: bool,
    /// Indicates whether the Virtual Channel negotiation (initialization or
    /// disabling) is in pending state
    pub vc_negotiation_pending: bool,
}

impl From<u16> for VcResourceStatus {
    fn from(word: u16) -> Self {
        let Lsb((function_arbitration_table_status, vc_negotiation_pending, ())) =
            P3::<_, 1, 1, 14>(word).into();
        Self {
            function_arbitration_table_status,
            vc_negotiation_pending,
        }
    }
}

impl From<VcResourceStatus> for u16 {
    fn from(st: VcResourceStatus) -> Self {
        u16::from_le_bytes([
            st.function_arbitration_table_status as u8 | (st.vc_negotiation_pending as u8) << 1,
            0,
        ])
    }
}

/// Function Arbitration Table is used to store the WRR or time-based WRR
/// arbitration table for Function Arbitration for the VC resource
#[derive(Debug, Clone)]
pub struct FunctionArbitrationTable<'a> {
    data: slice::Iter<'a, u8>,
    entry_size_bits: usize,
    shift: usize,
    byte: u8,
}

impl<'a> FunctionArbitrationTable<'a> {
    pub fn new(data: &'a [u8], entry_size_bits: usize) -> Self {
        Self {
            data: data.iter(),
            entry_size_bits,
            shift: 0,
            byte: 0,
        }
    }
}

impl<'a> Iterator for FunctionArbitrationTable<'a> {
    type Item = FunctionArbitrationTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.entry_size_bits == 8 {
            self.data.next().map(|&v| FunctionArbitrationTableEntry(v))
        } else {
            if self.shift == 0 {
                self.byte = *self.data.next()?;
            }
            let init_mask = !(u8::MAX << self.entry_size_bits);
            let mask = init_mask << self.shift;
            let result = (self.byte & mask) >> self.shift;
            self.shift = (self.shift + self.entry_size_bits) % 8;
            Some(FunctionArbitrationTableEntry(result))
        }
    }
}

impl<'a> PartialEq for FunctionArbitrationTable<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data.clone().eq(other.data.clone())
            && self.entry_size_bits == other.entry_size_bits
            && self.shift == other.shift
            && self.byte == other.byte
    }
}

impl<'a> Eq for FunctionArbitrationTable<'a> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionArbitrationTableEntry(u8);

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn function_arbitration_table_1_bit_entry() {
        let data = [0b10101010, 0b11001100, 0b11110000, 0xff];
        let sample = (0..32).fold(
            vec![FunctionArbitrationTableEntry(0); 32],
            |mut sample, i| {
                sample[i] = FunctionArbitrationTableEntry(((i >> (i / 8)) % 2) as u8);
                sample
            },
        );
        let result: Vec<_> = FunctionArbitrationTable::new(data.as_slice(), 1).collect();
        assert_eq!(sample, result);
    }

    #[test]
    fn function_arbitration_table_2_bit_entry() {
        let data = [
            0x00, 0b01010101, 0b10101010, 0b00110011, 0b11001100, 0b00001111, 0b11110000, 0xff,
        ];
        let sample = (0..32).fold(
            vec![FunctionArbitrationTableEntry(0); 32],
            |mut sample, i| {
                let v = match i {
                    4..=7 => 0b01,
                    8..=11 => 0b10,
                    12 | 14 | 17 | 19 | 20 | 21 | 26 | 27 | 28..=31 => 0b11,
                    _ => 0b00,
                };
                sample[i] = FunctionArbitrationTableEntry(v as u8);
                sample
            },
        );
        let result: Vec<_> = FunctionArbitrationTable::new(data.as_slice(), 2).collect();
        assert_eq!(sample, result);
    }

    #[test]
    fn function_arbitration_table_4_bit_entry() {
        let data = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let sample = (0..32).fold(
            vec![FunctionArbitrationTableEntry(0); 32],
            |mut sample, i| {
                sample[i] = FunctionArbitrationTableEntry((i / 2) as u8);
                sample
            },
        );
        let result: Vec<_> = FunctionArbitrationTable::new(data.as_slice(), 4).collect();
        assert_eq!(sample, result);
    }

    #[test]
    fn function_arbitration_table_8_bit_entry() {
        let data = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0,
            0xc0, 0xd0, 0xe0, 0xf0,
        ];
        let sample = (0..32).fold(
            vec![FunctionArbitrationTableEntry(0); 32],
            |mut sample, i| {
                sample[i] = FunctionArbitrationTableEntry(if i < 16 { i } else { i << 4 } as u8);
                sample
            },
        );
        let result: Vec<_> = FunctionArbitrationTable::new(data.as_slice(), 8).collect();
        assert_eq!(sample, result);
    }

    #[test]
    fn extended_virtual_channels() {
        #[rustfmt::skip]
        let data = [
            /* 00h */ 0x08, 0x00, 0x01, 0x00,             // Capability header
            /* 04h */ 0b0_000_0_001, 0b1_00, 0x00, 0x00,  // Port VC Capability Register 1
            /* 08h */ 0b1111, 0x00, 0x00, 0x00,           // Port VC Capability Register 2
            /* 0Ch */ 0b0010, 0x00,                       // Port VC Control Register
                      0x00, 0x00,                         // Port VC Status Register
            /* 10h */ 0b111, 0x00, 0x00, 0x04,            // VC Resource Capability Register (0)
            /* 14h */ 0x0f, 0x00, 0x04, 0x80,             // VC Resource Control Register (0)
            /* 18h */ 0x00, 0x00,                         // RsvdP
                      0b10, 0x00,                         // VC Resource Status Register (0)
            /* 1Ch */ 0b11000, 0x00, 0x7f, 0x05,          // VC Resource Capability Register (1)
            /* 20h */ 0xf0, 0x00, 0b100_0, 0x80,          // VC Resource Control Register (1)
            /* 24h */ 0x00, 0x00,                         // RsvdP
                      0b01, 0x00,                         // VC Resource Status Register (1)
        ];
        let result: Vec<_> = ExtendedVirtualChannels::new(data.as_slice(), 1, 0).collect();

        let sample = vec![
            ExtendedVirtualChannel {
                vc_resource_capability: VcResourceCapability {
                    function_arbitration_capability: FunctionArbitrationCapability {
                        hardware_fixed_arbitration: true,
                        wrr_32_phases: true,
                        wrr_64_phases: true,
                        wrr_128_phases: false,
                        time_based_wrr_128_phases: false,
                        wrr_256_phases: false,
                        reserved: 0,
                    },
                    maximum_time_slots: 0,
                    function_arbitration_table_offset: 0x40 / 0x10,
                },
                vc_resource_control: VcResourceControl {
                    tc_to_vc_map: 0x0f,
                    load_function_arbitration_table: false,
                    function_arbitration_select: FunctionArbitrationSelect::Wrr64phases,
                    vc_id: 0,
                    vc_enable: true,
                },
                vc_resource_status: VcResourceStatus {
                    function_arbitration_table_status: false,
                    vc_negotiation_pending: true,
                },
                function_arbitration_table: None,
            },
            ExtendedVirtualChannel {
                vc_resource_capability: VcResourceCapability {
                    function_arbitration_capability: FunctionArbitrationCapability {
                        hardware_fixed_arbitration: false,
                        wrr_32_phases: false,
                        wrr_64_phases: false,
                        wrr_128_phases: true,
                        time_based_wrr_128_phases: true,
                        wrr_256_phases: false,
                        reserved: 0,
                    },
                    maximum_time_slots: 0x7f,
                    function_arbitration_table_offset: 0x50 / 0x10,
                },
                vc_resource_control: VcResourceControl {
                    tc_to_vc_map: 0xf0,
                    load_function_arbitration_table: false,
                    function_arbitration_select: FunctionArbitrationSelect::TimeBasedWrr128phases,
                    vc_id: 0,
                    vc_enable: true,
                },
                vc_resource_status: VcResourceStatus {
                    function_arbitration_table_status: true,
                    vc_negotiation_pending: false,
                },
                function_arbitration_table: None,
            },
        ];

        assert_eq!(sample, result);
    }
}
