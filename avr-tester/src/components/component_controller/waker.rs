use std::ptr;
use std::task::{RawWaker, RawWakerVTable, Waker};

const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(waker_clone, noop, noop, noop);

pub fn waker() -> Waker {
    let raw = RawWaker::new(ptr::null(), &NOOP_WAKER_VTABLE);

    unsafe { Waker::from_raw(raw) }
}

unsafe fn waker_clone(_: *const ()) -> RawWaker {
    RawWaker::new(ptr::null(), &NOOP_WAKER_VTABLE)
}

unsafe fn noop(_: *const ()) {}
