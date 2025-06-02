use super::*;
use std::collections::VecDeque;
use std::ptr::NonNull;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum SpiOperationMode {
    Master,
    #[default]
    Slave,
}

#[derive(Debug)]
pub struct Spi {
    state: NonNull<SpiState>,
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
        let ioctl = IoCtl::SpiGetIrq { id };
        let irq_input = avr.try_io_getirq(ioctl, ffi::SPI_IRQ_INPUT)?;
        let irq_output = avr.try_io_getirq(ioctl, ffi::SPI_IRQ_OUTPUT)?;

        let this = Self {
            state: NonNull::from(Box::leak(Box::new(SpiState::new(irq_input)))),
            ticks: Default::default(),
            ready: Default::default(),
        };

        unsafe {
            Avr::irq_register_notify(
                irq_output,
                Some(Self::on_output),
                this.state.as_ptr(),
            );
        }

        Some(this)
    }

    pub fn set_operation_mode(&mut self, operation_mode: SpiOperationMode) {
        self.state_mut().operation_mode = operation_mode;
    }

    pub fn read(&mut self) -> Option<u8> {
        self.state_mut().rx.pop_front()
    }

    pub fn write(&mut self, byte: u8) {
        self.state_mut().tx.push_back(byte);
    }

    pub fn flush(&mut self) {
        if self.state_mut().operation_mode == SpiOperationMode::Slave
            || !self.ready
        {
            return;
        }

        let state = self.state_mut();
        if let Some(byte) = state.tx.pop_front() {
            unsafe {
                ffi::avr_raise_irq(state.irq_input.as_ptr(), byte as _);
            }

            self.ready = false;
        }
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

    fn state_mut(&mut self) -> &mut SpiState {
        // Safety: `state` points to a valid object; nothing else is writing
        // there at the moment, as guarded by `&mut self` here and on
        // `Avr::run()`
        unsafe { self.state.as_mut() }
    }

    unsafe extern "C" fn on_output(
        _: NonNull<ffi::avr_irq_t>,
        value: u32,
        mut state: NonNull<SpiState>,
    ) {
        unsafe {
            let state = state.as_mut();
            state.rx.push_back(value as u8);

            if state.operation_mode == SpiOperationMode::Slave {
                let byte = state.tx.pop_front().unwrap_or_default();
                ffi::avr_raise_irq(state.irq_input.as_ptr(), byte as _);
            }
        }
    }
}

impl Drop for Spi {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.state.as_ptr()));
        }
    }
}

#[derive(Debug)]
struct SpiState {
    /// irq used to signal that to the AVR that is has received a new byte
    irq_input: NonNull<ffi::avr_irq_t>,

    /// Queue of bytes scheduled to be sent into AVR.
    tx: VecDeque<u8>,

    /// Queue of bytes retrieved from AVR, pending to be read by the simulator.
    rx: VecDeque<u8>,

    /// Operation mode - whether we push bytes to the AVR, or respond to bytes pushed by the AVR
    operation_mode: SpiOperationMode,
}

impl SpiState {
    fn new(irq_input: NonNull<ffi::avr_irq_t>) -> Self {
        Self {
            irq_input,
            tx: Default::default(),
            rx: Default::default(),
            operation_mode: Default::default(),
        }
    }
}
