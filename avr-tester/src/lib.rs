//! Functional testing framework for [AVR] binaries, powered by [simavr]:
//!
//! ```no_run
//! use avr_tester::*;
//!
//! // Assuming `yourproject` implements an ROT-13 encoder:
//!
//! #[test]
//! fn test() {
//!     let mut avr = AvrTester::atmega328p()
//!         .with_clock_of_16_mhz()
//!         .load("../../yourproject/target/atmega328p/release/yourproject.elf");
//!
//!     // Let's give our firmware a moment to initialize:
//!     avr.run_for_ms(1);
//!
//!     // Now, let's send the string:
//!     avr.uart0().write("Hello, World!");
//!
//!     // ... give the AVR a moment to retrieve it & send back, encoded:
//!     avr.run_for_ms(1);
//!
//!     // ... and, finally, let's assert the outcome:
//!     assert_eq!("Uryyb, Jbeyq!", avr.uart0().read::<String>());
//! }
//! ```
//!
//! For more details, please see README.
//!
//! [AVR]: https://en.wikipedia.org/wiki/AVR_microcontrollers
//! [simavr]: https://github.com/buserror/simavr

mod builder;
mod components;
mod duration_ext;
mod pins;
mod read;
mod spi;
mod uart;
mod utils;
mod write;

use avr_simulator::{AvrSimulator, AvrState};
use std::{marker::PhantomData, path::Path};

pub use self::{
    builder::*, components::*, duration_ext::*, pins::*, read::*, spi::*, uart::*, utils::*,
    write::*,
};
pub use avr_simulator::AvrDuration;

/// Simulator's entry point; you can build it using [`AvrTester::atmega328p()`]
/// or a similar function.
#[derive(Debug)]
pub struct AvrTester {
    sim: Option<AvrSimulator>,
    clock_frequency: u32,
    remaining_clock_cycles: Option<u64>,
    components: Components,
}

impl AvrTester {
    pub(crate) fn new(
        mcu: &str,
        clock_frequency: u32,
        firmware: impl AsRef<Path>,
        remaining_clock_cycles: Option<u64>,
    ) -> Self {
        Self {
            sim: Some(AvrSimulator::new(mcu, clock_frequency, firmware)),
            clock_frequency,
            remaining_clock_cycles,
            components: Components::new(),
        }
    }

    /// Runs a full single instruction, returning the number of cycles it took
    /// to execute that instruction (e.g. `MUL` takes two cycles or so).
    ///
    /// Note that the number returned here is somewhat approximate (see
    /// [`AvrDuration`]), but it's guaranteed to be at least one cycle.
    ///
    /// See also:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_ms()`],
    /// - [`Self::run_for_us()`].
    pub fn run(&mut self) -> AvrDuration {
        let step = self.sim().step();

        self.components
            .run(&mut self.sim, self.clock_frequency, step.tt);

        if let Some(remaining_clock_cycles) = &mut self.remaining_clock_cycles {
            *remaining_clock_cycles = remaining_clock_cycles.saturating_sub(step.tt.as_cycles());

            if *remaining_clock_cycles == 0 {
                panic!("Test timed-out");
            }
        }

        match step.state {
            AvrState::Running => {
                //
            }

            AvrState::Crashed => {
                panic!(
                    "AVR crashed (e.g. the program stepped on an invalid \
                     instruction)"
                );
            }

            AvrState::Sleeping => {
                panic!(
                    "AVR went to sleep (this panics, because AvrTester doesn't \
                     provide any way to wake up the microcontroller yet)"
                );
            }

            state => {
                panic!("Unexpected AvrState: {:?}", state);
            }
        }

        step.tt
    }

    /// Runs firmware for given number of cycles (when given [`u64`]) or given
    /// [`AvrDuration`].
    ///
    /// See also:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_ms()`],
    /// - [`Self::run_for_us()`].
    pub fn run_for(&mut self, n: impl IntoCycles) {
        let mut cycles = n.into_cycles();

        while cycles > 0 {
            cycles = cycles.saturating_sub(self.run().as_cycles());
        }
    }

