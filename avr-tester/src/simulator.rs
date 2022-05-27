mod avr;
mod cpu_cycles_taken;
mod cpu_state;
mod elf_firmware;
mod ioctl;
mod port;
mod uart;

use self::{avr::*, elf_firmware::*, ioctl::*, port::*, uart::*};
use std::path::Path;

pub use self::{cpu_cycles_taken::*, cpu_state::*};

pub struct AvrSimulator {
    avr: Avr,
    uarts: [Option<Uart>; 2],
}

impl AvrSimulator {
    pub fn new(mcu: &'static str, clock: u32) -> Self {
        let mut avr = Avr::new(mcu).init(clock);
        let uart0 = Uart::new(0).try_init(&mut avr);
        let uart1 = Uart::new(1).try_init(&mut avr);

        Self {
            avr,
            uarts: [uart0, uart1],
        }
    }

    pub fn flash(&mut self, path: impl AsRef<Path>) {
        ElfFirmware::new().load(path).flash_to(&mut self.avr);
    }

    pub fn run(&mut self) -> (CpuState, CpuCyclesTaken) {
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

    pub fn is_pin_high(&mut self, port: u8, pin: u8) -> bool {
        Port::is_pin_high(&mut self.avr, port, pin)
    }

    // pub fn write_pin(&mut self, _port: u8, _pin: u8) {
    //     todo!()

    //     // avr_raise_irq(avr_io_getirq(avr, AVR_IOCTL_IOPORT_GETIRQ('D'), 1), 1);
    // }

    fn uart(&mut self, id: usize) -> &mut Uart {
        self.uarts
            .get_mut(id)
            .and_then(|uart| uart.as_mut())
            .unwrap_or_else(|| panic!("Selected AVR doesn't support UART{}", id))
    }
}
