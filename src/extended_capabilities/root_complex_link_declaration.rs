/*!
# PCI Express Root Complex Link Declaration`

The PCI Express Root Complex Link Declaration Capability is an optional Capability that is
permitted to be implemented by Root Ports, RCiEPs, or RCRBs to declare a Root Complex’s
internal topology.

## Struct diagram
[RootComplexLinkDeclaration]
- [ElementSelfDescription]
  - [ElementType]
- [LinkEntries]: [LinkEntry0](LinkEntry) .. [LinkEntryN](LinkEntry)

## Examples
> ```text
> Desc:  PortNumber=03 ComponentID=01 EltType=Config
> Link0: Desc: TargetPort=00 TargetComponent=01
>              AssocRCRB- LinkType=MemMapped LinkValid+
>        Addr: 00000000fed19000
> ```

```rust
# use pcics::extended_capabilities::root_complex_link_declaration::*;
let data = [
    0x05, 0x00, 0x01, 0x1c, // Extended Capability Header
    0x00, 0x01, 0x01, 0x03, // Element Self Description
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Reserved
    0x01, 0x00, 0x01, 0x00, // Link Entry 1 Description
    0x00, 0x00, 0x00, 0x00, // Link Entry 1 Reserved
    0x00, 0x90, 0xd1, 0xfe, // Link Entry 1 Address First DWORD
    0x00, 0x00, 0x00, 0x00, // Link Entry 1 Address Second DWORD
];
let mut rcld_result: RootComplexLinkDeclaration = data[4..].try_into().unwrap();
let rcld_sample = RootComplexLinkDeclaration {
    element_self_description: ElementSelfDescription {
        port_number: 3,
        component_id: 1,
        element_type: ElementType::ConfigurationSpaceElement,
        reserved: 0,
        number_of_link_entries: 1,
    },
    link_entries: LinkEntries::new(&data[0x10..0x20], 1),
};

let le_result = rcld_result.link_entries.next().unwrap();
let le_sample = LinkEntry {
    link_description: LinkDescription {
        target_port_number: 0,
        target_component_id: 1,
        associate_rcrb_header: false,
        link_type: 0,
        link_valid: true,
    },
    link_address: LinkAddress::MemoryMappedSpace(0xfed19000),
};
assert_eq!(le_sample, le_result);
```
*/

use heterob::{
    endianness::{Le, LeBytesTryInto},
    Seq, P2,
};

use super::ExtendedCapabilityDataError;

/// The Serial Number register is a 64-bit field that contains the IEEE defined 64-bit extended
/// unique identifier (EUI-64™).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSerialNumber {
    /// PCI Express Device Serial Number (1st DW)
    pub lower_dword: u32,
    /// PCI Express Device Serial Number (2nd DW)
    pub upper_dword: u32,
}
impl TryFrom<&[u8]> for DeviceSerialNumber {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((lower_dword, upper_dword)),
            ..
        } = P2(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Device Serial Number",
                size: 8,
            })?;
        Ok(Self {
            lower_dword,
            upper_dword,
        })
    }
}

use core::slice::Chunks;

use heterob::{bit_numbering::Lsb, endianness::FromLeBytes, P4, P5, P6};
use snafu::prelude::*;

/// Root Complex Link Declaration Error
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum RootComplexLinkDeclarationError {
    #[snafu(display("can't read Element Self Description (4 bytes) from slice"))]
    ElementSelfDescription,
    #[snafu(display("number of link entries must be > 1"))]
    NumberOfLinkEntries {
        element_self_description: ElementSelfDescription,
    },
    #[snafu(display("reserved space should be readable"))]
    ReservedSpace {
        element_self_description: ElementSelfDescription,
    },
    #[snafu(display("there must be at least one LinkEntry"))]
    LinkEntry1 {
        element_self_description: ElementSelfDescription,
    },
}

