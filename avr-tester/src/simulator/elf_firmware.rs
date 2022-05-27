use super::*;
use simavr_ffi as ffi;
use std::{alloc, ffi::CString, path::Path};

pub struct ElfFirmware {
    ptr: *mut ffi::elf_firmware_t,
}

impl ElfFirmware {
    pub fn new() -> Self {
        let ptr = unsafe { alloc::alloc_zeroed(Self::layout()) };

        Self { ptr: ptr as _ }
    }

    pub fn load(self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().display().to_string();
        let c_path = CString::new(path).unwrap();
        let status = unsafe { ffi::elf_read_firmware(c_path.as_ptr(), self.ptr) };

        if status != 0 {
            panic!(
                "elf_read_firmware() failed (status={}, path={})",
                status,
                c_path.into_string().unwrap()
            );
        }

        self
    }

    // TODO in theory this should be called only after `.load()`
    pub fn flash_to(self, avr: &mut Avr) {
        unsafe {
            ffi::avr_load_firmware(avr.ptr(), self.ptr);
        }
    }

    fn layout() -> alloc::Layout {
        alloc::Layout::new::<ffi::elf_firmware_t>()
    }
}

impl Drop for ElfFirmware {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, Self::layout());
        }
    }
}
