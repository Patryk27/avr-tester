//! Reader beware: <./README.md>
//!
//! # Scenario
//!
//! We're given an AVR that implements an expression evaluator.
//!
//! The AVR retrieves a LISP-like expression via the UART (think `* 1 + 2 3`,
//! but represented in a binary protocol) and sends back a single number
//! representing the expression's result.
//!
//! The evaluator supports all Rust integer types and f32, although certain
//! operations (e.g. 128-bit division) are skipped due to missing intrinsics.
//!
//! # Purpose
//!
//! This test allows us to ensure that all the UART operations work correctly
//! (there's a lot of send/recv with custom (de)serializers and whatnot).
//!
//! Also, this test serves as a proof that rustc + LLVM generate correct AVR
//! code that is able to work on 16+-bit numbers.
//!
//! # Firmware
//!
//! See: [../../../../avr-tester-tests/xx-eval/src/main.rs].

use crate::prelude::*;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[test]
fn smoke_i8() {
    let mut avr = avr("xx-eval");

    // Give the firmware a moment to initialize
    avr.run_for_ms(1);

    // Check ranges
    assert(&mut avr, Expression::Value(Value::I8(i8::MIN)));
    assert(&mut avr, Expression::Value(Value::I8(i8::MAX)));

    // Check non-overflowing arithmetic
    assert(&mut avr, Expression::Add(make_i8(10), make_i8(23)));
    assert(&mut avr, Expression::Sub(make_i8(10), make_i8(23)));
    assert(&mut avr, Expression::Mul(make_i8(10), make_i8(5)));
    assert(&mut avr, Expression::Div(make_i8(123), make_i8(10)));
    assert(&mut avr, Expression::Rem(make_i8(123), make_i8(10)));
    assert(&mut avr, Expression::Neg(make_i8(123)));

    // Check overflowing arithmetic
    assert(&mut avr, Expression::Add(make_i8(123), make_i8(100)));
    assert(&mut avr, Expression::Sub(make_i8(-100), make_i8(123)));
    assert(&mut avr, Expression::Mul(make_i8(123), make_i8(55)));
}

#[test]
fn smoke_u8() {
    let mut avr = avr("xx-eval");

    // Give the firmware a moment to initialize
    avr.run_for_ms(1);

    // Check ranges
    assert(&mut avr, Expression::Value(Value::U8(u8::MIN)));
    assert(&mut avr, Expression::Value(Value::U8(u8::MAX)));

    // Check non-overflowing arithmetic
    assert(&mut avr, Expression::Add(make_u8(10), make_u8(23)));
    assert(&mut avr, Expression::Sub(make_u8(23), make_u8(10)));
    assert(&mut avr, Expression::Mul(make_u8(10), make_u8(5)));
    assert(&mut avr, Expression::Div(make_u8(123), make_u8(10)));
    assert(&mut avr, Expression::Rem(make_u8(123), make_u8(10)));

    // Check overflowing arithmetic
    assert(&mut avr, Expression::Add(make_u8(123), make_u8(200)));
    assert(&mut avr, Expression::Sub(make_u8(100), make_u8(123)));
    assert(&mut avr, Expression::Mul(make_u8(123), make_u8(55)));
}

#[test]
fn smoke_i16() {
    let mut avr = avr("xx-eval");

    // Give the firmware a moment to initialize
    avr.run_for_ms(1);

    // Check ranges
    assert(&mut avr, Expression::Value(Value::I16(i16::MIN)));
    assert(&mut avr, Expression::Value(Value::I16(i16::MAX)));

    // Check non-overflowing arithmetic
    assert(&mut avr, Expression::Add(make_i16(1000), make_i16(2300)));
    assert(&mut avr, Expression::Sub(make_i16(1000), make_i16(2300)));
    assert(&mut avr, Expression::Mul(make_i16(1000), make_i16(50)));
    assert(&mut avr, Expression::Div(make_i16(12345), make_i16(123)));
    assert(&mut avr, Expression::Rem(make_i16(12345), make_i16(123)));
    assert(&mut avr, Expression::Neg(make_i16(12345)));

    // Check overflowing arithmetic
    assert(&mut avr, Expression::Add(make_i16(30000), make_i16(23456)));
    assert(&mut avr, Expression::Sub(make_i16(-30000), make_i16(23456)));
    assert(&mut avr, Expression::Mul(make_i16(12345), make_i16(234)));
}

