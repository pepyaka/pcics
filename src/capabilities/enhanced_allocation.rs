/*!
# Enhanced Allocation

The Enhanced Allocation (EA) Capability is an optional Capability that allows
the allocation of I/O, Memory and Bus Number resources in ways not possible
with the BAR and Base/Limit mechanisms in the Type 0 and Type 1 Configuration
Headers.

## Struct diagram
<pre>
<a href="struct.EnhancedAllocation.html">EnhancedAllocation</a>
└─ <a href="struct.EnhancedAllocationEntry.html">EnhancedAllocationEntry[0]</a> ... <a href="struct.EnhancedAllocationEntry.html">EnhancedAllocationEntry[n]</a>
   ├─ <a href="enum.BarEquivalentIndicator.html">BarEquivalentIndicator</a>
   ├─ <a href="struct.Properties.html">2 x Properties</a>
   │  └─ <a href="enum.ResourceDefinition.html">ResourceDefinition</a>
   └─ <a href="enum.ResourceRangeAddress.html">2 x ResourceRangeAddress</a>
</pre>

## Examples

```rust
# use pcics::capabilities::enhanced_allocation::*;
# let header = [0; 0x40].into();
let data = [
    0x14, 0x00, // Header
    0x01, 0x00, // Num Entries
    0b10_0100, 0x00, 0x42, 0b1100_0000, // First DW
    0x00 | 0b11, 0x11, 0x22, 0x33, // Base[31:2]
    0x00 | 0b10, 0x11, 0x22, 0x33, // MaxOffset[31:2]
    0x44, 0x55, 0x66, 0x77, // Base[63:32]
    0x44, 0x55, 0x66, 0x77, // MaxOffset[63:32]
];
let sample = EnhancedAllocationEntry {
    entry_size: 4,
    bar_equivalent_indicator: BarEquivalentIndicator::Location18h,
    primary_properties: ResourceDefinition::MemorySpaceNonPrefetchable,
    secondary_properties: ResourceDefinition::Reserved(0x42),
    writable: true,
    enable: true,
    base: ResourceRangeAddress::U64(0x7766554433221100),
    max_offset: ResourceRangeAddress::U64(0x7766554433221100 | 0b11),
};
let mut ea = EnhancedAllocation::try_new(&data[2..], &header).unwrap();
let result = ea.entries.next();
assert_eq!(Some(sample), result);
```
*/

use heterob::{
    bit_numbering::Lsb,
    endianness::{Le, LeBytesTryInto},
    Seq, P3, P8,
};

use crate::{header::HeaderType, Header};

use snafu::Snafu;

/// Enhanced Allocation Errors
#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum EnhancedAllocationError {
    #[snafu(display("number of entries is unreadable"))]
    NumEntries,
    #[snafu(display("second DW for Type 1 is unreadable"))]
    Type1SecondDw,
}

/// Enhanced Allocation (EA) Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnhancedAllocation<'a> {
    pub num_entries: u8,
    pub type_1_second_dw: Option<Type1SecondDw>,
    pub entries: EnhancedAllocationEntries<'a>,
}

impl<'a> EnhancedAllocation<'a> {
    pub fn try_new(slice: &'a [u8], header: &'a Header) -> Result<Self, EnhancedAllocationError> {
        if let [num_entries, _, slice @ ..] = slice {
            let num_entries = *num_entries & 0x3f;
            if matches!(header.header_type, HeaderType::Bridge(_)) {
                if let [sec, sub, _, _, slice @ ..] = slice {
                    Ok(Self {
                        num_entries,
                        type_1_second_dw: Some(Type1SecondDw {
                            fixed_secondary_bus_number: *sec,
                            fixed_subordinate_bus_number: *sub,
                        }),
                        entries: EnhancedAllocationEntries::new(slice, num_entries),
                    })
                } else {
                    Err(EnhancedAllocationError::NumEntries)
                }
            } else {
                Ok(Self {
                    num_entries,
                    type_1_second_dw: None,
                    entries: EnhancedAllocationEntries::new(slice, num_entries),
                })
            }
        } else {
            Err(EnhancedAllocationError::NumEntries)
        }
    }
}

/// An iterator through Enhanced Allocation entries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnhancedAllocationEntries<'a> {
    data: &'a [u8],
    count: u8,
}

