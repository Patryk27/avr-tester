#![no_std]
#![no_main]

use atmega_hal::clock::MHz16;
use atmega_hal::usart::{BaudrateExt, Usart0};
use atmega_hal::{pins, Peripherals};
use panic_halt as _;
use ufmt::uwrite;
use void::ResultVoidExt;

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

    uwrite!(&mut uart, "Ready!").void_unwrap();

    loop {
        // Retrieve message
        let len = uart.read_byte();
        let mut msg = [0; 250];

        for i in 0..len {
            msg[i as usize] = uart.read_byte();
        }

        // Send message back, rot13-ed
        for i in 0..len {
            uwrite!(&mut uart, "{}", rot13(msg[i as usize]) as char).void_unwrap();
        }
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