/// Root Complex Link Declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootComplexLinkDeclaration<'a> {
    pub element_self_description: ElementSelfDescription,
    pub link_entries: LinkEntries<'a>,
}
impl<'a> TryFrom<&'a [u8]> for RootComplexLinkDeclaration<'a> {
    type Error = RootComplexLinkDeclarationError;
    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: element_self_description,
            tail: slice,
        } = slice
            .le_bytes_try_into()
            .map_err(|_| RootComplexLinkDeclarationError::ElementSelfDescription)?;
        let element_self_description @ ElementSelfDescription {
            number_of_link_entries,
            ..
        } = From::<u32>::from(element_self_description);
        if number_of_link_entries < 1 {
            return Err(RootComplexLinkDeclarationError::NumberOfLinkEntries {
                element_self_description,
            });
        }
        let Seq {
            head: _reserved,
            tail: slice,
        } = slice.le_bytes_try_into().map_err(|_| {
            RootComplexLinkDeclarationError::ReservedSpace {
                element_self_description: element_self_description.clone(),
            }
        })?;
        let _: [u8; 8] = _reserved;
        if slice.len() < LinkEntry::SIZE {
            return Err(RootComplexLinkDeclarationError::LinkEntry1 {
                element_self_description,
            });
        }
        let link_entries = LinkEntries::new(slice, number_of_link_entries);

        Ok(Self {
            element_self_description,
            link_entries,
        })
    }
}

/// Provides information about the Root Complex element containing the Root
/// Complex Link Declaration Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementSelfDescription {
    /// Element Type
    pub element_type: ElementType,
    /// Reserved
    pub reserved: u8,
    /// Number of Link Entries
    pub number_of_link_entries: u8,
    /// Component ID
    pub component_id: u8,
    /// Port Number
    pub port_number: u8,
}
impl ElementSelfDescription {
    pub const BYTES: usize = 4;
}
impl From<u32> for ElementSelfDescription {
    fn from(dword: u32) -> Self {
        let Lsb((element_type, reserved, number_of_link_entries, component_id, port_number)) =
            P5::<_, 4, 4, 8, 8, 8>(dword).into();
        let element_type = From::<u8>::from(element_type);
        Self {
            element_type,
            reserved,
            number_of_link_entries,
            component_id,
            port_number,
        }
    }
}
impl FromLeBytes<4> for ElementSelfDescription {
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        u32::from_le_bytes(bytes).into()
    }
}

/// Indicates the type of the Root Complex Element
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementType {
    /// Configuration Space Element
    ConfigurationSpaceElement,
    /// System Egress Port or internal sink (memory)
    SystemEgressPortOrInternalSink,
    /// Internal Root Complex Link
    InternalRootComplexLink,
    /// Reserved
    Reserved(u8),
}
impl From<u8> for ElementType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::ConfigurationSpaceElement,
            0x01 => Self::SystemEgressPortOrInternalSink,
            0x02 => Self::InternalRootComplexLink,
            v => Self::Reserved(v),
        }
    }
}
impl From<ElementType> for u8 {
    fn from(et: ElementType) -> Self {
        match et {
            ElementType::ConfigurationSpaceElement => 0x00,
            ElementType::SystemEgressPortOrInternalSink => 0x01,
            ElementType::InternalRootComplexLink => 0x02,
            ElementType::Reserved(v) => v,
        }
    }
}

/// Link Entries
#[derive(Debug, Clone)]
pub struct LinkEntries<'a> {
    chunks: Chunks<'a, u8>,
    pub state: LinkEntriesState,
}
impl<'a> LinkEntries<'a> {
    pub const FIRST_ENTRY_OFFSET: usize = 0x10 - super::ECH_BYTES;
    pub fn new(slice: &'a [u8], number_of_link_entries: u8) -> Self {
        let length = (number_of_link_entries as usize) * LinkEntry::SIZE;
        Self {
            chunks: slice[..slice.len().min(length)].chunks(LinkEntry::SIZE),
            state: if slice.len() >= length {
                LinkEntriesState::Valid
            } else if slice.len() % LinkEntry::SIZE == 0 {
                LinkEntriesState::Incomplete
            } else {
                LinkEntriesState::Invalid
            },
        }
    }
}

