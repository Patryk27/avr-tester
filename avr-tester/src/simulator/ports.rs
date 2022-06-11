use super::*;

/// Provides access to simavr's ports (aka _digital pins_).
///
/// See also: [`ADCs`].
pub struct Ports;

impl Ports {
    pub fn new() -> Self {
        Self
    }

    pub fn set_pin_high(&self, avr: &mut Avr, port: char, pin: u8, high: bool) {
        let irq = avr
            .io_getirq(IoCtl::IoPortGetIrq { port }, pin as u32)
            .unwrap_or_else(|| panic!("Chosen AVR doesn't support pin P{}{}", port, pin));

        // Safety: We've got the IRQ from `IoCtl::IoPortGetIrq`, which we know
        //         is safe to call with 0 / 1
        unsafe {
            avr.raise_irq(irq, if high { 1 } else { 0 });
        }
    }

    pub fn is_pin_high(&self, avr: &mut Avr, port: char, pin: u8) -> bool {
        let mut state = ffi::avr_ioport_state_t {
            _bitfield_align_1: Default::default(),
            _bitfield_1: Default::default(),
            __bindgen_padding_0: Default::default(),
        };

        // Safety: `IoCtl::IoPortGetState` requires parameter of type
        //         `avr_ioport_state_t`, which is the case here
        let status = unsafe { avr.ioctl(IoCtl::IoPortGetState { port }, &mut state) };

        if status == -1 {
            panic!("Chosen AVR doesn't support pin P{}{}", port, pin);
        }

        let port = state._bitfield_1.get(7, 8) as u8;

        port & (1 << pin) > 0
    }
}
