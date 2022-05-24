use super::{Avr, IoCtl};
use simavr_ffi as ffi;
use std::{cell::UnsafeCell, collections::VecDeque, ffi::c_void};

pub struct Uart {
    ptr: *mut UartT,
    id: u8,
}

impl Uart {
    pub fn new(id: u8) -> Self {
        Self {
            ptr: Box::into_raw(Default::default()),
            id,
        }
    }

    pub fn init(self, avr: &mut Avr) -> Self {
        unsafe {
            let mut flags: u32 = 0;

            ffi::avr_ioctl(
                avr.ptr(),
                IoCtl::UartGetFlags { uart_id: self.id }.into_ffi(),
                &mut flags as *mut _ as *mut _,
            );

            flags &= !ffi::AVR_UART_FLAG_STDIO;

            ffi::avr_ioctl(
                avr.ptr(),
                IoCtl::UartSetFlags { uart_id: self.id }.into_ffi(),
                &mut flags as *mut _ as *mut _,
            );
        }

        unsafe {
            let ioctl = IoCtl::UartGetIrq { uart_id: self.id }.into_ffi();
            let irq_output = ffi::avr_io_getirq(avr.ptr(), ioctl, ffi::UART_IRQ_OUTPUT as _);
            let irq_xon = ffi::avr_io_getirq(avr.ptr(), ioctl, ffi::UART_IRQ_OUT_XON as _);
            let irq_xoff = ffi::avr_io_getirq(avr.ptr(), ioctl, ffi::UART_IRQ_OUT_XOFF as _);

            if irq_output.is_null() || irq_xon.is_null() || irq_xoff.is_null() {
                panic!("avr_io_getirq() failed (got a null pointer; maybe your AVR doesn't support UART{}?)", self.id);
            }

            ffi::avr_irq_register_notify(irq_output, Some(Self::on_output), self.ptr as *mut _);
            ffi::avr_irq_register_notify(irq_xon, Some(Self::on_xon), self.ptr as *mut _);
            ffi::avr_irq_register_notify(irq_xoff, Some(Self::on_xoff), self.ptr as *mut _);
        }

        self
    }

    pub fn flush(&mut self, avr: &mut Avr) {
        let this = unsafe { &*self.ptr };
        let mut irq = None;

        loop {
            if !this.is_xon() {
                break;
            }

            let byte = if let Some(byte) = this.tx_pop() {
                byte
            } else {
                break;
            };

            let irq = irq.get_or_insert_with(|| {
                let ioctl = IoCtl::UartGetIrq { uart_id: self.id }.into_ffi();
                let irq = unsafe { ffi::avr_io_getirq(avr.ptr(), ioctl, ffi::UART_IRQ_INPUT as _) };

                if irq.is_null() {
                    panic!("avr_io_getirq() failed (got a null pointer)")
                }

                irq
            });

            unsafe {
                ffi::avr_raise_irq(*irq, byte as _);
            }
        }
    }

    pub fn recv(&mut self) -> Option<u8> {
        let this = unsafe { &*self.ptr };

        this.rx_pop()
    }

    pub fn send(&mut self, byte: u8) {
        let this = unsafe { &*self.ptr };

        this.tx_push(byte);
    }

    unsafe extern "C" fn on_output(_: *mut ffi::avr_irq_t, value: u32, uart: *mut c_void) {
        UartT::from_ptr(uart).rx_push(value as u8);
    }

    unsafe extern "C" fn on_xon(_: *mut ffi::avr_irq_t, _: u32, uart: *mut c_void) {
        UartT::from_ptr(uart).set_xon();
    }

    unsafe extern "C" fn on_xoff(_: *mut ffi::avr_irq_t, _: u32, uart: *mut c_void) {
        UartT::from_ptr(uart).set_xoff();
    }
}

impl Drop for Uart {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}

#[derive(Debug)]
pub struct UartT {
    rx: UnsafeCell<VecDeque<u8>>,
    tx: UnsafeCell<VecDeque<u8>>,
    xon: UnsafeCell<bool>,
}

impl UartT {
    const MAX_BYTES: usize = 16 * 1024;

    unsafe fn from_ptr<'a>(uart: *mut c_void) -> &'a Self {
        &*(uart as *mut Self)
    }

    pub fn rx_push(&self, value: u8) {
        let rx = unsafe { &mut *self.rx.get() };

        if rx.len() < Self::MAX_BYTES {
            rx.push_back(value);
        }
    }

    pub fn rx_pop(&self) -> Option<u8> {
        let rx = unsafe { &mut *self.rx.get() };

        rx.pop_front()
    }

    pub fn tx_push(&self, value: u8) {
        let tx = unsafe { &mut *self.tx.get() };

        tx.push_back(value);
    }

    pub fn tx_pop(&self) -> Option<u8> {
        let tx = unsafe { &mut *self.tx.get() };

        tx.pop_front()
    }

    pub fn is_xon(&self) -> bool {
        let xon = unsafe { &mut *self.xon.get() };

        *xon
    }

    pub fn set_xon(&self) {
        let xon = unsafe { &mut *self.xon.get() };

        *xon = true;
    }

    pub fn set_xoff(&self) {
        let xon = unsafe { &mut *self.xon.get() };

        *xon = false;
    }
}

impl Default for UartT {
    fn default() -> Self {
        Self {
            rx: Default::default(),
            tx: Default::default(),
            xon: UnsafeCell::new(true),
        }
    }
}
