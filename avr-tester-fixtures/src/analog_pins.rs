//! See: [../../avr-tester/tests/examples/analog_pins.rs].

#![no_std]
#![no_main]

#[cfg(feature = "custom-compiler-builtins")]
extern crate custom_compiler_builtins;

use atmega_hal::adc::{AdcSettings, ReferenceVoltage};
use atmega_hal::clock::MHz16;
use atmega_hal::{pins, Adc, Peripherals};
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut adc = Adc::<MHz16>::new(
        dp.ADC,
        AdcSettings {
            ref_voltage: ReferenceVoltage::Internal,
            ..Default::default()
        },
    );

    let input = pins.pc2.into_analog_input(&mut adc);
    let mut output = pins.pd0.into_output();

    loop {
        if input.analog_read(&mut adc) > 123 {
            output.set_high();
        } else {
            output.set_low();
        }
    }
}
