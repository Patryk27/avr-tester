//! See: [../../../avr-tester/tests/tests/xx/eval.rs].

#![no_std]
#![no_main]

use atmega_hal::clock::MHz16;
use atmega_hal::usart::{BaudrateExt, Usart0};
use atmega_hal::{pins, Peripherals};
use core::array;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
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

    loop {
        eval(&mut uart).write(&mut uart);
    }
}

fn eval(uart: UartMut<'_>) -> Value {
    macro_rules! eval_math_op {
        ($expr:expr, $op:ident) => {{
            match $expr {
                Value::I8(expr) => Value::I8(expr.$op()),
                Value::I16(expr) => Value::I16(expr.$op()),
                Value::I32(expr) => Value::I32(expr.$op()),
                Value::I64(expr) => Value::I64(expr.$op()),
                Value::I128(expr) => Value::I128(expr.$op()),
                Value::F32(expr) => Value::F32(expr.$op()),
                expr => panic!(),
            }
        }};

        ($lhs:expr, $rhs:expr, $op:ident) => {
            match ($lhs, $rhs) {
                (Value::I8(lhs), Value::I8(rhs)) => Value::I8(lhs.$op(rhs)),
                (Value::U8(lhs), Value::U8(rhs)) => Value::U8(lhs.$op(rhs)),
                (Value::I16(lhs), Value::I16(rhs)) => Value::I16(lhs.$op(rhs)),
                (Value::U16(lhs), Value::U16(rhs)) => Value::U16(lhs.$op(rhs)),
                (Value::I32(lhs), Value::I32(rhs)) => Value::I32(lhs.$op(rhs)),
                (Value::U32(lhs), Value::U32(rhs)) => Value::U32(lhs.$op(rhs)),
                (Value::I64(lhs), Value::I64(rhs)) => Value::I64(lhs.$op(rhs)),
                (Value::U64(lhs), Value::U64(rhs)) => Value::U64(lhs.$op(rhs)),
                (Value::I128(lhs), Value::I128(rhs)) => Value::I128(lhs.$op(rhs)),
                (Value::U128(lhs), Value::U128(rhs)) => Value::U128(lhs.$op(rhs)),
                (Value::F32(lhs), Value::F32(rhs)) => Value::F32(lhs.$op(rhs)),
                (lhs, rhs) => panic!(),
            }
        };
    }

    match ExpressionKind::read(uart) {
        ExpressionKind::Value => Value::read(uart),

        ExpressionKind::Add => {
            let lhs = eval(uart);
            let rhs = eval(uart);

            eval_math_op!(lhs, rhs, add)
        }

        ExpressionKind::Sub => {
            let lhs = eval(uart);
            let rhs = eval(uart);

            eval_math_op!(lhs, rhs, sub)
        }

        ExpressionKind::Mul => {
            let lhs = eval(uart);
            let rhs = eval(uart);

            eval_math_op!(lhs, rhs, mul)
        }

        ExpressionKind::Div => {
            let lhs = eval(uart);
            let rhs = eval(uart);

            eval_math_op!(lhs, rhs, div)
        }

        ExpressionKind::Rem => {
            let lhs = eval(uart);
            let rhs = eval(uart);

            eval_math_op!(lhs, rhs, rem)
        }

        ExpressionKind::Neg => {
            let expr = eval(uart);

            eval_math_op!(expr, neg)
        }

        _ => todo!(),
    }
}

#[derive(Clone, Copy)]
enum ExpressionKind {
    Value,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    Eq,
    Neq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Cast,
}

