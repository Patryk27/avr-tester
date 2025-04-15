//! This test shows AvrTester's feature called components - please refer to
//! [`Components`] for more context.
//!
//! ---
//!
//! We're given an AVR that transmits some data through SPI - the interpretation
//! of that data depends on the `PD0` pin:
//!
//! - when AVR puts `PD0` high, SPI will contain autoincremental numbers
//!   starting from zero (i.e. it sends 0, then 1, then 2, etc.),
//!
//! - when AVR puts `PD0` low, SPI will contain magic number 0xCAFEBABE
//!   transmitted as four separate bytes.
//!
//! Overall, our job here is to separate this single SPI stream into two
//! separate arrays, depending on the value of the `PD0` pin.
//!
//! See: [../../../avr-tester-fixtures/spi-component/src/main.rs].

use avr_tester::{AvrTester, Reader, avr_rt};
use std::cell::RefCell;
use std::rc::Rc;

enum CatchWhenCs {
    IsHigh,
    IsLow,
}

async fn spi_input_catcher(
    catch_when_cs: CatchWhenCs,
    numbers: Rc<RefCell<Vec<u8>>>,
) {
    let avr = avr_rt();

    loop {
        let is_ready = match catch_when_cs {
            CatchWhenCs::IsHigh => avr.pins().pd0().is_high(),
            CatchWhenCs::IsLow => avr.pins().pd0().is_low(),
        };

        if is_ready {
            while let Some(number) = avr.spi0().try_read_byte() {
                numbers.borrow_mut().push(number);
            }
        }

        avr.run().await;
    }
}

#[test]
fn test() {
    let mut avr = AvrTester::atmega328().with_clock_of_16_mhz().load(
        "../avr-tester-fixtures/target/avr-none/release/spi-component.elf",
    );

    // Numbers transmitted through SPI when PD0 was high
    let high_numbers = Rc::new(RefCell::new(Vec::new()));

    // Numbers transmitted through SPI when PD0 was low
    let low_numbers = Rc::new(RefCell::new(Vec::new()));

    avr.components().add(spi_input_catcher(
        CatchWhenCs::IsHigh,
        Rc::clone(&high_numbers),
    ));

    avr.components().add(spi_input_catcher(
        CatchWhenCs::IsLow,
        Rc::clone(&low_numbers),
    ));

    avr.run_for_us(60);

    assert_eq!(vec![0, 1, 2, 3], *high_numbers.borrow());

    assert_eq!(
        vec![
            0xCA, 0xFE, 0xBA, 0xBE, //
            0xCA, 0xFE, 0xBA, 0xBE, //
            0xCA, 0xFE, 0xBA, 0xBE, //
            0xCA, 0xFE,
        ],
        *low_numbers.borrow()
    );
}
