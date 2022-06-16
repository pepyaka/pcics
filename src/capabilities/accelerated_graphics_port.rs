/*!
# Accelerated Graphics Port

Accelerated Graphics Port (AGP) is a parallel expansion card standard, designed
for attaching a video card to a computer system to assist in the acceleration
of 3D computer graphics.

## Struct diagram
<pre>
<a href="struct.AcceleratedGraphicsPort.html">AcceleratedGraphicsPort</a>
├─ <a href="struct.Identifier.html">Identifier</a>
├─ <a href="struct.Status.html">Status</a>
│  ├─ <a href="enum.CalCycle.html">CalCycle</a>
│  └─ <a href="enum.DataRateSupport.html">DataRateSupport</a>
├─ <a href="struct.Command.html">Command</a>
│  ├─ <a href="enum.CalCycle.html">CalCycle</a>
│  └─ <a href="enum.DataRateEnabled.html">DataRateEnabled</a>
├─ <a href="struct.IsochronousStatus.html">IsochronousStatus</a>
│  ├─ <a href="struct.IsochY.html">IsochY</a>
│  └─ <a href="struct.IsochErrorCode.html">IsochErrorCode</a>
├─ <a href="struct.Control.html">Control</a>
├─ <a href="struct.ApertureSize.html">ApertureSize</a>
├─ <a href="struct.EnabledAperturePageSize.html">EnabledAperturePageSize</a>
├─ <a href="struct.GartPointer.html">GartPointer</a>
└─ <a href="struct.IsochronousCommand.html">IsochronousCommand</a>
</pre>

## Examples

```rust
# use pcics::capabilities::accelerated_graphics_port::*;
let data = [
    0x02, 0x00, // Header
    0x33, 0x00, // Identifier
    0b0101_0001, 0b0101_1101, 0b0000_0010, 0x07, // Status
    0x51, 0b0101_1101, 0x55, 0x55, // Command
    0x55, 0x55, 0x55, 0x55, // IsochronousStatus
    0x55, 0x55, 0x55, 0x55, // Control
    0x55, 0x55, // ApertureSize
    0x55, 0x55, // EnabledAperturePageSize
    0x78, 0x56, 0x34, 0x12, // GartPointer Lo
    0xFF, 0x00, 0x00, 0x00, // GartPointer Hi
    0x55, 0x55, 0x55, 0x55, // IsochronousCommand
    0x55, 0x55, // Status
];
let result = data[2..].try_into().unwrap();
let sample = AcceleratedGraphicsPort {
    identifier: Identifier { minor: 3, major: 3 },
    status: Status {
        rq: 7,
        isoch_support: true,
        reserved: false,
        arqsz: 2,
        cal_cycle: CalCycle::CalibrationCycleNotNeeded,
        sba: false,
        ita_coh: true,
        gart64b: false,
        htrans: true,
        over4g: false,
        fw: true,
        agp_3_0_mode: false,
        rate: DataRateSupport::Speed4x,
    },
    command: Command {
        prq: 0x55,
        parqsz: 2,
        pcal_cycle: CalCycle::CalibrationCycleNotNeeded,
        sba_enable: false,
        agp_enable: true,
        gart64b: false,
        over4g: false,
        fw_enable: true,
        drate: DataRateEnabled::Speed4x,
    },
    isochronous_status: Some(IsochronousStatus {
        maxbw: 0x55,
        isoch_n: 0x55,
        isoch_y: IsochY(1),
        isoch_l: 2,
        isoch_error_code: IsochErrorCode(1),
    }),
    control: Some(Control {
        cal_cycle_dis: false,
        aperenb: true,
        gtlben: false,
    }),
    aperture_size: Some(ApertureSize(0x5555)),
    enabled_aperture_page_size: Some(EnabledAperturePageSize(0x5555)),
    gart_pointer: Some(GartPointer {
        gartlo: 0x12345678,
        garthi: 0xff,
    }),
    isochronous_command: Some(IsochronousCommand {
        pisoch_n: 0x55,
        pisoch_y: 1,
    }),
};
assert_eq!(sample, result);
```
*/

use heterob::{
    bit_numbering::Lsb,
    endianness::{Le, LeBytesTryInto},
    Seq, P13, P14, P3, P7,
};

use super::CapabilityDataError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceleratedGraphicsPort {
    pub identifier: Identifier,
    pub status: Status,
    pub command: Command,
    pub isochronous_status: Option<IsochronousStatus>,
    pub control: Option<Control>,
    pub aperture_size: Option<ApertureSize>,
    pub enabled_aperture_page_size: Option<EnabledAperturePageSize>,
    pub gart_pointer: Option<GartPointer>,
    pub isochronous_command: Option<IsochronousCommand>,
}

