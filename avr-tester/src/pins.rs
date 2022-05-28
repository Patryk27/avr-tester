use crate::simulator::AvrSimulator;

pub struct Pins<'a> {
    sim: &'a mut AvrSimulator,
}

impl<'a> Pins<'a> {
    pub(crate) fn new(sim: &'a mut AvrSimulator) -> Self {
        Self { sim }
    }

    // ----

    pub fn pb0(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 0)
    }

    pub fn pb1(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 1)
    }

    pub fn pb2(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 2)
    }

    pub fn pb3(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 3)
    }

    pub fn pb4(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 4)
    }

    pub fn pb5(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 5)
    }

    pub fn pb6(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 6)
    }

    pub fn pb7(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'B', 7)
    }

    // ----

    pub fn pc0(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 0)
    }

    pub fn pc1(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 1)
    }

    pub fn pc2(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 2)
    }

    pub fn pc3(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 3)
    }

    pub fn pc4(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 4)
    }

    pub fn pc5(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 5)
    }

    pub fn pc6(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 6)
    }

    pub fn pc7(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'C', 7)
    }

    // ----

    pub fn pd0(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 0)
    }

    pub fn pd1(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 1)
    }

    pub fn pd2(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 2)
    }

    pub fn pd3(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 3)
    }

    pub fn pd4(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 4)
    }

    pub fn pd5(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 5)
    }

    pub fn pd6(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 6)
    }

    pub fn pd7(&mut self) -> Pin<'_> {
        Pin::new(self.sim, 'D', 7)
    }
}

pub struct Pin<'a> {
    sim: &'a mut AvrSimulator,
    port: char,
    pin: u8,
}

impl<'a> Pin<'a> {
    fn new(sim: &'a mut AvrSimulator, port: char, pin: u8) -> Self {
        Self { sim, port, pin }
    }

    /// Changes pin's state to low / high.
    pub fn set(&mut self, high: bool) {
        self.sim.set_pin_state(self.port, self.pin, high);
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
        self.sim.pin_state(self.port, self.pin)
    }

    /// Asserts that pin's state is high / low.
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
        assert!(self.is_low());
    }

    /// Asserts that pin's state is high.
    #[track_caller]
    pub fn assert_high(&mut self) {
        assert!(self.is_high());
    }
}
