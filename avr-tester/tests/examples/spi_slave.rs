//! This example shows how you can test SPIs.
//!
//! We're given an AVR that implements a ROT13 encoder and we basically assert
//! that the encoder works.
//!
//! See: [../../avr-tester-fixtures/src/spi_slave.rs].

use avr_tester::AvrTester;

#[test]
fn test() {
    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/spi_slave.elf");

    // The avr is acting as a slave device, so the tester must emulate a master device
    avr.spi0().set_master();

    avr.spi0().write("Hello, World!");
    avr.spi0().write(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent \
    non maximus purus. Fusce a neque condimentum, finibus dui et, tempor",
    );

    // write a terminating byte so we can get the last result byte back
    avr.spi0().write("\0");

    avr.run_for_ms(10);

    assert_eq!(Ok("\0"), str::from_utf8(&avr.spi0().read::<[u8; 1]>()));

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
