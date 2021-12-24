//! Base Address Registers
//!
//! Base Address Registers (or BARs) can be used to hold memory addresses used by the device, or
//! offsets for port addresses. Typically, memory address BARs need to be located in physical ram
//! while I/O space BARs can reside at any memory address (even beyond physical memory). To
//! distinguish between them, you can check the value of the lowest bit.

use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddressesNormal(pub [u32; 6]);
impl<'a> TryRead<'a, Endian> for BaseAddressesNormal {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let bar = [
            bytes.read_with::<u32>(offset, endian)?,
            bytes.read_with::<u32>(offset, endian)?,
            bytes.read_with::<u32>(offset, endian)?,
            bytes.read_with::<u32>(offset, endian)?,
            bytes.read_with::<u32>(offset, endian)?,
            bytes.read_with::<u32>(offset, endian)?,
        ];
        Ok((Self(bar), *offset))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddressesBridge(pub [u32; 2]);
impl<'a> TryRead<'a, Endian> for BaseAddressesBridge {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let bar = [
            bytes.read_with::<u32>(offset, endian)?,
            bytes.read_with::<u32>(offset, endian)?,
        ];
        Ok((Self(bar), *offset))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAddressesCardbus(pub [u32; 1]);
impl<'a> TryRead<'a, Endian> for BaseAddressesCardbus {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let bar = [
            bytes.read_with::<u32>(offset, endian)?,
        ];
        Ok((Self(bar), *offset))
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
            let prefetchable = dword & 0b1000 != 0;
            let base_address: u32 = dword & !0b1111;
            let base_address_type = match dword & 0b111 {
                0b000 => {
                    BaseAddressType::MemorySpace32 { 
                        prefetchable,
                        base_address,
                    }
                },
                0b010 => {
                    BaseAddressType::MemorySpace1M { 
                        prefetchable,
                        base_address: base_address as u16,
                    }
                },
                0b100 => {
                    if let Some((_, dword)) = next() {
                        BaseAddressType::MemorySpace64 { 
                            prefetchable,
                            base_address: ((dword as u64) << 32) | (base_address as u64),
                        }
                    } else {
                        BaseAddressType::MemorySpace64Broken
                    }
                },
                0b001 | 0b101 => {
                    BaseAddressType::IoSpace {
                        base_address: dword & !0b11,
                    }
                },
                _ => {
                    BaseAddressType::Reserved
                },
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
    MemorySpace1M {
        prefetchable: bool,
        base_address: u16,
    },
    // 64-bit Memory Space mapping
    MemorySpace64 {
        prefetchable: bool,
        base_address: u64,
    },
    MemorySpace64Broken,
    /// Offset for port addresses
    IoSpace {
        base_address: u32,
    },
    /// bit #1 I/O space, and bits #2/1 values 01 and 11
    Reserved,
}




#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
}
