//! Single Root I/O Virtualization (SR-IOV)
//!
//! Single Root I/O Virtualization and Sharing (SR-IOV) consists of extensions to the PCI Express
//! (PCIe) specification suite to enable multiple System Images (SI) to share PCI hardware
//! resources. 

use core::mem::size_of;

use snafu::prelude::*;
use heterob::{P16,P5,P6,P1, endianness::{FromLeBytes, LeBytesInto}, bit_numbering::Lsb};

use crate::header::BaseAddressesNormal;
use super::ExtendedCapabilityDataError;


/// Single Root I/O Virtualization Error
#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum SingleRootIoVirtualizationError {
    #[snafu(display("can't read Element Self Description (4 bytes) from slice"))]
    ElementSelfDescription,
    #[snafu(display("can't read even one entry (4 bytes) from Link Entries"))]
    LinkEntries,
}


/// Single Root I/O Virtualization (SR-IOV)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleRootIoVirtualization {
    /// SR-IOV Capabilities
    pub sriov_capabilities: SriovCapabilities,
    /// SR-IOV Control
    pub sriov_control: SriovControl,
    /// SR-IOV Status
    pub sriov_status: SriovStatus,
    /// InitialVFs (RO)
    pub initial_vfs: u16,
    /// TotalVFs (RO)
    pub total_vfs: u16,
    /// NumVFs (RW)
    pub num_vfs: u16,
    /// Function Dependency Link (RO)
    pub function_dependency_link: u8,
    /// First VF Offset (RO)
    pub first_vf_offset: u16,
    /// VF Stride (RO)
    pub vf_stride: u16,
    /// VF Device ID (RO)
    pub vf_device_id: u16,
    pub page_sizes: PageSizes,
    pub base_addresses: BaseAddressesNormal,
    /// VF Migration State Array Offset (RO)
    pub vf_migration_state_array_offset: u32,
}
impl SingleRootIoVirtualization {
    pub const BYTES: usize = 0x40 - super::ECH_BYTES;
}


impl From<[u8; SingleRootIoVirtualization::BYTES]> for SingleRootIoVirtualization {
    fn from(bytes: [u8; Self::BYTES]) -> Self {
        let P16((
            sriov_capabilities, sriov_control, sriov_status, initial_vfs, total_vfs, num_vfs,
            function_dependency_link, rsvdp_0, first_vf_offset, vf_stride, rsvdp_1, vf_device_id,
            supported_page_sizes, system_page_sizes, base_addresses, vf_migration_state_array_offset,
        )) = bytes.le_bytes_into();
        let _: (u8, u16, [u8; size_of::<u32>() * 6]) = (rsvdp_0, rsvdp_1, base_addresses);
        Self {
            sriov_capabilities,
            sriov_control,
            sriov_status,
            initial_vfs,
            total_vfs,
            num_vfs,
            function_dependency_link,
            first_vf_offset,
            vf_stride,
            vf_device_id,
            page_sizes: PageSizes {
                supported: supported_page_sizes,
                system: system_page_sizes
            },
            base_addresses: BaseAddressesNormal(base_addresses.le_bytes_into()),
            vf_migration_state_array_offset,
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for SingleRootIoVirtualization {
    type Error = ExtendedCapabilityDataError;
    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        slice.get(..Self::BYTES)
            .and_then(|slice| <[u8; Self::BYTES]>::try_from(slice).ok())
            .ok_or(ExtendedCapabilityDataError {
                name: "Single Root I/O Virtualization",
                size: Self::BYTES
            })
            .map(Self::from)
    }
}

/// SR-IOV Capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SriovCapabilities {
    /// VF Migration Capable
    pub vf_migration_capable: bool,
    /// ARI Capable Hierarchy Preserved
    pub ari_capable_hierarchy_preserved: bool,
    /// VF 10-Bit Tag Requester Supported
    pub vf_10bit_tag_requester_supported: bool,
    /// VF Migration Interrupt Message Number
    pub vf_migration_interrupt_message_number: u16,
}
impl From<u32> for SriovCapabilities {
    fn from(dword: u32) -> Self {
        let Lsb((
            vf_migration_capable,
            ari_capable_hierarchy_preserved,
            vf_10bit_tag_requester_supported,
            (),
            vf_migration_interrupt_message_number,
        )) = P5::<_, 1, 1, 1, 18, 11>(dword).into();
        Self {
            vf_migration_capable,
            ari_capable_hierarchy_preserved,
            vf_10bit_tag_requester_supported,
            vf_migration_interrupt_message_number,
        }
    }
}
impl FromLeBytes<4> for SriovCapabilities {
    fn from_le_bytes(bytes: [u8;4]) -> Self { u32::from_le_bytes(bytes).into() }
}

/// SR-IOV Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SriovControl {
    /// VF Enable
    pub vf_enable: bool,
    /// VF Migration Enable
    pub vf_migration_enable: bool,
    /// VF Migration Interrupt Enable
    pub vf_migration_interrupt_enable: bool,
    /// VF MSE
    pub vf_mse: bool,
    /// ARI Capable Hierarchy – PCI Express Endpoint
    pub ari_capable_hierarchy: bool,
    /// VF 10-Bit Tag Requester Enable
    pub vf_10bit_tag_requester_enable: bool,
}
impl From<u16> for SriovControl {
    fn from(word: u16) -> Self {
        let Lsb((
            vf_enable,
            vf_migration_enable,
            vf_migration_interrupt_enable,
            vf_mse,
            ari_capable_hierarchy,
            vf_10bit_tag_requester_enable,
        )) = P6::<_, 1, 1, 1, 1, 1, 1>(word).into();
        Self {
            vf_enable,
            vf_migration_enable,
            vf_migration_interrupt_enable,
            vf_mse,
            ari_capable_hierarchy,
            vf_10bit_tag_requester_enable,
        }
    }
}
impl FromLeBytes<2> for SriovControl {
    fn from_le_bytes(bytes: [u8;2]) -> Self { u16::from_le_bytes(bytes).into() }
}

