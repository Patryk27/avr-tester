use crate::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Like [`core::time::Duration`], but in AVR's time; somewhat approximate¹.
///
/// ¹ <https://github.com/buserror/simavr/blob/b3ea4f93e18fa5059f76060ff718dc544ca48009/simavr/sim/sim_core.c#L621>
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CpuDuration {
    clock_frequency: u32,
    cycles: u64,
}

impl CpuDuration {
    /// Creates a new duration for given clock frequency and number of cycles:
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// CpuDuration::new(
    ///     16_000_000, /* 16 MHz */
    ///     8_000_000, /* 8 million clock cycles (=500ms here) */
    /// );
    /// ```
    ///
    /// Usually, for convenience, you'll want to use one of:
    ///
    /// - [`Self::zero()`],
    /// - [`Self::micros()`],
    /// - [`Self::millis()`],
    /// - [`Self::secs()`].
    pub const fn new(clock_frequency: u32, cycles: u64) -> Self {
        Self {
            clock_frequency,
            cycles,
        }
    }

    /// Creates a duration of zero cycles, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// See also: [`Self::new()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::CpuDuration;
    /// # let avr = todo!();
    /// #
    /// let duration = CpuDuration::zero(&avr).add_millis(150);
    /// ```
    pub fn zero(avr: &AvrTester) -> Self {
        Self::new(avr.clock_frequency, 0)
    }

    /// Creates a duration of `n` microseconds, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::CpuDuration;
    /// # let avr = todo!();
    /// #
    /// let duration = CpuDuration::micros(&avr, 15);
    /// ```
    pub fn micros(avr: &AvrTester, n: u64) -> Self {
        Self::zero(avr).add_micros(n)
    }

    /// Creates a duration of `n` milliseconds, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::CpuDuration;
    /// # let avr = todo!();
    /// #
    /// let duration = CpuDuration::millis(&avr, 15);
    /// ```
    pub fn millis(avr: &AvrTester, n: u64) -> Self {
        Self::zero(avr).add_millis(n)
    }

    /// Creates a duration of `n` seconds, using clock frequency from given
    /// [`AvrTester`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::CpuDuration;
    /// # let avr = todo!();
    /// #
    /// let duration = CpuDuration::secs(&avr, 15);
    /// ```
    pub fn secs(avr: &AvrTester, n: u64) -> Self {
        Self::zero(avr).add_secs(n)
    }

    /// Returns a new duration with increased number of cycles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(750, tt.add_cycles(4_000_000).as_millis());
    /// ```
    pub const fn add_cycles(mut self, n: u64) -> Self {
        self.cycles += n;
        self
    }

    /// Returns a new duration with increased number of cycles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(501, tt.add_micros(1_000).as_millis());
    /// ```
    pub const fn add_micros(self, n: u64) -> Self {
        self.add_cycles(n * (self.clock_frequency as u64 / 1_000_000))
    }

    /// Returns a new duration with increased number of cycles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(515, tt.add_millis(15).as_millis());
    /// ```
    pub const fn add_millis(self, millis: u64) -> Self {
        self.add_cycles(millis * (self.clock_frequency as u64) / 1_000)
    }

    /// Returns a new duration with increased number of cycles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(2500, tt.add_secs(2).as_millis());
    /// ```
    pub const fn add_secs(self, secs: u64) -> Self {
        self.add_cycles(secs * (self.clock_frequency as u64))
    }

    /// Returns the clock frequency associated with this duration (e.g. 16 MHz).
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(16_000_000, tt.clock_frequency());
    /// ```
    pub const fn clock_frequency(self) -> u32 {
        self.clock_frequency
    }

    /// Returns the number of cycles contained by this duration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(8_000_000, tt.as_cycles());
    /// ```
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 0).add_secs(3);
    ///
    /// assert_eq!(48_000_000, tt.as_cycles());
    /// ```
    pub const fn as_cycles(self) -> u64 {
        self.cycles
    }

    /// Returns the number of microseconds contained by this duration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 40);
    ///
    /// assert_eq!(3, tt.as_micros());
    /// ```
    pub fn as_micros(self) -> u64 {
        self.as_micros_f64().round() as _
    }

    /// Returns the number of microseconds contained by this duration as `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 40);
    ///
    /// assert_eq!(2.5, tt.as_micros_f64());
    /// ```
    pub fn as_micros_f64(self) -> f64 {
        (self.cycles as f64) / (self.clock_frequency as f64 / 1_000_000.0)
    }

    /// Returns the number of milliseconds contained by this duration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 40_000);
    ///
    /// assert_eq!(3, tt.as_millis());
    /// ```
    pub fn as_millis(self) -> u64 {
        self.as_millis_f64().round() as _
    }

    /// Returns the number of milliseconds contained by this duration as `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 40_000);
    ///
    /// assert_eq!(2.5, tt.as_millis_f64());
    /// ```
    pub fn as_millis_f64(self) -> f64 {
        (self.cycles as f64) / (self.clock_frequency as f64 / 1_000.0)
    }

    /// Returns the number of seconds contained by this duration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 40_000_000);
    ///
    /// assert_eq!(3, tt.as_secs());
    /// ```
    pub fn as_secs(self) -> u64 {
        self.as_secs_f64().round() as _
    }

    /// Returns the number of seconds contained by this duration as `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_tester::CpuDuration;
    /// #
    /// let tt = CpuDuration::new(16_000_000, 40_000_000);
    ///
    /// assert_eq!(2.5, tt.as_secs_f64());
    /// ```
    pub fn as_secs_f64(self) -> f64 {
        (self.cycles as f64) / (self.clock_frequency as f64)
    }
}

/// ```
/// # use avr_tester::CpuDuration;
/// #
/// let a = CpuDuration::new(16_000_000, 1_000);
/// let b = CpuDuration::new(16_000_000, 2_000);
///
/// assert_eq!(
///     CpuDuration::new(16_000_000, 3_000),
///     a + b,
/// );
/// ```
impl Add for CpuDuration {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

/// ```
/// # use avr_tester::CpuDuration;
/// #
/// let mut a = CpuDuration::new(16_000_000, 1_000);
///
/// a += CpuDuration::new(16_000_000, 2_000);
///
/// assert_eq!(CpuDuration::new(16_000_000, 3_000), a);
/// ```
impl AddAssign for CpuDuration {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.clock_frequency, rhs.clock_frequency,
            "Cannot add durations with different clock frequencies ({} vs {})",
            self.clock_frequency, rhs.clock_frequency
        );

        self.cycles += rhs.cycles;
    }
}

/// ```
/// # use avr_tester::CpuDuration;
/// #
/// let a = CpuDuration::new(16_000_000, 3_000);
/// let b = CpuDuration::new(16_000_000, 2_000);
///
/// assert_eq!(
///     CpuDuration::new(16_000_000, 1_000),
///     a - b,
/// );
/// ```
impl Sub for CpuDuration {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

/// ```
/// # use avr_tester::CpuDuration;
/// #
/// let mut a = CpuDuration::new(16_000_000, 3_000);
///
/// a -= CpuDuration::new(16_000_000, 2_000);
///
/// assert_eq!(CpuDuration::new(16_000_000, 1_000), a);
/// ```
impl SubAssign for CpuDuration {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.clock_frequency, rhs.clock_frequency,
            "Cannot subtract durations with different clock frequencies ({} vs {})",
            self.clock_frequency, rhs.clock_frequency
        );

        self.cycles -= rhs.cycles;
    }
}