    /// Runs firmware for given number of _AVR_ microseconds, considering the
    /// clock specified through [`AvrTesterBuilder::with_clock()`].
    ///
    /// See:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_ms()`].
    ///
    /// See also: [`Self::run()`].
    pub fn run_for_us(&mut self, n: u64) {
        self.run_for(AvrDuration::micros(self, n));
    }

    /// Runs firmware for given number of _AVR_ milliseconds, considering the
    /// clock specified through [`AvrTesterBuilder::with_clock()`].
    ///
    /// See:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_us()`].
    ///
    /// See also: [`Self::run()`].
    pub fn run_for_ms(&mut self, n: u64) {
        self.run_for(AvrDuration::millis(self, n));
    }

    /// Runs firmware for given number of _AVR_ seconds, considering the clock
    /// specified through [`AvrTesterBuilder::with_clock()`].
    ///
    /// See:
    ///
    /// - [`Self::run_for_ms()`],
    /// - [`Self::run_for_us()`].
    ///
    /// See also: [`Self::run()`].
    pub fn run_for_s(&mut self, n: u64) {
        self.run_for(AvrDuration::secs(self, n));
    }

    /// Returns an object providing read & write access to the analog & digital
    /// pins (such as `ADC1`, `PD4` etc.).
    ///
    /// Note that the returned object contains all possible pins for all of the
    /// existing AVRs, while the AVR of yours probably supports only a subset of
    /// those pins - trying to access a pin that does not exist for your AVR
    /// will panic.
    pub fn pins(&mut self) -> Pins<'_> {
        Pins::new(self)
    }

    /// Returns an object providing access to SPI0 (i.e. the default SPI).
    ///
    /// Note that if your AVR doesn't have SPI, operating on it will panic.
    pub fn spi0(&mut self) -> Spi<'_> {
        Spi::new(self.sim(), 0)
    }

    /// Returns an object providing access to SPI1.
    ///
    /// Note that if your AVR doesn't have SPI, operating on it will panic.
    pub fn spi1(&mut self) -> Spi<'_> {
        Spi::new(self.sim(), 1)
    }

    /// Returns an object providing access to UART0 (i.e. the default UART).
    ///
    /// Note that if your AVR doesn't have UART0, operating on it will panic.
    pub fn uart0(&mut self) -> Uart<'_> {
        Uart::new(self.sim(), '0')
    }

    /// Returns an object providing access to UART1.
    ///
    /// Note that if your AVR doesn't have UART1, operating on it will panic.
    pub fn uart1(&mut self) -> Uart<'_> {
        Uart::new(self.sim(), '1')
    }

    /// Returns an object providing acccess to components (aka _peripherals_)
    /// attached to the AVR.
    pub fn components(&mut self) -> &mut Components {
        &mut self.components
    }

    fn sim(&mut self) -> &mut AvrSimulator {
        self.sim
            .as_mut()
            .expect("AvrSimulator had been deallocated - has some component crashed?")
    }
}

/// Asynchronous equivalent of [`AvrTester`].
///
/// See [`avr_rt()`] for more details.
pub struct AvrTesterAsync {
    _pd: PhantomData<()>,
}

impl AvrTesterAsync {
    fn new() -> Self {
        Self {
            _pd: Default::default(),
        }
    }

    /// Asynchronous equivalent of [`AvrTester::run()`].
    ///
    /// See [`avr_rt()`] for more details.
    pub async fn run(&self) -> AvrDuration {
        ResumeFuture::new().await
    }

    /// Asynchronous equivalent of [`AvrTester::run_for()`].
    ///
    /// See [`avr_rt()`] for more details.
    pub async fn run_for(&self, n: impl IntoCycles) {
        let cycles = n.into_cycles();

        let fut = ComponentRuntime::with(|rt| {
            SleepFuture::new(AvrDuration::new(rt.clock_frequency(), cycles))
        });

        fut.await;
    }