/// SR-IOV Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SriovStatus {
    /// VF Migration Status
    pub vf_migration_status: bool,
}
impl From<u16> for SriovStatus {
    fn from(word: u16) -> Self {
        let Lsb((
            vf_migration_status,
        )) = P1::<_, 1>(word).into();
        Self {
            vf_migration_status,
        }
    }
}
impl FromLeBytes<2> for SriovStatus {
    fn from_le_bytes(bytes: [u8;2]) -> Self { u16::from_le_bytes(bytes).into() }
}


/// Defines the page size the system will use to map the VFs’ memory addresses
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageSizes {
    /// Supported Page Sizes (RO)
    pub supported: u32,
    /// System Page Size (RW)
    pub system: u32,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::{BaseAddress, BaseAddressType};
    use pretty_assertions::assert_eq;

    // IOVCap: Migration-, Interrupt Message Number: 000
    // IOVCtl: Enable- Migration- Interrupt- MSE- ARIHierarchy-
    // IOVSta: Migration-
    // Initial VFs: 8, Total VFs: 8, Number of VFs: 0, Function Dependency Link: 00
    // VF offset: 384, stride: 4, Device ID: 1520
    // Supported Page Size: 00000553, System Page Size: 00000001
    // Region 0: Memory at 00000000a0180000 (64-bit, prefetchable)
    // Region 3: Memory at 00000000a01a0000 (64-bit, prefetchable)
    const DATA: [u8; 60] = [
                            0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0x08,0x00,
        0x00,0x00,0x00,0x00,0x80,0x01,0x04,0x00,0x00,0x00,0x20,0x15,0x53,0x05,0x00,0x00,
        0x01,0x00,0x00,0x00,0x0c,0x00,0x18,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x0c,0x00,0x1a,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
    ];

    #[test]
    fn parse_full_struct() {
        let result: SingleRootIoVirtualization = DATA.into();
    
        let sample = SingleRootIoVirtualization {
            sriov_capabilities: SriovCapabilities {
                vf_migration_capable: false,
                ari_capable_hierarchy_preserved: true,
                vf_10bit_tag_requester_supported: false,
                vf_migration_interrupt_message_number: 0,
            },
            sriov_control: SriovControl {
                vf_enable: false,
                vf_migration_enable: false,
                vf_migration_interrupt_enable: false,
                vf_mse: false,
                ari_capable_hierarchy: false,
                vf_10bit_tag_requester_enable: false,
            },
            sriov_status: SriovStatus {
                vf_migration_status: false
            },
            initial_vfs: 8,
            total_vfs: 8,
            num_vfs: 0,
            function_dependency_link: 0,
            first_vf_offset: 384,
            vf_stride: 4,
            vf_device_id: 0x1520,
            page_sizes: PageSizes {
                supported: 0x00000553,
                system: 0x00000001,
            },
            base_addresses: [
                BaseAddress {
                    region: 0,
                    base_address_type: BaseAddressType::MemorySpace64 {
                        prefetchable: true, base_address: 0xa0180000
                    }, 
                },
                BaseAddress {
                    region: 3,
                    base_address_type: BaseAddressType::MemorySpace64 {
                        prefetchable: true, base_address: 0xa01a0000
                    }, 
                },
            ].iter().collect::<BaseAddressesNormal>().into(),
            vf_migration_state_array_offset: 0,
        };
        
        assert_eq!(sample, result);
    }
}
