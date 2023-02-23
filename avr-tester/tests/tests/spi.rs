//! # Scenario
//!
//! We're given an AVR that implements a ROT13 encoder using the SPI interface.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/spi/src/main.rs].
//! See also: [./uart.rs].

use crate::prelude::*;

#[test]
fn test() {
    let mut avr = avr("spi");

    avr.run_for_ms(1);

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
