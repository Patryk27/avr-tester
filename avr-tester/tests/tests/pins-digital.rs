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

/// If our AVR blinked twice as fast (switching the pin every 50 ms), the
/// previous test would've succeeded anyway - so it's a bit imprecise.
///
/// A bit better approach to measure the cycle time is not to passively wait,
/// but to actively check how many milliseconds it took for the pin to toggle.
///
/// Note that it's a good practice to pair that with
/// [`AvrTesterBuilder::with_timeout()`], so that the test doesn't block forever
/// if the firmware misbehaves (see: [`test_precise_on_wrong_pin()`]).
#[test]
fn test_precise() {
    let mut avr = avr_with("pins-digital", |avr| {
        avr.with_timeout_of_ms(
            0
            + 100 // Expected time for the pin to go high
            + 100 // Expected time for the pin to go low
            + 10, // Some leeway, just in case
        )
    });

    let mut delay_time = 0;

    // Wait for the pin to get high
    while avr.pins().pd0().is_low() {
        avr.run();
    }

    // Wait for the pin to get low
    while avr.pins().pd0().is_high() {
        delay_time += 1;
        avr.run_for_ms(1);
    }

    // Wait for the pin to get high
    while avr.pins().pd0().is_low() {
        delay_time += 1;
        avr.run_for_ms(1);
    }

    // Ensure we've got a 200 ms cycle
    assert_eq!(200, delay_time);
}

/// Similar to [`test_precise()`], but in this case we "accidentally" assert the
/// wrong pin, causing the configured timeout to kick-in.
///
/// (and so without specifying the timeout, the test would run forever.)
#[test]
#[should_panic(expected = "Test timed-out")]
fn test_pricise_on_wrong_pin() {
    let mut avr = avr_with("pins-digital", |avr| avr.with_timeout_of_ms(100));

    while avr.pins().pd1().is_low() {
        avr.run();
    }
}