#[track_caller]
fn assert(avr: &mut AvrTester, expr: Expression) {
    let expected = expr.eval();

    println!("{:?} => {:?}", expr, expected);

    avr.uart0().write(expr);
    avr.run_for_ms(50);

    let actual: Value = avr.uart0().read();

    if actual != expected {
        panic!("Mismatch: {:?} != {:?}", actual, expected);
    }
}

#[derive(Clone, Debug)]
enum Expression {
    /// `value`
    Value(Value),

    /// `lhs + rhs`
    Add(Box<Self>, Box<Self>),

    /// `lhs - rhs`
    Sub(Box<Self>, Box<Self>),

    /// `lhs * rhs`
    Mul(Box<Self>, Box<Self>),

    /// `lhs / rhs`
    Div(Box<Self>, Box<Self>),

    /// `lhs % rhs`
    Rem(Box<Self>, Box<Self>),

    /// `-expr`
    Neg(Box<Self>),

    /// `lhs == rhs`
    Eq(Box<Self>, Box<Self>),

    /// `lhs != rhs`
    Neq(Box<Self>, Box<Self>),

    /// `lhs > rhs`
    Gt(Box<Self>, Box<Self>),

    /// `lhs >= rhs`
    GtEq(Box<Self>, Box<Self>),

    /// `lhs < rhs`
    Lt(Box<Self>, Box<Self>),

    /// `lhs <= rhs`
    LtEq(Box<Self>, Box<Self>),

    /// `expr as type`
    Cast(Box<Self>, Type),
}

impl Expression {
    fn eval(&self) -> Value {
        macro_rules! eval_math_op {
            ($expr:expr, $op:ident) => {{
                match $expr.eval() {
                    Value::I8(expr) => Value::I8(expr.$op()),
                    Value::I16(expr) => Value::I16(expr.$op()),
                    Value::I32(expr) => Value::I32(expr.$op()),
                    Value::I64(expr) => Value::I64(expr.$op()),
                    Value::I128(expr) => Value::I128(expr.$op()),
                    Value::F32(expr) => Value::F32(expr.$op()),
                    expr => {
                        panic!("Invalid operation: {} {:?}", stringify!($op), expr);
                    }
                }
            }};

            ($lhs:expr, $rhs:expr, $op:ident) => {{
                match ($lhs.eval(), $rhs.eval()) {
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
                    (lhs, rhs) => {
                        panic!("Invalid operation: {:?} {} {:?}", lhs, stringify!($op), rhs);
                    }
                }
            }};
        }

        match self {
            Expression::Value(value) => *value,
            Expression::Add(lhs, rhs) => eval_math_op!(lhs, rhs, add),
            Expression::Sub(lhs, rhs) => eval_math_op!(lhs, rhs, sub),
            Expression::Mul(lhs, rhs) => eval_math_op!(lhs, rhs, mul),
            Expression::Div(lhs, rhs) => eval_math_op!(lhs, rhs, div),
            Expression::Rem(lhs, rhs) => eval_math_op!(lhs, rhs, rem),
            Expression::Neg(expr) => eval_math_op!(expr, neg),

            _ => todo!(),
        }
    }
}