impl<'a> EnhancedAllocationEntries<'a> {
    pub fn new(data: &'a [u8], count: u8) -> Self {
        Self { data, count }
    }
}

impl<'a> Iterator for EnhancedAllocationEntries<'a> {
    type Item = EnhancedAllocationEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        let Seq {
            head: Le((first_dw, base_lo, max_offset_lo)),
            tail: mut _slice,
        } = P3(self.data).try_into().ok()?;
        let _: (u32, u32) = (base_lo, max_offset_lo);
        self.count -= 1;

        let Lsb((
            entry_size,
            (),
            bar_equivalent_indicator,
            primary_properties,
            secondary_properties,
            (),
            writable,
            enable,
        )) = P8::<u32, 3, 1, 4, 8, 8, 6, 1, 1>(first_dw).into();
        let _: u8 = entry_size;

        let is_base_64 = base_lo & 0b10 != 0;
        // Bits [1:0] of the address are not included in the Base field,
        // and must always be interpreted as 00b.
        let base_lo = base_lo & !0b11;
        let base = if is_base_64 {
            let Seq { head, tail } = _slice.le_bytes_try_into().ok()?;
            let _: u32 = head;
            _slice = tail;
            let base_hi = (head as u64) << 32;
            ResourceRangeAddress::U64(base_hi | base_lo as u64)
        } else {
            ResourceRangeAddress::U32(base_lo)
        };

        let is_max_offset_64 = max_offset_lo & 0b10 != 0;
        // Bits [1:0] of the MaxOffset are not included in the MaxOffset
        // field, and must always be interpreted as 11b.
        let max_offset_lo = max_offset_lo | 0b11;
        let max_offset = if is_max_offset_64 {
            let Seq { head, tail } = _slice.le_bytes_try_into().ok()?;
            let _: u32 = head;
            _slice = tail;
            let max_offset_hi = (head as u64) << 32;
            ResourceRangeAddress::U64(max_offset_hi | max_offset_lo as u64)
        } else {
            ResourceRangeAddress::U32(max_offset_lo)
        };

        // The are 2 ways to set start of next entry:
        if cfg!(feature = "caps_ea_real_entry_size") {
            // according to last readed bytes
            self.data = _slice;
        } else {
            // according to entry size field
            let next_entry_start = (entry_size as usize + 1) * 4;
            self.data = self.data.get(next_entry_start..)?;
        }

        Some(EnhancedAllocationEntry {
            entry_size,
            bar_equivalent_indicator: From::<u8>::from(bar_equivalent_indicator),
            primary_properties: From::<u8>::from(primary_properties),
            secondary_properties: From::<u8>::from(secondary_properties),
            writable,
            enable,
            base,
            max_offset,
        })
    }
}

/// For Type 1 functions only, there is a second DW in the capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type1SecondDw {
    /// Bus number of the PCI bus segment to which the secondary interface of
    /// this Function is connected
    pub fixed_secondary_bus_number: u8,
    /// Bus number of the highest numbered PCI bus segment which is behind
    /// this Function
    pub fixed_subordinate_bus_number: u8,
}
/// Entry for Enhanced Allocation Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnhancedAllocationEntry {
    /// Entry Size
    pub entry_size: u8,
    pub bar_equivalent_indicator: BarEquivalentIndicator,
    /// Identify the type of resource indicated by the entry
    pub primary_properties: ResourceDefinition,
    /// Indicate an alternate resource type which can be used by software when
    /// the [Primary Properties](Self::primary_properties) field
    /// value is not comprehended by that software
    pub secondary_properties: ResourceDefinition,
    /// Indicates that the Base and MaxOffset fields for this entry are RW
    pub writable: bool,
    /// Indicates this entry is enabled
    pub enable: bool,
    /// Indicates the address of the start of the resource
    pub base: ResourceRangeAddress,
    /// The value in the [Base field](Self::base) plus the value in the MaxOffset
    /// field indicates the address of the last included DW of the resource range
    pub max_offset: ResourceRangeAddress,
}

