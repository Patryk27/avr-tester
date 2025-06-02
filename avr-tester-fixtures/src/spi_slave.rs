//! See: [../../avr-tester/tests/examples/spi_slave.rs].

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

#[cfg(feature = "custom-compiler-builtins")]
extern crate custom_compiler_builtins;

use core::cell::RefCell;

use atmega_hal::pac::SPI;
use atmega_hal::{Peripherals, pins};
use avr_device::interrupt::Mutex;
use panic_halt as _;

static SPI_DEVICE: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let spi = dp.SPI;
    let pins = pins!(dp);

    // SPI slave mode is not supported by avr_hal, so we'll do it ourselves
    let _miso = pins.pb4.into_output();
    spi.spcr.write(|w| w.spe().set_bit().spie().set_bit());

    avr_device::interrupt::free(|cs| {
        let b = SPI_DEVICE.borrow(cs);
        b.borrow_mut().replace(spi);
    });

    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::asm::nop();
    }
}

#[avr_device::interrupt(atmega328p)]
#[allow(non_snake_case)]
unsafe fn SPI_STC() {
    avr_device::interrupt::free(|cs| {
        let guard = SPI_DEVICE.borrow(cs);
        if let Some(spi) = guard.borrow_mut().as_mut() {
            spi.spdr.modify(|r, w| unsafe { w.bits(rot13(r.bits())) });
        }
    });
}

fn rot13(c: u8) -> u8 {
    if c >= b'a' && c <= b'z' {
        b'a' + (c - b'a' + 13) % 26
    } else if c >= b'A' && c <= b'Z' {
        b'A' + (c - b'A' + 13) % 26
    } else {
        c
    }
}
