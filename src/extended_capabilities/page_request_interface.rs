//! Page Request Interface (PRI)
//!
//! ATS improves the behavior of DMA based data movement. An associated Page Request Interface
//! (PRI) provides additional advantages by allowing DMA operations to be initiated without
//! requiring that all the data to be moved into or out of system memory be pinned.1

use modular_bitfield::prelude::*;
use byte::{
    ctx::*,
    self,
    TryRead,
    // TryWrite,
    BytesExt,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRequestInterface {
    /// Page Request Control
    pub page_request_control: PageRequestControl,
    /// Page Request Status
    pub page_request_status: PageRequestStatus,
    /// Outstanding Page Request Capacity
    pub outstanding_page_request_capacity: u32,
    /// Outstanding Page Request Allocation
    pub outstanding_page_request_allocation: u32,
}
impl<'a> TryRead<'a, Endian> for PageRequestInterface {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let pri = PageRequestInterface {
            page_request_control: bytes.read_with::<u16>(offset, endian)?.into(),
            page_request_status: bytes.read_with::<u16>(offset, endian)?.into(),
            outstanding_page_request_capacity: bytes.read_with::<u32>(offset, endian)?,
            outstanding_page_request_allocation: bytes.read_with::<u32>(offset, endian)?,
        };
        Ok((pri, *offset))
    }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct PageRequestControlProto {
    enable: bool,
    reset: bool,
    rsvdp: B14,
}

/// Page Request Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRequestControl {
    /// Enable (E)
    pub enable: bool,
    /// Reset (R)
    pub reset: bool,
}
impl From<PageRequestControlProto> for PageRequestControl {
    fn from(proto: PageRequestControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            enable: proto.enable(),
            reset: proto.reset(),
        }
    }
}
impl From<u16> for PageRequestControl {
    fn from(word: u16) -> Self { PageRequestControlProto::from(word).into() }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct PageRequestStatusProto {
    response_failure: bool,
    unexpected_page_request_group_index: bool,
    rsvdz: B6,
    stopped: bool,
    rsvdz_2: B6,
    prg_response_pasid_required: bool,
}

/// Page Request Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRequestStatus {
    /// Response Failure (RF)
    pub response_failure: bool,
    /// Unexpected Page Request Group Index (UPRGI)
    pub unexpected_page_request_group_index: bool,
    /// Stopped (S)
    pub stopped: bool,
    /// PRG Response PASID Required
    pub prg_response_pasid_required: bool,
}
impl From<PageRequestStatusProto> for PageRequestStatus {
    fn from(proto: PageRequestStatusProto) -> Self {
        let _ = proto.rsvdz();
        let _ = proto.rsvdz_2();
        Self {
            response_failure: proto.response_failure(),
            unexpected_page_request_group_index: proto.unexpected_page_request_group_index(),
            stopped: proto.stopped(),
            prg_response_pasid_required: proto.prg_response_pasid_required(),
        }
    }
}
impl From<u16> for PageRequestStatus {
    fn from(word: u16) -> Self { PageRequestStatusProto::from(word).into() }
}
