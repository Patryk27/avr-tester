use crate::AvrSimulator;

pub struct AnalogPin<'a> {
    sim: &'a mut AvrSimulator,
    pin: u32,
}

impl<'a> AnalogPin<'a> {
    pub(super) fn new(sim: &'a mut AvrSimulator, pin: u32) -> Self {
        Self { sim, pin }
    }

    /// Simulates applying `voltage` millivolts to this ADC.
    pub fn set_mv(&mut self, voltage: u32) {
        self.sim.set_adc_voltage(self.pin as _, voltage);
    }
}