impl TryFrom<&[u8]> for AcceleratedGraphicsPort {
    type Error = CapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head: Le((identifier, status, command)),
            tail,
        } = P3(slice).try_into().map_err(|_| CapabilityDataError {
            name: "AGP",
            size: 10,
        })?;
        let status: Status = From::<u32>::from(status);

        let mut isochronous_status = None;
        let mut control = None;
        let mut aperture_size = None;
        let mut enabled_aperture_page_size = None;
        let mut gartlo = None;
        let mut garthi = None;
        let isochronous_command = tail
            .le_bytes_try_into()
            .and_then(|seq| {
                if status.isoch_support {
                    isochronous_status = Some(From::<u32>::from(seq.head));
                }
                seq.tail.le_bytes_try_into()
            })
            .and_then(|seq| {
                control = Some(From::<u32>::from(seq.head));
                seq.tail.le_bytes_try_into()
            })
            .and_then(|seq| {
                aperture_size = Some(ApertureSize(seq.head));
                seq.tail.le_bytes_try_into()
            })
            .and_then(|seq| {
                enabled_aperture_page_size = Some(EnabledAperturePageSize(seq.head));
                seq.tail.le_bytes_try_into()
            })
            .and_then(|seq| {
                gartlo = Some(seq.head);
                seq.tail.le_bytes_try_into()
            })
            .and_then(|seq| {
                garthi = Some(seq.head);
                seq.tail.le_bytes_try_into()
            })
            .map(|Seq { head, .. }| From::<u16>::from(head))
            .ok()
            .filter(|_| status.isoch_support);

        let gart_pointer = match (gartlo, garthi) {
            (Some(gartlo), Some(garthi)) => Some(GartPointer { gartlo, garthi }),
            (Some(gartlo), None) => Some(GartPointer { gartlo, garthi: 0 }),
            _ => None,
        };
        Ok(Self {
            identifier: From::<u16>::from(identifier),
            status,
            command: From::<u32>::from(command),
            isochronous_status,
            control,
            aperture_size,
            enabled_aperture_page_size,
            gart_pointer,
            isochronous_command,
        })
    }
}

/// The Major and Minor Revision IDs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub minor: u8,
    pub major: u8,
}

impl From<u16> for Identifier {
    fn from(word: u16) -> Self {
        let Lsb((minor, major, ())) = P3::<u16, 4, 4, 8>(word).into();
        Self { minor, major }
    }
}

/// AGP Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    /// The maximum number of AGP3.0 command requests (both
    /// asynchronous and isochronous) that can be enqueued to the target
    pub rq: u8,
    /// Isochronous transactions support
    pub isoch_support: bool,
    /// Always returns 0 when read; write operations have no effect.
    pub reserved: bool,
    /// Log2 of the optimum asynchronous request size in bytes minus 4 to be
    /// used with the target. Optimum_request_size = 2 ^ (ARQSZ+4)
    pub arqsz: u8,
    pub cal_cycle: CalCycle,
    /// Sideband addressing support
    pub sba: bool,
    /// In-aperture accesses always coherent
    pub ita_coh: bool,
    /// Core-logic can support 64-bit and 32-bit GART entries
    pub gart64b: bool,
    /// Core-logic can translate host processor accesses through the AGP aperture
    pub htrans: bool,
    /// Device supports addresses greater than 4 GB
    pub over4g: bool,
    /// Fast-Write support
    pub fw: bool,
    /// AGP3.0 Mode support
    pub agp_3_0_mode: bool,
    pub rate: DataRateSupport,
}

impl From<u32> for Status {
    fn from(dword: u32) -> Self {
        let Lsb((
            rate,
            agp_3_0_mode,
            fw,
            over4g,
            htrans,
            gart64b,
            ita_coh,
            sba,
            cal_cycle,
            arqsz,
            reserved,
            isoch_support,
            (),
            rq,
        )) = P14::<_, 3, 1, 1, 1, 1, 1, 1, 1, 3, 3, 1, 1, 6, 8>(dword).into();
        Self {
            rq,
            isoch_support,
            reserved,
            arqsz,
            cal_cycle: From::<u8>::from(cal_cycle),
            sba,
            ita_coh,
            gart64b,
            htrans,
            over4g,
            fw,
            agp_3_0_mode,
            rate: From::<u8>::from(rate),
        }
    }
}

