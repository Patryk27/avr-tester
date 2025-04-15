//! This example shows how you can test SPIs.
//!
//! We're given an AVR that implements a ROT13 encoder and we basically assert
//! that the encoder works.
//!
//! See: [../../../avr-tester-fixtures/spi/src/main.rs].

use avr_tester::AvrTester;

#[test]
fn test() {
    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/spi.elf");

    avr.run_for_ms(1);

    assert_eq!("Ready!", avr.spi0().read::<String>());

    let mut assert = |given: &str, expected: &str| {
        avr.spi0().write(given);
        avr.run_for_ms(50);

        assert_eq!(expected, avr.spi0().read::<String>());
    };

    assert("Hello, World!", "Uryyb, Jbeyq!");

    assert(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent \
          non maximus purus. Fusce a neque condimentum, finibus dui et, tempor",
        "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg. Cenrfrag \
          aba znkvzhf chehf. Shfpr n ardhr pbaqvzraghz, svavohf qhv rg, grzcbe",
    );
}
