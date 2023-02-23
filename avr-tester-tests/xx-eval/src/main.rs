//! See: [../../../avr-tester/tests/tests/xx/eval.rs].

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

    let mut uart = Usart0::<MHz16>::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        115200u32.into_baudrate(),
    );

    let uart = &mut uart;

    loop {
        match Type::read(uart) {
            Type::I8 => eval::<i8>(uart).write(uart),
            Type::U8 => eval::<u8>(uart).write(uart),
            Type::I16 => eval::<i16>(uart).write(uart),
            Type::U16 => eval::<u16>(uart).write(uart),
            Type::I32 => eval::<i32>(uart).write(uart),
            Type::U32 => eval::<u32>(uart).write(uart),
            Type::I64 => eval::<i64>(uart).write(uart),
            Type::U64 => eval::<u64>(uart).write(uart),
            Type::I128 => eval::<i128>(uart).write(uart),
            Type::U128 => eval::<u128>(uart).write(uart),
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

impl Readable for Type {
    fn read(uart: UartMut<'_>) -> Self {
        match uart.read_byte() {
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

impl Readable for Token {
    fn read(uart: UartMut<'_>) -> Self {
        match uart.read_byte() {
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

fn eval<T>(uart: UartMut<'_>) -> T
where
    T: Number,
{
    let tok = Token::read(uart);

    if let Token::Const = tok {
        return T::read(uart);
    }

    let lhs = eval::<T>(uart);
    let rhs = eval::<T>(uart);

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

trait Readable {
    fn read(uart: UartMut<'_>) -> Self;
}

impl<const N: usize> Readable for [u8; N] {
    fn read(uart: UartMut<'_>) -> Self {
        array::from_fn(|_| uart.read_byte())
    }
}

// -----

trait Writable {
    fn write(&self, uart: UartMut<'_>);
}

impl<const N: usize> Writable for [u8; N] {
    fn write(&self, uart: UartMut<'_>) {
        for &value in self.iter() {
            uart.write_byte(value);
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
        + Readable
        + Writable,
{
    //
}

macro_rules! numbers {
    ([ $( $ty:ty ),* ]) => {
        $(
            impl Number for $ty {
                //
            }

            impl Readable for $ty {
                fn read(uart: UartMut<'_>) -> Self {
                    Self::from_le_bytes(Readable::read(uart))
                }
            }

            impl Writable for $ty {
                fn write(&self, uart: UartMut<'_>) {
                    self.to_le_bytes().write(uart);
                }
            }
        )*
    }
}

numbers!([u8, i8, u16, i16, u32, i32, u64, i64, u128, i128]);

// -----

#[no_mangle]
extern "C" fn __divti3() {
    panic!("not supported");
}

#[no_mangle]
extern "C" fn __modti3() {
    panic!("not supported");
}

#[no_mangle]
extern "C" fn __udivti3() {
    panic!("not supported");
}

#[no_mangle]
extern "C" fn __umodti3() {
    panic!("not supported");
}
