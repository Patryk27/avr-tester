//! See: [../../avr-tester/tests/examples/twi.rs].

#![no_std]
#![no_main]

#[cfg(feature = "custom-compiler-builtins")]
extern crate custom_compiler_builtins;

use atmega_hal::clock::MHz16;
use atmega_hal::usart::{BaudrateExt, Usart0};
use atmega_hal::{pins, Peripherals};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use panic_halt as _;

type I2c = atmega_hal::I2c<MHz16>;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut i2c = I2c::with_external_pullup(
        dp.TWI,
        pins.pc4.into_floating_input(),
        pins.pc5.into_floating_input(),
        100000,
    );

    // Write a couple of bytes
    i2c.write(123, &[0, 0xca]).unwrap();
    i2c.write(123, &[1, 0xfe]).unwrap();
    i2c.write(123, &[2, 0xba]).unwrap();
    i2c.write(123, &[3, 0xbe]).unwrap();

    // Issue a write at a special address of 255, which causes our simulated TWI
    // RAM to actually sum all numbers within the memory and return that value
    let mut resp = [0];

    i2c.write_read(123, &[255], &mut resp).unwrap();

    // Return outcome (the "summed RAM") via UART, for the test to assert
    let mut uart = Usart0::<MHz16>::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        115200u32.into_baudrate(),
    );

    uart.write_byte(resp[0]);

    loop {
        //
    }
}
