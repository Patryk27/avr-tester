use crate::AvrTester;
use avr_simulator::AvrDuration;

pub trait AvrDurationExt {
    /// Creates a duration of zero cycles, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// See also: [`AvrDuration::new()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_simulator::AvrDuration;
    /// # use avr_tester::AvrDurationExt;
    /// # let avr = panic!();
    /// #
    /// let duration = AvrDuration::zero(&avr).with_millis(150);
    /// ```
    fn zero(avr: &AvrTester) -> Self;

    /// Creates a duration of `n` microseconds, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_simulator::AvrDuration;
    /// # use avr_tester::AvrDurationExt;
    /// # let avr = panic!();
    /// #
    /// let duration = AvrDuration::micros(&avr, 15);
    /// ```
    fn micros(avr: &AvrTester, n: u64) -> Self;

    /// Creates a duration of `n` milliseconds, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_simulator::AvrDuration;
    /// # use avr_tester::AvrDurationExt;
    /// # let avr = panic!();
    /// #
    /// let duration = AvrDuration::millis(&avr, 15);
    /// ```
    fn millis(avr: &AvrTester, n: u64) -> Self;

    /// Creates a duration of `n` seconds, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_simulator::AvrDuration;
    /// # use avr_tester::AvrDurationExt;
    /// # let avr = panic!();
    /// #
    /// let duration = AvrDuration::secs(&avr, 15);
    /// ```
    fn secs(avr: &AvrTester, n: u64) -> Self;
}

impl AvrDurationExt for AvrDuration {
    fn zero(avr: &AvrTester) -> Self {
        Self::new(avr.clock_frequency, 0)
    }

    fn micros(avr: &AvrTester, n: u64) -> Self {
        Self::zero(avr).add_micros(n)
    }

    fn millis(avr: &AvrTester, n: u64) -> Self {
        Self::zero(avr).add_millis(n)
    }

    fn secs(avr: &AvrTester, n: u64) -> Self {
        Self::zero(avr).add_secs(n)
    }
}
