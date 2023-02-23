use super::*;

/// Provides access to simavr's digital pins.
pub struct Port;

impl Port {
    pub fn set_pin(avr: &mut Avr, port: char, pin: u8, high: bool) {
        let irq = avr
            .try_io_getirq(IoCtl::IoPortGetIrq { port }, pin as u32)
            .unwrap_or_else(|| panic!("Current AVR doesn't have pin P{}{}", port, pin));

        // Safety: `IoPortGetIrq` can be raised with a zero or one
        unsafe {
            ffi::avr_raise_irq(irq.as_ptr(), if high { 1 } else { 0 });
        }
    }

    pub fn get_pin(avr: &mut Avr, port: char, pin: u8) -> bool {
        let mut state = ffi::avr_ioport_state_t {
            _bitfield_align_1: Default::default(),
            _bitfield_1: Default::default(),
            __bindgen_padding_0: Default::default(),
        };

        // Safety: `IoCtl::IoPortGetState` requires parameter of type
        // `avr_ioport_state_t`, which is the case here
        let status = unsafe { avr.ioctl(IoCtl::IoPortGetState { port }, &mut state) };

        if status == -1 {
            panic!("Current AVR doesn't have pin P{}{}", port, pin);
        }

        let port = state._bitfield_1.get(7, 8) as u8;

        port & (1 << pin) > 0
    }
}
