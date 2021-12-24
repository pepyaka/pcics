//! Virtual Channel Capability
//!
//! The Virtual Channel (VC) Capability is an optional Extended Capability required for devices
//! that have Ports (or for individual Functions) that support functionality beyond the default
//! Traffic Class (TC0) over the default Virtual Channel (VC0).

use core::slice;

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

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
    /// Port VC Control Register
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
        let entries_number = self.port_vc_control.vc_arbitration_select
            .vc_arbitration_table_length();
        let start = offset as usize * DQWORD - ECH_BYTES;
        // VC Arbitration Table entry length is 4 bits, so there are 2 entries in one byte
        let end = start + entries_number / 2;
        let data = &self.data[start..end];
        VcArbitrationTable::new(data)
    }
    pub fn port_arbitration_table(&'a self, evc: &'a ExtendedVirtualChannel) -> PortArbitrationTable<'a> {
        let offset = evc.vc_resource_capability.port_arbitration_table_offset;
        let entry_size_bits = self.port_vc_capability_1.port_arbitration_table_entry_size.bits();
        let entries_number = evc.vc_resource_control.port_arbitration_select
            .port_arbitration_table_length();
        let start = offset as usize * DQWORD - ECH_BYTES;
        let end = start + entry_size_bits * entries_number / 8;
        let data = &self.data[start..end];
        PortArbitrationTable::new(data, entry_size_bits)
    }
}
impl<'a> TryRead<'a, Endian> for VirtualChannel<'a> {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let vc = VirtualChannel {
            data: bytes,
            port_vc_capability_1: bytes.read_with::<u32>(offset, endian)?.into(),
            port_vc_capability_2: bytes.read_with::<u32>(offset, endian)?.into(),
            port_vc_control: bytes.read_with::<u16>(offset, endian)?.into(),
            port_vc_status: bytes.read_with::<u16>(offset, endian)?.into(),
        };
        Ok((vc, *offset))
    }
}



#[bitfield(bits = 32)]
#[repr(u32)]
pub struct PortVcCapability1Proto {
    extended_vc_count: B3,
    rsvdp: B1,
    low_priority_extended_vc_count: B3,
    rsvdp_2: B1,
    reference_clock: B2,
    port_arbitration_table_entry_size: B2,
    rsvdp_3: B20,
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
impl From<PortVcCapability1Proto> for PortVcCapability1 {
    fn from(proto: PortVcCapability1Proto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        let _ = proto.rsvdp_3();
        Self {
            extended_vc_count: proto.extended_vc_count(),
            low_priority_extended_vc_count: proto.low_priority_extended_vc_count(),
            reference_clock: proto.reference_clock().into(),
            port_arbitration_table_entry_size: proto.port_arbitration_table_entry_size().into(),
        }
    }
}
impl From<u32> for PortVcCapability1 {
    fn from(dword: u32) -> Self { PortVcCapability1Proto::from(dword).into() }
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
    pub fn bits(&self) -> usize { 1 << self.0 }
}
impl From<u8> for PortArbitrationTableEntrySize {
    fn from(byte: u8) -> Self { Self(byte) }
}

/// An iterator through 0 - 7 (Extended Virtual Channels)[ExtendedVirtualChannel]
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
            offset: 0
        }
    }
}
impl<'a> Iterator for ExtendedVirtualChannels<'a> {
    type Item = ExtendedVirtualChannel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let evc: ExtendedVirtualChannel =
                self.data.read_with(&mut self.offset, LE).ok()?;
            self.count -= 1;
            Some(evc)
        }
    }
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
    pub fn is_unreadable(&self) -> bool {
        let caps: u32 = VcResourceCapabilityProto::from(self.vc_resource_capability.clone()).into();
        let ctrl: u32 = VcResourceControlProto::from(self.vc_resource_control.clone()).into();
        let sta: u16 = VcResourceStatusProto::from(self.vc_resource_status.clone()).into();
        caps == 0 && ctrl ==0 && sta == 0
    }
}
impl<'a> TryRead<'a, Endian> for ExtendedVirtualChannel {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let evc = ExtendedVirtualChannel {
            vc_resource_capability: bytes.read_with::<u32>(offset, endian)?.into(),
            vc_resource_control: bytes.read_with::<u32>(offset, endian)?.into(),
            vc_resource_status: {
                // Skip RsvdP part 18h - 19h Extended Virtual Channel structure
                let _ = bytes.read_with::<u16>(offset, endian)?;
                bytes.read_with::<u16>(offset, endian)?.into()
            },
        };
        Ok((evc, *offset))
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct VcResourceCapabilityProto {
    hardware_fixed_arbitration: bool,
    wrr_32_phases: bool,
    wrr_64_phases: bool,
    wrr_128_phases: bool,
    time_based_wrr_128_phases: bool,
    wrr_256_phases: bool,
    rsvdp: B8,
    advanced_packet_switching: bool,
    reject_snoop_transactions: bool,
    maximum_time_slots: B7,
    rsvdp_2: B1,
    port_arbitration_table_offset: u8,
}
impl From<VcResourceCapability> for VcResourceCapabilityProto {
    fn from(data: VcResourceCapability) -> Self {
        let pac = data.port_arbitration_capability;
        Self::new()
            .with_hardware_fixed_arbitration(pac.hardware_fixed_arbitration)
            .with_wrr_32_phases(pac.wrr_32_phases)
            .with_wrr_64_phases(pac.wrr_64_phases)
            .with_wrr_128_phases(pac.wrr_128_phases)
            .with_time_based_wrr_128_phases(pac.time_based_wrr_128_phases)
            .with_wrr_256_phases(pac.wrr_256_phases)
            .with_rsvdp(0)
            .with_advanced_packet_switching(data.advanced_packet_switching)
            .with_reject_snoop_transactions(data.reject_snoop_transactions)
            .with_maximum_time_slots(data.maximum_time_slots)
            .with_rsvdp_2(0)
            .with_port_arbitration_table_offset(data.port_arbitration_table_offset)
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
impl From<VcResourceCapabilityProto> for VcResourceCapability {
    fn from(proto: VcResourceCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            port_arbitration_capability: PortArbitrationCapability {
                hardware_fixed_arbitration: proto.hardware_fixed_arbitration(),
                wrr_32_phases: proto.wrr_32_phases(),
                wrr_64_phases: proto.wrr_64_phases(),
                wrr_128_phases: proto.wrr_128_phases(),
                time_based_wrr_128_phases: proto.time_based_wrr_128_phases(),
                wrr_256_phases: proto.wrr_256_phases(),
            },
            advanced_packet_switching: proto.advanced_packet_switching(),
            reject_snoop_transactions: proto.reject_snoop_transactions(),
            maximum_time_slots: proto.maximum_time_slots(),
            port_arbitration_table_offset: proto.port_arbitration_table_offset(),
        }
    }
}
impl From<u32> for VcResourceCapability {
    fn from(dword: u32) -> Self { VcResourceCapabilityProto::from(dword).into() }
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
        Self { data: data.iter(), entry_size_bits, shift: 0, byte: 0 }
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
            println!("{:08b} {} {:02x}", mask, self.shift, self.byte);
            let result = (self.byte & mask) >> self.shift;
            self.shift = (self.shift + self.entry_size_bits) % 8;
            Some(PortArbitrationTableEntry(result))
        }
    }

