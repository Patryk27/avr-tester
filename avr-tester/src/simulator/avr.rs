use super::CpuState;
use simavr_ffi as ffi;
use std::alloc;
use std::ffi::CString;

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

    pub fn run(&mut self) -> CpuState {
        let state = unsafe { ffi::avr_run(self.ptr) };

        CpuState::from_ffi(state)
    }

    // TODO in theory, it's only safe to call this function after `.init()` has
    //      happened
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
