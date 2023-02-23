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
//! The evaluator supports all Rust integer types, although certain operations
//! (e.g. 128-bit division) have to be skipped due to missing intrinsics.
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

#[test]
fn primitives() {
    const TRIES: usize = 100;

    let mut avr = avr("xx-eval");

    avr.run_for_ms(1);

    for ty in Type::all() {
        for op in Op::all() {
            if !ty.supports(op) {
                println!("-> {:?}.{:?} (skipping; not supported on AVR)", ty, op);
                continue;
            }

            println!("-> {:?}.{:?}", ty, op);

            let mut tries = 0;

            while tries < TRIES {
                let random_value = match op {
                    Op::Mul => Value::random_half,
                    _ => Value::random,
                };

                let lhs = random_value(ty);
                let rhs = random_value(ty);

                let expected = if let Some(value) = op.apply()(lhs, rhs) {
                    value
                } else {
                    continue;
                };

                avr.uart0().write(ty);
                avr.uart0().write(Token::Op(op));
                avr.uart0().write(Token::Const);
                avr.uart0().write(lhs);
                avr.uart0().write(Token::Const);
                avr.uart0().write(rhs);
                avr.run_for_ms(ty.weight());

                let actual = Value::read(ty, &mut avr.uart0());

                if actual != expected {
                    panic!(
                        "{:?} {:?} {:?} is equal to {:?}, but AVR returned {:?}",
                        lhs, op, rhs, expected, actual
                    );
                }

                tries += 1;
            }
        }
    }
}

#[test]
fn expressions() {
    const TRIES: usize = 10;
    const MAX_DEPTH: u64 = 8;

    let mut avr = avr("xx-eval");

    avr.run_for_ms(1);

    let mut rng = thread_rng();

    for ty in Type::all() {
        for depth in 3..MAX_DEPTH {
            println!("-> {:?}.{}", ty, depth);

            let mut tries = 0;

            while tries < TRIES {
                let mut expr = Expression::Const(Value::random_half(ty));

                for _ in 0..=depth {
                    let build_expression = Expression::from_op(Op::random(ty));
                    let value = Box::new(Expression::Const(Value::random_half(ty)));

                    expr = if rng.gen::<bool>() {
                        build_expression(Box::new(expr), value)
                    } else {
                        build_expression(value, Box::new(expr))
                    };
                }

                let expected = if let Some(value) = expr.eval() {
                    value
                } else {
                    continue;
                };

                avr.uart0().write(ty);
                avr.uart0().write(&expr);
                avr.run_for_ms(ty.weight() * depth);

                let actual = Value::read(ty, &mut avr.uart0());

                if actual != expected {
                    panic!(
                        "{:?} is equal to {:?}, but AVR returned {:?}",
                        expr, expected, actual
                    );
                }

                tries += 1;
            }
        }
    }
}

// ----

#[derive(Clone, Copy, Debug)]
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

impl Type {
    fn all() -> impl Iterator<Item = Self> {
        [
            Self::I8,
            Self::U8,
            Self::I16,
            Self::U16,
            Self::I32,
            Self::U32,
            Self::I64,
            Self::U64,
            Self::I128,
            Self::U128,
        ]
        .into_iter()
    }

    fn weight(self) -> u64 {
        match self {
            Self::I8 | Self::U8 | Self::I16 | Self::U16 => 1,
            Self::I32 | Self::U32 => 2,
            Self::I64 | Self::U64 => 4,
            Self::I128 | Self::U128 => 8,
        }
    }

    fn supports(self, op: Op) -> bool {
        !matches!((self, op), (Self::I128 | Self::U128, Op::Div | Op::Rem))
    }
}

impl Writable for Type {
    fn write(&self, tx: &mut dyn Writer) {
        tx.write(1 + *self as u8);
    }
}

// ----

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl Op {
    fn all() -> [Self; 5] {
        [Self::Add, Self::Sub, Self::Mul, Self::Div, Self::Rem]
    }

    fn random(ty: Type) -> Self {
        loop {
            let op = Self::all().choose(&mut thread_rng()).cloned().unwrap();

            if ty.supports(op) {
                break op;
            }
        }
    }

    fn apply(self) -> fn(Value, Value) -> Option<Value> {
        match self {
            Self::Add => Value::checked_add,
            Self::Sub => Value::checked_sub,
            Self::Mul => Value::checked_mul,
            Self::Div => Value::checked_div,
            Self::Rem => Value::checked_rem,
        }
    }
}

// ----

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token {
    Op(Op),
    Const,
}

impl Writable for Token {
    fn write(&self, tx: &mut dyn Writer) {
        tx.write(match self {
            Token::Op(Op::Add) => 1,
            Token::Op(Op::Sub) => 2,
            Token::Op(Op::Mul) => 3,
            Token::Op(Op::Div) => 4,
            Token::Op(Op::Rem) => 5,
            Token::Const => 255,
        })
    }
}

// ----

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
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
}

impl Value {
    fn random(ty: Type) -> Self {
        let mut rng = thread_rng();

        match ty {
            Type::I8 => Self::I8(rng.gen()),
            Type::U8 => Self::U8(rng.gen()),
            Type::I16 => Self::I16(rng.gen()),
            Type::U16 => Self::U16(rng.gen()),
            Type::I32 => Self::I32(rng.gen()),
            Type::U32 => Self::U32(rng.gen()),
            Type::I64 => Self::I64(rng.gen()),
            Type::U64 => Self::U64(rng.gen()),
            Type::I128 => Self::I128(rng.gen()),
            Type::U128 => Self::U128(rng.gen()),
        }
    }