impl<'a> PartialEq for LinkEntries<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.clone().eq(other.clone())
    }
}
impl<'a> Eq for LinkEntries<'a> {}
impl<'a> Iterator for LinkEntries<'a> {
    type Item = LinkEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = self.chunks.next()?;
        let Seq {
            head: Le((link_description, _reserved, addr_low, addr_high)),
            ..
        } = P4(slice).try_into().ok()?;
        let link_description @ LinkDescription { link_type, .. } =
            From::<u32>::from(link_description);
        let _: [u8; 4] = _reserved;
        Some(LinkEntry {
            link_description,
            link_address: LinkAddress::new(link_type, addr_low, addr_high),
        })
    }
}

/// Link Entries State depends on bytes number
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkEntriesState {
    /// Number of entries equal to [ElementSelfDescription::number_of_link_entries]
    Valid,
    /// Number of entries is less than [ElementSelfDescription::number_of_link_entries]
    Incomplete,
    /// Link entries bytes number is not aligned to [LinkEntry] size
    Invalid,
}

/// Declares an internal Link to another Root Complex Element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkEntry {
    /// Link Description
    pub link_description: LinkDescription,
    /// Link Address
    pub link_address: LinkAddress,
}
impl LinkEntry {
    pub const DWORDS: usize = 4;
    pub const SIZE: usize = 16;
}

/// Link Description
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkDescription {
    /// Link Valid
    pub link_valid: bool,
    /// Link Type
    pub link_type: u8,
    /// Associate RCRB Header
    pub associate_rcrb_header: bool,
    /// Target Component ID
    pub target_component_id: u8,
    /// Target Port Number
    pub target_port_number: u8,
}
impl From<u32> for LinkDescription {
    fn from(dword: u32) -> Self {
        let Lsb((
            link_valid,
            link_type,
            associate_rcrb_header,
            (),
            target_component_id,
            target_port_number,
        )) = P6::<_, 1, 1, 1, 13, 8, 8>(dword).into();
        Self {
            link_valid,
            link_type,
            associate_rcrb_header,
            target_component_id,
            target_port_number,
        }
    }
}
impl FromLeBytes<4> for LinkDescription {
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        u32::from_le_bytes(bytes).into()
    }
}