/// Specifies required period for core-logic initiated bus cycle for
/// calibrating I/O buffers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalCycle {
    /// 4 ms
    Period4ms,
    /// 16ms
    Period16ms,
    /// 64 ms
    Period64ms,
    /// 256 ms
    Period256ms,
    /// Reserved for future use
    Reserved(u8),
    /// Calibration Cycle Not Needed
    CalibrationCycleNotNeeded,
}

impl From<u8> for CalCycle {
    fn from(byte: u8) -> Self {
        match byte {
            0b000 => Self::Period4ms,
            0b001 => Self::Period16ms,
            0b010 => Self::Period64ms,
            0b011 => Self::Period256ms,
            0b111 => Self::CalibrationCycleNotNeeded,
            v => Self::Reserved(v),
        }
    }
}

impl From<CalCycle> for u8 {
    fn from(data: CalCycle) -> Self {
        match data {
            CalCycle::Period4ms => 0b000,
            CalCycle::Period16ms => 0b001,
            CalCycle::Period64ms => 0b010,
            CalCycle::Period256ms => 0b011,
            CalCycle::CalibrationCycleNotNeeded => 0b111,
            CalCycle::Reserved(v) => v,
        }
    }
}

/// Data Rate Support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRateSupport {
    /// 4x
    Speed4x,
    /// 8x
    Speed8x,
    /// 4x, and 8x
    Speed4xAnd8x,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for DataRateSupport {
    fn from(byte: u8) -> Self {
        match byte {
            0b001 => Self::Speed4x,
            0b010 => Self::Speed8x,
            0b011 => Self::Speed4xAnd8x,
            v => Self::Reserved(v),
        }
    }
}

impl From<DataRateSupport> for u8 {
    fn from(data: DataRateSupport) -> Self {
        match data {
            DataRateSupport::Speed4x => 0b001,
            DataRateSupport::Speed8x => 0b010,
            DataRateSupport::Speed4xAnd8x => 0b011,
            DataRateSupport::Reserved(v) => v,
        }
    }
}

/// AGP Command
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// Maximum number of requests
    pub prq: u8,
    /// LOG2 of the optimum asynchronous request size in bytes minus 4 to be
    /// used with the target. Optimum_request_size = 2 ^ (PARQSZ+4)
    pub parqsz: u8,
    /// Programmed with period for core-logic initiated bus cycle for
    /// calibrating I/O buffers for both master and target
    pub pcal_cycle: CalCycle,
    /// Sideband addressing enabled
    pub sba_enable: bool,
    /// Master: allows to initiate AGP operations  
    /// Target: allows to accept AGP operations
    pub agp_enable: bool,
    /// 64-bit GART entries enabled
    pub gart64b: bool,
    /// Master: allows initiate AGP3.0 Requests to addresses above the 4 GB  
    /// Target: enables the target to accept a Type 4 command
    pub over4g: bool,
    /// FW is enabled in Master or Target
    pub fw_enable: bool,
    pub drate: DataRateEnabled,
}

impl From<u32> for Command {
    fn from(dword: u32) -> Self {
        let Lsb((
            drate,
            (),
            fw_enable,
            over4g,
            (),
            gart64b,
            agp_enable,
            sba_enable,
            pcal_cycle,
            parqsz,
            (),
            (),
            prq,
        )) = P13::<_, 3, 1, 1, 1, 1, 1, 1, 1, 3, 3, 1, 7, 8>(dword).into();
        Self {
            prq,
            parqsz,
            pcal_cycle: From::<u8>::from(pcal_cycle),
            sba_enable,
            agp_enable,
            gart64b,
            over4g,
            fw_enable,
            drate: From::<u8>::from(drate),
        }
    }
}

/// Data Rate Enabled
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRateEnabled {
    /// 4x
    Speed4x,
    /// 8x
    Speed8x,
    /// Reserved
    Reserved(u8),
}

impl From<u8> for DataRateEnabled {
    fn from(byte: u8) -> Self {
        match byte {
            0b001 => Self::Speed4x,
            0b010 => Self::Speed8x,
            v => Self::Reserved(v),
        }
    }
}

impl From<DataRateEnabled> for u8 {
    fn from(data: DataRateEnabled) -> Self {
        match data {
            DataRateEnabled::Speed4x => 0b001,
            DataRateEnabled::Speed8x => 0b010,
            DataRateEnabled::Reserved(v) => v,
        }
    }
}

