use crate::*;
use std::array;

/// Manages a single UART.
pub struct Uart<'a> {
    sim: &'a mut AvrSimulator,
    id: usize,
}

impl<'a> Uart<'a> {
    pub(crate) fn new(sim: &'a mut AvrSimulator, id: usize) -> Self {
        Self { sim, id }
    }

    /// Transmits an object to AVR.
    ///
    /// See: [`UartSend`].
    ///
    /// See also: [`Self::send_byte()`].
    pub fn send<T>(&mut self, value: T)
    where
        T: UartSend,
    {
        value.send(self);
    }

    /// Transmits a byte to AVR.
    ///
    /// Note that you don't have to worry about AVR's UART's buffer limits - if
    /// transmitting this byte would overflow that buffer, AvrTester will
    /// automatically re-schedule it to be sent at the nearest opportunity.
    ///
    /// See also: [`Self::send()`].
    pub fn send_byte(&mut self, value: u8) {
        self.sim.uart_send(self.id, value);
    }

    /// Retrieves an object from AVR.
    ///
    /// See: [`UartRecv`].
    ///
    /// See also: [`Self::recv_byte()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::*;
    /// # fn avr() -> AvrTester { panic!() }
    /// #
    /// let mut avr = avr();
    ///
    /// // Retrieves a single byte:
    /// // (when the input buffer is empty, panics.)
    /// assert_eq!(72, avr.uart0().recv::<u8>());
    ///
    /// // Retrieves the entire buffer:
    /// // (when it's empty, returns an empty vector.)
    /// assert_eq!(vec![72, 101, 108, 108, 111], avr.uart0().recv::<Vec<u8>>());
    ///
    /// // Retrieves `n` bytes from the buffer:
    /// // (when there's not enough bytes, panics.)
    /// assert_eq!([72, 101, 108, 108, 111], avr.uart0().recv::<[u8; 5]>());
    ///
    /// // Retrieves the entire input buffer and converts it into string:
    /// // (when it's empty, returns an empty string.)
    /// assert_eq!("Hello", avr.uart0().recv::<String>());
    /// ```
    pub fn recv<T>(&mut self) -> T
    where
        T: UartRecv,
    {
        T::recv(self)
    }

    /// Retrieves a byte from AVR.
    ///
    /// Returns `None` if there are no more bytes in the buffer, in which case
    /// no more bytes will appear at least until the next [`AvrTester::run()`].
    ///
    /// See also: [`Self::recv()`].
    pub fn recv_byte(&mut self) -> Option<u8> {
        self.sim.uart_recv(self.id)
    }

    /// Returns the first byte that's waiting to be retrieved (if any).
    pub fn peek_byte(&mut self) -> Option<u8> {
        self.sim.uart_peek(self.id)
    }
}

/// Type that can be transmitted through [`Uart::send()`].
///
/// You can implement it for your types to make tests more readable.
pub trait UartSend {
    fn send<'a>(&self, uart: &mut Uart<'a>);
}

impl<T> UartSend for &T
where
    T: UartSend + ?Sized,
{
    fn send<'a>(&self, uart: &mut Uart<'a>) {
        T::send(self, uart)
    }
}

impl UartSend for u8 {
    fn send<'a>(&self, uart: &mut Uart<'a>) {
        uart.send_byte(*self);
    }
}

impl UartSend for [u8] {
    fn send<'a>(&self, uart: &mut Uart<'a>) {
        for value in self {
            value.send(uart);
        }
    }
}

impl UartSend for str {
    fn send<'a>(&self, uart: &mut Uart<'a>) {
        self.as_bytes().send(uart);
    }
}

impl<const N: usize> UartSend for [u8; N] {
    fn send<'a>(&self, uart: &mut Uart<'a>) {
        (self as &[u8]).send(uart);
    }
}

/// Type that can be retrieved through [`Uart::recv()`].
///
/// You can implement it for your types to make tests more readable.
pub trait UartRecv {
    fn recv(uart: &mut Uart<'_>) -> Self;
}

impl UartRecv for u8 {
    fn recv(uart: &mut Uart<'_>) -> Self {
        uart.recv_byte()
            .expect("UART's buffer is empty (i.e. AVR did not send more bytes)")
    }
}

impl<const N: usize> UartRecv for [u8; N] {
    fn recv(uart: &mut Uart<'_>) -> Self {
        array::from_fn(|_| uart.recv())
    }
}

impl UartRecv for Vec<u8> {
    fn recv(uart: &mut Uart<'_>) -> Self {
        let mut bytes = Vec::new();

        while let Some(byte) = uart.recv_byte() {
            bytes.push(byte);
        }

        bytes
    }
}

impl UartRecv for String {
    fn recv(uart: &mut Uart<'_>) -> Self {
        String::from_utf8_lossy(&Vec::<u8>::recv(uart)).to_string()
    }
}
