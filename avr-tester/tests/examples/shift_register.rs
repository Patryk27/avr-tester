//! This test shows AvrTester's feature called components - please refer to
//! [`Components`] for more context.
//!
//! ---
//!
//! We're given an AVR that is connected to a shift register on `PB0` (latch)
//! and `PB1` (data).
//!
//! Shift register (i.e. us, here) realizes a simple algorithm:
//! - wait for `PB0` to get high,
//! - read `PB1` status (high / low) and dump it into a vector,
//! - wait for `PB0` to get low,
//! - repeat.
//!
//! Bits dumped from `PB1` eventually construct four `u8`s, which represent a
//! magic number 0xCAFEBABE.
//!
//! See: [../../../avr-tester-fixtures/src/shift_register.rs].

use avr_tester::{avr_rt, AvrTester};
use std::cell::RefCell;
use std::rc::Rc;

async fn shift_register(numbers: Rc<RefCell<Vec<u8>>>) {
    let avr = avr_rt();
    let mut bits = Vec::new();

    loop {
        if avr.pins().pb0().is_high() {
            bits.push(avr.pins().pb1().is_high());

            if bits.len() == 8 {
                let number = bits
                    .drain(..)
                    .rev()
                    .fold(0, |acc, bit| (acc << 1) | (bit as u8));

                numbers.borrow_mut().push(number);
            }

            avr.pins().pb0().wait_while_high().await;
        } else {
            avr.run().await;
        }
    }
}

#[test]
fn test() {
    let mut avr = AvrTester::atmega328().with_clock_of_16_mhz().load(
        "../avr-tester-fixtures/target/atmega328p/release/shift-register.elf",
    );

    let numbers = Rc::new(RefCell::new(Vec::new()));

    // Start the shift register
    avr.components().add(shift_register(Rc::clone(&numbers)));

    // Wait for AVR to push stuff into it
    avr.run_for_us(150);

    assert_eq!(vec![0xCA, 0xFE, 0xBA, 0xBE], *numbers.borrow());
}

/// `.components().add()` returns [`ComponentHandle`] which allows you to remove
/// a component, stopping it from working:
#[test]
fn remove() {
    let mut avr = AvrTester::atmega328().with_clock_of_16_mhz().load(
        "../avr-tester-fixtures/target/atmega328p/release/shift-register.elf",
    );

    let numbers = Rc::new(RefCell::new(Vec::new()));

    let shift_register =
        avr.components().add(shift_register(Rc::clone(&numbers)));

    // Wait until we receive the first number
    while numbers.borrow().len() == 0 {
        avr.run();
    }

    // Destroy the shift register
    shift_register.remove();

    // Wait a little more, to make sure the shift register was actually removed
    avr.run_for_us(150);

    // ta-da
    assert_eq!(vec![0xCA], *numbers.borrow());
}

/// Components can be also paused and resumed at one's will:
#[test]
fn pause_and_resume() {
    let mut avr = AvrTester::atmega328().with_clock_of_16_mhz().load(
        "../avr-tester-fixtures/target/atmega328p/release/shift-register.elf",
    );

    let numbers = Rc::new(RefCell::new(Vec::new()));

    let shift_register =
        avr.components().add(shift_register(Rc::clone(&numbers)));

    // Wait until we receive the first number
    while numbers.borrow().len() == 0 {
        avr.run();
    }

    // Pause the shift register
    shift_register.pause();

    // Wait a little bit
    avr.run_for_us(10);

    // Resume the shift register
    shift_register.resume();

    // Wait for AVR to finish sending stuff
    avr.run_for_us(100);

    // Assert.
    //
    // Note that only the first number here (0xCA) is correct, since the rest of
    // those have been retrieved "out of sync" (the shift register was paused
    // for a while, so it couldn't see some of the bits being transmitted).
    assert_eq!(vec![0xCA, 0xD7, 0xF5], *numbers.borrow());
}