/// AGP Isochronous Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsochronousStatus {
    /// Maximum Bandwidth (both asynchronous and isochronous) of the device per
    /// us in units of 32 bytes
    pub maxbw: u8,
    /// Maximum number of isochronous transactions in a single isochronous
    /// period
    pub isoch_n: u8,
    pub isoch_y: IsochY,
    /// Maximum isochronous data transfer latency
    pub isoch_l: u8,
    pub isoch_error_code: IsochErrorCode,
}
impl From<u32> for IsochronousStatus {
    fn from(dword: u32) -> Self {
        let Lsb((isoch_error_code, (), isoch_l, isoch_y, isoch_n, maxbw, ())) =
            P7::<_, 2, 1, 3, 2, 8, 8, 8>(dword).into();
        Self {
            maxbw,
            isoch_n,
            isoch_y: IsochY(isoch_y),
            isoch_l,
            isoch_error_code: IsochErrorCode(isoch_error_code),
        }
    }
}

/// Isochronous payload sizes supported
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsochY(pub u8);

/// Isochronous Error Codes
///
/// Target:
/// - 00 = No Error
/// - 01 = Isoch Req Overflow29
/// - 10 = Reserved
/// - 11 = Reserved
///
/// Master:
/// - 00 = No Error
/// - 01 = Read Buffer under-run
/// - 10 = Write Buffer Overflow
/// - 11 = Reserved
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsochErrorCode(pub u8);

/// AGP Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
    /// Calibration cycle operation is disabled by the core-logic
    pub cal_cycle_dis: bool,
    /// Enabling of the graphics AGP aperture for the AGP3.0 Port
    pub aperenb: bool,
    /// Enables normal operations of the Graphics Translation Lookaside Buffer
    pub gtlben: bool,
}
impl From<u32> for Control {
    fn from(dword: u32) -> Self {
        let Lsb(((), gtlben, aperenb, cal_cycle_dis, (), (), ())) =
            P7::<_, 7, 1, 1, 1, 6, 8, 8>(dword).into();
        Self {
            cal_cycle_dis,
            aperenb,
            gtlben,
        }
    }
}

/// AGP Aperture size
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApertureSize(pub u16);

/// AGP Enabled Aperture Page Size
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnabledAperturePageSize(pub u16);

/// Graphics AGP aperture Remapping Table (GART) Pointer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GartPointer {
    /// Bits\[31:12\] of the start of the GART
    pub gartlo: u32,
    /// Bits\[63:32\] of the start of the GART
    pub garthi: u32,
}

/// AGP Isochronous Command (NICMD)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsochronousCommand {
    /// Software programs this field with the maximum number of isochronous
    /// transactions that the master is allowed to request from the target in a single
    /// isochronous period
    pub pisoch_n: u8,
    /// Software programs this field with the Isochronous payload size to be
    /// used by all AGP3.0 devices
    pub pisoch_y: u8,
}
impl From<u16> for IsochronousCommand {
    fn from(word: u16) -> Self {
        let Lsb(((), pisoch_y, pisoch_n)) = P3::<_, 6, 2, 8>(word).into();
        Self { pisoch_n, pisoch_y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode_all_bits_are_one() {
        let data = [0xffu8; 48];
        let result = data[2..].try_into().unwrap();
        let sample = AcceleratedGraphicsPort {
            identifier: Identifier {
                minor: 15,
                major: 15,
            },
            status: Status {
                rq: 255,
                isoch_support: true,
                reserved: true,
                arqsz: 7,
                cal_cycle: CalCycle::CalibrationCycleNotNeeded,
                sba: true,
                ita_coh: true,
                gart64b: true,
                htrans: true,
                over4g: true,
                fw: true,
                agp_3_0_mode: true,
                rate: DataRateSupport::Reserved(7),
            },
            command: Command {
                prq: 0xff,
                parqsz: 7,
                pcal_cycle: CalCycle::CalibrationCycleNotNeeded,
                sba_enable: true,
                agp_enable: true,
                gart64b: true,
                over4g: true,
                fw_enable: true,
                drate: DataRateEnabled::Reserved(7),
            },
            isochronous_status: Some(IsochronousStatus {
                maxbw: 0xff,
                isoch_n: 0xff,
                isoch_y: IsochY(3),
                isoch_l: 7,
                isoch_error_code: IsochErrorCode(3),
            }),
            control: Some(Control {
                cal_cycle_dis: true,
                aperenb: true,
                gtlben: true,
            }),
            aperture_size: Some(ApertureSize(0xffff)),
            enabled_aperture_page_size: Some(EnabledAperturePageSize(0xffff)),
            gart_pointer: Some(GartPointer {
                gartlo: u32::MAX,
                garthi: u32::MAX,
            }),
            isochronous_command: Some(IsochronousCommand {
                pisoch_n: 0xff,
                pisoch_y: 3,
            }),
        };
        assert_eq!(sample, result);
    }
}
