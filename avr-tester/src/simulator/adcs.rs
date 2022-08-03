use super::*;
use std::{cell::UnsafeCell, collections::VecDeque, ptr::NonNull};

pub type AdcId = u8;
pub type AdcVoltage = u32;

/// Provides access to simavr's ADCs (aka _analog pins_).
///
/// See also: [`Ports`].
pub struct Adcs {
    ptr: NonNull<AdcInner>,
}

impl Adcs {
    pub fn new() -> Self {
        let ptr = Box::into_raw(Default::default());

        // Unwrap-safety: `Box::into_raw()` doesn't return null pointers
        let ptr = NonNull::new(ptr).unwrap();

        Self { ptr }
    }

    pub fn try_init(self, avr: &mut Avr) -> Option<Self> {
        let irq = avr.io_getirq(IoCtl::AdcGetIrq, ffi::ADC_IRQ_OUT_TRIGGER)?;

        unsafe {
            avr.irq_register_notify(irq, Some(Self::on_adc_ready), self.ptr.as_ptr());
        }

        Some(self)
    }

    pub fn set_voltage(&mut self, id: AdcId, voltage: AdcVoltage) {
        unsafe {
            (*self.ptr.as_ptr()).push_voltage(id, voltage);
        }
    }

    unsafe extern "C" fn on_adc_ready(irq: NonNull<ffi::avr_irq_t>, _: u32, adc: *mut AdcInner) {
        let (adc_id, adc_voltage) = if let Some(voltage) = (*adc).pop_voltage() {
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

impl Drop for Adcs {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

#[derive(Default)]
struct AdcInner {
    voltages: UnsafeCell<VecDeque<(AdcId, AdcVoltage)>>,
}

impl AdcInner {
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::pop_voltage()`].
    unsafe fn push_voltage(&self, id: AdcId, voltage: AdcVoltage) {
        (*self.voltages.get()).push_back((id, voltage));
    }

    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::push_voltage()`].
    unsafe fn pop_voltage(&self) -> Option<(AdcId, AdcVoltage)> {
        (*self.voltages.get()).pop_front()
    }
}
