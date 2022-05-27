use crate::prelude::*;

#[test]
fn test() {
    let firmware = build("pins");
    let mut avr = AvrTester::atmega328p(firmware, 16_000_000);

    avr.pins().pd0().assert_low();
    avr.run_for_ms(50);
    avr.pins().pd0().assert_high();
    avr.run_for_ms(50);
    avr.pins().pd0().assert_low();
    avr.run_for_ms(50);
    avr.pins().pd0().assert_high();
    avr.run_for_ms(50);
    avr.pins().pd0().assert_low();
}
