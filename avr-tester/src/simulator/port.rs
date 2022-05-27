use super::*;
use simavr_ffi as ffi;

pub struct Port;

impl Port {
    pub fn is_pin_high(avr: &mut Avr, port: u8, pin: u8) -> bool {
        let mut state = ffi::avr_ioport_state_t {
            _bitfield_align_1: Default::default(),
            _bitfield_1: Default::default(),
            __bindgen_padding_0: Default::default(),
        };

        let status = unsafe { avr.ioctl(IoCtl::IoPortGetState { port }, &mut state) };

        if status == -1 {
            panic!("avr_ioctl() failed (status = {})", status);
        }

        let port = state._bitfield_1.get(7, 8) as u8;

        port & (1 << pin) > 0
    }
}
