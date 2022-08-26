/*!
# PCIe DVSEC for CXL Device

## Struct diagram
<pre>
<a href="struct.PcieDvsecForCxlDevice.html">PcieDvsecForCxlDevice</a>
├─ <a href="struct.CxlCapability.html">CxlCapability</a>
│  ├─ <a href="enum.HdmCount.html">HdmCount</a>
│  └─ <a href="enum.CxlResetTimeout.html">CxlResetTimeout</a>
├─ <a href="struct.CxlControl.html">CxlControl</a>
│  ├─ <a href="enum.CacheSfCoverage.html">CacheSfCoverage</a>
│  ├─ <a href="enum.CacheSfGranularity.html">CacheSfGranularity</a>
│  └─ <a href="enum.CacheCleanEviction.html">CacheCleanEviction</a>
├─ <a href="struct.CxlStatus.html">CxlStatus</a>
├─ <a href="struct.CxlControl2.html">CxlControl2</a>
├─ <a href="struct.CxlStatus2.html">CxlStatus2</a>
├─ <a href="struct.CxlLock.html">CxlLock</a>
├─ <a href="struct.CxlCapability2.html">CxlCapability2</a>
│  └─ <a href="enum.CacheSizeUnit.html">CacheSizeUnit</a>
├─ <a href="struct.CxlRangeSize.html">CxlRangeSize x 2</a>
│  ├─ <a href="enum.MediaType.html">MediaType</a>
│  ├─ <a href="enum.MemoryClass.html">MemoryClass</a>
│  ├─ <a href="enum.DesiredInterleave.html">DesiredInterleave</a>
│  └─ <a href="enum.MemoryActiveTimeout.html">MemoryActiveTimeout</a>
└─ <a href="struct.CxlRangeBase.html">CxlRangeBase x 2</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::designated_vendor_specific_extended_capability::{
#     Dvsec, DvsecType,
#     compute_express_link::{
#         ComputeExpressLink,
#         pcie_dvsec_for_cxl_device::*
#     }
# };
let data = [
    /* 00h */ 0x23, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x98, 0x1E, 0x81, 0x03, // Designated Vendor-Specific Header 1
    /* 08h */ 0x00, 0x00,             // Designated Vendor-Specific Header 2
              0x55, 0xAA,             // CXL Capability
    /* 0Ch */ 0x55, 0x55,             // CXL Control
              0x00, 0x40,             // CXL Status
    /* 10h */ 0x0A, 0x00,             // CXL Control2
              0x0A, 0x80,             // CXL Status2
    /* 14h */ 0xFE, 0xFF,             // CXL Lock
              0x01, 0xAA,             // CXL Capability2
    /* 18h */ 0x11, 0x22, 0x33, 0x44, // Range 1 Size High
    /* 1Ch */ 0x0A, 0x44, 0x00, 0x00, // Range 1 Size Low
    /* 20h */ 0x55, 0x66, 0x77, 0x88, // Range 1 Base High
    /* 24h */ 0x00, 0x00, 0x00, 0x40, // Range 1 Base Low
    /* 28h */ 0x99, 0xAA, 0xBB, 0xCC, // Range 2 Size High
    /* 2Ch */ 0x55, 0x05, 0x05, 0x85, // Range 2 Size Low
    /* 30h */ 0xDD, 0xEE, 0xFF, 0x00, // Range 2 Base High
    /* 34h */ 0x00, 0x00, 0x00, 0xC0, // Range 2 Base Low
];

let result: Dvsec = data.as_slice().try_into().unwrap();

