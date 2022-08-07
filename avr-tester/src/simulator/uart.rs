use super::*;
use std::{cell::UnsafeCell, collections::VecDeque, ptr::NonNull};

/// Provides access to simavr's UARTs.
pub struct Uart {
    ptr: NonNull<UartInner>,
    id: char,
}

impl Uart {
    pub fn new(id: char) -> Self {
        let ptr = Box::into_raw(Default::default());

        // Unwrap-safety: `Box::into_raw()` doesn't return null pointers
        let ptr = NonNull::new(ptr).unwrap();

        Self { ptr, id }
    }

    pub fn try_init(self, avr: &mut Avr) -> Option<Self> {
        let mut flags: u32 = 0;

        // First, let's see if the AVR we're running at supports this UART (e.g.
        // there's no UART2 on Atmega328p)
        //
        // Safety: `IoCtl::UartGetFlags` requires parameter of type `u32`, which
        //         is the case here
        let status = unsafe { avr.ioctl(IoCtl::UartGetFlags { uart: self.id }, &mut flags) };

        if status != 0 {
            return None;
        }

        // Our AVR supports this UART, neat!
        //
        // Now let's detach it from the standard output so that simavr doesn't
        // try to write there (this is especially important if someone's trying
        // to send binary data through this UART)
        flags &= !ffi::AVR_UART_FLAG_STDIO;

        // Safety: `IoCtl::UartSetFlags` requires parameter of type `u32`, which
        //         is the case here
        unsafe {
            avr.ioctl(IoCtl::UartSetFlags { uart: self.id }, &mut flags);
        }

        // ----
        // Now let's finalize everything by attaching to simavr's IRQs, so that
        // we can easily get notified when AVR sends something through UART.

        let ioctl = IoCtl::UartGetIrq { uart: self.id };

        let irq_output = avr
            .io_getirq(ioctl, ffi::UART_IRQ_OUTPUT)
            .unwrap_or_else(|| {
                panic!(
                    "avr_io_getirq() failed (got a null pointer for UART{}'s output)",
                    self.id
                )
            });

        let irq_xon = avr
            .io_getirq(ioctl, ffi::UART_IRQ_OUT_XON)
            .unwrap_or_else(|| {
                panic!(
                    "avr_io_getirq() failed (got a null pointer for UART{}'s XON)",
                    self.id
                )
            });

        let irq_xoff = avr
            .io_getirq(ioctl, ffi::UART_IRQ_OUT_XOFF)
            .unwrap_or_else(|| {
                panic!(
                    "avr_io_getirq() failed (got a null pointer for UART{}'s XOFF)",
                    self.id
                )
            });

        // Safety: All of our callbacks match the expected IRQs
        unsafe {
            avr.irq_register_notify(irq_output, Some(Self::on_output), self.ptr.as_ptr());
            avr.irq_register_notify(irq_xon, Some(Self::on_xon), self.ptr.as_ptr());
            avr.irq_register_notify(irq_xoff, Some(Self::on_xoff), self.ptr.as_ptr());
        }

        Some(self)
    }

    pub fn flush(&mut self, avr: &mut Avr) {
        let this = self.get();
        let mut irq = None;

        loop {
            // Safety: `&mut self` ensures that while we are working, simavr
            //         won't interrupt us
            if unsafe { !this.is_xon() } {
                break;
            }

            // Safety: `&mut self` ensures that while we are working, simavr
            //         won't interrupt us
            let byte = if let Some(byte) = unsafe { this.pop_tx() } {
                byte
            } else {
                break;
            };

            let irq = irq.get_or_insert_with(|| {
                // Unwrap-safety: Since we've come this far, then the chosen AVR
                //                certainly supports this UART and there's no
                //                reason for that instruction to panic
                avr.io_getirq(IoCtl::UartGetIrq { uart: self.id }, ffi::UART_IRQ_INPUT)
                    .unwrap()
            });

            // Safety: `UART_IRQ_INPUT` is meant to send data through UART and
            //         supports being raised with any byte-parameter
            unsafe {
                avr.raise_irq(*irq, byte as _);
            }
        }
    }

