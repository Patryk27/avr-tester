use crate::*;

pub struct DigitalPin<'a> {
    avr: &'a mut AvrTester,
    port: char,
    pin: u8,
}

impl<'a> DigitalPin<'a> {
    pub(super) fn new(avr: &'a mut AvrTester, port: char, pin: u8) -> Self {
        Self { avr, port, pin }
    }

    /// Changes pin's state to low or high.
    pub fn set(&mut self, high: bool) {
        self.avr.sim.set_pin_high(self.port, self.pin, high);
    }

    /// Changes pin's state to low.
    pub fn set_low(&mut self) {
        self.set(false);
    }

    /// Changes pin's state to high.
    pub fn set_high(&mut self) {
        self.set(true);
    }

    /// Returns whether pin's state is low.
    pub fn is_low(&mut self) -> bool {
        !self.is_high()
    }

    /// Returns whether pin's state is high.
    pub fn is_high(&mut self) -> bool {
        self.avr.sim.is_pin_high(self.port, self.pin)
    }

    /// Asserts that pin's state is high or low.
    #[track_caller]
    pub fn assert(&mut self, high: bool) {
        if high {
            self.assert_high();
        } else {
            self.assert_low();
        }
    }

    /// Asserts that pin's state is low.
    #[track_caller]
    pub fn assert_low(&mut self) {
        assert!(self.is_low(), "{} is not low", self.name());
    }

    /// Asserts that pin's state is high.
    #[track_caller]
    pub fn assert_high(&mut self) {
        assert!(self.is_high(), "{} is not high", self.name());
    }

    /// Waits until pin switches state (e.g. from low to high or from high to
    /// low).
    ///
    /// Returns duration it took for the pin to switch state.
    pub fn pulse_in(&mut self) -> CpuDuration {
        let mut tt = CpuDuration::zero(self.avr);
        let state = self.is_high();

        while self.is_high() == state {
            tt += self.avr.run();
        }

        tt
    }

    /// Waits until pin becomes high; if the pin is already high, exits
    /// immediately.
    ///
    /// Returns duration it took for the pin to get high.
    pub fn wait_while_low(&mut self) -> CpuDuration {
        let mut tt = CpuDuration::zero(self.avr);

        while self.is_low() {
            tt += self.avr.run();
        }

        tt
    }

    /// Waits until pin becomes low; if the pin is already low, exits
    /// immediately.
    ///
    /// Returns duration it took for the pin to get low.
    pub fn wait_while_high(&mut self) -> CpuDuration {
        let mut tt = CpuDuration::zero(self.avr);

        while self.is_high() {
            tt += self.avr.run();
        }

        tt
    }

    /// Return pin's name, e.g. `PC6`.
    pub fn name(&self) -> String {
        format!("P{}{}", self.port, self.pin)
    }
}
