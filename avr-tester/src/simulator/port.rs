use super::*;
use simavr_ffi as ffi;

pub struct Port;

impl Port {
    pub fn set_pin_state(avr: &mut Avr, port: char, pin: u8, high: bool) {
        let irq = avr
            .io_getirq(IoCtl::IoPortGetIrq { port }, pin as u32)
            .unwrap_or_else(|| panic!("Chosen AVR doesn't support pin {}{}", port, pin));

        // Safety: We've got IRQ from `IoCtl::IoPortGetIrq`, which is safe to be
        //         raised with 0/1
        unsafe {
            avr.raise_irq(irq, high as _);
        }
    }

    pub fn pin_state(avr: &mut Avr, port: char, pin: u8) -> bool {
        let mut state = ffi::avr_ioport_state_t {
            _bitfield_align_1: Default::default(),
            _bitfield_1: Default::default(),
            __bindgen_padding_0: Default::default(),
        };

        // Safety: `IoCtl::IoPortGetState` requires parameter of type
        //         `avr_ioport_state_t`, which is the case here
        let status = unsafe { avr.ioctl(IoCtl::IoPortGetState { port }, &mut state) };

        if status == -1 {
            panic!("Chosen AVR doesn't support pin {}{}", port, pin);
        }

        let port = state._bitfield_1.get(7, 8) as u8;

        port & (1 << pin) > 0
    }
}
