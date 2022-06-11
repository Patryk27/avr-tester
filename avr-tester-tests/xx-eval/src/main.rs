//! See: [../../../avr-tester/tests/tests/xx/eval.rs]

#![no_std]
#![no_main]

use atmega_hal::clock::MHz16;
use atmega_hal::usart::{BaudrateExt, Usart0};
use atmega_hal::{pins, Peripherals};
use core::{array, ops};
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut serial = Usart0::<MHz16>::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        115200u32.into_baudrate(),
    );

    let serial = &mut serial;

    loop {
        let ty = Type::recv(serial);

        match ty {
            Type::I8 => eval::<i8>(serial).send(serial),
            Type::U8 => eval::<u8>(serial).send(serial),
            Type::I16 => eval::<i16>(serial).send(serial),
            Type::U16 => eval::<u16>(serial).send(serial),
            Type::I32 => eval::<i32>(serial).send(serial),
            Type::U32 => eval::<u32>(serial).send(serial),
            Type::I64 => eval::<i64>(serial).send(serial),
            Type::U64 => eval::<u64>(serial).send(serial),
            Type::I128 => eval::<i128>(serial).send(serial),
            Type::U128 => eval::<u128>(serial).send(serial),
        }
    }
}

// ----

enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
}

impl UartRecv for Type {
    fn recv(serial: UartMut<'_>) -> Self {
        match serial.read_byte() {
            1 => Self::I8,
            2 => Self::U8,
            3 => Self::I16,
            4 => Self::U16,
            5 => Self::I32,
            6 => Self::U32,
            7 => Self::I64,
            8 => Self::U64,
            9 => Self::I128,
            10 => Self::U128,
            _ => unreachable!(),
        }
    }
}

// ----

enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Const,
}

impl UartRecv for Token {
    fn recv(serial: UartMut<'_>) -> Self {
        match serial.read_byte() {
            1 => Self::Add,
            2 => Self::Sub,
            3 => Self::Mul,
            4 => Self::Div,
            5 => Self::Rem,
            255 => Self::Const,
            _ => unreachable!(),
        }
    }
}

// ----

fn eval<T>(serial: UartMut<'_>) -> T
where
    T: Number,
{
    let tok = Token::recv(serial);

    if let Token::Const = tok {
        return T::recv(serial);
    }

    let lhs = eval::<T>(serial);
    let rhs = eval::<T>(serial);

    match tok {
        Token::Add => lhs + rhs,
        Token::Sub => lhs - rhs,
        Token::Mul => lhs * rhs,
        Token::Div => lhs / rhs,
        Token::Rem => lhs % rhs,

        Token::Const => {
            unreachable!()
        }
    }
}

// -----

type UartMut<'a> = &'a mut Usart0<MHz16>;

// -----

trait UartRecv {
    fn recv(uart: UartMut<'_>) -> Self;
}

impl<const N: usize> UartRecv for [u8; N] {
    fn recv(serial: UartMut<'_>) -> Self {
        array::from_fn(|_| serial.read_byte())
    }
}

// -----

trait UartSend {
    fn send(&self, uart: UartMut<'_>);
}

impl<const N: usize> UartSend for [u8; N] {
    fn send(&self, serial: UartMut<'_>) {
        for &value in self.iter() {
            serial.write_byte(value);
        }
    }
}

// -----

trait Number
where
    Self: Sized
        + ops::Add<Self, Output = Self>
        + ops::Sub<Self, Output = Self>
        + ops::Mul<Self, Output = Self>
        + ops::Div<Self, Output = Self>
        + ops::Rem<Self, Output = Self>
        + UartRecv
        + UartSend,
{
    //
}

macro_rules! numbers {
    ([ $( $ty:ty ),* ]) => {
        $(
            impl Number for $ty {
                //
            }

            impl UartRecv for $ty {
                fn recv(uart: UartMut<'_>) -> Self {
                    Self::from_le_bytes(UartRecv::recv(uart))
                }
            }

            impl UartSend for $ty {
                fn send(&self, uart: UartMut<'_>) {
                    self.to_le_bytes().send(uart);
                }
            }
        )*
    }
}

numbers!([u8, i8, u16, i16, u32, i32, u64, i64, u128, i128]);

// -----

#[no_mangle]
extern "C" fn __divti3() {
    todo!("not supported");
}

#[no_mangle]
extern "C" fn __modti3() {
    todo!("not supported");
}

#[no_mangle]
extern "C" fn __udivti3() {
    todo!("not supported");
}

#[no_mangle]
extern "C" fn __umodti3() {
    todo!("not supported");
}