let sample = PcieDvsecForCxlDevice {
    cxl_capability: CxlCapability {
        cache_capable: true,
        io_capable: false,
        mem_capable: true,
        mem_hwinit_mode: false,
        hdm_count: HdmCount::OneHdmRange,
        cache_writeback_and_invalidate_capable: true,
        cxl_reset_capable: false,
        cxl_reset_timeout: CxlResetTimeout::MaxTime1s,
        cxl_reset_mem_clr_capable: true,
        multiple_logical_device: true,
        viral_capable: false,
        pm_init_completion_reporting_capable: true,
    },
    cxl_control: CxlControl {
        cache_enable: true,
        io_enable: false,
        mem_enable: true,
        cache_sf_coverage: CacheSfCoverage::SnoopFilter(1 << (0b01010 + 15)),
        cache_sf_granularity: CacheSfGranularity::Tracking2KB,
        cache_clean_eviction: CacheCleanEviction::Needed,
        viral_enable: true,
    },
    cxl_status: CxlStatus { viral_status: true },
    cxl_control2: CxlControl2 {
        disable_caching: false,
        initiate_cache_write_back_and_invalidation: true,
        initiate_cxl_reset: false,
        cxl_reset_mem_clr_enable: true,
    },
    cxl_status2: CxlStatus2 {
        cache_invalid: false,
        cxl_reset_complete: true,
        cxl_reset_error: false,
        power_management_initialization_complete: true,
    },
    cxl_lock: CxlLock { config_lock: false },
    cxl_capability2: CxlCapability2 {
        cache_size_unit: CacheSizeUnit::Unit64K,
        cache_size: 0xAA,
    },
    cxl_range_1_size: CxlRangeSize {
        memory_info_valid: false,
        memory_active: true,
        media_type: MediaType::Cdat,
        memory_class: MemoryClass::Memory,
        desired_interleave: DesiredInterleave::Granularity1KB,
        memory_active_timeout: MemoryActiveTimeout::MaxTime16s,
        memory_size: 0x4433221100000000,
    },
    cxl_range_1_base: CxlRangeBase {
        memory_base: 0x8877665540000000,
    },
    cxl_range_2_size: CxlRangeSize {
        memory_info_valid: true,
        memory_active: false,
        media_type: MediaType::Reserved(0b101),
        memory_class: MemoryClass::Cdat,
        desired_interleave: DesiredInterleave::Granularity2KB,
        memory_active_timeout: MemoryActiveTimeout::MaxTime1s,
        memory_size: 0xCCBBAA9980000000,
    },
    cxl_range_2_base: CxlRangeBase {
        memory_base: 0xFFEEDDC0000000,
    },
};
let sample = Dvsec {
    dvsec_vendor_id: 0x1e98,
    dvsec_revision: 1,
    dvsec_length: 0x38,
    dvsec_id: 0,
    dvsec_type: DvsecType::ComputeExpressLink(
        ComputeExpressLink::PcieDvsecForCxlDevice(sample)
    ),
};

assert_eq!(sample, result);
*/

use heterob::{bit_numbering::Lsb, Bool, P13, P2, P3, P5, P8, P9, U8};

/// PCIe DVSEC for CXL Device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PcieDvsecForCxlDevice {
    pub cxl_capability: CxlCapability,
    pub cxl_control: CxlControl,
    pub cxl_status: CxlStatus,
    pub cxl_control2: CxlControl2,
    pub cxl_status2: CxlStatus2,
    pub cxl_lock: CxlLock,
    pub cxl_capability2: CxlCapability2,
    pub cxl_range_1_size: CxlRangeSize,
    pub cxl_range_1_base: CxlRangeBase,
    pub cxl_range_2_size: CxlRangeSize,
    pub cxl_range_2_base: CxlRangeBase,
}

impl PcieDvsecForCxlDevice {
    pub const SIZE: usize = CxlCapability::SIZE
        + CxlControl::SIZE
        + CxlStatus::SIZE
        + CxlControl2::SIZE
        + CxlStatus2::SIZE
        + CxlLock::SIZE
        + CxlCapability2::SIZE
        + CxlRangeSize::SIZE * 2
        + CxlRangeBase::SIZE * 2;
}

/// DVSEC CXL Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlCapability {
    /// Indicates CXL.cache protocol support when operating in Flex Bus.CXL mode
    pub cache_capable: bool,
    /// Indicates CXL.io protocol support when operating in Flex Bus.CXL mode
    pub io_capable: bool,
    /// Indicates CXL.mem protocol support when operating in Flex Bus.CXL mode
    pub mem_capable: bool,
    /// Indicates this CXL.mem capable device initializes memory with
    /// Assistance from hardware and firmware located on the device
    pub mem_hwinit_mode: bool,
    pub hdm_count: HdmCount,
    /// Indicates the device implements Disable Caching and Initiate Cache
    /// Write Back and Invalidation control bits in CXL Control2 register and Cache
    /// Invalid status bit in CXL Status2 register
    pub cache_writeback_and_invalidate_capable: bool,
    /// Indicates the device supports CXL Reset and implements the CXL Reset Timeout field
    pub cxl_reset_capable: bool,
    pub cxl_reset_timeout: CxlResetTimeout,
    /// Device is capable of clearing or randomizing of volatile HDM Ranges during CLX Reset
    pub cxl_reset_mem_clr_capable: bool,
    /// Indicates this is a Logical Device in an MLD, including the FM owned LD
    pub multiple_logical_device: bool,
    /// Indicates CXL device supports Viral handling
    pub viral_capable: bool,
    /// Indicates that the CXL device is capable of supporting Power Management
    /// Initialization Complete flag
    pub pm_init_completion_reporting_capable: bool,
}

