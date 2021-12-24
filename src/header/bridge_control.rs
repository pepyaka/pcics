//! Bridge Control Register
//!
//! The Bridge Control register provides extensions to the Command register that are specific to a
//! bridge. The Bridge Control register provides many of the same controls for the secondary
//! interface that are provided by the Command register for the primary interface. There are some
//! bits that affect the operation of both interfaces of the bridge.

use modular_bitfield::prelude::*;


/// Bridge Control Register
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BridgeControl {
    /// Controls the bridge’s response to address and data parity errors on the secondary interface
    pub parity_error_response_enable: bool,
    /// Controls the forwarding of secondary interface **SERR#** assertions to the primary interface
    pub serr_enable: bool,
    /// Modifies the response by the bridge to ISA I/O addresses
    pub isa_enable: bool,
    /// Modifies the response by the bridge to VGA compatible addresses
    pub vga_enable: bool,
    /// Modifies the response by the bridge to VGA16 compatible addresses
    pub vga_16_enable: bool,
    /// Controls the behavior of a bridge when a Master-Abort termination occurs on either
    /// interface while the bridge is the master of the transaction
    pub master_abort_mode: bool,
    /// Forces the assertion of **RST#** on the secondary interface
    pub secondary_bus_reset: bool,
    /// Controls ability of the bridge to generate fast back-to-back transactions to different
    /// devices on the secondary interface.
    pub fast_back_to_back_enable: bool,
    /// Selects the number of PCI clocks that the bridge will wait for a master on the primary
    /// interface to repeat a Delayed Transaction request
    pub primary_discard_timer: bool,
    ///  Selects the number of PCI clocks that the bridge will wait for a master on the secondary
    ///  interface to repeat a Delayed Transaction request
    pub secondary_discard_timer: bool,
    /// This bit is set to a 1 when either the Primary Discard Timer or Secondary Discard Timer
    /// expires and a Delayed Completion is discarded from a queue in the bridge
    pub discard_timer_status: bool,
    /// When set to 1, this bit enables the bridge to assert **SERR#** on the primary interface when
    /// either the Primary Discard Timer or Secondary Discard Timer expires and a Delayed
    /// Transaction is discarded from a queue in the bridge
    pub discard_timer_serr_enable: bool,
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct BridgeControlProto {
    parity_error_response_enable: bool,
    serr_enable: bool,
    isa_enable: bool,
    vga_enable: bool,
    vga_16_enable: bool,
    master_abort_mode: bool,
    secondary_bus_reset: bool,
    fast_back_to_back_enable: bool,
    primary_discard_timer: bool,
    secondary_discard_timer: bool,
    discard_timer_status: bool,
    discard_timer_serr_enable: bool,
    pub reserved: B4,
}



impl From<BridgeControlProto> for BridgeControl {
    fn from(proto: BridgeControlProto) -> Self {
        Self {
            parity_error_response_enable: proto.parity_error_response_enable(),
            serr_enable: proto.serr_enable(),
            isa_enable: proto.isa_enable(),
            vga_enable: proto.vga_enable(),
            vga_16_enable: proto.vga_16_enable(),
            master_abort_mode: proto.master_abort_mode(),
            secondary_bus_reset: proto.secondary_bus_reset(),
            fast_back_to_back_enable: proto.fast_back_to_back_enable(),
            primary_discard_timer: proto.primary_discard_timer(),
            secondary_discard_timer: proto.secondary_discard_timer(),
            discard_timer_status: proto.discard_timer_status(),
            discard_timer_serr_enable: proto.discard_timer_serr_enable(),
        }
    }
}
impl From<u16> for BridgeControl {
    fn from(word: u16) -> Self { BridgeControlProto::from(word).into() }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn from_word() {
        let result = 0xAAAA.into();
        let sample = BridgeControl {
            parity_error_response_enable: false,
            serr_enable: true,
            isa_enable: false,
            vga_enable: true,
            vga_16_enable: false,
            master_abort_mode: true,
            secondary_bus_reset: false,
            fast_back_to_back_enable: true,
            primary_discard_timer: false,
            secondary_discard_timer: true,
            discard_timer_status: false,
            discard_timer_serr_enable: true,
        };
        assert_eq!(sample, result);
    }
}
