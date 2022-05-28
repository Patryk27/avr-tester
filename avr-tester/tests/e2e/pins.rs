use crate::prelude::*;

#[test]
fn test() {
    let firmware = build("pins");
    let mut avr = AvrTester::atmega328p(firmware, 16_000_000);

    avr.run_for_ms(1);
    avr.pins().pb1().assert_low();
    avr.pins().pc2().assert_low();
    avr.pins().pd3().assert_low();

    for i in 0..=8 {
        dbg!(i);

        avr.pins().pb0().set_high();
        avr.run_for_ms(1);
        avr.pins().pb0().set_low();
        avr.run_for_ms(1);

        avr.pins().pb1().assert((i & 0b001) > 0);
        avr.pins().pc2().assert((i & 0b010) > 0);
        avr.pins().pd3().assert((i & 0b100) > 0);
    }
}
