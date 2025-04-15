use super::*;
use std::collections::VecDeque;
use std::ptr::NonNull;

pub type AdcId = u8;
pub type AdcMillivolts = u32;

#[derive(Debug)]
pub struct Adc {
    state: NonNull<AdcState>,
}

impl Adc {
    /// Initializes the subsystem; returns `None` if current AVR doesn't have an
    /// ADC.
    ///
    /// # Safety
    ///
    /// - Because this function registers an IRQ notification, the object
    ///   returned from here must be kept alive for at least as long as `avr`.
    pub unsafe fn new(avr: &Avr) -> Option<Self> {
        let irq =
            avr.try_io_getirq(IoCtl::AdcGetIrq, ffi::ADC_IRQ_OUT_TRIGGER)?;

        let this = Self {
            state: NonNull::from(Box::leak(Default::default())),
        };

        unsafe {
            Avr::irq_register_notify(
                irq,
                Some(Self::on_adc_ready),
                this.state.as_ptr(),
            );
        }

        Some(this)
    }

    pub fn set_voltage(&mut self, id: AdcId, voltage: AdcMillivolts) {
        self.state_mut().voltages.push_back((id, voltage));
    }

    fn state_mut(&mut self) -> &mut AdcState {
        // Safety: `state` points to a valid object; nothing else is writing
        // there at the moment, as guarded by `&mut self` here and on
        // `Avr::run()`
        unsafe { self.state.as_mut() }
    }

    unsafe extern "C" fn on_adc_ready(
        irq: NonNull<ffi::avr_irq_t>,
        _: u32,
        mut state: NonNull<AdcState>,
    ) {
        unsafe {
            let (adc_id, adc_voltage) =
                if let Some(voltage) = state.as_mut().voltages.pop_front() {
                    voltage
                } else {
                    return;
                };

            let irq = irq
                .as_ptr()
                .sub(ffi::ADC_IRQ_OUT_TRIGGER as _)
                .add(adc_id as _);

            ffi::avr_raise_irq(irq, adc_voltage);
        }
    }
}

impl Drop for Adc {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.state.as_ptr()));
        }
    }
}

#[derive(Default)]
struct AdcState {
    voltages: VecDeque<(AdcId, AdcMillivolts)>,
}
