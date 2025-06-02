//! See: [../../avr-tester/tests/examples/spi_master.rs].

#![no_std]
#![no_main]

#[cfg(feature = "custom-compiler-builtins")]
extern crate custom_compiler_builtins;

use atmega_hal::{Peripherals, Spi, pins, spi};
use avr_hal_generic::nb;
use avr_hal_generic::void::ResultVoidExt;
use embedded_hal::spi::FullDuplex;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let (mut spi, _) = Spi::new(
        dp.SPI,
        pins.pb5.into_output(),
        pins.pb3.into_output(),
        pins.pb4.into_pull_up_input(),
        pins.pb2.into_output(),
        spi::Settings::default(),
    );

    for c in "Ready!".bytes() {
        nb::block!(spi.send(c)).void_unwrap();
    }

    loop {
        let c = nb::block!(spi.read()).void_unwrap();
        let c = rot13(c);

        nb::block!(spi.send(c)).void_unwrap();
    }
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