impl Readable for ExpressionKind {
    fn read(uart: UartMut<'_>) -> Self {
        match uart.read_byte() {
            0 => Self::Value,
            1 => Self::Add,
            2 => Self::Sub,
            3 => Self::Mul,
            4 => Self::Div,
            5 => Self::Rem,
            6 => Self::Neg,
            7 => Self::Eq,
            8 => Self::Neq,
            9 => Self::Gt,
            10 => Self::GtEq,
            11 => Self::Lt,
            12 => Self::LtEq,
            13 => Self::Cast,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Value {
    Bool(bool),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
}

impl Readable for Value {
    fn read(uart: UartMut<'_>) -> Self {
        match Type::read(uart) {
            Type::Bool => Self::Bool(uart.read_byte() == 1),
            Type::I8 => Self::I8(i8::from_le_bytes(Readable::read(uart))),
            Type::U8 => Self::U8(u8::from_le_bytes(Readable::read(uart))),
            Type::I16 => Self::I16(i16::from_le_bytes(Readable::read(uart))),
            Type::U16 => Self::U16(u16::from_le_bytes(Readable::read(uart))),
            Type::I32 => Self::I32(i32::from_le_bytes(Readable::read(uart))),
            Type::U32 => Self::U32(u32::from_le_bytes(Readable::read(uart))),
            Type::I64 => Self::I64(i64::from_le_bytes(Readable::read(uart))),
            Type::U64 => Self::U64(u64::from_le_bytes(Readable::read(uart))),
            Type::I128 => Self::I128(i128::from_le_bytes(Readable::read(uart))),
            Type::U128 => Self::U128(u128::from_le_bytes(Readable::read(uart))),
            Type::F32 => Self::F32(f32::from_le_bytes(Readable::read(uart))),
        }
    }
}

impl Writable for Value {
    fn write(&self, uart: UartMut<'_>) {
        match *self {
            Value::Bool(value) => {
                uart.write(Type::Bool);
                uart.write(value as u8);
            }
            Value::I8(value) => {
                uart.write(Type::I8);
                uart.write(value.to_le_bytes());
            }
            Value::U8(value) => {
                uart.write(Type::U8);
                uart.write(value.to_le_bytes());
            }
            Value::I16(value) => {
                uart.write(Type::I16);
                uart.write(value.to_le_bytes());
            }
            Value::U16(value) => {
                uart.write(Type::U16);
                uart.write(value.to_le_bytes());
            }
            Value::I32(value) => {
                uart.write(Type::I32);
                uart.write(value.to_le_bytes());
            }
            Value::U32(value) => {
                uart.write(Type::U32);
                uart.write(value.to_le_bytes());
            }
            Value::I64(value) => {
                uart.write(Type::I64);
                uart.write(value.to_le_bytes());
            }
            Value::U64(value) => {
                uart.write(Type::U64);
                uart.write(value.to_le_bytes());
            }
            Value::I128(value) => {
                uart.write(Type::I128);
                uart.write(value.to_le_bytes());
            }
            Value::U128(value) => {
                uart.write(Type::U128);
                uart.write(value.to_le_bytes());
            }
            Value::F32(value) => {
                uart.write(Type::F32);
                uart.write(value.to_le_bytes());
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Type {
    Bool,
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
    F32,
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
            11 => Self::F32,
            _ => unreachable!(),
        }
    }
}

impl Writable for Type {
    fn write(&self, uart: UartMut<'_>) {
        uart.write(*self as u8);
    }
}

// -----

type UartMut<'a> = &'a mut Usart0<MHz16>;

trait UartHelper {
    fn read<T>(self) -> T
    where
        T: Readable;

    fn write<T>(self, value: T)
    where
        T: Writable;
}

impl UartHelper for UartMut<'_> {
    fn read<T>(self) -> T
    where
        T: Readable,
    {
        T::read(self)
    }

    fn write<T>(self, value: T)
    where
        T: Writable,
    {
        value.write(self);
    }
}

trait Readable {
    fn read(uart: UartMut<'_>) -> Self;
}

impl<const N: usize> Readable for [u8; N] {
    fn read(uart: UartMut<'_>) -> Self {
        array::from_fn(|_| uart.read_byte())
    }
}

trait Writable {
    fn write(&self, uart: UartMut<'_>);
}

impl Writable for u8 {
    fn write(&self, uart: UartMut<'_>) {
        uart.write_byte(*self);
    }
}

impl<const N: usize> Writable for [u8; N] {
    fn write(&self, uart: UartMut<'_>) {
        for &value in self.iter() {
            uart.write_byte(value);
        }
    }
}

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
