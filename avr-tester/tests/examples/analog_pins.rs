//! This example shows how you can test analog pins.
//!
//! We're given an AVR that constantly polls `ADC2`, turning `PD0` on if the
//! read analog value is greater than 123.
//!
//! See: [../../../avr-tester-fixtures/src/analog_pins.rs].

use avr_tester::AvrTester;

#[test]
fn test() {
    let mut avr = AvrTester::atmega328().with_clock_of_16_mhz().load(
        "../avr-tester-fixtures/target/atmega328p/release/analog-pins.elf",
    );

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
