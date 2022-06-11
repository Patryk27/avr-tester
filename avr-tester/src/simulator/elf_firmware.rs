use super::*;
use std::{alloc, ffi::CString, path::Path, ptr::NonNull};

/// Convenient wrapper over [`ffi::elf_firmware_t`].
pub struct ElfFirmware {
    ptr: NonNull<ffi::elf_firmware_t>,
}

impl ElfFirmware {
    pub fn new() -> Self {
        // Safety: We know that `elf_firmware_t`'s layout has a non-zero size.
        //
        // (we also use `alloc_zeroed`, because that's what simavr's docs
        // suggest to do.)
        let ptr = unsafe { alloc::alloc_zeroed(Self::layout()) as *mut ffi::elf_firmware_t };

        // Unwrap-safety: This can fail only if the underlying allocator failed
        //                to find enough memory to allocate the chunk. In that
        //                case, panicking is the best we can afford anyway.
        let ptr = NonNull::new(ptr).unwrap();

        Self { ptr }
    }

    pub fn load(self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().display().to_string();

        // Unwrap-safety: Paths cannot contain null-terminators, so a string
        //                we've got from `.display().to_string()` cannot either
        let c_path = CString::new(path).unwrap();

        // Safety: `self.ptr` points at a valid, zeroed instance of
        //         `elf_firmware_t`; `c_path` points at a valid `CString`
        let status = unsafe { ffi::elf_read_firmware(c_path.as_ptr(), self.ptr.as_ptr()) };

        if status != 0 {
            panic!(
                "Couldn't load firmware from: {} (status = {})",
                c_path.into_string().unwrap(),
                status
            );
        }

        self
    }

    pub fn flash_on(self, avr: &mut Avr) {
        avr.load_firmware(self.ptr);
    }

    fn layout() -> alloc::Layout {
        alloc::Layout::new::<ffi::elf_firmware_t>()
    }
}

impl Drop for ElfFirmware {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, Self::layout());
        }
    }
}
