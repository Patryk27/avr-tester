use crate::*;

/// Provides access to the SPI interface.
///
/// See: [`Spi::read()`] and [`Spi::write()`].
pub struct Spi<'a> {
    sim: &'a mut AvrSimulator,
    id: u8,
}

impl<'a> Spi<'a> {
    pub(crate) fn new(sim: &'a mut AvrSimulator, id: u8) -> Self {
        Self { sim, id }
    }

    /// Retrieves a value from AVR.
    ///
    /// See: [`Readable`].
    /// See also: [`Self::try_read_byte()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::*;
    /// # fn avr() -> AvrTester { panic!() }
    /// #
    /// let mut avr = avr();
    ///
    /// // Retrieves a single byte:
    /// // (when the input buffer is empty, panics.)
    /// assert_eq!(72, avr.spi0().read::<u8>());
    ///
    /// // Retrieves the entire buffer:
    /// // (when it's empty, returns an empty vector.)
    /// assert_eq!(vec![72, 101, 108, 108, 111], avr.spi0().read::<Vec<u8>>());
    ///
    /// // Retrieves `n` bytes from the buffer:
    /// // (when there's not enough bytes, panics.)
    /// assert_eq!([72, 101, 108, 108, 111], avr.spi0().read::<[u8; 5]>());
    ///
    /// // Retrieves the entire input buffer and converts it into string:
    /// // (when it's empty, returns an empty string.)
    /// assert_eq!("Hello", avr.spi0().read::<String>());
    /// ```
    pub fn read<T>(&mut self) -> T
    where
        T: Readable,
    {
        T::read(self)
    }

    /// Transmits a value to AVR.
    ///
    /// See: [`Writable`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::*;
    /// # fn avr() -> AvrTester { panic!() }
    /// #
    /// let mut avr = avr();
    ///
    /// // Transmits a single byte:
    /// avr.spi0().write(123);
    ///
    /// // Transmits many bytes:
    /// avr.spi0().write([10, 20, 30]);
    ///
    /// // Transmits a string:
    /// avr.spi0().write("Hello!");
    ///
    /// // Strings are transmitted as a series of their bytes, so the above is
    /// // equivalent to:
    /// avr.spi0().write([72, 101, 108, 108, 111, 33]);
    /// //                H   e    l    l    o    !
    /// ```
    pub fn write<T>(&mut self, value: T)
    where
        T: Writable,
    {
        value.write(self);
    }
}

impl Reader for Spi<'_> {
    fn read_byte(&mut self) -> u8 {
        self.try_read_byte().expect(
            "SPI's buffer is empty - got no more bytes to read; if you're \
             receiving a large buffer, try running the simulator for a bit \
             longer so that the simulated AVR has more time to respond",
        )
    }

    fn try_read_byte(&mut self) -> Option<u8> {
        self.sim.read_spi(self.id)
    }
}

impl Writer for Spi<'_> {
    fn write_byte(&mut self, value: u8) {
        self.sim.write_spi(self.id, value);
    }
}

/// Asynchronous equivalent of [`Spi`].
///
/// See [`avr_rt()`] for more details.
pub struct SpiAsync {
    id: u8,
}

impl SpiAsync {
    pub(crate) fn new(id: u8) -> Self {
        Self { id }
    }

    /// Asynchronous equivalent of [`Spi::read()`].
    pub fn read<T>(&self) -> T
    where
        T: Readable,
    {
        self.with(|spi| spi.read())
    }

    /// Asynchronous equivalent of [`Spi::write()`].
    pub fn write<T>(&mut self, value: T)
    where
        T: Writable,
    {
        self.with(|spi| spi.write(value))
    }

    fn with<T>(&self, f: impl FnOnce(&mut Spi) -> T) -> T {
        ComponentRuntime::with(|rt| {
            let mut spi = Spi::new(rt.sim(), self.id);

            f(&mut spi)
        })
    }
}

impl Reader for SpiAsync {
    fn read_byte(&mut self) -> u8 {
        self.with(|spi| spi.read_byte())
    }

    fn try_read_byte(&mut self) -> Option<u8> {
        self.with(|spi| spi.try_read_byte())
    }
}

impl Writer for SpiAsync {
    fn write_byte(&mut self, value: u8) {
        self.with(|spi| spi.write_byte(value))
    }
}
