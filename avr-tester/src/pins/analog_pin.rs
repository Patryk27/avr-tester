use crate::*;

pub struct AnalogPin<'a> {
    avr: &'a mut AvrTester,
    pin: u32,
}

impl<'a> AnalogPin<'a> {
    pub(super) fn new(avr: &'a mut AvrTester, pin: u32) -> Self {
        Self { avr, pin }
    }

    /// Applies `voltage` millivolts to this ADC.
    pub fn set_mv(&mut self, voltage: u32) {
        self.avr.sim.set_adc_voltage(self.pin as _, voltage);
    }
}
