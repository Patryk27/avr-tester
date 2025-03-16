//! See: [../../avr-tester/tests/examples/timeout.rs].

#![no_std]
#![no_main]

#[cfg(feature = "custom-compiler-builtins")]
extern crate custom_compiler_builtins;

use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    loop {
        //
    }
}
