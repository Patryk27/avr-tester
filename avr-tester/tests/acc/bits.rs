//! We're given an AVR that waits for pulse on `PB0` and then increases an
//! internal 4-bit counter, lightning up pins as bits in that counter are
//! activated and deactivated.
//!
//! This test makes sure that avr-tester doesn't mix up pins and ports (e.g. by
//! accidentally confusing `PB1` with `PB0`).
//!
//! See: [../../../avr-tester-fixtures/src/acc_bits.rs].

use avr_tester::AvrTester;

#[test]
fn test() {
    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/acc-bits.elf");

    avr.run_for_us(1);
    avr.pins().pb1().assert_low();
    avr.pins().pb6().assert_low();
    avr.pins().pc3().assert_low();
    avr.pins().pd7().assert_low();

    for i in 0..=2u16.pow(4) {
        dbg!(i);

        avr.pins().pb0().set_high();
        avr.run_for_us(100);
        avr.pins().pb0().set_low();
        avr.run_for_us(100);

        avr.pins().pb1().assert((i & 0b0001) > 0);
        avr.pins().pb6().assert((i & 0b0010) > 0);
        avr.pins().pc3().assert((i & 0b0100) > 0);
        avr.pins().pd7().assert((i & 0b1000) > 0);
    }
}
