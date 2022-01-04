use modular_bitfield::prelude::*;


#[bitfield(bits = 16)]
#[repr(u16)]
pub struct CommandProto {
    io_space: bool,
    memory_space: bool,
    bus_master: bool,
    special_cycles: bool,
    memory_write_and_invalidate_enable: bool,
    vga_palette_snoop: bool,
    parity_error_response: bool,
    stepping: bool,
    serr_enable: bool,
    fast_back_to_back_enable: bool,
    interrupt_disable: bool,
    reserved: B5,
}

/// Provides control over a device's ability to generate and respond to PCI cycles.
///
/// Where the only functionality guaranteed to be supported by all devices is, when a 0 is written
/// to this register, the device is disconnected from the PCI bus for all accesses except
/// Configuration Space access.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Command {
    pub io_space: bool,
    pub memory_space: bool,
    pub bus_master: bool,
    pub special_cycles: bool,
    pub memory_write_and_invalidate_enable: bool,
    pub vga_palette_snoop: bool,
    pub parity_error_response: bool,
    pub stepping: bool,
    pub serr_enable: bool,
    pub fast_back_to_back_enable: bool,
    pub interrupt_disable: bool,
    pub reserved: u8,
}
impl From<CommandProto> for Command {
    fn from(proto: CommandProto) -> Self {
        Self {
            io_space: proto.io_space(),
            memory_space: proto.memory_space(),
            bus_master: proto.bus_master(),
            special_cycles: proto.special_cycles(),
            memory_write_and_invalidate_enable: proto.memory_write_and_invalidate_enable(),
            vga_palette_snoop: proto.vga_palette_snoop(),
            parity_error_response: proto.parity_error_response(),
            stepping: proto.stepping(),
            serr_enable: proto.serr_enable(),
            fast_back_to_back_enable: proto.fast_back_to_back_enable(),
            interrupt_disable: proto.interrupt_disable(),
            reserved: proto.reserved(),
        }
    }
}
impl From<u16> for Command {
    fn from(word: u16) -> Self { CommandProto::from(word).into() }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn from_word() {
        let result = 0xAAAA.into();
        let sample = Command {
            io_space: false,
            memory_space: true,
            bus_master: false,
            special_cycles: true,
            memory_write_and_invalidate_enable: false,
            vga_palette_snoop: true,
            parity_error_response: false,
            stepping: true,
            serr_enable: false,
            fast_back_to_back_enable: true,
            interrupt_disable: false,
            reserved: 0b10101,
        };
        assert_eq!(sample, result);
    }
}
