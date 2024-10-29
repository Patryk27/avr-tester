//! See: [../../avr-tester/tests/examples/shift_register.rs].

#![no_std]
#![no_main]

use atmega_hal::{pins, Peripherals};
use avr_hal_generic::prelude::*;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut pb0 = pins.pb0.into_output();
    let mut pb1 = pins.pb1.into_output();

    // Transmits a single bit to the shift register
    let mut out_bool = |val: bool| {
        let _ = pb1.set_state(val.into());

        pb0.set_high();
        pb0.set_low();
    };

    // Transmits eight bits to the shift register
    let mut out_u8 = |val: u8| {
        for n in 0..8 {
            out_bool(val & (2 << n - 1) > 0);
        }
    };

    out_u8(0xCA);
    out_u8(0xFE);
    out_u8(0xBA);
    out_u8(0xBE);

    loop {
        //
    }
}
