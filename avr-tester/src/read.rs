use std::{array, iter};

/// Object that can be read from, e.g. [`crate::Uart`].
pub trait Reader {
    fn read_byte(&mut self) -> u8;
    fn try_read_byte(&mut self) -> Option<u8>;
}

pub trait ReaderHelper {
    fn read<T>(&mut self) -> T
    where
        T: Readable;
}

impl<'a> ReaderHelper for dyn Reader + 'a {
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
