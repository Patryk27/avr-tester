//! # Scenario
//!
//! In this test we're given an AVR that does nothing, while we think it should
//! toggle a pin on-and-off. By carefully constructing the test, using timeout,
//! we prevent it from running forever, waiting for a toggle that will never
//! happen.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/timeout/src/main.rs].

use crate::prelude::*;

#[test]
#[should_panic(expected = "Test timed-out")]
fn test() {
    let mut avr = avr_with("timeout", |avr| {
        // We think our AVR should complete in 100ms, so:
        avr.with_timeout_of_ms(100)
    });

    avr.pins().pd0().pulse_in();

    // Without the timeout, `.pulse_in()` would get stuck forever (well, until
    // someone Ctrl+C'd and stopped `cargo test`), since it couldn't possibly
    // know whether the AVR would have eventually toggled the pin or not.
}
