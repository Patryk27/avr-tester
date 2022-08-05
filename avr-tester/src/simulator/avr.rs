use super::*;
use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr::NonNull;
use std::{alloc, mem};

/// Convenient wrapper over [`ffi::avr_t`].
pub struct Avr {
    ptr: NonNull<ffi::avr_t>,
}

impl Avr {
    pub fn new(mcu: &str) -> Self {
        let c_mcu = CString::new(mcu).unwrap();
        let ptr = unsafe { ffi::avr_make_mcu_by_name(c_mcu.as_ptr()) };

        let ptr = NonNull::new(ptr)
            .unwrap_or_else(|| panic!("avr_make_mcu_by_name() failed: AVR `{}` is not known", mcu));

        Self { ptr }
    }

    pub fn init(mut self, frequency: u32) -> Self {
        let status = unsafe { ffi::avr_init(self.ptr.as_ptr()) };

        if status != 0 {
            panic!("avr_init() failed (status={})", status);
        }

        unsafe {
            self.ptr.as_mut().frequency = frequency;
        }

        self
    }

    pub fn cycle(&self) -> u64 {
        unsafe { self.ptr.as_ref().cycle }
    }

    pub fn frequency(&self) -> u32 {
        unsafe { self.ptr.as_ref().frequency }
    }

    pub fn run(&mut self) -> (CpuState, CpuDuration) {
        let cycle = self.cycle();
        let state = unsafe { ffi::avr_run(self.ptr.as_ptr()) };
        let tt = self.cycle() - cycle;
        let tt = tt.max(1);

        let state = CpuState::from_ffi(state);
        let tt = CpuDuration::new(self.frequency(), tt);

        (state, tt)
    }

    /// Shorthand for: [`ffi::avr_load_firmware()`].
    pub fn load_firmware(&mut self, elf: NonNull<ffi::elf_firmware_t>) {
        // Safety: We're non-null, the firmware is non-null, what can go wrong
        unsafe {
            ffi::avr_load_firmware(self.ptr.as_ptr(), elf.as_ptr());
        }
    }

    /// Shorthand for: [`ffi::avr_ioctl()`].
    ///
    /// # Safety
    ///
    /// Callers must ensure that given `ioctl` and `T` match (that is: different
    /// ioctls require parameters of different kinds, from u32 to structs).
    pub unsafe fn ioctl<T>(&mut self, ioctl: IoCtl, param: &mut T) -> c_int {
        ffi::avr_ioctl(
            self.ptr.as_ptr(),
            ioctl.into_ffi(),
            param as *mut _ as *mut _,
        )
    }

    /// Shorthand for: [`ffi::avr_io_getirq()`].
    pub fn io_getirq(&self, ioctl: IoCtl, irq: u32) -> Option<NonNull<ffi::avr_irq_t>> {
        // Safety: This function only searches for a pointer in `avr_t`'s data
        //         structures, so it's safe to call on all parameters
        let ptr = unsafe { ffi::avr_io_getirq(self.ptr.as_ptr(), ioctl.into_ffi(), irq as _) };

        NonNull::new(ptr)
    }

    /// Shorthand for: [`ffi::avr_raise_rq()`].
    ///
    /// # Safety
    ///
    /// Callers must ensure that given `value` makes sense when raised on `irq`.
    pub unsafe fn raise_irq(&mut self, irq: NonNull<ffi::avr_irq_t>, value: u32) {
        ffi::avr_raise_irq(irq.as_ptr(), value);
    }

    /// Shorthand for: [`ffi::avr_irq_register_notify()`].
    ///
    /// # Safety
    ///
    /// Callers must ensure that given callback is meant for `irq`.
    pub unsafe fn irq_register_notify<T>(
        &mut self,
        irq: NonNull<ffi::avr_irq_t>,
        notify: Option<unsafe extern "C" fn(NonNull<ffi::avr_irq_t>, u32, *mut T)>,
        param: *mut T,
    ) {
        // Safety: We're transmuting two parameters:
        // - `NonNull<ffi::avr_irq_t>` -> `*mut ffi::avr_irq_t`,
        // - `*mut T` -> `*mut c_void`
        //
        // ... where both conversions are legal.
        let notify = mem::transmute(notify);

        // Safety: We're transmuting `*mut T` -> `*mut c_void`, which is legal
        let param = mem::transmute(param);

        ffi::avr_irq_register_notify(irq.as_ptr(), notify, param);
    }

    fn layout() -> alloc::Layout {
        alloc::Layout::new::<ffi::avr_t>()
    }
}

impl Drop for Avr {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, Self::layout());
        }
    }
}
