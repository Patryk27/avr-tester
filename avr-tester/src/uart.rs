use crate::simulator::AvrSimulator;
use std::mem::MaybeUninit;

pub struct Uart<'a> {
    sim: &'a mut AvrSimulator,
    id: usize,
}

impl<'a> Uart<'a> {
    pub(crate) fn new(sim: &'a mut AvrSimulator, id: usize) -> Self {
        Self { sim, id }
    }

    pub fn send<T>(&mut self, value: &T)
    where
        T: UartSend,
    {
        value.send(self);
    }

    pub fn send_byte(&mut self, value: u8) {
        self.sim.uart_send(self.id, value);
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
        T: UartRecv,
    {
        T::recv(self)
    }

    pub fn recv_byte(&mut self) -> Option<u8> {
        self.sim.uart_recv(self.id)
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

pub trait UartSend {
    fn send<'a>(&self, uart: &mut Uart<'a>);
}

impl<T> UartSend for &T
where
    T: UartSend,
{
    fn send<'a>(&self, uart: &mut Uart<'a>) {
        T::send(self, uart)
    }
}

impl<const N: usize> UartSend for [u8; N] {
    fn send<'a>(&self, uart: &mut Uart<'a>) {
        uart.send_bytes(self);
    }
}

pub trait UartRecv {
    fn recv(uart: &mut Uart<'_>) -> Self;
}

impl UartRecv for u8 {
    fn recv(uart: &mut Uart<'_>) -> Self {
        uart.recv_byte().expect("Unexpected EOF")
    }
}

impl<const N: usize> UartRecv for [u8; N] {
    fn recv(uart: &mut Uart<'_>) -> Self {
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
