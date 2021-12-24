//! The Bridge Control register provides extensions of the Command Register that are specific to
//! PCI to PCI and PCI-to-CardBus bridges.

use modular_bitfield::prelude::*;


/// Bridge Control Register (Offset = 3EH)
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CardbusBridgeControl {
    /// Controls the response to parity errors on the CardBus
    pub parity_error_response_enable: bool,
    /// Controls forwarding of SERR# signals indicated on the CardBus
    pub serr_enable: bool,
    /// This applies only to addresses that are enabled by the I/O Base and Limit registers and are
    /// also in the first 64 KBytes of PCI I/O space
    pub isa_enable: bool,
    /// Modifies the bridge's response to VGA compatible addresses
    pub vga_enable: bool,
    /// Controls the behavior of the bridge when a master abort occurs on either PCI or CardBus
    /// interface when the bridge is master
    pub master_abort_mode: bool,
    /// When set the bridge will assert and hold CRST#
    pub cardbus_reset: bool,
    /// When set this bit enables the IRQ routing register for 16-bit PC Cards
    pub ireq_int_enable: bool,
    /// When set enables Read prefetching from the memory window defined to by the Memory Base 0
    /// and Memory Limit 0 registers
    pub memory_0_prefetch_enable: bool,
    /// When set enables Read prefetching from the memory window defined to by the Memory Base 1
    /// and Memory Limit 1 registers
    pub memory_1_prefetch_enable: bool,
    /// Enables posting of Write data to and from the socket
    pub write_posting_enable: bool,
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct CardbusBridgeControlProto {
    parity_error_response_enable: bool,
    serr_enable: bool,
    isa_enable: bool,
    vga_enable: bool,
    pub reserved0: B1,
    master_abort_mode: bool,
    cardbus_reset: bool,
    ireq_int_enable: bool,
    memory_0_prefetch_enable: bool,
    memory_1_prefetch_enable: bool,
    write_posting_enable: bool,
    pub reserved1: B5,
}



impl From<CardbusBridgeControlProto> for CardbusBridgeControl {
    fn from(proto: CardbusBridgeControlProto) -> Self {
        Self {
            parity_error_response_enable: proto.parity_error_response_enable(),
            serr_enable: proto.serr_enable(),
            isa_enable: proto.isa_enable(),
            vga_enable: proto.vga_enable(),
            master_abort_mode: proto.master_abort_mode(),
            cardbus_reset: proto.cardbus_reset(),
            ireq_int_enable: proto.ireq_int_enable(),
            memory_0_prefetch_enable: proto.memory_0_prefetch_enable(),
            memory_1_prefetch_enable: proto.memory_1_prefetch_enable(),
            write_posting_enable: proto.write_posting_enable(),
        }
    }
}
impl From<u16> for CardbusBridgeControl {
    fn from(word: u16) -> Self { CardbusBridgeControlProto::from(word).into() }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn from_word() {
        let result = 0xAAAA.into();
        let sample = CardbusBridgeControl {
            parity_error_response_enable: false,
            serr_enable: true,
            isa_enable: false,
            vga_enable: true,
            master_abort_mode: true,
            cardbus_reset: false,
            ireq_int_enable: true,
            memory_0_prefetch_enable: false,
            memory_1_prefetch_enable: true,
            write_posting_enable: false,
        };
        assert_eq!(sample, result);
    }
}