impl CxlCapability {
    pub const SIZE: usize = 2;
}

impl From<u16> for CxlCapability {
    fn from(word: u16) -> Self {
        let Lsb((
            cache_capable,
            io_capable,
            mem_capable,
            mem_hwinit_mode,
            U8(hdm_count),
            cache_writeback_and_invalidate_capable,
            cxl_reset_capable,
            U8(cxl_reset_timeout),
            cxl_reset_mem_clr_capable,
            (),
            multiple_logical_device,
            viral_capable,
            pm_init_completion_reporting_capable,
        )) = P13::<_, 1, 1, 1, 1, 2, 1, 1, 3, 1, 1, 1, 1, 1>(word).into();
        Self {
            cache_capable,
            io_capable,
            mem_capable,
            mem_hwinit_mode,
            hdm_count,
            cache_writeback_and_invalidate_capable,
            cxl_reset_capable,
            cxl_reset_timeout,
            cxl_reset_mem_clr_capable,
            multiple_logical_device,
            viral_capable,
            pm_init_completion_reporting_capable,
        }
    }
}

/// Number of HDM ranges implemented by the CXL device and reported through this function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdmCount {
    /// Zero ranges
    ZeroRanges,
    /// One HDM range
    OneHdmRange,
    /// Two HDM ranges
    TwoHdmRanges,
    /// Reserved
    Reserved,
}

impl From<u8> for HdmCount {
    fn from(byte: u8) -> Self {
        match byte {
            0b00 => Self::ZeroRanges,
            0b01 => Self::OneHdmRange,
            0b10 => Self::TwoHdmRanges,
            0b11 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

/// Indicates the maximum time that the device may take to complete the CXL Reset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CxlResetTimeout {
    /// 10 ms
    MaxTime10ms,
    /// 100 ms
    MaxTime100ms,
    /// 1 s
    MaxTime1s,
    /// 10 s
    MaxTime10s,
    /// 100 s
    MaxTime100s,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for CxlResetTimeout {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::MaxTime10ms,
            0b001 => Self::MaxTime100ms,
            0b010 => Self::MaxTime1s,
            0b011 => Self::MaxTime10s,
            0b100 => Self::MaxTime100s,
            v => Self::Reserved(v),
        }
    }
}

/// DVSEC CXL Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlControl {
    /// Enables CXL.cache protocol operation when in Flex Bus.CXL mode
    pub cache_enable: bool,
    /// Enables CXL.io protocol operation when in Flex Bus.CXL mode
    pub io_enable: bool,
    /// Enables CXL.mem protocol operation when in Flex Bus.CXL mode
    pub mem_enable: bool,
    pub cache_sf_coverage: CacheSfCoverage,
    pub cache_sf_granularity: CacheSfGranularity,
    pub cache_clean_eviction: CacheCleanEviction,
    /// Enables Viral handling in the CXL device
    pub viral_enable: bool,
}

impl CxlControl {
    pub const SIZE: usize = 2;
}

impl From<u16> for CxlControl {
    fn from(word: u16) -> Self {
        let Lsb((
            cache_enable,
            io_enable,
            mem_enable,
            U8(cache_sf_coverage),
            U8(cache_sf_granularity),
            Bool(cache_clean_eviction),
            (),
            viral_enable,
            (),
        )) = P9::<_, 1, 1, 1, 5, 3, 1, 2, 1, 1>(word).into();
        Self {
            cache_enable,
            io_enable,
            mem_enable,
            cache_sf_coverage,
            cache_sf_granularity,
            cache_clean_eviction,
            viral_enable,
        }
    }
}

/// Indicates Snoop Filter coverage on the Host
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheSfCoverage {
    /// No Snoop Filter coverage on the Host
    NoSnoopFilter,
    /// Snoop Filter coverage on the Host of Bytes
    SnoopFilter(u64),
}

impl From<u8> for CacheSfCoverage {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::NoSnoopFilter,
            v @ 1..=31 => Self::SnoopFilter(1 << (v + 15)),
            _ => unreachable!(),
        }
    }
}

impl From<CacheSfCoverage> for u8 {
    fn from(c: CacheSfCoverage) -> Self {
        match c {
            CacheSfCoverage::NoSnoopFilter => 0,
            CacheSfCoverage::SnoopFilter(v) => v.trailing_zeros().wrapping_sub(15) as u8,
        }
    }
}

