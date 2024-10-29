//! See: [../../avr-tester/tests/examples/uart.rs].

#![no_std]
#![no_main]

use atmega_hal::clock::MHz16;
use atmega_hal::usart::{BaudrateExt, Usart0};
use atmega_hal::{pins, Peripherals};
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut uart = Usart0::<MHz16>::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        115200u32.into_baudrate(),
    );

    for c in "Ready!".bytes() {
        uart.write_byte(c);
    }

    loop {
        let c = uart.read_byte();
        let c = rot13(c);

        uart.write_byte(c);
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