/// BAR Equivalent Indicator (BEI)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarEquivalentIndicator {
    /// Entry is equivalent to BAR at location 10h
    Location10h,
    /// Entry is equivalent to BAR at location 14h
    Location14h,
    /// Entry is equivalent to BAR at location 18h
    Location18h,
    /// Entry is equivalent to BAR at location 1Ch
    Location1Ch,
    /// Entry is equivalent to BAR at location 20h
    Location20h,
    /// Entry is equivalent to BAR at location 24h
    Location24h,
    /// Permitted to be used by a Function with a Type 1 Configuration Space
    /// header only, optionally used to indicate a resource that is located behind
    /// the Function
    BehindType1Function,
    /// Equivalent Not Indicated
    EquivalentNotIndicated,
    /// Expansion ROM Base Address
    ExpansionRomBaseAddress,
    /// Entry relates to VF BARs 0
    VfBar0,
    /// Entry relates to VF BARs 1
    VfBar1,
    /// Entry relates to VF BARs 2
    VfBar2,
    /// Entry relates to VF BARs 3
    VfBar3,
    /// Entry relates to VF BARs 4
    VfBar4,
    /// Entry relates to VF BARs 5
    VfBar5,
    /// Reserved
    Reserved,
}

impl From<u8> for BarEquivalentIndicator {
    fn from(byte: u8) -> Self {
        match byte {
            0x0 => Self::Location10h,
            0x1 => Self::Location14h,
            0x2 => Self::Location18h,
            0x3 => Self::Location1Ch,
            0x4 => Self::Location20h,
            0x5 => Self::Location24h,
            0x6 => Self::BehindType1Function,
            0x7 => Self::EquivalentNotIndicated,
            0x8 => Self::ExpansionRomBaseAddress,
            0x9 => Self::VfBar0,
            0xA => Self::VfBar1,
            0xB => Self::VfBar2,
            0xC => Self::VfBar3,
            0xD => Self::VfBar4,
            0xE => Self::VfBar5,
            0xF => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

/// Enhanced Allocation Entry Field Value Definitions for both the Primary
/// Properties and Secondary Properties Fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceDefinition {
    /// Memory Space, Non-Prefetchable
    MemorySpaceNonPrefetchable,
    /// Memory Space, Prefetchable
    MemorySpacePrefetchable,
    /// I/O Space
    IoSpace,
    /// For use only by Physical Functions to indicate resources for Virtual
    /// Function use, Memory Space, Prefetchable
    PfForVfMemorySpacePrefetchable,
    /// For use only by Physical Functions to indicate resources for Virtual
    /// Function use, Memory Space, Non-Prefetchable
    PfForVfMemorySpaceNonPrefetchable,
    /// For use only by Type 1 Functions to indicate Memory, NonPrefetchable, for
    /// Allocation Behind that Bridge
    Type1ForAbbMemoryNonPrefetchable,
    /// For use only by Type 1 Functions to indicate Memory, Prefetchable, for
    /// Allocation Behind that Bridge
    Type1ForAbbMemoryPrefetchable,
    /// For use only by Type 1 Functions to indicate I/O Space for Allocation
    /// Behind that Bridge
    Type1ForAbbIoSpace,
    /// Reserved for future use
    Reserved(u8),
    /// Memory       Resource Unavailable For Use
    UnavailableMemorySpace,
    /// I/O Space Resource Unavailable For Use
    UnavailableIoSpace,
    /// Entry Unavailable For Use
    Unavailable,
}

impl From<u8> for ResourceDefinition {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => ResourceDefinition::MemorySpaceNonPrefetchable,
            0x01 => ResourceDefinition::MemorySpacePrefetchable,
            0x02 => ResourceDefinition::IoSpace,
            0x03 => ResourceDefinition::PfForVfMemorySpacePrefetchable,
            0x04 => ResourceDefinition::PfForVfMemorySpaceNonPrefetchable,
            0x05 => ResourceDefinition::Type1ForAbbMemoryNonPrefetchable,
            0x06 => ResourceDefinition::Type1ForAbbMemoryPrefetchable,
            0x07 => ResourceDefinition::Type1ForAbbIoSpace,
            v @ 0x08..=0xFC => ResourceDefinition::Reserved(v),
            0xFD => ResourceDefinition::UnavailableMemorySpace,
            0xFE => ResourceDefinition::UnavailableIoSpace,
            0xFF => ResourceDefinition::Unavailable,
        }
    }
}

/// 32b or 64b address type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceRangeAddress {
    U32(u32),
    U64(u64),
}