/// Identifies the target element for the Link entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkAddress {
    /// Pointing to a memory-mapped RCRB
    MemoryMappedSpace(u64),
    /// Pointing to the Configuration Space of a Root Complex element
    ConfigurationSpace {
        /// Encoded number of Bus Number bits
        n: u8,
        /// Reserved
        reserved: u16,
        /// Function Number
        function: u8,
        /// Device Number
        device: u8,
        /// Bus Number
        bus: u8,
        /// PCI Express Configuration Space Base Address
        address: u64,
    },
}
impl LinkAddress {
    const DWORDS: usize = 2;
}
impl LinkAddress {
    #[must_use]
    pub fn new(link_type: u8, first_dword: u32, second_dword: u32) -> Self {
        const BUS_AND_ADDR_OFFSET: u8 = 20;
        let address = (second_dword as u64) << 32 | first_dword as u64;
        if link_type == 0 {
            Self::MemoryMappedSpace(address)
        } else {
            let Lsb((n, reserved, function, device)) = P4::<_, 3, 9, 3, 5>(first_dword).into();
            // with N = 000b specifying n = 8 and all other encodings specifying n = <value of N>
            let n: u8 = if n == 0 { 8 } else { n };
            // Bits (19 + n):20 specify the Bus Number, with 1 ≤ n ≤ 8
            let bus = ((first_dword >> BUS_AND_ADDR_OFFSET) & !(u32::MAX << n)) as u8;
            let laddr_mask = u64::MAX << (BUS_AND_ADDR_OFFSET + n);
            // Bits 31:(20 + n) of the first DWORD together with the second DWORD
            let address = address & laddr_mask;
            Self::ConfigurationSpace {
                n,
                reserved,
                function,
                device,
                bus,
                address,
            }
        }
    }
}
impl From<LinkAddress> for [u32; LinkAddress::DWORDS] {
    fn from(la: LinkAddress) -> Self {
        match la {
            LinkAddress::MemoryMappedSpace(qword) => [qword as u32, (qword >> 32) as u32],
            LinkAddress::ConfigurationSpace {
                n,
                reserved,
                function,
                device,
                bus,
                address,
            } => [
                address as u32
                    | (bus as u32) << 20
                    | (device as u32) << 15
                    | (function as u32) << 12
                    | (reserved as u32) << 3
                    | (if n == 8 { 0 } else { n }) as u32,
                (address >> 32) as u32,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn link_description() {
        let result: LinkDescription = 0b10101010_10101010_0000000000000_1_0_1.into();
        let sample = LinkDescription {
            link_valid: true,
            link_type: 0,
            associate_rcrb_header: true,
            target_component_id: 0xAA,
            target_port_number: 0xAA,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn link_address_memory_mapped_space() {
        let data = [0b1111_1111_1111_1111_1111_0000_0000_0000u32, 0x33221100];
        let result = LinkAddress::new(0, data[0], data[1]);
        let sample = LinkAddress::MemoryMappedSpace(0x33221100FFFFF000);
        assert_eq!(sample, result);
    }

    #[test]
    fn link_address_configuration_space() {
        //           address    bus  dev   fn  reserved  N
        let data = [0b11111111_0101_01010_101_000000000_100u32, 0x33221100];
        let result = LinkAddress::new(1, data[0], data[1]);
        let sample = LinkAddress::ConfigurationSpace {
            n: 4,
            reserved: 0,
            function: 0b101,
            device: 0b01010,
            bus: 0b0101,
            address: 0x33221100FF000000,
        };
        assert_eq!(sample, result);
    }

    #[test]
    fn link_address_into() {
        let data = [
            [0x00, 0x00],
            [0xaa, 0xaa],
            [0x55, 0x55],
            [0x0f, 0x0f],
            [0xf0, 0xf0],
            [0xff, 0xff],
        ];
        for sample in data {
            let la = LinkAddress::new(1, sample[0], sample[1]);
            let result: [u32; 2] = la.clone().into();
            assert_eq!(sample, result, "{:04x?}", (sample, la));
        }
    }

    #[test]
    fn link_entries() {
        // Link0:  Desc:   TargetPort=00 TargetComponent=01 AssocRCRB- LinkType=MemMapped LinkValid+
        //         Addr:   00000000fed19000
        let data = [
            0x01, 0x00, 0x01, 0x00, // Link entry 1 description
            0x00, 0x00, 0x00, 0x00, // Link entry 1 reserved
            0x00, 0x90, 0xd1, 0xfe, 0x00, 0x00, 0x00, 0x00, // Link entry 1 address
        ];
        let le = LinkEntries::new(data.as_slice(), 1);

        assert_eq!(LinkEntriesState::Valid, le.state, "State");

        let result = le.collect::<Vec<_>>();

        let sample = vec![LinkEntry {
            link_description: LinkDescription {
                link_valid: true,
                link_type: 0,
                associate_rcrb_header: false,
                target_component_id: 0x01,
                target_port_number: 0x00,
            },
            link_address: LinkAddress::MemoryMappedSpace(0x00000000fed19000),
        }];
        assert_eq!(sample, result, "Entry");
    }

    #[test]
    fn parse_full_struct() {
        // Desc:   PortNumber=02 ComponentID=01 EltType=Config
        // Link0:  Desc:   TargetPort=00 TargetComponent=01 AssocRCRB- LinkType=MemMapped LinkValid+
        //         Addr:   00000000fed19000
        let data = [
            0x05, 0x00, 0x01,
            0x1c, // Root Complex Link Declaration Extended Capability Header
            0x00, 0x01, 0x01, 0x02, // Element self description
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserved
            0x01, 0x00, 0x01, 0x00, // Link entry 1 description
            0x00, 0x00, 0x00, 0x00, // Link entry 1 reserved
            0x00, 0x90, 0xd1, 0xfe, 0x00, 0x00, 0x00, 0x00, // Link entry 1 address
        ];
        let result: RootComplexLinkDeclaration = data[4..].try_into().unwrap();

        let sample = RootComplexLinkDeclaration {
            element_self_description: ElementSelfDescription {
                element_type: ElementType::ConfigurationSpaceElement,
                reserved: 0,
                number_of_link_entries: 1,
                component_id: 0x01,
                port_number: 0x02,
            },
            link_entries: LinkEntries::new(&data[0x10..], 1),
        };
        assert_eq!(sample, result);
    }
}
