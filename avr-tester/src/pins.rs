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
        Pin::new(self.sim, b'B', 0)
    }

    pub fn pb1(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'B', 1)
    }

    pub fn pb2(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'B', 2)
    }

    pub fn pb3(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'B', 3)
    }

    pub fn pb4(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'B', 4)
    }

    pub fn pb5(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'B', 5)
    }

    pub fn pb6(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'B', 6)
    }

    pub fn pb7(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'B', 7)
    }

    // ----

    pub fn pc0(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 0)
    }

    pub fn pc1(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 1)
    }

    pub fn pc2(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 2)
    }

    pub fn pc3(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 3)
    }

    pub fn pc4(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 4)
    }

    pub fn pc5(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 5)
    }

    pub fn pc6(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 6)
    }

    pub fn pc7(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'C', 7)
    }

    // ----

    pub fn pd0(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 0)
    }

    pub fn pd1(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 1)
    }

    pub fn pd2(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 2)
    }

    pub fn pd3(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 3)
    }

    pub fn pd4(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 4)
    }

    pub fn pd5(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 5)
    }

    pub fn pd6(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 6)
    }

    pub fn pd7(&mut self) -> Pin<'_> {
        Pin::new(self.sim, b'D', 7)
    }
}

pub struct Pin<'a> {
    sim: &'a mut AvrSimulator,
    port: u8,
    pin: u8,
}

impl<'a> Pin<'a> {
    fn new(sim: &'a mut AvrSimulator, port: u8, pin: u8) -> Self {
        Self { sim, port, pin }
    }

    pub fn is_low(&mut self) -> bool {
        !self.is_high()
    }

    #[track_caller]
    pub fn assert_low(&mut self) {
        assert!(self.is_low());
    }

    pub fn is_high(&mut self) -> bool {
        self.sim.is_pin_high(self.port, self.pin)
    }

    #[track_caller]
    pub fn assert_high(&mut self) {
        assert!(self.is_high());
    }
}
