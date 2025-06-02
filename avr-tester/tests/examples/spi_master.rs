//! This example shows how you can test SPIs.
//!
//! We're given an AVR that implements a ROT13 encoder and we basically assert
//! that the encoder works.
//!
//! See: [../../../avr-tester-fixtures/src/spi_master.rs].

use avr_tester::AvrTester;

#[test]
fn test() {
    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/spi_master.elf");

    avr.spi0().write([0u8; 5]);
    avr.spi0().write("Hello, World!");
    avr.spi0().write(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent \
    non maximus purus. Fusce a neque condimentum, finibus dui et, tempor",
    );

    avr.run_for_ms(10);

    assert_eq!(Ok("Ready!"), str::from_utf8(&avr.spi0().read::<[u8; 6]>()));
    assert_eq!(
        Ok("Uryyb, Jbeyq!"),
        str::from_utf8(&avr.spi0().read::<[u8; 13]>())
    );
    assert_eq!(
        Ok(
            "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg. Cenrfrag \
        aba znkvzhf chehf. Shfpr n ardhr pbaqvzraghz, svavohf qhv rg, grzcbe"
        ),
        str::from_utf8(&avr.spi0().read::<[u8; 134]>())
    );
}