    //    match (self.entry_size_bits, self.shift) {
    //        (8, 0) => self.byte & 0b1111_1111 >> 0,
    //        (4, 0) => self.byte & 0b0000_1111 >> 0,
    //        (4, 1) => self.byte & 0b1111_0000 >> 4,
    //        (2, 0) => self.byte & 0b0000_0011 >> 0,
    //        (2, 1) => self.byte & 0b0000_1100 >> 2,
    //        (2, 2) => self.byte & 0b0011_0000 >> 4,
    //        (2, 3) => self.byte & 0b1100_0000 >> 6,
    //        (1, 0) => self.byte & 0b0000_0001 >> 0,
    //        (1, 1) => self.byte & 0b0000_0010 >> 1,
    //        (1, 2) => self.byte & 0b0000_0100 >> 2,
    //        (1, 3) => self.byte & 0b0000_1000 >> 3,
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


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct VcResourceControlProto {
    tc_or_vc_map: u8,
    rsvdp: B8,
    load_port_arbitration_table: bool,
    port_arbitration_select: B3,
    rsvdp_2: B4,
    vc_id: B3,
    rsvdp_3: B4,
    vc_enable: bool,
}
impl From<VcResourceControl> for VcResourceControlProto {
    fn from(data: VcResourceControl) -> Self {
        Self::new()
            .with_tc_or_vc_map(data.tc_or_vc_map)
            .with_rsvdp(0)
            .with_load_port_arbitration_table(data.load_port_arbitration_table)
            .with_port_arbitration_select(data.port_arbitration_select.into())
            .with_rsvdp_2(0)
            .with_vc_id(data.vc_id)
            .with_rsvdp_3(0)
            .with_vc_enable(data.vc_enable)
    }
}

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
impl From<VcResourceControlProto> for VcResourceControl {
    fn from(proto: VcResourceControlProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        let _ = proto.rsvdp_3();
        Self {
            tc_or_vc_map: proto.tc_or_vc_map(),
            load_port_arbitration_table: proto.load_port_arbitration_table(),
            port_arbitration_select: proto.port_arbitration_select().into(),
            vc_id: proto.vc_id(),
            vc_enable: proto.vc_enable(),
        }
    }
}
impl From<u32> for VcResourceControl {
    fn from(dword: u32) -> Self { VcResourceControlProto::from(dword).into() }
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
            Self::Reserved(_) => 0
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

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct VcResourceStatusProto {
    port_arbitration_table_status: bool,
    vc_negotiation_pending: bool,
    rsvdz: B14,
}
impl From<VcResourceStatus> for VcResourceStatusProto {
    fn from(data: VcResourceStatus) -> Self {
        Self::new()
            .with_port_arbitration_table_status(data.port_arbitration_table_status)
            .with_vc_negotiation_pending(data.vc_negotiation_pending)
            .with_rsvdz(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcResourceStatus {
    /// Port Arbitration Table Status
    pub port_arbitration_table_status: bool,
    /// VC Negotiation Pending
    pub vc_negotiation_pending: bool,
}
impl From<VcResourceStatusProto> for VcResourceStatus {
    fn from(proto: VcResourceStatusProto) -> Self {
        let _ = proto.rsvdz();
        Self {
            port_arbitration_table_status: proto.port_arbitration_table_status(),
            vc_negotiation_pending: proto.vc_negotiation_pending(),
        }
    }
}
impl From<u16> for VcResourceStatus {
    fn from(word: u16) -> Self { VcResourceStatusProto::from(word).into() }
}


#[bitfield(bits = 32)]
#[repr(u32)]
pub struct PortVcCapability2Proto {
    // VC Arbitration Capability
    hardware_fixed_arbitration: bool,
    wrr_32_phases: bool,
    wrr_64_phases: bool,
    wrr_128_phases: bool,
    rsvdp: B4,
    // Reserved
    rsvdp_2: B16,
    // VC Arbitration Table Offset
    vc_arbitration_table_offset: u8,
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
impl From<PortVcCapability2Proto> for PortVcCapability2 {
    fn from(proto: PortVcCapability2Proto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            vc_arbitration_capability: VcArbitrationCapability {
                hardware_fixed_arbitration: proto.hardware_fixed_arbitration(),
                wrr_32_phases: proto.wrr_32_phases(),
                wrr_64_phases: proto.wrr_64_phases(),
                wrr_128_phases: proto.wrr_128_phases(),
            },
            vc_arbitration_table_offset: proto.vc_arbitration_table_offset(),
        }
    }
}
impl From<u32> for PortVcCapability2 {
    fn from(dword: u32) -> Self { PortVcCapability2Proto::from(dword).into() }
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
}

/// The VC Arbitration Table is a read-write register array that is used to store the arbitration
/// table for VC Arbitration
#[derive(Debug, Clone)]
pub struct VcArbitrationTable<'a>{
    data: slice::Iter<'a, u8>,
    vate: Option<VcArbitrationTableEntry>
}
impl<'a> VcArbitrationTable<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data: data.iter(), vate: None }
    }
}
impl<'a> Iterator for VcArbitrationTable<'a> {
    type Item = VcArbitrationTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vate) = self.vate.take() {
            Some(vate)
        } else {
            let VatePair([curr, next]) = (*self.data.next()?).into();
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
    vc_id: u8,
}
struct VatePair([VcArbitrationTableEntry; 2]);
impl From<u8> for VatePair {
    /// One byte contains two [VcArbitrationTableEntry]
    fn from(byte: u8) -> Self {
        Self([
            VcArbitrationTableEntry { vc_id: byte & 0b111 },
            VcArbitrationTableEntry { vc_id: (byte >> 4) & 0b111 },
        ])
    }
}



#[bitfield(bits = 16)]
#[repr(u16)]
pub struct PortVcControlProto {
    load_vc_arbitration_table: bool,
    vc_arbitration_select: B3,
    rsvdp: B12,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortVcControl {
    /// Load VC Arbitration Table
    pub load_vc_arbitration_table: bool,
    /// VC Arbitration Select
    pub vc_arbitration_select: VcArbitrationSelect,
}
impl From<PortVcControlProto> for PortVcControl {
    fn from(proto: PortVcControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            load_vc_arbitration_table: proto.load_vc_arbitration_table(),
            vc_arbitration_select: proto.vc_arbitration_select().into(),
        }
    }
}
impl From<u16> for PortVcControl {
    fn from(word: u16) -> Self { PortVcControlProto::from(word).into() }
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



#[bitfield(bits = 16)]
#[repr(u16)]
pub struct PortVcStatusProto {
    vc_arbitration_table_status: bool,
    rsvdp: B15,
}
/// The Port VC Status register provides status of the configuration of Virtual Channels associated
/// with a Port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortVcStatus {
    /// VC Arbitration Table Status
    pub vc_arbitration_table_status: bool,
}
impl From<PortVcStatusProto> for PortVcStatus {
    fn from(proto: PortVcStatusProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            vc_arbitration_table_status: proto.vc_arbitration_table_status(),
        }
    }
}
impl From<u16> for PortVcStatus {
    fn from(word: u16) -> Self { PortVcStatusProto::from(word).into() }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

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
        let data = [0x10,0x23];
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
            0x01,0x23,0x45,0x67, 0x89,0xab,0xcd,0xef,
            0x01,0x23,0x45,0x67, 0x89,0xab,0xcd,0xef,

