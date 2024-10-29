use crate::AvrTester;
use std::path::Path;
use std::time::Duration;

pub struct AvrTesterBuilder {
    mcu: String,
    clock: Option<u32>,
    timeout: Option<Duration>,
}

impl AvrTesterBuilder {
    /// Creates `AvrTesterBuilder`.
    ///
    /// To avoid typos, it's preferred that you use helper functions such as
    /// [`AvrTester::atmega328p()`]; this additional constructor in here has
    /// been provided just in case there's some AVR supported by simavr that has
    /// not been yet exposed through AvrTester.
    pub fn new(mcu: impl ToString) -> Self {
        Self {
            mcu: mcu.to_string(),
            clock: None,
            timeout: None,
        }
    }

    /// Specifies AVR's clock.
    ///
    /// This value doesn't affect how fast the simulation is run - it's used
    /// mostly so that [`AvrTester::run_for_s()`] and similar functions know how
    /// long a second, a millisecond etc. should be.
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
    pub fn with_clock_of_12_mhz(self) -> Self {
        self.with_clock(12_000_000)
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

    /// Specifies a timeout (in _AVR's_ time¹) after which calling
    /// [`AvrTester::run()`] (or a similar function) will panic, aborting the
    /// test to signal that it timed-out.
    ///
    /// This might come handy in tests that wait for AVR to do something:
    ///
    /// ```no_run
    /// # use avr_tester::AvrTester;
    /// #
    /// let mut avr = AvrTester::atmega328p()
    ///     .with_clock_of_16_mhz()
    ///     .with_timeout_of_s(1)
    ///     .load("...");
    ///
    /// while avr.pins().pb1().is_low() {
    ///     avr.run_for_ms(1);
    /// }
    ///
    /// /* do something else later */
    /// ```
    ///
    /// ... as otherwise, without specifying the timeout, if the tested firmware
    /// misbehaves (and e.g. doesn't toggle the expected pin), the test would be
    /// running in an infinite loop (instead of failing).
    ///
    /// ¹ using AVR's time instead of the host's time allows to assert this
    ///   reliably - whatever timeout you set here will be consistent across all
    ///   machines.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Specifies a timeout in seconds (of AVR's time).
    ///
    /// See: [`Self::with_timeout()`].
    pub fn with_timeout_of_s(self, s: u64) -> Self {
        self.with_timeout(Duration::from_secs(s))
    }

    /// Specifies a timeout in milliseconds (of AVR's time).
    ///
    /// See: [`Self::with_timeout()`].
    pub fn with_timeout_of_ms(self, ms: u64) -> Self {
        self.with_timeout(Duration::from_millis(ms))
    }

    /// Specifies a timeout in microseconds (of AVR's time).
    ///
    /// See: [`Self::with_timeout()`].
    pub fn with_timeout_of_us(self, us: u64) -> Self {
        self.with_timeout(Duration::from_micros(us))
    }

    /// Loads given firmware (an `*.elf` file) and boots the simulator.
    pub fn load(self, firmware: impl AsRef<Path>) -> AvrTester {
        let clock_frequency = self
            .clock
            .expect("Clock frequency was not specified; please call `.with_clock()` before");

        let remaining_clock_cycles = self
            .timeout
            .map(|timeout| (timeout.as_secs_f32() * (clock_frequency as f32)))
            .map(|cc| cc as _);

        AvrTester::new(
            &self.mcu,
            clock_frequency,
            firmware,
            remaining_clock_cycles,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target() -> AvrTesterBuilder {
        AvrTesterBuilder::new("some-random-avr")
    }

    #[test]
    fn with_clock() {
        let target = target().with_clock(123);

        assert_eq!(Some(123), target.clock);
    }

    #[test]
    fn with_clock_of_1_mhz() {
        let target = target().with_clock_of_1_mhz();

        assert_eq!(Some(1_000_000), target.clock);
    }

    #[test]
    fn with_clock_of_4_mhz() {
        let target = target().with_clock_of_4_mhz();

        assert_eq!(Some(4_000_000), target.clock);
    }

    #[test]
    fn with_clock_of_8_mhz() {
        let target = target().with_clock_of_8_mhz();

        assert_eq!(Some(8_000_000), target.clock);
    }

    #[test]
    fn with_clock_of_12_mhz() {
        let target = target().with_clock_of_12_mhz();

        assert_eq!(Some(12_000_000), target.clock);
    }

    #[test]
    fn with_clock_of_16_mhz() {
        let target = target().with_clock_of_16_mhz();

        assert_eq!(Some(16_000_000), target.clock);
    }

    #[test]
    fn with_clock_of_20_mhz() {
        let target = target().with_clock_of_20_mhz();

        assert_eq!(Some(20_000_000), target.clock);
    }

    #[test]
    fn with_clock_of_24_mhz() {
        let target = target().with_clock_of_24_mhz();

        assert_eq!(Some(24_000_000), target.clock);
    }

    #[test]
    fn with_timeout() {
        let target = target().with_timeout(Duration::from_secs(321));

        // Note that the actual timeouting logic is already covered by through
        // our integration tests - in here we just want to assert that the field
        // is stored correctly:
        assert_eq!(Some(Duration::from_secs(321)), target.timeout);
    }

    #[test]
    fn with_timeout_of_s() {
        let target = target().with_timeout_of_s(123);

        assert_eq!(Some(Duration::from_secs(123)), target.timeout);
    }

    #[test]
    fn with_timeout_of_ms() {
        let target = target().with_timeout_of_ms(123);

        assert_eq!(Some(Duration::from_millis(123)), target.timeout);
    }

    #[test]
    fn with_timeout_of_us() {
        let target = target().with_timeout_of_us(123);

        assert_eq!(Some(Duration::from_micros(123)), target.timeout);
    }
}
