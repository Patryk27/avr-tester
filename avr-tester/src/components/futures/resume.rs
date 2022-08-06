use super::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct ResumeFuture {
    yielded: bool,
}

impl ResumeFuture {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for ResumeFuture {
    type Output = CpuDuration;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.yielded {
            ComponentRuntime::with(|rt| Poll::Ready(rt.tt()))
        } else {
            this.yielded = true;
            Poll::Pending
        }
    }
}
