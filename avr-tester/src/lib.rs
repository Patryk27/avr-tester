#![feature(maybe_uninit_uninit_array)]

mod simulator;

use self::simulator::*;
use std::{marker::PhantomData, mem::MaybeUninit, path::Path};

pub struct AvrTester<M> {
    sim: AvrSimulator,
    clock: u32,
    _mcu: PhantomData<M>,
}

impl<M> AvrTester<M> {
    fn new(mcu: &'static str, firmware: impl AsRef<Path>, clock: u32) -> Self {
        let mut sim = AvrSimulator::new(mcu, clock);

        sim.flash(firmware);

        Self {
            sim,
            clock,
            _mcu: Default::default(),
        }
    }

    pub fn run(&mut self) {
        let state = self.sim.run();

        if state != CpuState::Running {
            panic!("Unexpected CpuState: {:?}", state)
        }
    }

    pub fn run_for_ms(&mut self, ms: u32) {
        let cycles = self.clock / 1000 * ms;

        for _ in 0..cycles {
            self.run();
        }
    }
}

// ---

pub struct Atmega328p;

impl AvrTester<Atmega328p> {
    pub fn atmega_328p(firmware: impl AsRef<Path>, clock: u32) -> Self {
        Self::new("atmega328p", firmware, clock)
    }

    pub fn uart0(&mut self) -> Uart<Atmega328p> {
        Uart::new(&mut self.sim)
    }

    pub fn pins(&mut self) -> Pins<Atmega328p> {
        Pins::new(&mut self.sim)
    }
}

// ---

pub struct Uart<'a, M> {
    sim: &'a mut AvrSimulator,
    _mcu: PhantomData<M>,
}

impl<'a, M> Uart<'a, M> {
    fn new(sim: &'a mut AvrSimulator) -> Self {
        Self {
            sim,
            _mcu: Default::default(),
        }
    }

    pub fn send<T>(&mut self, value: &T)
    where
        T: UartSend<M>,
    {
        value.send(self);
    }

    pub fn send_byte(&mut self, value: u8) {
        self.sim.uart0_send(value);
    }

    pub fn send_bytes(&mut self, values: &[u8]) {
        for &value in values {
            self.send_byte(value);
        }
    }

    pub fn send_string(&mut self, value: &str) {
        self.send_bytes(value.as_bytes());
    }

    pub fn recv<T>(&mut self) -> T
    where
        T: UartRecv<M>,
    {
        T::recv(self)
    }

    pub fn recv_byte(&mut self) -> Option<u8> {
        self.sim.uart0_recv()
    }

    pub fn recv_bytes(&mut self) -> Vec<u8> {
        let mut bytes = Vec::new();

        while let Some(byte) = self.recv_byte() {
            bytes.push(byte);
        }

        bytes
    }

    pub fn recv_string(&mut self) -> String {
        String::from_utf8_lossy(&self.recv_bytes()).to_string()
    }
}

pub trait UartSend<M> {
    fn send<'a>(&self, uart: &mut Uart<'a, M>);
}

impl<M, T> UartSend<M> for &T
where
    T: UartSend<M>,
{
    fn send<'a>(&self, uart: &mut Uart<'a, M>) {
        T::send(self, uart)
    }
}

impl<const N: usize, M> UartSend<M> for [u8; N] {
    fn send<'a>(&self, uart: &mut Uart<'a, M>) {
        uart.send_bytes(self);
    }
}

pub trait UartRecv<M> {
    fn recv(uart: &mut Uart<'_, M>) -> Self;
}

impl<M> UartRecv<M> for u8 {
    fn recv(uart: &mut Uart<'_, M>) -> Self {
        uart.recv_byte().expect("Unexpected EOF")
    }
}

impl<const N: usize, M> UartRecv<M> for [u8; N] {
    fn recv(uart: &mut Uart<'_, M>) -> Self {
        let mut values: [MaybeUninit<u8>; N] = MaybeUninit::uninit_array();

        for value in &mut values {
            value.write(uart.recv());
        }

        // ---

        // TODO describe
        let ptr = &mut values as *mut _ as *mut [u8; N];
        unsafe { ptr.read() }
    }
}

// ---

pub struct Pins<'a, M> {
    sim: &'a mut AvrSimulator,
    _mcu: PhantomData<M>,
}

impl<'a, M> Pins<'a, M> {
    fn new(sim: &'a mut AvrSimulator) -> Self {
        Self {
            sim,
            _mcu: Default::default(),
        }
    }
}

impl<'a> Pins<'a, Atmega328p> {
    pub fn pd1(&mut self) -> Pin<'_, Atmega328p> {
        Pin::new(self.sim, b'D', 0)
    }
}

// ---

pub struct Pin<'a, M> {
    _sim: &'a mut AvrSimulator,
    _port: u8,
    _pin: u8,
    _mcu: PhantomData<M>,
}

impl<'a, M> Pin<'a, M> {
    fn new(sim: &'a mut AvrSimulator, port: u8, pin: u8) -> Self {
        Self {
            _sim: sim,
            _port: port,
            _pin: pin,
            _mcu: Default::default(),
        }
    }

    pub fn is_low(&self) -> bool {
        todo!()
    }

    pub fn is_high(&self) -> bool {
        todo!()
    }
}
