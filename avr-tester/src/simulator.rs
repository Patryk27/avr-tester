mod adcs;
mod avr;
mod cpu_duration;
mod cpu_state;
mod elf_firmware;
mod ioctl;
mod logs;
mod ports;
mod uart;

use self::{adcs::*, avr::*, elf_firmware::*, ioctl::*, ports::*, uart::*};
use simavr_ffi as ffi;
use std::path::Path;

pub use self::{cpu_duration::*, cpu_state::*};

/// Middle-ground between simavr and AvrTester; provides access to the UARTs,
/// pins etc. without wrapping them in a beautiful plastic foil.
pub struct AvrSimulator {
    avr: Avr,
    adcs: Option<Adcs>,
    ports: Ports,
    uarts: [Option<Uart>; 2],
}

impl AvrSimulator {
    pub fn new(mcu: &str, frequency: u32) -> Self {
        logs::init();

        let mut avr = Avr::new(mcu).init(frequency);
        let adcs = Adcs::new().try_init(&mut avr);
        let ports = Ports::new();
        let uart0 = Uart::new('0').try_init(&mut avr);
        let uart1 = Uart::new('1').try_init(&mut avr);

        Self {
            avr,
            adcs,
            ports,
            uarts: [uart0, uart1],
        }
    }

    pub fn flash(&mut self, path: impl AsRef<Path>) {
        ElfFirmware::new().load(path).flash_on(&mut self.avr);
    }

    pub fn run(&mut self) -> (CpuState, CpuDuration) {
        for uart in self.uarts.iter_mut().flatten() {
            uart.flush(&mut self.avr);
        }

        self.avr.run()
    }

    pub fn uart_recv(&mut self, id: usize) -> Option<u8> {
        self.uart(id).recv()
    }

    pub fn uart_send(&mut self, id: usize, byte: u8) {
        self.uart(id).send(byte)
    }

    pub fn set_pin_high(&mut self, port: char, pin: u8, high: bool) {
        self.ports.set_pin_high(&mut self.avr, port, pin, high);
    }

    pub fn is_pin_high(&mut self, port: char, pin: u8) -> bool {
        self.ports.is_pin_high(&mut self.avr, port, pin)
    }

    pub fn set_adc_voltage(&mut self, pin: u8, voltage: u32) {
        self.adcs
            .as_mut()
            .expect("Chosen AVR doesn't support ADC")
            .set_voltage(pin, voltage);
    }

    fn uart(&mut self, id: usize) -> &mut Uart {
        self.uarts
            .get_mut(id)
            .and_then(|uart| uart.as_mut())
            .unwrap_or_else(|| panic!("Chosen AVR doesn't support UART{}", id))
    }
}