    fn random_half(ty: Type) -> Self {
        let mut this = Self::random(ty);

        match &mut this {
            Value::I8(val) => *val >>= 4,
            Value::U8(val) => *val >>= 4,
            Value::I16(val) => *val >>= 8,
            Value::U16(val) => *val >>= 8,
            Value::I32(val) => *val >>= 16,
            Value::U32(val) => *val >>= 16,
            Value::I64(val) => *val >>= 32,
            Value::U64(val) => *val >>= 32,
            Value::U128(val) => *val >>= 64,
            Value::I128(val) => *val >>= 64,
        }

        this
    }

    fn read(ty: Type, uart: &mut Uart<'_>) -> Self {
        match ty {
            Type::I8 => Self::I8(i8::from_le_bytes(uart.read())),
            Type::U8 => Self::U8(u8::from_le_bytes(uart.read())),
            Type::I16 => Self::I16(i16::from_le_bytes(uart.read())),
            Type::U16 => Self::U16(u16::from_le_bytes(uart.read())),
            Type::I32 => Self::I32(i32::from_le_bytes(uart.read())),
            Type::U32 => Self::U32(u32::from_le_bytes(uart.read())),
            Type::I64 => Self::I64(i64::from_le_bytes(uart.read())),
            Type::U64 => Self::U64(u64::from_le_bytes(uart.read())),
            Type::I128 => Self::I128(i128::from_le_bytes(uart.read())),
            Type::U128 => Self::U128(u128::from_le_bytes(uart.read())),
        }
    }
}

impl Writable for Value {
    fn write(&self, tx: &mut dyn Writer) {
        match *self {
            Value::I8(value) => tx.write(value.to_le_bytes()),
            Value::U8(value) => tx.write(value.to_le_bytes()),
            Value::I16(value) => tx.write(value.to_le_bytes()),
            Value::U16(value) => tx.write(value.to_le_bytes()),
            Value::I32(value) => tx.write(value.to_le_bytes()),
            Value::U32(value) => tx.write(value.to_le_bytes()),
            Value::I64(value) => tx.write(value.to_le_bytes()),
            Value::U64(value) => tx.write(value.to_le_bytes()),
            Value::I128(value) => tx.write(value.to_le_bytes()),
            Value::U128(value) => tx.write(value.to_le_bytes()),
        }
    }
}

macro_rules! impl_ops {
    (
        $value:ty,
        $types:tt,
        [ $( $fn:ident ),+ ]
    ) => {
        $(
            impl_ops!(@expand $value, $types, $fn);
        )+
    };

    (
        @expand
        $value:ty,
        [ $( $ty:ident ),+ ],
        $fn:ident
    ) => {
        impl $value {
            fn $fn(self, rhs: Self) -> Option<Self> {
                match (self, rhs) {
                    $(
                        (Self::$ty(lhs), Self::$ty(rhs)) => lhs.$fn(rhs).map(Self::$ty),
                    )+
                    _ => unreachable!(),
                }
            }
        }
    };
}

impl_ops!(
    Value,
    [I8, U8, I16, U16, I32, U32, I64, U64, I128, U128],
    [
        checked_add,
        checked_sub,
        checked_mul,
        checked_div,
        checked_rem
    ]
);

// ----

#[derive(Clone, Debug)]
enum Expression {
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Rem(Box<Self>, Box<Self>),
    Const(Value),
}

impl Expression {
    fn from_op(op: Op) -> fn(Box<Self>, Box<Self>) -> Self {
        match op {
            Op::Add => Self::Add,
            Op::Sub => Self::Sub,
            Op::Mul => Self::Mul,
            Op::Div => Self::Div,
            Op::Rem => Self::Rem,
        }
    }

    fn eval(&self) -> Option<Value> {
        type OpFunction = fn(Value, Value) -> Option<Value>;

        let (lhs, rhs, op): (_, _, OpFunction) = match self {
            Self::Add(lhs, rhs) => (lhs, rhs, Value::checked_add),
            Self::Sub(lhs, rhs) => (lhs, rhs, Value::checked_sub),
            Self::Mul(lhs, rhs) => (lhs, rhs, Value::checked_mul),
            Self::Div(lhs, rhs) => (lhs, rhs, Value::checked_div),
            Self::Rem(lhs, rhs) => (lhs, rhs, Value::checked_rem),

            Self::Const(value) => {
                return Some(value.to_owned());
            }
        };

        let lhs = lhs.eval()?;
        let rhs = rhs.eval()?;

        op(lhs, rhs)
    }
}

impl Writable for Expression {
    fn write(&self, tx: &mut dyn Writer) {
        let (lhs, rhs, op) = match self {
            Self::Add(lhs, rhs) => (lhs, rhs, Op::Add),
            Self::Sub(lhs, rhs) => (lhs, rhs, Op::Sub),
            Self::Mul(lhs, rhs) => (lhs, rhs, Op::Mul),
            Self::Div(lhs, rhs) => (lhs, rhs, Op::Div),
            Self::Rem(lhs, rhs) => (lhs, rhs, Op::Rem),

            Self::Const(value) => {
                tx.write(Token::Const);
                tx.write(value);
                return;
            }
        };

        tx.write(Token::Op(op));
        tx.write(&**lhs);
        tx.write(&**rhs);
    }
}
