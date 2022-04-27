//! PCI Express Root Complex Link Declaration`
//!
//! The PCI Express Root Complex Link Declaration Capability is an optional Capability that is
//! permitted to be implemented by Root Ports, RCiEPs, or RCRBs to declare a Root Complex’s
//! internal topology.


use core::slice;

use snafu::prelude::*;
use heterob::{P4,P5,P6, endianness::{LeBytesInto, FromLeBytes}, bit_numbering::Lsb};



/// Root Complex Link Declaration Error
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum RootComplexLinkDeclarationError {
    #[snafu(display("can't read Element Self Description (4 bytes) from slice"))]
    ElementSelfDescription,
    #[snafu(display("can't read even one entry (4 bytes) from Link Entries"))]
    LinkEntries,
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
        let (start, end) = (0, ElementSelfDescription::BYTES);
        let element_self_description: ElementSelfDescription = slice.get(start..end)
            .and_then(|slice| <[u8; ElementSelfDescription::BYTES]>::try_from(slice).ok())
            .ok_or(RootComplexLinkDeclarationError::ElementSelfDescription)?
            .le_bytes_into();
        let start = LinkEntries::FIRST_ENTRY_OFFSET;
        let end = start +
            (element_self_description.number_of_link_entries as usize) * LinkEntry::BYTES;
        let bytes = slice.get(start..end)
            .ok_or(RootComplexLinkDeclarationError::LinkEntries)?;
        let link_entries = LinkEntries(bytes.iter());

        Ok(Self { element_self_description, link_entries, })
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementSelfDescription {
    /// Element Type
    pub element_type: ElementType,
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
        let Lsb((
            element_type, (), number_of_link_entries, component_id, port_number,
        )) = P5::<_, 4, 4, 8, 8, 8>(dword).into();
        let element_type = From::<u8>::from(element_type);
        Self { element_type, number_of_link_entries, component_id, port_number, }
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


/// Link Entries
#[derive(Debug, Clone)]
pub struct LinkEntries<'a>(pub slice::Iter<'a, u8>);
impl<'a> LinkEntries<'a> {
    pub const FIRST_ENTRY_OFFSET: usize = 0x10 - super::ECH_BYTES;
}
impl<'a> PartialEq for LinkEntries<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.clone().eq(other.0.clone())
    }
}
impl<'a> Eq for LinkEntries<'a> {}
impl<'a> Iterator for LinkEntries<'a> {
    type Item = LinkEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bytes = [0u8; LinkEntry::BYTES];
        for byte in bytes.iter_mut() {
            *byte = *self.0.next()?;
        }
        let dwords: [u32; LinkEntry::DWORDS] = bytes.le_bytes_into();
        Some(dwords.into())
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkEntry {
    /// Link Description
    pub link_description: LinkDescription,
    /// Link Address
    pub link_address: LinkAddress,
}
impl LinkEntry {
    pub const DWORDS: usize = 4;
    pub const BYTES: usize = 16;
}
impl From<[u32; LinkEntry::DWORDS]> for LinkEntry {
    fn from(data: [u32; LinkEntry::DWORDS]) -> Self {
        let link_description: LinkDescription = data[0].into();
        let link_type = link_description.link_type;
        Self {
            link_description,
            link_address: LinkAddress::new(link_type, [data[2],data[3]]),
        }
    }
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
    fn from_le_bytes(bytes: [u8;4]) -> Self { u32::from_le_bytes(bytes).into() }
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
    pub fn new(link_type: u8, data: [u32; LinkAddress::DWORDS]) -> Self {
        const BUS_AND_ADDR_OFFSET: u8 = 20;
        let first_dword = data[0] as u64;
        let second_dword = data[1] as u64;
        let address = (second_dword << 32) | first_dword;
        if link_type == 0 {
            Self::MemoryMappedSpace(address)
        } else {
            let Lsb((
                n, (), function, device
            )) = P4::<_, 3, 9, 3, 5>(first_dword).into();
            // with N = 000b specifying n = 8 and all other encodings specifying n = <value of N>
            let n: u8 = if n == 0 { 8 } else { n };
            // Bits (19 + n):20 specify the Bus Number, with 1 ≤ n ≤ 8
            let bus = ((first_dword >> BUS_AND_ADDR_OFFSET) & !(u64::MAX << n)) as u8;
            let laddr_mask = u64::MAX << (BUS_AND_ADDR_OFFSET + n);
            // Bits 31:(20 + n) of the first DWORD together with the second DWORD
            let address = address & laddr_mask;
            Self::ConfigurationSpace { n, function, device, bus, address }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    use pretty_assertions::assert_eq;
    use super::*;

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
        let data = [ 0b1111_1111_1111_1111_1111_0000_0000_0000u32, 0x33221100];
        let result = LinkAddress::new(0, data);
        let sample = LinkAddress::MemoryMappedSpace(0x33221100FFFFF000);
        assert_eq!(sample, result);
    }

    #[test]
    fn link_address_configuration_space() {
        //           address    bus  dev   fn  reserved  N
        let data = [ 0b11111111_0101_01010_101_000000000_100u32, 0x33221100];
        let result = LinkAddress::new(1, data);
        let sample = LinkAddress::ConfigurationSpace {
            n: 4,
            function: 0b101,
            device: 0b01010,
            bus: 0b0101,
            address: 0x33221100FF000000,
        };
        assert_eq!(sample, result);
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
        let result = LinkEntries(data.iter()).collect::<Vec<_>>();

        let sample = vec![
            LinkEntry {
                link_description: LinkDescription {
                    link_valid: true,
                    link_type: 0,
                    associate_rcrb_header: false,
                    target_component_id: 0x01,
                    target_port_number: 0x00,
                },
                link_address: LinkAddress::MemoryMappedSpace(0x00000000fed19000),
            }
        ];
        assert_eq!(sample, result);
    }
    
    #[test]
    fn parse_full_struct() {
        // Desc:   PortNumber=02 ComponentID=01 EltType=Config
        // Link0:  Desc:   TargetPort=00 TargetComponent=01 AssocRCRB- LinkType=MemMapped LinkValid+
        //         Addr:   00000000fed19000
        let data = [
            0x05, 0x00, 0x01, 0x1c, // Root Complex Link Declaration Extended Capability Header
            0x00, 0x01, 0x01, 0x02, // Element self description
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserved
            0x01, 0x00, 0x01, 0x00, // Link entry 1 description
            0x00, 0x00, 0x00, 0x00, // Link entry 1 reserved
            0x00, 0x90, 0xd1, 0xfe, 0x00, 0x00, 0x00, 0x00, // Link entry 1 address
        ];
        let result: RootComplexLinkDeclaration = data[4..].try_into().unwrap();

        let sample = RootComplexLinkDeclaration{
            element_self_description: ElementSelfDescription {
                element_type: ElementType::ConfigurationSpaceElement,
                number_of_link_entries: 1,
                component_id: 0x01,
                port_number: 0x02,
            },
            link_entries: LinkEntries(data[0x10..].iter()),
        };
        assert_eq!(sample, result);
    }
}
