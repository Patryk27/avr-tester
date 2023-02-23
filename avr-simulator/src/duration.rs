use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Like [`std::time::Duration`], but in AVR cycles; somewhat approximate¹.
///
/// ¹ <https://github.com/buserror/simavr/blob/b3ea4f93e18fa5059f76060ff718dc544ca48009/simavr/sim/sim_core.c#L621>
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AvrDuration {
    clock_frequency: u32,
    cycles: u64,
}

impl AvrDuration {
    /// Creates a new duration for given clock frequency and number of cycles:
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// AvrDuration::new(
    ///     16_000_000, /* 16 MHz */
    ///     8_000_000, /* 8 million clock cycles (=500ms here) */
    /// );
    /// ```
    ///
    /// If you're using AvrTester, you might find `AvrDurationExt` more
    /// convenient than this constructor.
    pub const fn new(clock_frequency: u32, cycles: u64) -> Self {
        Self {
            clock_frequency,
            cycles,
        }
    }

    /// Returns a new duration with increased number of cycles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(750, tt.add_cycles(4_000_000).as_millis());
    /// ```
    pub const fn add_cycles(mut self, n: u64) -> Self {
        self.cycles += n;
        self
    }

    /// Returns a new duration with increased number of microseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(501, tt.add_micros(1_000).as_millis());
    /// ```
    pub const fn add_micros(self, n: u64) -> Self {
        self.add_cycles(n * (self.clock_frequency as u64 / 1_000_000))
    }

    /// Returns a new duration with increased number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(515, tt.add_millis(15).as_millis());
    /// ```
    pub const fn add_millis(self, millis: u64) -> Self {
        self.add_cycles(millis * (self.clock_frequency as u64) / 1_000)
    }

    /// Returns a new duration with increased number of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(500, tt.as_millis());
    /// assert_eq!(2500, tt.add_secs(2).as_millis());
    /// ```
    pub const fn add_secs(self, secs: u64) -> Self {
        self.add_cycles(secs * (self.clock_frequency as u64))
    }

    /// Returns a new duration with specified number of cycles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(250, tt.with_cycles(4_000_000).as_millis());
    /// ```
    pub const fn with_cycles(mut self, n: u64) -> Self {
        self.cycles = n;
        self
    }

    /// Returns a new duration with specified number of microseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(1, tt.with_micros(1_000).as_millis());
    /// ```
    pub const fn with_micros(self, n: u64) -> Self {
        self.with_cycles(0).add_micros(n)
    }

    /// Returns a new duration with specified number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(15, tt.with_millis(15).as_millis());
    /// ```
    pub const fn with_millis(self, n: u64) -> Self {
        self.with_cycles(0).add_millis(n)
    }

    /// Returns a new duration with specified number of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(2000, tt.with_secs(2).as_millis());
    /// ```
    pub const fn with_secs(self, n: u64) -> Self {
        self.with_cycles(0).add_secs(n)
    }

    /// Returns the clock frequency associated with this duration (e.g. 16 MHz).
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
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
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 8_000_000);
    ///
    /// assert_eq!(8_000_000, tt.as_cycles());
    /// ```
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 0).add_secs(3);
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
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 40);
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
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 40);
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
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 40_000);
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
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 40_000);
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
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 40_000_000);
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
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt = AvrDuration::new(16_000_000, 40_000_000);
    ///
    /// assert_eq!(2.5, tt.as_secs_f64());
    /// ```
    pub fn as_secs_f64(self) -> f64 {
        (self.cycles as f64) / (self.clock_frequency as f64)
    }

    /// Returns whether the number of cycles contained by this duration is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use avr_simulator::AvrDuration;
    /// #
    /// let tt1 = AvrDuration::new(16_000_000, 0);
    /// let tt2 = AvrDuration::new(16_000_000, 10_000);
    ///
    /// assert!(tt1.is_zero());
    /// assert!(!tt2.is_zero());
    /// ```
    pub fn is_zero(self) -> bool {
        self.as_cycles() == 0
    }
}

/// # Examples
///
/// ```
/// # use avr_simulator::AvrDuration;
/// #
/// let a = AvrDuration::new(16_000_000, 1_000);
/// let b = AvrDuration::new(16_000_000, 2_000);
///
/// assert_eq!(
///     AvrDuration::new(16_000_000, 3_000),
///     a + b,
/// );
/// ```
impl Add for AvrDuration {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

/// # Examples
///
/// ```
/// # use avr_simulator::AvrDuration;
/// #
/// let mut a = AvrDuration::new(16_000_000, 1_000);
///
/// a += AvrDuration::new(16_000_000, 2_000);
///
/// assert_eq!(AvrDuration::new(16_000_000, 3_000), a);
/// ```
impl AddAssign for AvrDuration {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.clock_frequency, rhs.clock_frequency,
            "Cannot add durations with different clock frequencies ({} vs {})",
            self.clock_frequency, rhs.clock_frequency
        );

        self.cycles += rhs.cycles;
    }
}

/// # Examples
///
/// ```
/// # use avr_simulator::AvrDuration;
/// #
/// let a = AvrDuration::new(16_000_000, 3_000);
/// let b = AvrDuration::new(16_000_000, 2_000);
///
/// assert_eq!(
///     AvrDuration::new(16_000_000, 1_000),
///     a - b,
/// );
/// ```
///
/// ```
/// # use avr_simulator::AvrDuration;
/// #
/// let a = AvrDuration::new(16_000_000, 3_000);
/// let b = AvrDuration::new(16_000_000, 4_000);
///
/// assert_eq!(
///     AvrDuration::new(16_000_000, 0),
///     a - b,
/// );
/// ```
impl Sub for AvrDuration {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

/// # Examples
///
/// ```
/// # use avr_simulator::AvrDuration;
/// #
/// let mut a = AvrDuration::new(16_000_000, 3_000);
///
/// a -= AvrDuration::new(16_000_000, 2_000);
///
/// assert_eq!(AvrDuration::new(16_000_000, 1_000), a);
/// ```
///
/// ```
/// # use avr_simulator::AvrDuration;
/// #
/// let mut a = AvrDuration::new(16_000_000, 3_000);
///
/// a -= AvrDuration::new(16_000_000, 4_000);
///
/// assert_eq!(AvrDuration::new(16_000_000, 0), a);
/// ```
impl SubAssign for AvrDuration {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.clock_frequency, rhs.clock_frequency,
            "Cannot subtract durations with different clock frequencies ({} vs {})",
            self.clock_frequency, rhs.clock_frequency
        );

        self.cycles = self.cycles.saturating_sub(rhs.cycles);
    }
}

/// # Examples
///
/// ```rust
/// # use avr_simulator::AvrDuration;
/// #
/// let tt = AvrDuration::new(16_000_000, 0).add_millis(123);
///
/// assert_eq!("123000 µs", tt.to_string());
/// ```
impl fmt::Display for AvrDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} µs", self.as_micros())
    }
}
