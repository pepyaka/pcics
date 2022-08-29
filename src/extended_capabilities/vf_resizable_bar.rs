/*!
# Resizable BAR

The Resizable BAR Capability is an optional capability that allows hardware to communicate
resource sizes, and system software, after determining the optimal size, to communicate this optimal
size back to the hardware.

## Struct diagram
<pre>
<a href="struct.ResizableBar.html">ResizableBar</a>
└─ <a href="struct.ResizableBarEntry.html">ResizableBarEntry (1..=6)</a>
   ├─ <a href="struct.ResizableBarCapability.html">ResizableBarCapability</a>
   └─ <a href="struct.ResizableBarControl.html">ResizableBarControl</a>
</pre>

## Examples
```rust
# use pcics::extended_capabilities::resizable_bar::*;
let data = [
    /* 00h */ 0x15, 0x00, 0x01, 0x00, // Capability header
    /* 04h */ 0x00, 0x11, 0x22, 0x44, // Resizable BAR Capability (0)
    /* 08h */ 0x40, 0x03, 0x24, 0x01, // Resizable BAR Control (0)
    /* 0Ch */ 0x10, 0x20, 0x40, 0x80, // Resizable BAR Capability (1)
    /* 10h */ 0x01, 0x0f, 0x10, 0x42, // Resizable BAR Control (1)
];

let mut rebar: ResizableBar = data.as_slice().try_into().unwrap();
let result = rebar.clone().collect::<Vec<_>>();

let sample = vec![
    ResizableBarEntry {
        capability: ResizableBarCapability {
            support_map_from_1mb_to_128tb: 0x44221100,
        },
        control: ResizableBarControl {
            bar_index: 0,
            number_of_resizable_bars: 2,
            bar_size: 3,
            support_map_from_256tb_to_8eb: 0x0124,
        },
    },
    ResizableBarEntry {
        capability: ResizableBarCapability {
            support_map_from_1mb_to_128tb: 0x80402010,
        },
        control: ResizableBarControl {
            bar_index: 1,
            number_of_resizable_bars: 0,
            bar_size: 15,
            support_map_from_256tb_to_8eb: 0x4210,
        },
    },
];

assert_eq!(sample, result);


let second_entry = rebar.nth(1).unwrap();
let supported_sizes = ResizableBarEntry::BAR_SIZES
    .iter()
    .enumerate()
    .filter_map(|(n, s)| {
        (second_entry.is_function_supports_power_of_two(n + 20)).then(|| s)
    })
    .collect::<Vec<_>>();

assert_eq!(
    vec![&"1MB", &"512MB", &"256GB", &"128TB", &"4PB", &"128PB", &"4EB",],
    supported_sizes
);

```
*/

use core::slice;
use heterob::{
    bit_numbering::Lsb,
    endianness::{Le, LeBytesTryInto},
    Seq, P2, P3, P6,
};
use snafu::Snafu;

use super::ExtendedCapabilityHeader;

#[derive(Snafu, Debug, Clone, PartialEq, Eq)]
pub enum ResizableBarError {
    #[snafu(display("should have at least one entry"))]
    FirstEntry,
    #[snafu(display("should have 1..=6 number of entries"))]
    NumberOfResizableBars { value: usize },
    #[snafu(display("entries data too short"))]
    ShortData,
}

/// An iterator through [Resible BAR Entries](ResizableBarEntry)
#[derive(Debug, Clone)]
pub struct ResizableBar<'a>(pub slice::Chunks<'a, u8>);

impl<'a> ResizableBar<'a> {
    /// Entry size = Resizable Bar Capability size + Resizable Bar Control size
    pub const ENTRY_SIZE: usize = 4 + 4;
}

impl<'a> PartialEq for ResizableBar<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.clone().eq(other.0.clone())
    }
}

impl<'a> Eq for ResizableBar<'a> {}

impl<'a> TryFrom<&'a [u8]> for ResizableBar<'a> {
    type Error = ResizableBarError;

    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let Seq { head, .. } = slice
            .get(ExtendedCapabilityHeader::SIZE..)
            .unwrap_or_default()
            .le_bytes_try_into()
            .map_err(|_| ResizableBarError::FirstEntry)?;
        let Lsb(((), num_bars, ())) = P3::<u64, 37, 3, 24>(head).into();
        let _: usize = num_bars;
        if let 1..=6 = num_bars {
            let start = ExtendedCapabilityHeader::SIZE;
            let end = start + num_bars * ResizableBar::ENTRY_SIZE;
            let chunks = slice
                .get(start..end)
                .ok_or(ResizableBarError::ShortData)?
                .chunks(ResizableBar::ENTRY_SIZE);
            Ok(ResizableBar(chunks))
        } else {
            Err(ResizableBarError::NumberOfResizableBars { value: num_bars })
        }
    }
}

impl<'a> Iterator for ResizableBar<'a> {
    type Item = ResizableBarEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.0.next()?;
        let Seq {
            head: Le((cap, ctrl)),
            ..
        } = P2(chunk).try_into().ok()?;
        Some(ResizableBarEntry {
            capability: From::<u32>::from(cap),
            control: From::<u32>::from(ctrl),
        })
    }
}

/// Resizable BAR Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResizableBarEntry {
    pub capability: ResizableBarCapability,
    pub control: ResizableBarControl,
}

impl ResizableBarEntry {
    /// Function supports operating with the BAR sized to values
    pub const BAR_SIZES: [&'static str; 44] = [
        "1MB", "2MB", "4MB", "8MB", "16MB", "32MB", "64MB", "128MB", "256MB", "512MB",
        "1GB", "2GB", "4GB", "8GB", "16GB", "32GB", "64GB", "128GB", "256GB", "512GB",
        "1TB", "2TB", "4TB", "8TB", "16TB", "32TB", "64TB", "128TB", "256TB", "512TB",
        "1PB", "2PB", "4PB", "8PB", "16PB", "32PB", "64PB", "128PB", "256PB", "512PB",
        "1EB", "2EB", "4EB", "8EB",
    ];
    /// Check if Function supports operating with the BAR sized to 2ᵖᵒʷᵉʳ
    pub fn is_function_supports_power_of_two(&self, power: usize) -> bool {
        match power {
            20..=47 => self.capability.support_map_from_1mb_to_128tb & (1 << (power - 16)) != 0,
            48..=63 => self.control.support_map_from_256tb_to_8eb & (1 << (power - 48)) != 0,
            _ => false,
        }
    }
}

/// Resizable BAR Capability
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResizableBarCapability {
    pub support_map_from_1mb_to_128tb: u32,
}

impl From<u32> for ResizableBarCapability {
    fn from(dword: u32) -> Self {
        Self {
            support_map_from_1mb_to_128tb: dword,
        }
    }
}

/// Resizable BAR Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResizableBarControl {
    pub bar_index: u8,
    pub number_of_resizable_bars: u8,
    pub bar_size: u8,
    pub support_map_from_256tb_to_8eb: u16,
}

impl From<u32> for ResizableBarControl {
    fn from(dword: u32) -> Self {
        let Lsb((
            bar_index,
            (),
            number_of_resizable_bars,
            bar_size,
            (),
            support_map_from_256tb_to_8eb,
        )) = P6::<_, 3, 2, 3, 6, 2, 16>(dword).into();
        Self {
            bar_index,
            number_of_resizable_bars,
            bar_size,
            support_map_from_256tb_to_8eb,
        }
    }
}
