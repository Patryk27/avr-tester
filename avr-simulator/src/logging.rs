use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Overwrites simavr's default logger so that it doesn't print stuff to stdout
/// and stderr.
pub fn init() {
    let just_initialized =
        INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);

    if just_initialized.is_ok() {
        // Safety: Callback has correct signature (as proven by bindgen) and,
        // thanks to the `.compare_exchange()` above, we avoid data race with
        // other threads potentially also trying to initialize the logger
        unsafe {
            ffi::avr_global_logger_set(Some(on_message_logged));
        }
    }
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn on_message_logged(_: *mut ffi::avr_t, _: i32, _: *const i8, _: *mut i8) {
    //
}

#[cfg(not(target_os = "macos"))]
unsafe extern "C" fn on_message_logged(
    _: *mut ffi::avr_t,
    _: i32,
    _: *const i8,
    _: *mut ffi::__va_list_tag,
) {
    //
}
