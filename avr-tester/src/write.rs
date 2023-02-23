/// Object that can be written to, e.g. [`crate::Uart`].
pub trait Writer {
    fn write_byte(&mut self, value: u8);
}

pub trait WriterHelper {
    fn write<T>(&mut self, value: T)
    where
        T: Writable;
}

impl<'a> WriterHelper for dyn Writer + 'a {
    fn write<T>(&mut self, value: T)
    where
        T: Writable,
    {
        value.write(self);
    }
}

/// Value that can be transmitted through a [`Writer`].
pub trait Writable {
    fn write(&self, tx: &mut dyn Writer);
}

impl<T> Writable for &T
where
    T: Writable + ?Sized,
{
    fn write(&self, tx: &mut dyn Writer) {
        T::write(self, tx)
    }
}

impl Writable for u8 {
    fn write(&self, tx: &mut dyn Writer) {
        tx.write_byte(*self);
    }
}

impl Writable for [u8] {
    fn write(&self, tx: &mut dyn Writer) {
        for value in self {
            tx.write(value);
        }
    }
}

impl<const N: usize> Writable for [u8; N] {
    fn write(&self, tx: &mut dyn Writer) {
        tx.write(self.as_slice());
    }
}

impl Writable for str {
    fn write(&self, tx: &mut dyn Writer) {
        tx.write(self.as_bytes());
    }
}

impl Writable for String {
    fn write(&self, tx: &mut dyn Writer) {
        tx.write(self.as_str());
    }
}
