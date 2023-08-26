use std::sync::Arc;
use std::task::{Wake, Waker};

pub fn waker() -> Waker {
    Arc::new(NoopWaker).into()
}

struct NoopWaker;

impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {
        //
    }
}
