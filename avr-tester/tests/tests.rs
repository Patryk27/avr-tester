#![feature(exit_status_error)]

#[path = "tests/pins-analog.rs"]
mod pins_analog;

#[path = "tests/pins-digital.rs"]
mod pins_digital;

#[path = "tests/uart.rs"]
mod uart;

#[path = "tests/xx/bits.rs"]
mod xx_bits;

#[path = "tests/xx/eval.rs"]
mod xx_eval;

mod prelude {
    pub use super::{avr, avr_with};
    pub use avr_tester::*;
    pub use rand::prelude::*;
}

use avr_tester::{AvrTester, AvrTesterBuilder};
use std::{path::Path, process::Command};

/// Compiles `test` and returns `AvrTester` for it.
///
/// Note that this function has been written with `AvrTester`'s test suite in
/// mind - if you wanted to re-use it, consider doing something simpler - say:
///
/// ```
/// fn avr() -> AvrTester {
///     AvrTester::atmega328p()
///         .with_clock_of_16_mhz()
///         .load("../../yourproject/target/atmega328p/release/yourproject.elf")
/// }
/// ```
pub fn avr(test: &str) -> AvrTester {
    avr_with(test, |avr| avr)
}

/// See: [`avr()`].
pub fn avr_with(
    test: &str,
    configure: impl FnOnce(AvrTesterBuilder) -> AvrTesterBuilder,
) -> AvrTester {
    eprintln!("Building firmware");

    let tests_dir = Path::new("..").join("avr-tester-tests");
    let test_dir = tests_dir.join(test).canonicalize().unwrap();

    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(tests_dir.join(test_dir))
        .status()
        .expect("Couldn't build firmware")
        .exit_ok()
        .expect("Couldn't build firmware");

    let firmware = tests_dir
        .join("target")
        .join("atmega328p")
        .join("release")
        .join(format!("tests-{}.elf", test));

    eprintln!("Starting test");

    let avr = AvrTester::atmega328().with_clock_of_16_mhz();
    let avr = configure(avr);

    avr.load(firmware)
}
