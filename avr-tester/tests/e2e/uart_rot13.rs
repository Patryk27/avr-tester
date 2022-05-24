use crate::prelude::*;

#[test]
fn test() {
    let firmware = build("uart-rot13");
    let mut avr = AvrTester::atmega_328p(firmware, 16_000_000);

    avr.run_for_ms(2);
    assert_eq!(b"Ready!", avr.uart0().recv_bytes().as_slice());

    let mut assert = |given: &str, expected: &str| {
        avr.uart0().send_byte(given.len() as u8);
        avr.uart0().send_string(given);
        avr.run_for_ms(50);

        assert_eq!(expected, avr.uart0().recv_string());
    };

    assert("Hello, World!", "Uryyb, Jbeyq!");

    assert(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent \
          non maximus purus. Fusce a neque condimentum, finibus dui et, tempor",
        "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg. Cenrfrag \
          aba znkvzhf chehf. Shfpr n ardhr pbaqvzraghz, svavohf qhv rg, grzcbe",
    );
}
