/*!
# Page Request Interface (PRI)

ATS improves the behavior of DMA based data movement. An associated Page Request Interface
(PRI) provides additional advantages by allowing DMA operations to be initiated without
requiring that all the data to be moved into or out of system memory be pinned.1

## Struct diagram
[PageRequestInterface]
- [PageRequestControl]
- [PageRequestStatus]

## Examples

```rust
# use pcics::extended_capabilities::page_request_interface::*;
let data = [
    0x13, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01
];
let result = data[4..].try_into().unwrap();
let sample = PageRequestInterface {
    page_request_control: PageRequestControl {
        enable: true,
        reset: false
    },
    page_request_status: PageRequestStatus {
        response_failure: true,
        unexpected_page_request_group_index: false,
        stopped: true,
        prg_response_pasid_required: false,
    },
    outstanding_page_request_capacity: 0x01010101,
    outstanding_page_request_allocation: 0x01010101,
};
assert_eq!(sample, result);
```
*/

use heterob::{bit_numbering::Lsb, endianness::Le, Seq, P3, P4, P6};

use super::ExtendedCapabilityDataError;

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
impl TryFrom<&[u8]> for PageRequestInterface {
    type Error = ExtendedCapabilityDataError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Seq {
            head:
                Le((
                    control,
                    status,
                    outstanding_page_request_capacity,
                    outstanding_page_request_allocation,
                )),
            ..
        } = P4(slice)
            .try_into()
            .map_err(|_| ExtendedCapabilityDataError {
                name: "Page Request Interface",
                size: 12,
            })?;
        let Lsb((enable, reset, ())) = P3::<u16, 1, 1, 14>(control).into();
        let Lsb((
            response_failure,
            unexpected_page_request_group_index,
            (),
            stopped,
            (),
            prg_response_pasid_required,
        )) = P6::<u16, 1, 1, 6, 1, 6, 1>(status).into();
        Ok(Self {
            page_request_control: PageRequestControl { enable, reset },
            page_request_status: PageRequestStatus {
                response_failure,
                unexpected_page_request_group_index,
                stopped,
                prg_response_pasid_required,
            },
            outstanding_page_request_capacity,
            outstanding_page_request_allocation,
        })
    }
}
/// Page Request Control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRequestControl {
    /// Enable (E)
    pub enable: bool,
    /// Reset (R)
    pub reset: bool,
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
