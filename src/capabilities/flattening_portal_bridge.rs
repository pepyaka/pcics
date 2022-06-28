/*!
# Flattening Portal Bridge

The Flattening Portal Bridge (FPB) is an optional mechanism which can be used
to improve the scalability and runtime reallocation of Routing IDs and Memory
Space resources.

## Struct diagram
<pre>
<a href="struct.FlatteningPortalBridge.html">FlatteningPortalBridge</a>
├─ <a href="struct.FpbCapabilities.html">FpbCapabilities</a>
│  ├─ <a href="enum.FpbRidVectorSizeSupported.html">FpbRidVectorSizeSupported</a>
│  ├─ <a href="enum.FpbMemLowVectorSizeSupported.html">FpbMemLowVectorSizeSupported</a>
│  └─ <a href="enum.FpbMemHighVectorSizeSupported.html">FpbMemHighVectorSizeSupported</a>
├─ <a href="struct.FpbRidVectorControl.html">FpbRidVectorControl</a>
│  └─ <a href="enum.FpbRidVectorGranularity.html">FpbRidVectorGranularity</a>
├─ <a href="struct.FpbMemLowVectorControl.html">FpbMemLowVectorControl</a>
│  └─ <a href="enum.FpbMemLowVectorGranularity.html">FpbMemLowVectorGranularity</a>
├─ <a href="struct.FpbMemHighVectorControl.html">FpbMemHighVectorControl</a>
│  └─ <a href="enum.FpbMemHighVectorGranularity.html">FpbMemHighVectorGranularity</a>
└─ <a href="struct.FpbVectorAccessControl.html">FpbVectorAccessControl</a>
   └─ <a href="enum.FpbVectorSelect.html">FpbVectorSelect</a>
</pre>

## Examples

```rust
# use pcics::capabilities::flattening_portal_bridge::*;
let data = [
    0x15, 0x00, // Header
    0x00, 0x00, // Reserved
    0x55, 0x55, 0x55, 0x55, // FPB Capabilities
    0x55, 0x55, 0x55, 0x55, // FPB RID Vector Control 1
    0x55, 0x55, 0x55, 0x55, // FPB RID Vector Control 2
    0x55, 0x55, 0x55, 0x55, // FPB MEM Low Vector Control
    0x55, 0x55, 0x55, 0x55, // FPB MEM High Vector Control 1
    0x55, 0x55, 0x55, 0x55, // FPB MEM High Vector Control 2
    0x55, 0x55, 0x55, 0x55, // FPB Vector Access Control
    0x67, 0x45, 0x23, 0x01, // FPB Vector Access Data
];
let sample = FlatteningPortalBridge {
    reserved: 0,
    fpb_capabilities: FpbCapabilities {
        fpb_rid_decode_mechanism_supported: true,
        fpb_mem_low_decode_mechanism_supported: false,
        fpb_mem_high_decode_mechanism_supported: true,
        fpb_num_sec_dev: 0b1010,
        fpb_rid_vector_size_supported: FpbRidVectorSizeSupported::Size8kbits,
        fpb_mem_low_vector_size_supported: FpbMemLowVectorSizeSupported::Reserved(5),
        fpb_mem_high_vector_size_supported: FpbMemHighVectorSizeSupported::Size8Kbits,
    },
    fpb_rid_vector_control: FpbRidVectorControl {
        fpb_rid_decode_mechanism_enable: true,
        fpb_rid_vector_granularity: FpbRidVectorGranularity::Granularity256RIDs,
        fpb_rid_vector_start: 0x5500,
        rid_secondary_start: 0x5550 >> 3,
    },
    fpb_mem_low_vector_control: FpbMemLowVectorControl {
        fpb_mem_low_decode_mechanism_enable: true,
        fpb_mem_low_vector_granularity: FpbMemLowVectorGranularity::Reserved(5),
        fpb_mem_low_vector_start: 0x5555_5555 & (u32::MAX << 25),
    },
    fpb_mem_high_vector_control: FpbMemHighVectorControl {
        fpb_mem_high_decode_mechanism_enable: true,
        fpb_mem_high_vector_granularity: FpbMemHighVectorGranularity::Granularity8GB,
        fpb_mem_high_vector_start: 0x5555_5555_5555_5555 & (u64::MAX << 33),
    },
    fpb_vector_access_control: FpbVectorAccessControl {
        fpb_vector_access_offset: 0x55,
        fpb_vector_select: FpbVectorSelect::MemLow,
    },
    fpb_vector_access_data: 0x1234567,
};
let result = data[2..].try_into().unwrap();
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P10, P3, P4, P5, P9};

use super::CapabilityDataError;

/// Flattening Portal Bridge (FPB) Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatteningPortalBridge {
    pub reserved: u16,
    pub fpb_capabilities: FpbCapabilities,
    pub fpb_rid_vector_control: FpbRidVectorControl,
    pub fpb_mem_low_vector_control: FpbMemLowVectorControl,
    pub fpb_mem_high_vector_control: FpbMemHighVectorControl,
    pub fpb_vector_access_control: FpbVectorAccessControl,
    /// Data from the FPB Vector at the location determined by the value in the
    /// [FPB Vector Access Offset](FpbVectorAccessControl::fpb_vector_access_offset)
    pub fpb_vector_access_data: u32,
}

impl FlatteningPortalBridge {
    pub const SIZE: usize = 2 + 8 * 4;
}

impl TryFrom<&[u8]> for FlatteningPortalBridge {
    type Error = CapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        slice
            .try_into()
            .map(|Seq { head, .. }| From::<[u8; Self::SIZE]>::from(head))
            .map_err(|_| CapabilityDataError {
                name: "Flattening Portal Bridge",
                size: Self::SIZE,
            })
    }
}

impl From<[u8; Self::SIZE]> for FlatteningPortalBridge {
    fn from(bytes: [u8; Self::SIZE]) -> Self {
        let Le((
            reserved,
            capabilities,
            rid_vector_control_1,
            rid_vector_control_2,
            mem_low_vector_control,
            mem_high_vector_control_1,
            mem_high_vector_control_2,
            vector_access_control,
            fpb_vector_access_data,
        )) = P9(bytes).into();
        Self {
            reserved,
            fpb_capabilities: From::<u32>::from(capabilities),
            fpb_rid_vector_control: FpbRidVectorControl::new(
                rid_vector_control_1,
                rid_vector_control_2,
            ),
            fpb_mem_low_vector_control: From::<u32>::from(mem_low_vector_control),
            fpb_mem_high_vector_control: FpbMemHighVectorControl::new(
                mem_high_vector_control_1,
                mem_high_vector_control_2,
            ),
            fpb_vector_access_control: From::<u32>::from(vector_access_control),
            fpb_vector_access_data,
        }
    }
}

/// FPB Capabilities Register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpbCapabilities {
    /// Indicates that the FPB RID Vector mechanism is supported
    pub fpb_rid_decode_mechanism_supported: bool,
    /// Indicates that the FPB MEM Low Vector mechanism is supported
    pub fpb_mem_low_decode_mechanism_supported: bool,
    /// Indicates that the FPB Mem High mechanism is supported
    pub fpb_mem_high_decode_mechanism_supported: bool,
    /// For Upstream Ports of Switches only, this field indicates the quantity
    /// of Device Numbers associated with the Secondary Side of the Upstream Port
    /// bridge
    pub fpb_num_sec_dev: u8,
    pub fpb_rid_vector_size_supported: FpbRidVectorSizeSupported,
    pub fpb_mem_low_vector_size_supported: FpbMemLowVectorSizeSupported,
    pub fpb_mem_high_vector_size_supported: FpbMemHighVectorSizeSupported,
}

impl From<u32> for FpbCapabilities {
    fn from(dword: u32) -> Self {
        let Lsb((
            fpb_rid_decode_mechanism_supported,
            fpb_mem_low_decode_mechanism_supported,
            fpb_mem_high_decode_mechanism_supported,
            fpb_num_sec_dev,
            rid,
            (),
            mem_low,
            (),
            mem_high,
            (),
        )) = P10::<_, 1, 1, 1, 5, 3, 5, 3, 5, 3, 5>(dword).into();
        Self {
            fpb_rid_decode_mechanism_supported,
            fpb_mem_low_decode_mechanism_supported,
            fpb_mem_high_decode_mechanism_supported,
            fpb_num_sec_dev,
            fpb_rid_vector_size_supported: From::<u8>::from(rid),
            fpb_mem_low_vector_size_supported: From::<u8>::from(mem_low),
            fpb_mem_high_vector_size_supported: From::<u8>::from(mem_high),
        }
    }
}

/// Size of the FPB RID Vector implemented in hardware, and constrains the
/// allowed values software is permitted to write to the FPB RID Vector Granularity
/// field
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FpbRidVectorSizeSupported {
    /// 256 bits, allowed granularities in RID units: 8, 64, 256
    Size256bits,
    /// 1 K bits, allowed granularities in RID units: 8, 64
    Size1Kbits,
    /// 8 K bits, allowed granularities in RID units: 8
    Size8kbits,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for FpbRidVectorSizeSupported {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Size256bits,
            0b010 => Self::Size1Kbits,
            0b101 => Self::Size8kbits,
            v => Self::Reserved(v),
        }
    }
}

/// Indicates the size of the FPB MEM Low Vector implemented in hardware, and
/// constrains the allowed values software is permitted to write to the FPB MEM Low
/// Vector Start field
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FpbMemLowVectorSizeSupported {
    /// 256 bits, allowed granularities in MB units: 1, 2, 4, 8, 16
    Size256bits,
    /// 512 bits, allowed granularities in MB units: 1, 2, 4, 8
    Size512bits,
    /// 1 K bits, allowed granularities in MB units: 1, 2, 4
    Size1Kbits,
    /// 2 K bits, allowed granularities in MB units: 1, 2
    Size2Kbits,
    /// 4 K bits, allowed granularities in MB units: 1
    Size4Kbits,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for FpbMemLowVectorSizeSupported {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Size256bits,
            0b001 => Self::Size512bits,
            0b010 => Self::Size1Kbits,
            0b011 => Self::Size2Kbits,
            0b100 => Self::Size4Kbits,
            v => Self::Reserved(v),
        }
    }
}

/// Indicates the size of the FPB MEM High Vector implemented in hardware
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FpbMemHighVectorSizeSupported {
    /// 256 bits
    Size256bits,
    /// 512 bits
    Size512bits,
    /// 1 K bits
    Size1Kbits,
    /// 2 K bits
    Size2Kbits,
    /// 4 K bits
    Size4Kbits,
    /// 8 K bits
    Size8Kbits,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for FpbMemHighVectorSizeSupported {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Size256bits,
            0b001 => Self::Size512bits,
            0b010 => Self::Size1Kbits,
            0b011 => Self::Size2Kbits,
            0b100 => Self::Size4Kbits,
            0b101 => Self::Size8Kbits,
            v => Self::Reserved(v),
        }
    }
}

/// FPB RID Vector Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpbRidVectorControl {
    /// Enables the FPB RID Decode mechanism
    pub fpb_rid_decode_mechanism_enable: bool,
    pub fpb_rid_vector_granularity: FpbRidVectorGranularity,
    pub fpb_rid_vector_start: u16,
    /// Controls the RID offset at which Type 1 Configuration Requests passing
    /// downstream through the bridge must be converted to Type 0
    pub rid_secondary_start: u16,
}

impl FpbRidVectorControl {
    fn new(control_1: u32, control_2: u32) -> Self {
        let Lsb((fpb_rid_decode_mechanism_enable, (), granularity, (), start)) =
            P5::<_, 1, 3, 4, 11, 13>(control_1).into();
        let _: u16 = start;
        let Lsb(((), rid_secondary_start, ())) = P3::<_, 3, 13, 16>(control_2).into();
        Self {
            fpb_rid_decode_mechanism_enable,
            fpb_rid_vector_granularity: From::<u8>::from(granularity),
            fpb_rid_vector_start: (start << 3) & u16::MAX.wrapping_shl(3 + granularity as u32),
            rid_secondary_start,
        }
    }
}

/// Controls the granularity and the required alignment of the FPB RID Vector
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FpbRidVectorGranularity {
    /// 8 RIDs
    Granularity8RIDs,
    /// 64 RIDs
    Granularity64RIDs,
    /// 256 RIDs
    Granularity256RIDs,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for FpbRidVectorGranularity {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::Granularity8RIDs,
            0b0011 => Self::Granularity64RIDs,
            0b0101 => Self::Granularity256RIDs,
            v => Self::Reserved(v),
        }
    }
}

/// FPB MEM Low Vector Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpbMemLowVectorControl {
    /// Enables the FPB MEM Low Decode mechanism
    pub fpb_mem_low_decode_mechanism_enable: bool,
    pub fpb_mem_low_vector_granularity: FpbMemLowVectorGranularity,
    pub fpb_mem_low_vector_start: u32,
}

impl From<u32> for FpbMemLowVectorControl {
    fn from(dword: u32) -> Self {
        let Lsb((fpb_mem_low_decode_mechanism_enable, (), granularity, (), start)) =
            P5::<_, 1, 3, 4, 12, 12>(dword).into();
        let _: u32 = start;
        Self {
            fpb_mem_low_decode_mechanism_enable,
            fpb_mem_low_vector_granularity: From::<u8>::from(granularity),
            fpb_mem_low_vector_start: (start << 20)
                & u32::MAX.wrapping_shl(20 + granularity as u32),
        }
    }
}

/// Controls the granularity and the required alignment of the FPB MEM Low Vector
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FpbMemLowVectorGranularity {
    /// 1 MB
    Granularity1MB,
    /// 2 MB
    Granularity2MB,
    /// 4 MB
    Granularity4MB,
    /// 8 MB
    Granularity8MB,
    /// 16 MB
    Granularity16MB,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for FpbMemLowVectorGranularity {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::Granularity1MB,
            0b0001 => Self::Granularity2MB,
            0b0010 => Self::Granularity4MB,
            0b0011 => Self::Granularity8MB,
            0b0100 => Self::Granularity16MB,
            v => Self::Reserved(v),
        }
    }
}

/// FPB MEM High Vector Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpbMemHighVectorControl {
    pub fpb_mem_high_decode_mechanism_enable: bool,
    pub fpb_mem_high_vector_granularity: FpbMemHighVectorGranularity,
    pub fpb_mem_high_vector_start: u64,
}

impl FpbMemHighVectorControl {
    pub fn new(control_1: u32, control_2: u32) -> Self {
        let Lsb((fpb_mem_high_decode_mechanism_enable, (), granularity, (), start)) =
            P5::<_, 1, 3, 4, 20, 4>(control_1).into();
        let _: u64 = start;
        let start_low = start << 28;
        let start_high = (control_2 as u64) << 32;
        let mask = u64::MAX.wrapping_shl(28 + granularity as u32);
        Self {
            fpb_mem_high_decode_mechanism_enable,
            fpb_mem_high_vector_granularity: From::<u8>::from(granularity),
            fpb_mem_high_vector_start: (start_low | start_high) & mask,
        }
    }
}

/// Controls the granularity and the required alignment of the FPB MEM High Vector
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FpbMemHighVectorGranularity {
    /// 256 MB
    Granularity256MB,
    /// 512 MB
    Granularity512MB,
    /// 1 GB
    Granularity1GB,
    /// 2 GB
    Granularity2GB,
    /// 4 GB
    Granularity4GB,
    /// 8 GB
    Granularity8GB,
    /// 16 GB
    Granularity16GB,
    /// 32 GB
    Granularity32GB,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for FpbMemHighVectorGranularity {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::Granularity256MB,
            0b0001 => Self::Granularity512MB,
            0b0010 => Self::Granularity1GB,
            0b0011 => Self::Granularity2GB,
            0b0100 => Self::Granularity4GB,
            0b0101 => Self::Granularity8GB,
            0b0110 => Self::Granularity16GB,
            0b0111 => Self::Granularity32GB,
            v => Self::Reserved(v),
        }
    }
}

/// FPB Vector Access Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpbVectorAccessControl {
    /// Indicates the offset of the DWORD portion of the FPB RID, MEM Low or
    /// MEM High, Vector that can be read or written
    pub fpb_vector_access_offset: u8,
    pub fpb_vector_select: FpbVectorSelect,
}

impl From<u32> for FpbVectorAccessControl {
    fn from(dword: u32) -> Self {
        let Lsb((fpb_vector_access_offset, (), select, ())) = P4::<_, 8, 6, 2, 16>(dword).into();
        Self {
            fpb_vector_access_offset,
            fpb_vector_select: From::<u8>::from(select),
        }
    }
}

/// Selects the Vector to be accessed at the indicated FPB Vector Access Offset
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FpbVectorSelect {
    /// RID
    Rid,
    /// MEM Low
    MemLow,
    /// MEM High
    MemHigh,
    /// Reserved
    Reserved,
}

impl From<u8> for FpbVectorSelect {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::Rid,
            0b01 => Self::MemLow,
            0b10 => Self::MemHigh,
            0b11 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}
