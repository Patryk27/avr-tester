//! # Scenario
//!
//! We're given an AVR that toggles `PD0` on and off alternately every 100 and
//! 150 milliseconds.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/pins-digital/src/main.rs].

use crate::prelude::*;

#[test]
fn simple() {
    let mut avr = avr("pins-digital");

    // Give the firmware a moment to initialize
    avr.run_for_ms(1);

    // Assert the first `for`
    avr.pins().pd0().assert_high();
    avr.run_for_ms(100);
    avr.pins().pd0().assert_low();
    avr.run_for_ms(100);

    // Assert the second `for`
    avr.pins().pd0().assert_high();
    avr.run_for_ms(150);
    avr.pins().pd0().assert_low();
    avr.run_for_ms(150);
    avr.pins().pd0().assert_high();

    // Assert the first `for` again
    avr.pins().pd0().assert_high();
    avr.run_for_ms(100);
    avr.pins().pd0().assert_low();
    avr.run_for_ms(100);
}

/// If our AVR blinked twice as fast (switching the pin every 50 ms & 75 ms),
/// the previous test would've succeeded anyway - so it's kinda imprecise.
///
/// A better approach would be not to passively wait & assert, but rather
/// constantly listen on pin, waiting for it to change.
///
/// Note that with this approach if the firmware misbehaves, our `while` could
/// loop forever - so it's a good practice to pair these kind of tests with
/// [`AvrTesterBuilder::with_timeout()`].
#[test]
fn precise() {
    let mut avr = avr_with("pins-digital", |avr| {
        avr.with_timeout_of_ms(
            100 // Expected time for the pin to go high
            + 100 // Expected time for the pin to go low
            + 10, // Some leeway, just in case
        )
    });

    // Give the firmware a moment to initialize
    avr.run_for_ms(1);

    // Wait for the pin to get low
    let mut time_taken_ms = 0;

    while avr.pins().pd0().is_high() {
        time_taken_ms += 1;
        avr.run_for_ms(1);
    }

    assert_eq!(99, time_taken_ms);

    // Wait for the pin to get high
    let mut time_taken_ms = 0;

    while avr.pins().pd0().is_low() {
        time_taken_ms += 1;
        avr.run_for_ms(1);
    }

    assert_eq!(100, time_taken_ms);
}

/// Similar to [`precise()`], but using built-in functions instead of
/// hand-written loops.
#[test]
fn precise_idiomatic() {
    let mut avr = avr_with("pins-digital", |avr| avr.with_timeout_of_s(2));

    // Give the firmware a moment to initialize
    avr.run_for_us(100);

    // Assert the first `for`
    assert_eq!(100, avr.pins().pd0().wait_while_high().as_millis());
    assert_eq!(100, avr.pins().pd0().wait_while_low().as_millis());

    // Assert the second `for`
    assert_eq!(150, avr.pins().pd0().wait_while_high().as_millis());
    assert_eq!(150, avr.pins().pd0().wait_while_low().as_millis());

    // Alternatively, using `pulse_in()`:
    assert_eq!(100, avr.pins().pd0().pulse_in().as_millis());
    assert_eq!(100, avr.pins().pd0().pulse_in().as_millis());
    assert_eq!(150, avr.pins().pd0().pulse_in().as_millis());
    assert_eq!(150, avr.pins().pd0().pulse_in().as_millis());

    // Wait for a pin change or timeout:
    let timeout = AvrDuration::millis(&avr, 50);
    assert_eq!(
        Err(50),
        avr.pins()
            .pd0()
            .wait_while_high_timeout(timeout)
            .map_err(|d| d.as_millis())
    );
    assert_eq!(
        Ok(50),
        avr.pins()
            .pd0()
            .wait_while_high_timeout(timeout)
            .map(|d| d.as_millis())
    );
    assert_eq!(
        Err(50),
        avr.pins()
            .pd0()
            .wait_while_low_timeout(timeout)
            .map_err(|d| d.as_millis())
    );
    assert_eq!(
        Ok(50),
        avr.pins()
            .pd0()
            .wait_while_low_timeout(timeout)
            .map(|d| d.as_millis())
    );
}

/// Similar to [`precise()`], but in this case we "accidentally" assert the
/// wrong pin, causing the configured timeout to kick-in.
///
/// (and so without specifying the timeout, the test would run forever.)
#[test]
#[should_panic(expected = "Test timed-out")]
fn precise_stuck() {
    let mut avr = avr_with("pins-digital", |avr| avr.with_timeout_of_ms(100));

    while avr.pins().pd1().is_low() {
        avr.run();
    }
}
