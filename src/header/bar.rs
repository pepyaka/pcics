//! Base Address Registers
//!
//! Base Address Registers (or BARs) can be used to hold memory addresses used by the device, or
//! offsets for port addresses. Typically, memory address BARs need to be located in physical ram
//! while I/O space BARs can reside at any memory address (even beyond physical memory). To
//! distinguish between them, you can check the value of the lowest bit.

use heterob::endianness::LeBytesInto;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddressesNormal(pub [u32; 6]);
impl From<[u8; 6 * 4]> for BaseAddressesNormal {
    fn from(bytes: [u8; 6 * 4]) -> Self {
        Self(bytes.le_bytes_into())
    }
}
impl<'a> FromIterator<&'a BaseAddress> for BaseAddressesNormal {
    fn from_iter<I: IntoIterator<Item = &'a BaseAddress>>(iter: I) -> Self {
        let mut dwords = [0;6];
        for ba in iter.into_iter() {
            let i = ba.region;
            match ba.base_address_type {
                BaseAddressType::MemorySpace32 { prefetchable, base_address, } => {
                    dwords[i] = base_address & !0b1111 | ((prefetchable as u32) << 3);
                },
                BaseAddressType::MemorySpaceBelow1M { prefetchable, base_address, } => {
                    dwords[i] = (base_address as u32) & !0b1111 | 0b010 | ((prefetchable as u32) << 3);
                },
                BaseAddressType::MemorySpace64 { prefetchable, base_address, } => {
                    dwords[i] = (base_address as u32) & !0b1111 | 0b100 | ((prefetchable as u32) << 3);
                    dwords[i + 1] = (base_address >> 32) as u32;
                },
                BaseAddressType::IoSpace { base_address, } => {
                    dwords[i] = base_address & !0b11 | 0b01;
                },
                _ => (),
            }

        }
        Self(dwords)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddressesBridge(pub [u32; 2]);
impl From<[u8; 2 * 4]> for BaseAddressesBridge {
    fn from(bytes: [u8; 2 * 4]) -> Self {
        Self(bytes.le_bytes_into())
    }
}
impl<'a> FromIterator<&'a BaseAddress> for BaseAddressesBridge {
    fn from_iter<I: IntoIterator<Item = &'a BaseAddress>>(iter: I) -> Self {
        let mut dwords = [0;2];
        for ba in iter.into_iter() {
            let i = ba.region;
            match ba.base_address_type {
                BaseAddressType::MemorySpace32 { prefetchable, base_address, } => {
                    dwords[i] = base_address & !0b1111 | ((prefetchable as u32) << 3);
                },
                BaseAddressType::MemorySpaceBelow1M { prefetchable, base_address, } => {
                    dwords[i] = (base_address as u32) & !0b1111 | 0b010 | ((prefetchable as u32) << 3);
                },
                BaseAddressType::MemorySpace64 { prefetchable, base_address, } => {
                    dwords[i] = (base_address as u32) & !0b1111 | 0b100 | ((prefetchable as u32) << 3);
                    dwords[i + 1] = (base_address >> 32) as u32;
                },
                BaseAddressType::IoSpace { base_address, } => {
                    dwords[i] = base_address & !0b11 | 0b01;
                },
                _ => (),
            }

        }
        Self(dwords)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddressesCardbus(pub [u32; 1]);
impl From<[u8; 1 * 4]> for BaseAddressesCardbus {
    fn from(bytes: [u8; 1 * 4]) -> Self {
        Self(bytes.le_bytes_into())
    }
}
impl<'a> FromIterator<&'a BaseAddress> for BaseAddressesCardbus {
    fn from_iter<I: IntoIterator<Item = &'a BaseAddress>>(iter: I) -> Self {
        let mut dwords = [0;1];
        for ba in iter.into_iter() {
            let i = ba.region;
            match ba.base_address_type {
                BaseAddressType::MemorySpace32 { prefetchable, base_address, } => {
                    dwords[i] = base_address & !0b1111 | ((prefetchable as u32) << 3);
                },
                BaseAddressType::MemorySpaceBelow1M { prefetchable, base_address, } => {
                    dwords[i] = (base_address as u32) & !0b1111 | 0b010 | ((prefetchable as u32) << 3);
                },
                BaseAddressType::IoSpace { base_address, } => {
                    dwords[i] = base_address & !0b11 | 0b01;
                },
                _ => (),
            }

        }
        Self(dwords)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BaseAddressesType {
    Normal([u32; 6]),
    Bridge([u32; 2]),
    Cardbus([u32;1]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddresses {
    bat: BaseAddressesType,
    region: usize,
}
impl From<BaseAddressesNormal> for BaseAddresses {
    fn from(ba: BaseAddressesNormal) -> Self {
        Self { bat: BaseAddressesType::Normal(ba.0), region: 0 }
    }
}
impl From<BaseAddressesBridge> for BaseAddresses {
    fn from(ba: BaseAddressesBridge) -> Self {
        Self { bat: BaseAddressesType::Bridge(ba.0), region: 0 }
    }
}
impl From<BaseAddressesCardbus> for BaseAddresses {
    fn from(ba: BaseAddressesCardbus) -> Self {
        Self { bat: BaseAddressesType::Cardbus(ba.0), region: 0 }
    }
}
impl Iterator for BaseAddresses {
    type Item = BaseAddress;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = || -> Option<(usize, u32)> {
            let region = self.region;
            let dword = match (self.bat.clone(), region) {
                (BaseAddressesType::Normal(a), 0..=5) => a[region],
                (BaseAddressesType::Bridge(a), 0..=1) => a[region],
                (BaseAddressesType::Cardbus(a), 0) => a[region],
                _ => return None,
            };
            self.region += 1;
            Some((region, dword))
        };
        loop {
            let (region, dword) = next()?;
            if dword == 0 {
                continue;
            }
            let is_io_space = (dword & 0b1) != 0;
            let base_address_type = if is_io_space {
                BaseAddressType::IoSpace {
                    base_address: dword & !0b11,
                }
            } else {
                let prefetchable = (dword & 0b1000) != 0;
                let base_address: u32 = dword & !0b1111;
                match dword & 0b110 {
                    0b000 => {
                        BaseAddressType::MemorySpace32 {
                            prefetchable,
                            base_address,
                        }
                    },
                    0b010 => {
                        BaseAddressType::MemorySpaceBelow1M {
                            prefetchable,
                            base_address,
                        }
                    },
                    0b100 => if let Some((_, dword)) = next() {
                        BaseAddressType::MemorySpace64 {
                            prefetchable,
                            base_address: ((dword as u64) << 32) | (base_address as u64),
                        }
                    } else {
                        BaseAddressType::MemorySpace64Broken {
                            prefetchable,
                        }
                    },
                    _ => {
                        BaseAddressType::MemorySpaceReserved {
                            prefetchable,
                            base_address,
                        }
                    },
                }
            };
            return Some(BaseAddress { region, base_address_type });
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddress {
    pub region: usize,
    pub base_address_type: BaseAddressType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseAddressType {
    /// 32-bit Memory Space mapping
    MemorySpace32 {
        prefetchable: bool,
        base_address: u32,
    },
    /// In earlier versions it was used to support memory space below 1MB (16-bit wide base
    /// register that can be mapped anywhere in the 16-bit Memory Space).
    MemorySpaceBelow1M {
        prefetchable: bool,
        base_address: u32,
    },
    // 64-bit Memory Space mapping
    MemorySpace64 {
        prefetchable: bool,
        base_address: u64,
    },
    /// bit #1 I/O space, and bits #2/1 values 01 and 11
    MemorySpaceReserved {
        prefetchable: bool,
        base_address: u32,
    },
    /// 64-bit memory space should be aligned on two 32-bit registers
    MemorySpace64Broken {
        prefetchable: bool,
    },
    /// Offset for port addresses
    IoSpace {
        base_address: u32,
    },
}




#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    use pretty_assertions::assert_eq;
    use heterob::endianness::LeBytesInto;
    use super::*;

    #[test]
    fn iter_once() {
        let sample = vec![
            BaseAddress {
                region: 0,
                base_address_type: BaseAddressType::MemorySpace32 {
                    prefetchable: false, base_address: 0xb3000000
                },
            },
        ];
        let result: BaseAddresses = BaseAddressesNormal([0xb3000000, 0, 0, 0, 0, 0]).into();
        assert_eq!(sample, result.collect::<Vec<_>>());
    }

    #[test]
    fn iter_multiple() {
        // Region 0: Memory at b3000000 (32-bit, non-prefetchable)
        // Region 1: Memory at a0000000 (64-bit, prefetchable)
        // Region 3: Memory at 3bffff1c000 (64-bit, non-prefetchable)
        // Region 5: I/O ports at 3000
        let sample = vec![
            BaseAddress {
                region: 0,
                base_address_type: BaseAddressType::MemorySpace32 {
                    prefetchable: false,
                    base_address: 0xb3000000,
                },
            },
            BaseAddress {
                region: 1,
                base_address_type: BaseAddressType::MemorySpace64 {
                    prefetchable: true,
                    base_address: 0xa0000000
                },
            },
            BaseAddress {
                region: 3,
                base_address_type: BaseAddressType::MemorySpace64 {
                    prefetchable: false,
                    base_address: 0x3bffff1c000
                },
            },
            BaseAddress {
                region: 5,
                base_address_type: BaseAddressType::IoSpace {
                    base_address: 0x3000,
                },
            },
        ];
        let result: BaseAddresses =
            BaseAddressesNormal([
                0xb3000000,
                0xa000000c,
                0,
                0xfff1c004,
                0x000003bf,
                0x00003001,
            ]).into();
        assert_eq!(sample, result.collect::<Vec<_>>());
    }

    #[test]
    fn base_address_normal_from_iterator() {
        // Region 0: Memory at 7bd12000 (32-bit, non-prefetchable) [size=8K]
        // Region 1: Memory at 7bd1d000 (32-bit, non-prefetchable) [size=256]
        // Region 2: I/O ports at 2040 [size=8]
        // Region 3: I/O ports at 2048 [size=4]
        // Region 4: I/O ports at 2020 [size=32]
        // Region 5: Memory at 7bd1c000 (32-bit, non-prefetchable) [size=2K]
        let bytes = [
            0x00, 0x20, 0xd1, 0x7b,
            0x00, 0xd0, 0xd1, 0x7b,
            0x41, 0x20, 0x00, 0x00,
            0x49, 0x20, 0x00, 0x00,
            0x21, 0x20, 0x00, 0x00,
            0x00, 0xc0, 0xd1, 0x7b
        ];

        let result: BaseAddressesNormal = [
            BaseAddress {
                region: 0,
                base_address_type: BaseAddressType::MemorySpace32 {
                    prefetchable: false, base_address: 0x7bd12000
                },
            },
            BaseAddress {
                region: 1,
                base_address_type: BaseAddressType::MemorySpace32 {
                    prefetchable: false, base_address: 0x7bd1d000
                },
            },
            BaseAddress {
                region: 2,
                base_address_type: BaseAddressType::IoSpace {
                    base_address: 0x2040
                },
            },
            BaseAddress {
                region: 3,
                base_address_type: BaseAddressType::IoSpace {
                    base_address: 0x2048
                },
            },
            BaseAddress {
                region: 4,
                base_address_type: BaseAddressType::IoSpace {
                    base_address: 0x2020
                },
            },
            BaseAddress {
                region: 5,
                base_address_type: BaseAddressType::MemorySpace32 {
                    prefetchable: false, base_address: 0x7bd1c000
                },
            },
        ].iter().collect();
        let dwords: [u32; 6] = bytes.le_bytes_into();

        assert_eq!(BaseAddressesNormal(dwords), result);
    }

    #[test]
    fn base_address_bridge_from_iterator() {
        // Region 0: Memory at fce12000 (32-bit, non-prefetchable) [size=4K]
        let bytes = [ 0x00, 0x20, 0xe1, 0xfc, 0x00, 0x00, 0x00, 0x00 ];

        let result: BaseAddressesBridge = [
            BaseAddress {
                region: 0,
                base_address_type: BaseAddressType::MemorySpace32 {
                    prefetchable: false, base_address: 0xfce12000
                },
            },
        ].iter().collect();
        let dwords: [u32; 2] = bytes.le_bytes_into();

        assert_eq!(BaseAddressesBridge(dwords), result);
    }
}
