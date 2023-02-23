//! # Scenario
//!
//! We're given an AVR that constantly polls `ADC2`, turning `PD0` on if the
//! read analog value is greater than 123.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/pins-analog/src/main.rs].

use crate::prelude::*;

#[test]
fn test() {
    let mut avr = avr("pins-analog");

    // Give the firmware a moment to initialize
    avr.run_for_ms(1);

    let assertions = [
        // (for this amount of millivolts, should output be high or low?),
        (0, false),
        (50, false),
        (100, false),
        (120, false),
        (150, true),
        (256, true),
        (1024, true),
    ];

    for (voltage, expected_pd0) in assertions {
        dbg!(&voltage);

        avr.pins().adc2().set_mv(voltage);
        avr.run_for_ms(1);
        avr.pins().pd0().assert(expected_pd0);
    }
}
