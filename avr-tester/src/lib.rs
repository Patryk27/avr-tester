#![feature(maybe_uninit_uninit_array)]

mod pins;
mod simulator;
mod uart;

use self::simulator::*;
use std::path::Path;

pub use self::{pins::*, uart::*};

/// Comfortable wrapper over simavr [simavr](https://github.com/buserror/simavr)
/// that allows to easily test AVR code end-to-end.
///
/// # Example
///
/// TODO
pub struct AvrTester {
    sim: AvrSimulator,
    clock: u32,
}

impl AvrTester {
    pub fn new(mcu: &'static str, firmware: impl AsRef<Path>, clock: u32) -> Self {
        let mut sim = AvrSimulator::new(mcu, clock);

        sim.flash(firmware);

        Self { sim, clock }
    }

    /// Shorthand for `AvrTester::new("atmega328p", ...)`.
    pub fn atmega328p(firmware: impl AsRef<Path>, clock: u32) -> Self {
        Self::new("atmega328p", firmware, clock)
    }

    /// Runs a full single instruction, returning the number of cycles it took
    /// to execute that instruction (e.g. `MUL` takes two cycles or so).
    ///
    /// Note that the number returned here is somewhat approximate (see:
    /// [`CpuCyclesTaken`]).
    pub fn run(&mut self) -> CpuCyclesTaken {
        let (state, cycles_taken) = self.sim.run();

        if state != CpuState::Running {
            panic!("Unexpected CpuState: {:?}", state)
        }

        cycles_taken
    }

    /// Runs code for given number of _AVR_ milliseconds, considering the clock
    /// frequency passed into the constructor.
    pub fn run_for_ms(&mut self, ms: u32) {
        let mut cycles = (self.clock as u64) * (ms as u64) / 1000;

        while cycles > 0 {
            cycles = cycles.saturating_sub(self.run().get().min(1));
        }
    }

    /// Returns an object providing access to output / input pins.
    pub fn pins(&mut self) -> Pins<'_> {
        Pins::new(&mut self.sim)
    }

    /// Returns an object providing access to UART0 (i.e. the default UART).
    pub fn uart0(&mut self) -> Uart<'_> {
        Uart::new(&mut self.sim, 0)
    }

    /// Returns an object providing access to UART1.
    pub fn uart1(&mut self) -> Uart<'_> {
        Uart::new(&mut self.sim, 1)
    }
}