            0x01,0x23,0x45,0x67, 0x89,0xab,0xcd,0xef,
            0x01,0x23,0x45,0x67, 0x89,0xab,0xcd,0xef,
        ];

        let result = PortArbitrationTable::new(&data[..4], 1).collect::<Vec<_>>();
        let sample = vec![
            E(1), E(0), E(0), E(0), E(0), E(0), E(0), E(0),
            E(1), E(1), E(0), E(0), E(0), E(1), E(0), E(0),
            E(1), E(0), E(1), E(0), E(0), E(0), E(1), E(0),
            E(1), E(1), E(1), E(0), E(0), E(1), E(1), E(0),
        ];
        assert_eq!(sample, result, "Entry size: 1 bit");

        let result = PortArbitrationTable::new(&data[..8], 2).collect::<Vec<_>>();
        let sample = vec![
            E(1),E(0),E(0),E(0), E(3),E(0),E(2),E(0),
            E(1),E(1),E(0),E(1), E(3),E(1),E(2),E(1),
            E(1),E(2),E(0),E(2), E(3),E(2),E(2),E(2),
            E(1),E(3),E(0),E(3), E(3),E(3),E(2),E(3),
        ];
        assert_eq!(sample, result, "Entry size: 2 bit");

        let result = PortArbitrationTable::new(&data[..16], 4).collect::<Vec<_>>();
        let sample = vec![
            E(0x1),E(0x0), E(0x3),E(0x2), E(0x5),E(0x4), E(0x7),E(0x6),
            E(0x9),E(0x8), E(0xb),E(0xa), E(0xd),E(0xc), E(0xf),E(0xe),
            E(0x1),E(0x0), E(0x3),E(0x2), E(0x5),E(0x4), E(0x7),E(0x6),
            E(0x9),E(0x8), E(0xb),E(0xa), E(0xd),E(0xc), E(0xf),E(0xe),
        ];
        assert_eq!(sample, result, "Entry size: 4 bit");