impl Writable for Expression {
    fn write(&self, tx: &mut dyn Writer) {
        match self {
            Expression::Value(value) => {
                tx.write(0u8);
                tx.write(value);
            }
            Expression::Add(lhs, rhs) => {
                tx.write(1u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Sub(lhs, rhs) => {
                tx.write(2u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Mul(lhs, rhs) => {
                tx.write(3u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Div(lhs, rhs) => {
                tx.write(4u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Rem(lhs, rhs) => {
                tx.write(5u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Neg(expr) => {
                tx.write(6u8);
                tx.write(&**expr);
            }
            Expression::Eq(lhs, rhs) => {
                tx.write(7u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Neq(lhs, rhs) => {
                tx.write(8u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Gt(lhs, rhs) => {
                tx.write(9u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::GtEq(lhs, rhs) => {
                tx.write(10u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Lt(lhs, rhs) => {
                tx.write(11u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::LtEq(lhs, rhs) => {
                tx.write(12u8);
                tx.write(&**lhs);
                tx.write(&**rhs);
            }
            Expression::Cast(expr, ty) => {
                tx.write(13u8);
                tx.write(&**expr);
                tx.write(*ty);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    fn read(rx: &mut dyn Reader) -> Self {
        match rx.read() {
            Type::Bool => Self::Bool(rx.read_byte() == 1),
            Type::I8 => Self::I8(i8::from_le_bytes(rx.read())),
            Type::U8 => Self::U8(u8::from_le_bytes(rx.read())),
            Type::I16 => Self::I16(i16::from_le_bytes(rx.read())),
            Type::U16 => Self::U16(u16::from_le_bytes(rx.read())),
            Type::I32 => Self::I32(i32::from_le_bytes(rx.read())),
            Type::U32 => Self::U32(u32::from_le_bytes(rx.read())),
            Type::I64 => Self::I64(i64::from_le_bytes(rx.read())),
            Type::U64 => Self::U64(u64::from_le_bytes(rx.read())),
            Type::I128 => Self::I128(i128::from_le_bytes(rx.read())),
            Type::U128 => Self::U128(u128::from_le_bytes(rx.read())),
            Type::F32 => Self::F32(f32::from_le_bytes(rx.read())),
        }
    }
}

impl Writable for Value {
    fn write(&self, tx: &mut dyn Writer) {
        match *self {
            Value::Bool(value) => {
                tx.write(Type::Bool);
                tx.write(value as u8);
            }
            Value::I8(value) => {
                tx.write(Type::I8);
                tx.write(value.to_le_bytes());
            }
            Value::U8(value) => {
                tx.write(Type::U8);
                tx.write(value.to_le_bytes());
            }
            Value::I16(value) => {
                tx.write(Type::I16);
                tx.write(value.to_le_bytes());
            }
            Value::U16(value) => {
                tx.write(Type::U16);
                tx.write(value.to_le_bytes());
            }
            Value::I32(value) => {
                tx.write(Type::I32);
                tx.write(value.to_le_bytes());
            }
            Value::U32(value) => {
                tx.write(Type::U32);
                tx.write(value.to_le_bytes());
            }
            Value::I64(value) => {
                tx.write(Type::I64);
                tx.write(value.to_le_bytes());
            }
            Value::U64(value) => {
                tx.write(Type::U64);
                tx.write(value.to_le_bytes());
            }
            Value::I128(value) => {
                tx.write(Type::I128);
                tx.write(value.to_le_bytes());
            }
            Value::U128(value) => {
                tx.write(Type::U128);
                tx.write(value.to_le_bytes());
            }
            Value::F32(value) => {
                tx.write(Type::F32);
                tx.write(value.to_le_bytes());
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    fn read(rx: &mut dyn Reader) -> Self {
        match rx.read_byte() {
            0 => Self::Bool,
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
            n => panic!("Unknown type: {n}"),
        }
    }
}

impl Writable for Type {
    fn write(&self, tx: &mut dyn Writer) {
        tx.write(*self as u8)
    }
}

fn make_i8(value: i8) -> Box<Expression> {
    Box::new(Expression::Value(Value::I8(value)))
}

fn make_u8(value: u8) -> Box<Expression> {
    Box::new(Expression::Value(Value::U8(value)))
}

fn make_i16(value: i16) -> Box<Expression> {
    Box::new(Expression::Value(Value::I16(value)))
}
