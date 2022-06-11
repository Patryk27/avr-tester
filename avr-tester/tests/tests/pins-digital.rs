//! # Scenario
//!
//! In this test we're given an AVR that toggles `PD0` on and off every 100
//! milliseconds.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/pins-digital/src/main.rs].

use crate::prelude::*;

#[test]
fn test_simple() {
    let mut avr = avr("pins-digital");

    avr.pins().pd0().assert_low();

    avr.run_for_ms(1);
    avr.pins().pd0().assert_high();

    avr.run_for_ms(100);
    avr.pins().pd0().assert_low();

    avr.run_for_ms(100);
    avr.pins().pd0().assert_high();

    avr.run_for_ms(100);
    avr.pins().pd0().assert_low();
}

/// Note that if our AVR blinked e.g. twice as fast (with a 50ms cycle), the
/// previous test would've succeeded anyway - so it's a bit imprecise.
///
/// A bit better approach to measure the cycle time is not to passively wait,
/// but to actively check how many milliseconds it took for the pin to toggle:
#[test]
fn test_precise() {
    let mut avr = avr("pins-digital");

    // Wait for the pin to get high
    while avr.pins().pd0().is_low() {
        avr.run();
    }

    // Wait for the pin to get low, measuring how many milliseconds it took
    let mut delay_time = 0;

    while avr.pins().pd0().is_high() {
        delay_time += 1;
        avr.run_for_ms(1);
    }

    // Ensure we've got a 100ms cycle
    assert_eq!(100, delay_time);
}