/// Indicates granularity tracking on the Host
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheSfGranularity {
    /// 64B granular tracking on the Host
    Tracking64B,
    /// 128B granular tracking on the Host
    Tracking128B,
    /// 256B granular tracking on the Host
    Tracking256B,
    /// 512B granular tracking on the Host
    Tracking512B,
    /// 1KB granular tracking on the Host
    Tracking1KB,
    /// 2KB granular tracking on the Host
    Tracking2KB,
    /// 4KB granular tracking on the Host
    Tracking4KB,
    Reserved,
}

impl From<u8> for CacheSfGranularity {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Tracking64B,
            0b001 => Self::Tracking128B,
            0b010 => Self::Tracking256B,
            0b011 => Self::Tracking512B,
            0b100 => Self::Tracking1KB,
            0b101 => Self::Tracking2KB,
            0b110 => Self::Tracking4KB,
            0b111 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

/// Indicates necessity of clean evictions from device caches for best performance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheCleanEviction {
    /// Clean evictions from device caches are needed for best performance
    Needed,
    /// Clean evictions from device caches are NOT needed for best performance
    NotNeeded,
}

impl From<bool> for CacheCleanEviction {
    fn from(b: bool) -> Self {
        if b {
            Self::NotNeeded
        } else {
            Self::Needed
        }
    }
}

/// DVSEC CXL Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlStatus {
    /// Indicates that the CXL device has entered Viral
    pub viral_status: bool,
}

impl CxlStatus {
    pub const SIZE: usize = 2;
}

impl From<u16> for CxlStatus {
    fn from(word: u16) -> Self {
        let Lsb(((), viral_status, ())) = P3::<_, 14, 1, 1>(word).into();
        Self { viral_status }
    }
}

/// DVSEC CXL Control2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlControl2 {
    /// When set, device shall no longer cache new modified lines in its local cache
    pub disable_caching: bool,
    /// When set, device shall write back all modified lines in the local cache and invalidate all lines
    pub initiate_cache_write_back_and_invalidation: bool,
    /// When set, device shall initiate CXL Reset
    pub initiate_cxl_reset: bool,
    /// When set, and CXL Reset Mem Clr Capable returns 1, Device shall clear
    /// or randomize volatile HDM ranges as part of the CXL Reset operation
    pub cxl_reset_mem_clr_enable: bool,
}

impl CxlControl2 {
    pub const SIZE: usize = 2;
}

impl From<u16> for CxlControl2 {
    fn from(word: u16) -> Self {
        let Lsb((
            disable_caching,
            initiate_cache_write_back_and_invalidation,
            initiate_cxl_reset,
            cxl_reset_mem_clr_enable,
            (),
        )) = P5::<_, 1, 1, 1, 1, 12>(word).into();
        Self {
            disable_caching,
            initiate_cache_write_back_and_invalidation,
            initiate_cxl_reset,
            cxl_reset_mem_clr_enable,
        }
    }
}

/// DVSEC CXL Status2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlStatus2 {
    /// When set, device guarantees that it does not hold any valid lines and Disable Caching=1
    pub cache_invalid: bool,
    /// When set, device has successfully completed CXL Reset
    pub cxl_reset_complete: bool,
    /// When set, device has completed CXL Reset with errors
    pub cxl_reset_error: bool,
    /// Indicates that the device has successfully completed Power Management Initialization Flow
    pub power_management_initialization_complete: bool,
}

impl CxlStatus2 {
    pub const SIZE: usize = 2;
}

impl From<u16> for CxlStatus2 {
    fn from(word: u16) -> Self {
        let Lsb((
            cache_invalid,
            cxl_reset_complete,
            cxl_reset_error,
            (),
            power_management_initialization_complete,
        )) = P5::<_, 1, 1, 1, 12, 1>(word).into();
        Self {
            cache_invalid,
            cxl_reset_complete,
            cxl_reset_error,
            power_management_initialization_complete,
        }
    }
}

/// DVSEC CXL Lock
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlLock {
    /// When set, all register fields in the PCIe DVSEC for CXL Devices
    /// Capability with RWL attribute become read onl
    pub config_lock: bool,
}

impl CxlLock {
    pub const SIZE: usize = 2;
}

impl From<u16> for CxlLock {
    fn from(word: u16) -> Self {
        let Lsb((config_lock, ())) = P2::<_, 1, 15>(word).into();
        Self { config_lock }
    }
}

/// DVSEC CXL Capability2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlCapability2 {
    pub cache_size_unit: CacheSizeUnit,
    pub cache_size: u8,
}

impl CxlCapability2 {
    pub const SIZE: usize = 2;
}

impl From<u16> for CxlCapability2 {
    fn from(word: u16) -> Self {
        let Lsb((U8(cache_size_unit), (), cache_size)) = P3::<_, 4, 4, 8>(word).into();
        Self {
            cache_size_unit,
            cache_size,
        }
    }
}

