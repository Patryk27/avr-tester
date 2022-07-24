use crate::AvrTester;
use std::path::Path;

pub struct AvrTesterBuilder {
    mcu: String,
    clock: Option<u32>,
}

impl AvrTesterBuilder {
    /// Creates `AvrTesterBuilder`.
    ///
    /// To avoid typos, it's preferred that you use helper functions such as
    /// [`AvrTester::atmega328p()`]; this additional constructor in here has
    /// been provided just in case there's some AVR supported by simavr that has
    /// not been yet exposed through [`AvrTester`].
    pub fn new(mcu: impl ToString) -> Self {
        Self {
            mcu: mcu.to_string(),
            clock: None,
        }
    }

    /// Specifies AVR's clock.
    ///
    /// This value doesn't affect how fast the simulation is run - it's used
    /// mostly so that [`AvrTester::run_for_s()`] and similar functions know how
    /// long a second, millisecond etc. should be.
    ///
    /// See:
    ///
    /// - [`Self::with_clock_of_1_mhz()`],
    /// - [`Self::with_clock_of_4_mhz()`],
    /// - [`Self::with_clock_of_8_mhz()`],
    /// - [`Self::with_clock_of_16_mhz()`],
    /// - [`Self::with_clock_of_20_mhz()`],
    /// - [`Self::with_clock_of_24_mhz()`].
    pub fn with_clock(mut self, clock: u32) -> Self {
        self.clock = Some(clock);
        self
    }

    /// See: [`Self::with_clock()`].
    pub fn with_clock_of_1_mhz(self) -> Self {
        self.with_clock(1_000_000)
    }

    /// See: [`Self::with_clock()`].
    pub fn with_clock_of_4_mhz(self) -> Self {
        self.with_clock(4_000_000)
    }

    /// See: [`Self::with_clock()`].
    pub fn with_clock_of_8_mhz(self) -> Self {
        self.with_clock(8_000_000)
    }

    /// See: [`Self::with_clock()`].
    pub fn with_clock_of_16_mhz(self) -> Self {
        self.with_clock(16_000_000)
    }

    /// See: [`Self::with_clock()`].
    pub fn with_clock_of_20_mhz(self) -> Self {
        self.with_clock(20_000_000)
    }

    /// See: [`Self::with_clock()`].
    pub fn with_clock_of_24_mhz(self) -> Self {
        self.with_clock(24_000_000)
    }

    /// Loads given firmware (an `*.elf` file) and starts the simulator.
    pub fn load(self, firmware: impl AsRef<Path>) -> AvrTester {
        let clock = self
            .clock
            .expect("Clock frequency was not specified; please call `.with_clock()` before");

        AvrTester::new(&self.mcu, firmware, clock)
    }
}