        let result = PortArbitrationTable::new(&data, 8).collect::<Vec<_>>();
        let sample = vec![
            E(0x01), E(0x23), E(0x45), E(0x67), E(0x89), E(0xab), E(0xcd), E(0xef),
            E(0x01), E(0x23), E(0x45), E(0x67), E(0x89), E(0xab), E(0xcd), E(0xef),
            E(0x01), E(0x23), E(0x45), E(0x67), E(0x89), E(0xab), E(0xcd), E(0xef),
            E(0x01), E(0x23), E(0x45), E(0x67), E(0x89), E(0xab), E(0xcd), E(0xef),
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
            0x00,0x00,0x00,0x00,0xff,0x00,0x00,0x80,0x00,0x00,0x00,0x00,
            
            // Caps:   PATOffset=02 MaxTimeSlots=1 RejSnoopTrans-
            // Arb:    Fixed+ WRR32+ WRR64- WRR128- TWRR128- WRR256-
            // Ctrl:   Enable+ ID=0 ArbSelect=Fixed TC/VC=ff
            // Status: NegoPending- InProgress-
            // Port Arbitration Table <?>
            0x03,0x00,0x00,0x02,0xff,0x00,0x00,0x80,0x00,0x00,0x00,0x00,
        ];
        let result = ExtendedVirtualChannels::new(&data, 2).collect::<Vec<_>>();
        let sample = vec![
            ExtendedVirtualChannel {
                vc_resource_capability: VcResourceCapability {
                    port_arbitration_capability: PortArbitrationCapability {
                            hardware_fixed_arbitration: false,
                            wrr_32_phases: false,
                            wrr_64_phases: false,
                            wrr_128_phases: false,
                            time_based_wrr_128_phases: false,
                            wrr_256_phases: false,
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
            },
            ExtendedVirtualChannel {
                vc_resource_capability: VcResourceCapability {
                    port_arbitration_capability: PortArbitrationCapability {
                            hardware_fixed_arbitration: true,
                            wrr_32_phases: true,
                            wrr_64_phases: false,
                            wrr_128_phases: false,
                            time_based_wrr_128_phases: false,
                            wrr_256_phases: false,
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
            },
        ];
        assert_eq!(sample, result);
    }

}
