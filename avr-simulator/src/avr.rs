use super::*;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_int;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Avr {
    inner: NonNull<ffi::avr_t>,
}

impl Avr {
    pub fn new(mcu: &str, frequency: u32) -> Self {
        let c_mcu = CString::new(mcu).unwrap();

        // Safety: `c_mcu` points to a valid C-style string
        let inner = unsafe { ffi::avr_make_mcu_by_name(c_mcu.as_ptr()) };
        let inner = NonNull::new(inner).unwrap_or_else(|| panic!("Unknown AVR: {}", mcu));

        let mut this = Self { inner };

        // Safety: `inner` points to a valid `avr_t`; we've just received from
        // simavr, after all!
        let status = unsafe { ffi::avr_init(this.inner.as_ptr()) };

        if status != 0 {
            panic!("avr_init() failed (status={})", status);
        }

        // Safety: `inner` points to a valid `avr_t`; nothing else is writing
        // there at the moment
        unsafe {
            this.inner.as_mut().frequency = frequency;
        }

        this
    }

    pub fn cycle(&self) -> u64 {
        // Safety: `inner` points to a valid `avr_t`; nothing else is writing
        // there at the moment, as guarded by `&mut self` on `fn run()`
        unsafe { self.inner.as_ref().cycle }
    }

    pub fn frequency(&self) -> u32 {
        // Safety: `inner` points to a valid `avr_t`; nothing else is writing
        // there at the moment, as guarded by `&mut self` on `fn run()`
        unsafe { self.inner.as_ref().frequency }
    }

    pub fn run(&mut self) -> AvrState {
        // Safety: `inner` points to a valid `avr_t`; nothing else is writing
        // there at the moment, as guarded by `&mut self` here
        let state = unsafe { ffi::avr_run(self.inner.as_ptr()) };

        AvrState::from_ffi(state)
    }

    pub fn load_firmware(&mut self, elf: NonNull<ffi::elf_firmware_t>) {
        // Safety: `inner` points to a valid `avr_t`, `elf` points to a valid
        // firmware; nothing else is writing into the AVR at the moment, as
        // guarded by `&mut self` here
        unsafe {
            ffi::avr_load_firmware(self.inner.as_ptr(), elf.as_ptr());
        }
    }

    /// # Safety
    ///
    /// Callers must ensure that given `ioctl` and `T` match (that is: different
    /// ioctls require parameters of different kinds, from u32 to structs).
    pub unsafe fn ioctl<T>(&mut self, ioctl: IoCtl, param: &mut T) -> c_int {
        ffi::avr_ioctl(
            self.inner.as_ptr(),
            ioctl.into_ffi(),
            param as *mut _ as *mut _,
        )
    }

    pub fn io_getirq(&self, ioctl: IoCtl, irq: u32) -> NonNull<ffi::avr_irq_t> {
        self.try_io_getirq(ioctl, irq)
            .unwrap_or_else(|| panic!("avr_io_getirq({ioctl:#?}, {irq}) failed"))
    }

    pub fn try_io_getirq(&self, ioctl: IoCtl, irq: u32) -> Option<NonNull<ffi::avr_irq_t>> {
        // Safety: `avr_io_getirq()` only looks for the matching ioctl - it
        // doesn't require any specific payload or anything
        let ptr = unsafe { ffi::avr_io_getirq(self.inner.as_ptr(), ioctl.into_ffi(), irq as _) };

        NonNull::new(ptr)
    }

    /// # Safety
    ///
    /// Callers must ensure that given callback is meant for given `irq`.
    pub unsafe fn irq_register_notify<T>(
        irq: NonNull<ffi::avr_irq_t>,
        notify: Option<unsafe extern "C" fn(NonNull<ffi::avr_irq_t>, u32, NonNull<T>)>,
        param: *mut T,
    ) {
        // Safety: We're transmuting two parameters:
        //
        // - `NonNull<ffi::avr_irq_t>` -> `*mut ffi::avr_irq_t`,
        // - `NonNull<T>` -> `*mut c_void`
        //
        // ... where both conversions are legal.
        let notify = mem::transmute(notify);

        // Safety: We're transmuting `*mut T` -> `*mut c_void`, which is legal
        let param = mem::transmute(param);

        ffi::avr_irq_register_notify(irq.as_ptr(), notify, param);
    }
}

impl Drop for Avr {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.inner.as_ptr() as *mut _);
        }
    }
}
