//! Bare-bones wrapper for simavr.
//!
//! The main purpose of this crate is to serve as a building block for
//! AvrTester - so while this crate does provide a somewhat high-level interface
//! over simavr, this interface is limited and curated (i.e. not as generic as
//! simavr itself).
//!
//! See: [`AvrSimulator::new()`].

mod adc;
mod avr;
mod duration;
mod firmware;
mod ioctl;
mod logging;
mod port;
mod spi;
mod state;
mod uart;

use self::{adc::*, avr::*, firmware::*, ioctl::*, port::*, spi::*, uart::*};
use std::{
    collections::{hash_map::Entry, HashMap},
    path::Path,
};

pub use self::{duration::*, state::*};
pub use simavr_ffi as ffi;

/// Bare-bones wrapper for simavr.
#[derive(Debug)]
pub struct AvrSimulator {
    avr: Avr,
    adc: Option<Adc>,
    spis: HashMap<u8, Spi>,
    uarts: HashMap<char, Uart>,
}

impl AvrSimulator {
    pub fn new(mcu: &str, frequency: u32, firmware: impl AsRef<Path>) -> Self {
        logging::init();

        let mut avr = Avr::new(mcu, frequency);

        // Safety: `avr` lives as long as `adc`
        let adc = unsafe { Adc::new(&mut avr) };

        Firmware::new().load_elf(firmware).flash_to(&mut avr);

        Self {
            avr,
            adc,
            spis: Default::default(),
            uarts: Default::default(),
        }
    }

    /// Executes a single instruction.
    pub fn step(&mut self) -> StepOutcome {
        for spi in self.spis.values_mut() {
            spi.flush();
        }

        for uart in self.uarts.values_mut() {
            uart.flush();
        }

        let cycle = self.avr.cycle();
        let state = self.avr.run();
        let tt = (self.avr.cycle() - cycle).max(1);
        let tt = AvrDuration::new(self.avr.frequency(), tt);

        for spi in self.spis.values_mut() {
            spi.tick(tt.as_cycles());
        }

        StepOutcome { state, tt }
    }

    pub fn read_spi(&mut self, id: u8) -> Option<u8> {
        self.spi(id).read()
    }

    pub fn write_spi(&mut self, id: u8, byte: u8) {
        self.spi(id).write(byte)
    }

    pub fn read_uart(&mut self, id: char) -> Option<u8> {
        self.uart(id).read()
    }

    pub fn write_uart(&mut self, id: char, byte: u8) {
        self.uart(id).write(byte)
    }

    pub fn get_digital_pin(&mut self, port: char, pin: u8) -> bool {
        Port::get_pin(&mut self.avr, port, pin)
    }

    pub fn set_digital_pin(&mut self, port: char, pin: u8, high: bool) {
        Port::set_pin(&mut self.avr, port, pin, high);
    }

    pub fn set_analog_pin(&mut self, pin: u8, voltage: u32) {
        self.adc
            .as_mut()
            .expect("Current AVR doesn't have ADC")
            .set_voltage(pin, voltage);
    }

    fn spi(&mut self, id: u8) -> &mut Spi {
        if let Entry::Vacant(entry) = self.spis.entry(id) {
            // Safety: `self.avr` lives as long as `spi`
            let spi = unsafe { Spi::new(id, &mut self.avr) };

            if let Some(spi) = spi {
                entry.insert(spi);
            }
        }

        self.spis
            .get_mut(&id)
            .unwrap_or_else(|| panic!("Current AVR doesn't have SPI{}", id))
    }

    fn uart(&mut self, id: char) -> &mut Uart {
        if let Entry::Vacant(entry) = self.uarts.entry(id) {
            // Safety: `self.avr` lives as long as `uart`
            let uart = unsafe { Uart::new(id, &mut self.avr) };

            if let Some(uart) = uart {
                entry.insert(uart);
            }
        }

        self.uarts
            .get_mut(&id)
            .unwrap_or_else(|| panic!("Current AVR doesn't support UART{}", id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepOutcome {
    /// AVR's state after the instruction
    pub state: AvrState,

    /// How long it took to execute the instruction, in AVR cycles; approximate;
    /// always greater than zero
    pub tt: AvrDuration,
}
