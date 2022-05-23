//! PCI Bridge Subsystem Vendor ID

use heterob::{endianness::Le, P3};

use super::CapabilityDataError;

/// PCI Bridge Subsystem Vendor ID
/// ```
/// # use pcics::capabilities::BridgeSubsystemVendorId;
/// # use pretty_assertions::assert_eq;
/// # use byte::{ ctx::*, self, TryRead, BytesExt, };
/// let data = [0x00,0x00,0x11,0x22,0x33,0x44];
///
/// let result: BridgeSubsystemVendorId = data.try_into().unwrap();
///
/// let sample = BridgeSubsystemVendorId {
///     reserved: 0x0000,
///     subsystem_vendor_id: 0x2211,
///     subsystem_id: 0x4433,
/// };
/// assert_eq!(sample, result);
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubsystemVendorId {
    pub reserved: u16,
    /// PCI Bridge Subsystem Vendor ID
    pub subsystem_vendor_id: u16,
    /// PCI-Bridge subsystem device id register
    pub subsystem_id: u16,
}
impl BridgeSubsystemVendorId {
    pub const SIZE: usize = 2 + 2 + 2;
}

impl From<[u8; BridgeSubsystemVendorId::SIZE]> for BridgeSubsystemVendorId {
    fn from(bytes: [u8; BridgeSubsystemVendorId::SIZE]) -> Self {
        let Le((reserved, subsystem_vendor_id, subsystem_id)) = P3(bytes).into();
        Self {
            reserved,
            subsystem_vendor_id,
            subsystem_id,
        }
    }
}
impl<'a> TryFrom<&'a [u8]> for BridgeSubsystemVendorId {
    type Error = CapabilityDataError;
    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        slice
            .get(..Self::SIZE)
            .and_then(|slice| <[u8; Self::SIZE]>::try_from(slice).ok())
            .ok_or(CapabilityDataError {
                name: "Bridge Subsystem Vendor ID",
                size: Self::SIZE,
            })
            .map(Self::from)
    }
}
