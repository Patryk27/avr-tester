//! # Scenario
//!
//! We're given an AVR that implements a ROT13 encoder on UART0.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/uart/src/main.rs].
//! See also: [./spi.rs].

use crate::prelude::*;

#[test]
fn test() {
    let mut avr = avr("uart");

    avr.run_for_ms(1);

    assert_eq!("Ready!", avr.uart0().read::<String>());

    let mut assert = |given: &str, expected: &str| {
        avr.uart0().write(given);
        avr.run_for_ms(50);

        assert_eq!(expected, avr.uart0().read::<String>());
    };

    assert("Hello, World!", "Uryyb, Jbeyq!");

    assert(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent \
          non maximus purus. Fusce a neque condimentum, finibus dui et, tempor",
        "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg. Cenrfrag \
          aba znkvzhf chehf. Shfpr n ardhr pbaqvzraghz, svavohf qhv rg, grzcbe",
    );
}
