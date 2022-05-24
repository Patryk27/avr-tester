mod avr;
mod cpu_state;
mod elf_firmware;
mod ioctl;
mod uart;

pub use self::cpu_state::*;
use self::{avr::*, elf_firmware::*, ioctl::*, uart::*};
use std::path::Path;

pub struct AvrSimulator {
    avr: Avr,
    uart0: Uart,
}

impl AvrSimulator {
    pub fn new(mcu: &'static str, clock: u32) -> Self {
        let mut avr = Avr::new(mcu).init(clock);
        let uart0 = Uart::new(0).init(&mut avr);

        Self { avr, uart0 }
    }

    pub fn flash(&mut self, path: impl AsRef<Path>) {
        ElfFirmware::new().load(path).flash_to(&mut self.avr);
    }

    pub fn run(&mut self) -> CpuState {
        self.uart0.flush(&mut self.avr);
        self.avr.run()
    }

    pub fn uart0_recv(&mut self) -> Option<u8> {
        self.uart0.recv()
    }

    pub fn uart0_send(&mut self, byte: u8) {
        self.uart0.send(byte);
    }

    // pub fn read_pin(&self, _port: u8, _pin: u8) -> bool {
    //     todo!()

    //     // let mut state = ffi::avr_ioport_state_t {
    //     //     _bitfield_align_1: Default::default(),
    //     //     _bitfield_1: Default::default(),
    //     //     __bindgen_padding_0: Default::default(),
    //     // };

    //     // let status = unsafe {
    //     //     ffi::avr_ioctl(
    //     //         self.avr.ptr,
    //     //         Self::ioctl([b'i', b'o', b's', port]),
    //     //         &mut state as *mut _ as *mut c_void,
    //     //     )
    //     // };

    //     // if status == -1 {
    //     //     panic!("avr_ioctl() failed (status = {})", status);
    //     // }

    //     // // println!("{:#?}", state);

    //     // let port = state._bitfield_1.get(8, 8) as u8;

    //     // port & (1 << pin) > 0
    // }

    // pub fn write_pin(&mut self, _port: u8, _pin: u8) {
    //     todo!()

    //     // avr_raise_irq(avr_io_getirq(avr, AVR_IOCTL_IOPORT_GETIRQ('D'), 1), 1);
    // }
}
