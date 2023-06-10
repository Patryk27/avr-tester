use crate::*;

/// Provides access to an analog pin, e.g. `ADC1`.
pub struct AnalogPin<'a> {
    avr: &'a mut AvrTester,
    pin: u32,
}

impl<'a> AnalogPin<'a> {
    pub(crate) fn new(avr: &'a mut AvrTester, pin: u32) -> Self {
        Self { avr, pin }
    }

    /// Applies `voltage` millivolts to this ADC.
    pub fn set_mv(&mut self, voltage: u32) {
        self.avr.sim().set_analog_pin(self.pin as _, voltage);
    }
}

/// Asynchronous equivalent of [`AnalogPin`].
///
/// See [`avr_rt()`] for more details.
pub struct AnalogPinAsync {
    pin: u32,
}

impl AnalogPinAsync {
    pub(crate) fn new(pin: u32) -> Self {
        Self { pin }
    }

    /// Asynchronous equivalent of [`AnalogPin::set_mv()`].
    pub fn set_mv(&self, voltage: u32) {
        ComponentRuntime::with(|rt| {
            rt.sim().set_analog_pin(self.pin as _, voltage);
        });
    }
}