    /// Asynchronous equivalent of [`AvrTester::run_for_us()`].
    ///
    /// See [`avr_rt()`] for more details.
    pub async fn run_for_us(&self, n: u64) {
        let fut = ComponentRuntime::with(|rt| {
            SleepFuture::new(AvrDuration::new(rt.clock_frequency(), 0).with_micros(n))
        });

        fut.await;
    }

    /// Asynchronous equivalent of [`AvrTester::run_for_ms()`].
    ///
    /// See [`avr_rt()`] for more details.
    pub async fn run_for_ms(&self, n: u64) {
        let fut = ComponentRuntime::with(|rt| {
            SleepFuture::new(AvrDuration::new(rt.clock_frequency(), 0).with_millis(n))
        });

        fut.await;
    }

    /// Asynchronous equivalent of [`AvrTester::run_for_s()`].
    ///
    /// See [`avr_rt()`] for more details.
    pub async fn run_for_s(&self, n: u64) {
        let fut = ComponentRuntime::with(|rt| {
            SleepFuture::new(AvrDuration::new(rt.clock_frequency(), 0).with_secs(n))
        });

        fut.await;
    }

    /// Asynchronous equivalent of [`AvrTester::pins()`].
    ///
    /// See [`avr_rt()`] for more details.
    pub fn pins(&self) -> PinsAsync {
        PinsAsync::new()
    }
}

/// Returns [`AvrTesterAsync`] for usage inside **components**.
///
/// See [`Components`] for more details.
pub fn avr_rt() -> AvrTesterAsync {
    AvrTesterAsync::new()
}

macro_rules! constructors {
    ( $( $name:ident ),* $(,)? ) => {
        impl AvrTester {
            $(
                pub fn $name() -> AvrTesterBuilder {
                    AvrTesterBuilder::new(stringify!($name))
                }
            )*
        }
    }
}

constructors! {
    // sim_mega8.c
    atmega8, atmega81,

    // sim_mega16.c
    atmega16,

    // sim_mega16m1.c
    atmega16m1,

    // sim_mega32.c
    atmega32,

    // sim_mega32u4.c
    atmega32u4,

    // sim_mega48.c
    atmega48, atmega48p, atmega48pa,

    // sim_mega64m1.c
    atmega64m1,

    // sim_mega88.c
    atmega88, atmega88p, atmega88pa,

    // sim_mega128.c
    atmega128, atmega128l,

    // sim_mega128rfa1.c
    atmega128rfa1,

    // sim_mega128rfr2.c
    atmega128rfr2,

    // sim_mega164.c
    atmega164, atmega164p, atmega164pa,

    // sim_mega168.c
    atmega168, atmega168p, atmega168pa,

    // sim_mega169.c
    atmega169p,

    // sim_mega324.c
    atmega324, atmega324p,

    // sim_mega324a.c
    atmega324a, atmega324pa,

    // sim_mega328.c
    atmega328, atmega328p,

    // sim_mega328pb.c
    atmega328pb,

    // sim_mega644.c
    atmega644, atmega644p,

    // sim_mega1280.c
    atmega1280,

    // sim_mega1281.c
    atmega1281,

    // sim_mega1284.c
    atmega1284p, atmega1284,

    // sim_mega2560.c
    atmega2560, atmega2561,

    // sim_tiny13.c
    attiny13, attiny13a,

    // sim_tiny24.c
    attiny24,

    // sim_tiny25.c
    attiny25,

    // sim_tiny44.c
    attiny44,

    // sim_tiny45.c
    attiny45,

    // sim_tiny84.c
    attiny84,

    // sim_tiny85.c
    attiny85,

    // sim_tiny2313.c
    attiny2313, attiny2313v,

    // sim_tiny2313a.c
    attiny2313a,

    // sim_tiny4313.c
    attiny4313,
}
