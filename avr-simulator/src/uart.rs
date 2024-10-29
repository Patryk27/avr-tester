use super::*;
use std::collections::VecDeque;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Uart {
    state: NonNull<UartState>,
    irq_input: NonNull<ffi::avr_irq_t>,
}

impl Uart {
    /// Initializes the subsystem; returns `None` if current AVR doesn't have
    /// this UART.
    ///
    /// # Safety
    ///
    /// - Because this function registers an IRQ notification, the object
    ///   returned from here must be kept alive for at least as long as `avr`.
    pub unsafe fn new(id: char, avr: &mut Avr) -> Option<Self> {
        let mut flags: u32 = 0;

        // First, let's make sure if the currently selected AVR supports this
        // UART; if not, let's quickly bail out with `None`.
        //
        // (e.g. ATmega328P has only one UART and so initializing UART2 would
        // fail there.)
        //
        // Safety: `IoCtl::UartGetFlags` requires a parameter of type `u32`
        let status = avr.ioctl(IoCtl::UartGetFlags { uart: id }, &mut flags);

        if status != 0 {
            return None;
        }

        // Our AVR supports this UART, neat!
        //
        // Now let's detach it from the standard output so that simavr doesn't
        // try to write there (this is especially important if someone's trying
        // to send binary data through this UART, which otherwise would've
        // gotten emitted into stdout as well).
        flags &= !ffi::AVR_UART_FLAG_STDIO;

        // Safety: `IoCtl::UartSetFlags` requires a parameter of type `u32`
        avr.ioctl(IoCtl::UartSetFlags { uart: id }, &mut flags);

        // ----
        // Now let's finalize everything by attaching to simavr's IRQs so that
        // we can get notified when AVR sends something through this UART.

        let ioctl = IoCtl::UartGetIrq { uart: id };
        let state = NonNull::from(Box::leak(Default::default()));

        // Safety: All of callbacks match the expected IRQs
        Avr::irq_register_notify(
            avr.io_getirq(ioctl, ffi::UART_IRQ_OUTPUT),
            Some(Self::on_output),
            state.as_ptr(),
        );

        Avr::irq_register_notify(
            avr.io_getirq(ioctl, ffi::UART_IRQ_OUT_XON),
            Some(Self::on_xon),
            state.as_ptr(),
        );

        Avr::irq_register_notify(
            avr.io_getirq(ioctl, ffi::UART_IRQ_OUT_XOFF),
            Some(Self::on_xoff),
            state.as_ptr(),
        );

        let irq_input =
            avr.io_getirq(IoCtl::UartGetIrq { uart: id }, ffi::UART_IRQ_INPUT);

        Some(Self { state, irq_input })
    }

    pub fn read(&mut self) -> Option<u8> {
        // Safety: We're releasing the borrow right-away
        unsafe { self.state_mut() }.rx.pop_front()
    }

    /// Schedules a byte to be sent during the nearest [`Self::flush()`].
    pub fn write(&mut self, byte: u8) {
        // Safety: We're releasing the borrow right-away
        unsafe { self.state_mut() }.tx.push_back(byte);
    }

    pub fn flush(&mut self) {
        loop {
            let byte = {
                // Safety: We're releasing the borrow before calling `.raise_irq()`
                let state = unsafe { self.state_mut() };

                if !state.xon {
                    break;
                }

                if let Some(byte) = state.tx.pop_front() {
                    byte
                } else {
                    break;
                }
            };

            unsafe {
                ffi::avr_raise_irq(self.irq_input.as_ptr(), byte as u32);
            }
        }
    }

    /// # Safety
    ///
    /// - UART interrupts are re-entrant, so the caller must make sure to
    ///   release the borrow before calling [`AvrManager::raise_irq()`].
    unsafe fn state_mut(&mut self) -> &mut UartState {
        self.state.as_mut()
    }

    unsafe extern "C" fn on_output(
        _: NonNull<ffi::avr_irq_t>,
        value: u32,
        mut state: NonNull<UartState>,
    ) {
        state.as_mut().rx.push_back(value as u8);
    }

    unsafe extern "C" fn on_xon(
        _: NonNull<ffi::avr_irq_t>,
        _: u32,
        mut state: NonNull<UartState>,
    ) {
        state.as_mut().xon = true;
    }

    unsafe extern "C" fn on_xoff(
        _: NonNull<ffi::avr_irq_t>,
        _: u32,
        mut state: NonNull<UartState>,
    ) {
        state.as_mut().xon = false;
    }
}

impl Drop for Uart {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.state.as_ptr()));
        }
    }
}

#[derive(Debug)]
struct UartState {
    /// Queue of bytes scheduled to be sent into AVR.
    tx: VecDeque<u8>,

    /// Queue of bytes retrieved from AVR, pending to be read by the simulator.
    rx: VecDeque<u8>,

    /// When true, AVR is ready to retrieve the next UART byte; AVR toggles this
    /// value on and off as we flush the next bytes.
    xon: bool,
}

impl Default for UartState {
    fn default() -> Self {
        Self {
            tx: Default::default(),
            rx: Default::default(),
            xon: true,
        }
    }
}
