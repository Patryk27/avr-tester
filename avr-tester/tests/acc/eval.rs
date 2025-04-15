//! We're given an AVR that implements an expression evaluator.
//!
//! The AVR retrieves a LISP-like expression via the UART (think `* 1 + 2 3`,
//! but represented in a binary protocol) and sends back a single number
//! representing the expression's result.
//!
//! This test makes sure that:
//!
//! - UART operations work correctly (as there's a lot of send/recv with custom
//!   serializers / deserializers),
//!
//! - AVR properly operates on all integer types.
//!
//! Also, this test serves as a proof that rustc + LLVM generate correct AVR
//! code that is able to work on all types of numbers.
//!
//! See: [../../../avr-tester-fixtures/src/acc_eval.rs].

use avr_tester::{AvrTester, Uart, Writable, Writer, WriterHelper};
use rand::Rng;
use rand::seq::SliceRandom;
use std::fmt;

#[test]
fn simple() {
    const TRIES: usize = 128;

    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/acc-eval.elf");

    avr.run_for_ms(1);

    for ty in Type::all() {
        for op in Op::all() {
            println!("-> {:?}.{:?}", ty, op);

            let mut tries = 0;

            while tries < TRIES {
                let random_value = match op {
                    Op::Mul => Value::random_half,
                    _ => Value::random,
                };

                let lhs = random_value(ty);
                let rhs = random_value(ty);

                let expected = if let Some(value) = op.as_fn()(lhs, rhs) {
                    value
                } else {
                    continue;
                };

                let expr = Expr::from_op(op)(
                    Box::new(Expr::Const(lhs)),
                    Box::new(Expr::Const(rhs)),
                );

                avr.uart0().write(ty);
                avr.uart0().write(&expr);
                avr.run_for_ms(ty.weight());

                let actual = Value::read(ty, &mut avr.uart0());

                if actual != expected {
                    panic!("{expr} = {expected}, but AVR said {actual}");
                }

                tries += 1;
            }
        }
    }
}

#[test]
fn complex() {
    const TRIES: usize = 10;
    const MAX_DEPTH: u64 = 8;

    let mut rng = rand::thread_rng();

    let mut avr = AvrTester::atmega328()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/acc-eval.elf");

    avr.run_for_ms(1);

    for ty in Type::all() {
        for depth in 3..MAX_DEPTH {
            println!("-> {:?}.{}", ty, depth);

            let mut tries = 0;

            while tries < TRIES {
                let mut expr = Expr::Const(Value::random_half(ty));

                for _ in 0..=depth {
                    let build_expression = Expr::from_op(Op::random());
                    let value = Box::new(Expr::Const(Value::random_half(ty)));

                    expr = if rng.r#gen::<bool>() {
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
                    panic!("{expr} = {expected}, but AVR said {actual}");
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

    fn random() -> Self {
        Self::all()
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap()
    }

    fn as_fn(self) -> fn(Value, Value) -> Option<Value> {
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
        let mut rng = rand::thread_rng();

        match ty {
            Type::I8 => Self::I8(rng.r#gen()),
            Type::U8 => Self::U8(rng.r#gen()),
            Type::I16 => Self::I16(rng.r#gen()),
            Type::U16 => Self::U16(rng.r#gen()),
            Type::I32 => Self::I32(rng.r#gen()),
            Type::U32 => Self::U32(rng.r#gen()),
            Type::I64 => Self::I64(rng.r#gen()),
            Type::U64 => Self::U64(rng.r#gen()),
            Type::I128 => Self::I128(rng.r#gen()),
            Type::U128 => Self::U128(rng.r#gen()),
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::I8(value) => write!(f, "{value}_i8"),
            Value::U8(value) => write!(f, "{value}_u8"),
            Value::I16(value) => write!(f, "{value}_i16"),
            Value::U16(value) => write!(f, "{value}_u16"),
            Value::I32(value) => write!(f, "{value}_i32"),
            Value::U32(value) => write!(f, "{value}_u32"),
            Value::I64(value) => write!(f, "{value}_i64"),
            Value::U64(value) => write!(f, "{value}_u64"),
            Value::I128(value) => write!(f, "{value}_i128"),
            Value::U128(value) => write!(f, "{value}_u128"),
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
enum Expr {
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Rem(Box<Self>, Box<Self>),
    Const(Value),
}

impl Expr {
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

impl Writable for Expr {
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Add(lhs, rhs) => write!(f, "({lhs}) + ({rhs})"),
            Expr::Sub(lhs, rhs) => write!(f, "({lhs}) - ({rhs})"),
            Expr::Mul(lhs, rhs) => write!(f, "({lhs}) * ({rhs})"),
            Expr::Div(lhs, rhs) => write!(f, "({lhs}) / ({rhs})"),
            Expr::Rem(lhs, rhs) => write!(f, "({lhs}) % ({rhs})"),
            Expr::Const(value) => write!(f, "{value}"),
        }
    }
}
