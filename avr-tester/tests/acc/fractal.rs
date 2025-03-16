//! We're given an AVR that prints the Mandelbrot fractal; this test makes sure
//! that floating-point operations work correctly.
//!
//! See: [../../../avr-tester-fixtures/src/acc_fractal.rs].

use avr_tester::AvrTester;
use indoc::indoc;
use pretty_assertions as pa;

#[test]
fn test() {
    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/acc-fractal.elf");

    avr.run_for_s(8);

    let actual = avr.uart0().read::<String>();

    let expected = indoc! {r#"
                                     .
                                      ...
                                    ..==..
                                   ..-###..
                               ,,..,,,##,......
                              ..############,##.
                            ..################..
                    ....#....,##################
                   ...#####:,-##################
                 ...,##########################.
         ####################################,.
                 ...,##########################.
                   ...#####:,-##################
                    ....#....,##################
                            ..################..
                              ..############,##.
                               ,,..,,,##,......
                                   ..-###..
                                    ..==..
                                      ...
    "#};

    // ---

    let actual = actual
        .lines()
        .map(|line| line.trim_end().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    let expected = expected
        .lines()
        .map(|line| format!(" {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    pa::assert_eq!(expected, actual);
}
