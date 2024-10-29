//! This example shows how you can use timeouts to make your tests abort if they
//! take too long.
//!
//! We're given an AVR that does nothing, while we think it should toggle a pin
//! on-and-off. By carefully constructing the test, using timeout, we prevent it
//! from running forever, waiting for a toggle that will never happen.
//!
//! See: [../../../avr-tester-fixtures/timeout/src/main.rs].

use avr_tester::AvrTester;

#[test]
#[should_panic(expected = "Test timed-out")]
fn test() {
    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .with_timeout_of_ms(100) // We think our AVR should complete within 100ms
        .load("../avr-tester-fixtures/target/atmega328p/release/timeout.elf");

    avr.pins().pd0().pulse_in();

    // Without the timeout, `.pulse_in()` would get stuck forever (well, until
    // someone Ctrl+C'd and stopped `cargo test`), since it couldn't possibly
    // know whether the AVR would have eventually toggled the pin or not.
}
