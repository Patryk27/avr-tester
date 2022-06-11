//! # Scenario
//!
//! In this test we're given an AVR that constantly polls `ADC2`, turning `PD0`
//! on if the read analog value is greater than 123.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/pins-analog/src/main.rs].

use crate::prelude::*;

#[test]
fn test() {
    let mut avr = avr("pins-analog");

    avr.run_for_ms(1);

    let assertions = [
        // (for this amount of millivolts, should output be high or low?),
        (0, false),
        (50, false),
        (100, false),
        (140, true),
        (150, true),
        (1024, true),
    ];

    for (millivolts, expected_pd0) in assertions {
        dbg!(&millivolts);

        avr.pins().adc2().set_mv(millivolts);
        avr.run_for_ms(1);
        avr.pins().pd0().assert(expected_pd0);
    }
}
