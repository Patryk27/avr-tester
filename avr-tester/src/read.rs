use std::{array, iter};

/// Object that can be read from, e.g. [`crate::Uart`].
pub trait Reader {
    /// Reads a byte from the AVR; when the buffer is empty, panics.
    fn read_byte(&mut self) -> u8;

    /// Reads a byte from the AVR; when the buffer is empty, returns `None`.
    fn try_read_byte(&mut self) -> Option<u8>;
}

pub trait ReaderHelper {
    fn read<T>(&mut self) -> T
    where
        T: Readable;
}

impl ReaderHelper for dyn Reader + '_ {
    fn read<T>(&mut self) -> T
    where
        T: Readable,
    {
        T::read(self)
    }
}

/// Value that can be retrieved from a [`Reader`].
pub trait Readable {
    fn read(rx: &mut dyn Reader) -> Self;
}

impl Readable for u8 {
    fn read(rx: &mut dyn Reader) -> Self {
        rx.read_byte()
    }
}

impl Readable for Option<u8> {
    fn read(rx: &mut dyn Reader) -> Self {
        rx.try_read_byte()
    }
}

impl<const N: usize> Readable for [u8; N] {
    fn read(rx: &mut dyn Reader) -> Self {
        array::from_fn(|_| rx.read())
    }
}

impl Readable for Vec<u8> {
    fn read(rx: &mut dyn Reader) -> Self {
        iter::from_fn(|| rx.try_read_byte()).collect()
    }
}

impl Readable for String {
    fn read(rx: &mut dyn Reader) -> Self {
        let bytes: Vec<u8> = Readable::read(rx);

        String::from_utf8_lossy(&bytes).to_string()
    }
}
