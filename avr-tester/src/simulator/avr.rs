use super::*;
use simavr_ffi as ffi;
use std::alloc;
use std::ffi::CString;
use std::os::raw::c_int;

pub struct Avr {
    ptr: *mut ffi::avr_t,
}

impl Avr {
    pub fn new(mcu: &'static str) -> Self {
        let c_mcu = CString::new(mcu).unwrap();
        let ptr = unsafe { ffi::avr_make_mcu_by_name(c_mcu.as_ptr()) };

        if ptr.is_null() {
            panic!("avr_make_mcu_by_name() failed: AVR `{}` is not known", mcu);
        }

        Self { ptr }
    }

    pub fn init(mut self, clock: u32) -> Self {
        let status = unsafe { ffi::avr_init(self.ptr) };

        if status != 0 {
            panic!("avr_init() failed (status={})", status);
        }

        unsafe {
            (*self.ptr).frequency = clock;
        }

        self
    }

    pub fn cycle(&self) -> u64 {
        unsafe { (*self.ptr).cycle }
    }

    pub fn run(&mut self) -> (CpuState, CpuCyclesTaken) {
        let cycle = self.cycle();
        let state = unsafe { ffi::avr_run(self.ptr) };
        let cycles_taken = self.cycle() - cycle;

        let state = CpuState::from_ffi(state);
        let cycles_taken = CpuCyclesTaken::new(cycles_taken);

        (state, cycles_taken)
    }

    pub unsafe fn ioctl<T>(&mut self, ioctl: IoCtl, param: &mut T) -> c_int {
        ffi::avr_ioctl(self.ptr, ioctl.into_ffi(), param as *mut _ as *mut _)
    }

    pub unsafe fn io_getirq(&mut self, ioctl: IoCtl, irq: u32) -> *mut ffi::avr_irq_t {
        ffi::avr_io_getirq(self.ptr, ioctl.into_ffi(), irq as _)
    }

    pub fn ptr(&mut self) -> *mut ffi::avr_t {
        self.ptr
    }

    fn layout() -> alloc::Layout {
        alloc::Layout::new::<ffi::avr_t>()
    }
}

impl Drop for Avr {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, Self::layout());
        }
    }
}
