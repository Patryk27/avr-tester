//! See: [../../../avr-tester/tests/tests/xx/bits.rs].

#![no_std]
#![no_main]

use atmega_hal::{pins, Peripherals};
use avr_hal_generic::hal::digital::v2::OutputPin;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let input = pins.pb0.into_floating_input();
    let mut output1 = pins.pb1.into_output();
    let mut output2 = pins.pb6.into_output();
    let mut output3 = pins.pc3.into_output();
    let mut output4 = pins.pd7.into_output();

    let mut i = 0u16;

    loop {
        // Wait for pulse
        while input.is_low() {
            //
        }

        while input.is_high() {
            //
        }

        // Write the number
        let _ = output1.set_state((i & 0b0001 > 0).into());
        let _ = output2.set_state((i & 0b0010 > 0).into());
        let _ = output3.set_state((i & 0b0100 > 0).into());
        let _ = output4.set_state((i & 0b1000 > 0).into());

        i += 1;
    }
}