/// Cache Size Unit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheSizeUnit {
    /// Cache size is not reported
    NotReported,
    /// 64 KB
    Unit64K,
    /// 1 MB
    Unit1MB,
    Reserved(u8),
}

impl From<u8> for CacheSizeUnit {
    fn from(byte: u8) -> Self {
        match byte {
            0b0000 => Self::NotReported,
            0b0001 => Self::Unit64K,
            0b0010 => Self::Unit1MB,
            v => Self::Reserved(v),
        }
    }
}

/// DVSEC CXL Range Size
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlRangeSize {
    pub memory_info_valid: bool,
    pub memory_active: bool,
    pub media_type: MediaType,
    pub memory_class: MemoryClass,
    pub desired_interleave: DesiredInterleave,
    pub memory_active_timeout: MemoryActiveTimeout,
    pub memory_size: u64,
}

impl CxlRangeSize {
    pub const SIZE: usize = 8;
}

impl CxlRangeSize {
    pub fn new(low: u32, high: u32) -> Self {
        let Lsb((
            memory_info_valid,
            memory_active,
            U8(media_type),
            U8(memory_class),
            U8(desired_interleave),
            U8(memory_active_timeout),
            (),
            memory_size_low,
        )) = P8::<_, 1, 1, 3, 3, 5, 3, 12, 4>(low).into();
        let _: u64 = memory_size_low;
        let memory_size = (memory_size_low << 28) | ((high as u64) << 32);
        Self {
            memory_info_valid,
            memory_active,
            media_type,
            memory_class,
            desired_interleave,
            memory_active_timeout,
            memory_size,
        }
    }
}

/// Indicates the memory media characteristics
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    /// Volatile memory
    Volatile,
    /// Non-volatile memory
    NonVolatile,
    /// The memory characteristics are communicated via CDAT
    Cdat,
    Reserved(u8),
}

impl From<u8> for MediaType {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Volatile,
            0b001 => Self::NonVolatile,
            0b010 => Self::Cdat,
            v => Self::Reserved(v),
        }
    }
}

/// Indicates the class of memory
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryClass {
    /// Memory Class (e.g., normal DRAM)
    Memory,
    /// Storage Class
    Storage,
    /// The memory characteristics are communicated via CDAT
    Cdat,
    Reserved(u8),
}

impl From<u8> for MemoryClass {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Memory,
            0b001 => Self::Storage,
            0b010 => Self::Cdat,
            v => Self::Reserved(v),
        }
    }
}

/// Represents the memory interleaving desired by the device
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesiredInterleave {
    /// No Interleave
    NoInterleave,
    ///  256 Byte Granularity
    Granularity256B,
    /// 4K Interleave
    Granularity4KB,
    /// 512 Bytes
    Granularity512B,
    /// 1024 Bytes
    Granularity1KB,
    /// 2048 Bytes
    Granularity2KB,
    ///  8192 Bytes
    Granularity8KB,
    /// 16384 Bytes
    Granularity16KB,
    Reserved(u8),
}

impl From<u8> for DesiredInterleave {
    fn from(byte: u8) -> Self {
        match byte {
            0b00000 => Self::NoInterleave,
            0b00001 => Self::Granularity256B,
            0b00010 => Self::Granularity4KB,
            0b00011 => Self::Granularity512B,
            0b00100 => Self::Granularity1KB,
            0b00101 => Self::Granularity2KB,
            0b00110 => Self::Granularity8KB,
            0b00111 => Self::Granularity16KB,
            v => Self::Reserved(v),
        }
    }
}

/// Indicates the maximum time that the device is permitted to take to set
/// Memory_Active bit after a hot reset, warm reset or a cold reset
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryActiveTimeout {
    MaxTime1s,
    MaxTime4s,
    MaxTime16s,
    MaxTime64s,
    MaxTime256s,
    Reserved(u8),
}

impl From<u8> for MemoryActiveTimeout {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::MaxTime1s,
            0b001 => Self::MaxTime4s,
            0b010 => Self::MaxTime16s,
            0b011 => Self::MaxTime64s,
            0b100 => Self::MaxTime256s,
            v => Self::Reserved(v),
        }
    }
}

/// DVSEC CXL Range Base
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CxlRangeBase {
    pub memory_base: u64,
}

impl CxlRangeBase {
    pub const SIZE: usize = 8;
}

impl CxlRangeBase {
    pub fn new(low: u32, high: u32) -> Self {
        Self {
            memory_base: (low as u64) | ((high as u64) << 32),
        }
    }
}
