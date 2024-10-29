//! Oxidized interface for simavr.
//!
//! The main purpose of this crate is to serve as a building block for the
//! `avr-tester` crate, providing a safe and curated access to simavr.
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

use self::adc::*;
use self::avr::*;
pub use self::duration::*;
use self::firmware::*;
use self::ioctl::*;
use self::port::*;
use self::spi::*;
pub use self::state::*;
use self::uart::*;
pub use simavr_ffi as ffi;
use std::collections::HashMap;
use std::path::Path;

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

        Firmware::new().load_elf(firmware).flash_to(&mut avr);

        // Safety: `avr` lives as long as `adc`, `spis` etc.
        let adc = unsafe { Adc::new(&avr) };
        let spis = unsafe { Self::init_spis(&avr) };
        let uarts = unsafe { Self::init_uarts(&mut avr) };

        Self {
            avr,
            adc,
            spis,
            uarts,
        }
    }

    unsafe fn init_spis(avr: &Avr) -> HashMap<u8, Spi> {
        let mut spis = HashMap::new();

        for id in 0..8 {
            if let Some(spi) = Spi::new(id, avr) {
                spis.insert(id, spi);
            } else {
                break;
            }
        }

        spis
    }

    unsafe fn init_uarts(avr: &mut Avr) -> HashMap<char, Uart> {
        let mut uarts = HashMap::new();

        for id in 0..8 {
            #[allow(clippy::char_lit_as_u8)]
            let id = (('0' as u8) + id) as char;

            if let Some(uart) = Uart::new(id, avr) {
                uarts.insert(id, uart);
            } else {
                break;
            }
        }

        uarts
    }

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
        Port::set_pin(&self.avr, port, pin, high);
    }

    pub fn set_analog_pin(&mut self, pin: u8, voltage: u32) {
        self.adc
            .as_mut()
            .expect("Current AVR doesn't have ADC")
            .set_voltage(pin, voltage);
    }

    fn spi(&mut self, id: u8) -> &mut Spi {
        self.spis
            .get_mut(&id)
            .unwrap_or_else(|| panic!("Current AVR doesn't have SPI{}", id))
    }

    fn uart(&mut self, id: char) -> &mut Uart {
        self.uarts
            .get_mut(&id)
            .unwrap_or_else(|| panic!("Current AVR doesn't have UART{}", id))
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
