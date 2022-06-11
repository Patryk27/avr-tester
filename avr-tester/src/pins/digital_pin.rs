use crate::AvrSimulator;

pub struct DigitalPin<'a> {
    sim: &'a mut AvrSimulator,
    port: char,
    pin: u8,
}

impl<'a> DigitalPin<'a> {
    pub(super) fn new(sim: &'a mut AvrSimulator, port: char, pin: u8) -> Self {
        Self { sim, port, pin }
    }

    /// Changes pin's state to low or high.
    ///
    /// See: [`Self::set_low()`], [`Self::set_high()`].
    pub fn set(&mut self, high: bool) {
        self.sim.set_pin_high(self.port, self.pin, high);
    }

    /// Changes pin's state to low.
    ///
    /// See: [`Self::set()`].
    pub fn set_low(&mut self) {
        self.set(false);
    }

    /// Changes pin's state to high.
    ///
    /// See: [`Self::set()`].
    pub fn set_high(&mut self) {
        self.set(true);
    }

    /// Returns whether pin's state is low.
    pub fn is_low(&mut self) -> bool {
        !self.is_high()
    }

    /// Returns whether pin's state is high.
    pub fn is_high(&mut self) -> bool {
        self.sim.is_pin_high(self.port, self.pin)
    }

    /// Asserts that pin's state is high or low.
    ///
    /// See: [`Self::assert_low()`], [`Self::assert_high()`].
    #[track_caller]
    pub fn assert(&mut self, high: bool) {
        if high {
            self.assert_high();
        } else {
            self.assert_low();
        }
    }

    /// Asserts that pin's state is low.
    ///
    /// See: [`Self::assert()`].
    #[track_caller]
    pub fn assert_low(&mut self) {
        assert!(self.is_low(), "{} is not low", self.name());
    }

    /// Asserts that pin's state is high.
    ///
    /// See: [`Self::assert()`].
    #[track_caller]
    pub fn assert_high(&mut self) {
        assert!(self.is_high(), "{} is not high", self.name());
    }

    /// Return pin's name, e.g. `PC6`.
    pub fn name(&self) -> String {
        format!("P{}{}", self.port, self.pin)
    }
}
