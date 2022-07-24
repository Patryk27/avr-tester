//! Functional testing framework for [AVR] binaries, powered by [simavr]:
//!
//! ```no_run
//! use avr_tester::AvrTester;
//!
//! // Assuming `yourproject` is a ROT-13 encoder, one could write a test such
//! // as this:
//!
//! #[test]
//! fn test() {
//!     let mut avr = AvrTester::atmega328p()
//!         .with_clock_of_16_mhz()
//!         .load("../../yourproject/target/atmega328p/release/yourproject.elf");
//!
//!     // Let's give our AVR a moment to initialize itself and UART:
//!     avr.run_for_ms(1);
//!
//!     // Now, let's send the string:
//!     avr.uart0().send("Hello, World!");
//!
//!     // ... give the AVR a moment to retrieve it & send back, encoded:
//!     avr.run_for_ms(1);
//!
//!     // ... and, finally, assert the outcome:
//!     assert_eq!("Uryyb, Jbeyq!", avr.uart0().recv::<String>());
//! }
//! ```
//!
//! [AVR]: https://en.wikipedia.org/wiki/AVR_microcontrollers
//! [simavr]: https://github.com/buserror/simavr
//!
//! For more details, please see: [./README.md].

#![feature(maybe_uninit_uninit_array)]

mod builder;
mod pins;
mod simulator;
mod uart;

use self::simulator::*;
use std::path::Path;

pub use self::{builder::*, pins::*, simulator::CpuCyclesTaken, uart::*};

/// Simulator's entry point; you can build it using [`AvrTester::atmega328p()`]
/// or a similar function.
pub struct AvrTester {
    sim: AvrSimulator,
    clock: u32,
}

impl AvrTester {
    pub(crate) fn new(mcu: &str, firmware: impl AsRef<Path>, clock: u32) -> Self {
        let mut sim = AvrSimulator::new(mcu, clock);

        sim.flash(firmware);

        Self { sim, clock }
    }

    /// Runs a full single instruction, returning the number of cycles it took
    /// to execute that instruction (e.g. `MUL` takes two cycles or so).
    ///
    /// Note that the number returned here is somewhat approximate (see:
    /// [`CpuCyclesTaken`]).
    ///
    /// See also:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_ms()`],
    /// - [`Self::run_for_us()`].
    pub fn run(&mut self) -> CpuCyclesTaken {
        let (state, cycles_taken) = self.sim.run();

        match state {
            CpuState::Running => {
                //
            }

            CpuState::Crashed => {
                panic!("AVR crashed (e.g. the program stepped on an invalid instruction)");
            }

            CpuState::Sleeping => {
                panic!(
                    "AVR went to sleep (this panics, because AvrTester doesn't \
                     provide any way to wake up the microcontroller yet)"
                );
            }

            state => {
                panic!("Unexpected CpuState: {:?}", state)
            }
        }

        cycles_taken
    }

    /// Runs code for given number of cycles.
    ///
    /// See also:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_ms()`],
    /// - [`Self::run_for_us()`].
    pub fn run_for(&mut self, mut cycles: u64) {
        while cycles > 0 {
            cycles = cycles.saturating_sub(self.run().get().max(1));
        }
    }

    /// Runs code for given number of _AVR_ seconds, considering the clock
    /// specified through [`AvrTesterBuilder::with_clock()`].
    ///
    /// See:
    ///
    /// - [`Self::run_for_ms()`],
    /// - [`Self::run_for_us()`].
    ///
    /// See also: [`Self::run()`].
    pub fn run_for_s(&mut self, s: u32) {
        let clock = self.clock as u64;
        let s = s as u64;

        self.run_for(clock * s);
    }

    /// Runs code for given number of _AVR_ milliseconds, considering the clock
    /// specified through [`AvrTesterBuilder::with_clock()`].
    ///
    /// See:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_us()`].
    ///
    /// See also: [`Self::run()`].
    pub fn run_for_ms(&mut self, ms: u32) {
        let clock = self.clock as f32;
        let ms = ms as f32;

        self.run_for((clock * ms / 1_000.0).ceil() as _);
    }

    /// Runs code for given number of _AVR_ microseconds, considering the clock
    /// specified through [`AvrTesterBuilder::with_clock()`].
    ///
    /// See:
    ///
    /// - [`Self::run_for_s()`],
    /// - [`Self::run_for_ms()`].
    ///
    /// See also: [`Self::run()`].
    pub fn run_for_us(&mut self, us: u32) {
        let clock = self.clock as f32;
        let us = us as f32;

        self.run_for((clock * us / 1_000_000.0).ceil() as _);
    }

    /// Returns an object providing access to the input and output pins (such as
    /// `ADC1`, `PD4` etc.).
    ///
    /// Note that the returned object contains all possible pins for all of the
    /// existing AVRs, while the AVR of yours probably supports only a subset of
    /// those pins - trying to access a pin that does not exist for your AVR
    /// will gracefully `panic!()`.
    pub fn pins(&mut self) -> Pins<'_> {
        Pins::new(&mut self.sim)
    }

    /// Returns an object providing access to UART0 (i.e. the default UART).
    ///
    /// Note that if your AVR doesn't support UART0, operating on it will
    /// gracefully `panic!()`.
    pub fn uart0(&mut self) -> Uart<'_> {
        Uart::new(&mut self.sim, 0)
    }

    /// Returns an object providing access to UART1.
    ///
    /// Note that if your AVR doesn't support UART1, operating on it will
    /// gracefully `panic!()`.
    pub fn uart1(&mut self) -> Uart<'_> {
        Uart::new(&mut self.sim, 1)
    }
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