    pub fn send(&mut self, byte: u8) {
        // Safety: `&mut self` ensures that while we are working, simavr won't
        //         interrupt us
        unsafe {
            self.get().push_tx(byte);
        }
    }

    pub fn recv(&mut self) -> Option<u8> {
        // Safety: `&mut self` ensures that while we are working, simavr won't
        //         interrupt us
        unsafe { self.get().pop_rx() }
    }

    pub fn peek(&mut self) -> Option<u8> {
        // Safety: `&mut self` ensures that while we are working, simavr won't
        //         interrupt us
        unsafe { self.get().peek_rx() }
    }

    fn get(&self) -> &UartInner {
        // Safety: `self.ptr` is alive as long as `self`
        unsafe { self.ptr.as_ref() }
    }

    unsafe extern "C" fn on_output(_: NonNull<ffi::avr_irq_t>, value: u32, uart: *mut UartInner) {
        UartInner::from_ptr(uart).push_rx(value as u8);
    }

    unsafe extern "C" fn on_xon(_: NonNull<ffi::avr_irq_t>, _: u32, uart: *mut UartInner) {
        UartInner::from_ptr(uart).set_xon();
    }

    unsafe extern "C" fn on_xoff(_: NonNull<ffi::avr_irq_t>, _: u32, uart: *mut UartInner) {
        UartInner::from_ptr(uart).set_xoff();
    }
}

impl Drop for Uart {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

#[derive(Debug)]
struct UartInner {
    rx: UnsafeCell<VecDeque<u8>>,
    tx: UnsafeCell<VecDeque<u8>>,
    xon: UnsafeCell<bool>,
}

impl UartInner {
    const RX_BUFFER_MAX_BYTES: usize = 128 * 1024;

    unsafe fn from_ptr<'a>(uart: *mut UartInner) -> &'a Self {
        &*(uart as *mut Self)
    }

    /// Called by simavr when the AVR transmits a byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::pop_rx()`] or
    /// [`Self::peek_rx()`].
    unsafe fn push_rx(&self, value: u8) {
        let rx = &mut *self.rx.get();

        if rx.len() < Self::RX_BUFFER_MAX_BYTES {
            rx.push_back(value);
        }
    }

    /// Called by AvrTester when user wants to retrieve a single byte from the
    /// buffer.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::push_rx()`] or
    /// [`Self::peek_rx()`].
    unsafe fn pop_rx(&self) -> Option<u8> {
        (*self.rx.get()).pop_front()
    }

    /// Called by AvrTester when user wants to peek at the currently-pending
    /// byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::push_rx()`] or
    /// [`Self::pop_rx()`].
    unsafe fn peek_rx(&self) -> Option<u8> {
        (*self.rx.get()).front().copied()
    }

    /// Called by AvrTester when user wants to transmit a byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::pop_tx()`].
    unsafe fn push_tx(&self, value: u8) {
        (*self.tx.get()).push_back(value);
    }

    /// Called by simavr when the AVR is ready to retrieve a byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::push_tx()`].
    unsafe fn pop_tx(&self) -> Option<u8> {
        (*self.tx.get()).pop_front()
    }

    /// Called by AvrTester to check whether the AVR is ready to retrieve a
    /// byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::set_xon()`] or
    /// [`Self::set_xoff()`].
    unsafe fn is_xon(&self) -> bool {
        *self.xon.get()
    }

    /// Called by simavr when the AVR's UART buffer is full.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::is_xon()`] or
    /// [`Self::set_xoff()`].
    unsafe fn set_xon(&self) {
        *self.xon.get() = true;
    }

    /// Called by simavr when the AVR's UART buffer is ready to accept more
    /// bytes.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::is_xon()`] or
    /// [`Self::set_xon()`].
    unsafe fn set_xoff(&self) {
        *self.xon.get() = false;
    }
}

impl Default for UartInner {
    fn default() -> Self {
        Self {
            rx: Default::default(),
            tx: Default::default(),
            xon: UnsafeCell::new(true),
        }
    }
}
