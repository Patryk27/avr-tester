//! See: [../../avr-tester/tests/examples/spi_component.rs].

#![no_std]
#![no_main]

#[cfg(feature = "custom-compiler-builtins")]
extern crate custom_compiler_builtins;

use atmega_hal::clock::MHz16;
use atmega_hal::delay::Delay;
use atmega_hal::{pins, Peripherals};
use atmega_hal::{spi, Spi};
use avr_hal_generic::nb;
use avr_hal_generic::prelude::*;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut delay = Delay::<MHz16>::new();
    let mut cs = pins.pd0.into_output();

    let (mut spi, _) = Spi::new(
        dp.SPI,
        pins.pb5.into_output(),
        pins.pb3.into_output(),
        pins.pb4.into_pull_up_input(),
        pins.pb2.into_output(),
        spi::Settings::default(),
    );

    let mut n = 0;

    loop {
        cs.set_high();
        nb::block!(spi.send(n)).void_unwrap();
        delay.delay_us(5u8);

        cs.set_low();
        nb::block!(spi.send(0xCA)).void_unwrap();
        nb::block!(spi.send(0xFE)).void_unwrap();
        nb::block!(spi.send(0xBA)).void_unwrap();
        nb::block!(spi.send(0xBE)).void_unwrap();
        delay.delay_us(5u8);

        n += 1;
    }
}
