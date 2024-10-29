use super::*;
use std::collections::VecDeque;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Spi {
    state: NonNull<SpiState>,
    irq_input: NonNull<ffi::avr_irq_t>,
    ticks: u64,
    ready: bool,
}

impl Spi {
    /// Initializes the subsystem; returns `None` if current AVR doesn't have
    /// this SPI.
    ///
    /// # Safety
    ///
    /// - Because this function registers an IRQ notification, the object
    ///   returned from here must be kept alive for at least as long as `avr`.
    pub unsafe fn new(id: u8, avr: &Avr) -> Option<Self> {
        let ioctl = IoCtl::SpiGetIrq { spi: id };
        let irq_input = avr.try_io_getirq(ioctl, ffi::SPI_IRQ_INPUT)?;
        let irq_output = avr.try_io_getirq(ioctl, ffi::SPI_IRQ_OUTPUT)?;

        let this = Self {
            state: NonNull::from(Box::leak(Default::default())),
            irq_input,
            ticks: 0,
            ready: true,
        };

        Avr::irq_register_notify(
            irq_output,
            Some(Self::on_output),
            this.state.as_ptr(),
        );

        Some(this)
    }

    pub fn read(&mut self) -> Option<u8> {
        self.borrow_mut().rx.pop_front()
    }

    pub fn write(&mut self, byte: u8) {
        self.borrow_mut().tx.push_back(byte);
    }

    pub fn flush(&mut self) {
        if !self.ready {
            return;
        }

        if let Some(byte) = self.borrow_mut().tx.pop_front() {
            unsafe {
                ffi::avr_raise_irq(self.irq_input.as_ptr(), byte as _);
            }

            self.ready = false;
        }
    }

    fn borrow_mut(&mut self) -> &mut SpiState {
        // Safety: `state` points to a valid object; nothing else is writing
        // there at the moment, as guarded by `&mut self` here and on
        // `Avr::run()`
        unsafe { self.state.as_mut() }
    }

    pub fn tick(&mut self, tt: u64) {
        self.ticks += tt;

        // HACK unfortunately, as compared to UART, the SPI interface doesn't
        //      expose any kind of xon / xoff flag, forcing us to improvise
        if self.ticks >= 128 + 64 {
            self.ticks = 0;
            self.ready = true;
        }
    }

    unsafe extern "C" fn on_output(
        _: NonNull<ffi::avr_irq_t>,
        value: u32,
        mut state: NonNull<SpiState>,
    ) {
        state.as_mut().rx.push_back(value as u8);
    }
}

impl Drop for Spi {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.state.as_ptr()));
        }
    }
}

#[derive(Debug, Default)]
struct SpiState {
    /// Queue of bytes scheduled to be sent into AVR.
    tx: VecDeque<u8>,

    /// Queue of bytes retrieved from AVR, pending to be read by the simulator.
    rx: VecDeque<u8>,
}
