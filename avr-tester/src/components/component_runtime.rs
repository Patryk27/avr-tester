use super::*;
use std::cell::RefCell;

pub struct ComponentRuntime {
    sim: AvrSimulator,
    clock_frequency: u32,
    tt: AvrDuration,
}

impl ComponentRuntime {
    pub fn setup(sim: AvrSimulator, clock_frequency: u32, tt: AvrDuration) {
        RUNTIME.with(move |rt| {
            *rt.borrow_mut() = Some(Self {
                sim,
                clock_frequency,
                tt,
            });
        })
    }

    pub fn destroy() -> AvrSimulator {
        RUNTIME.with(|rt| {
            rt.borrow_mut()
                .take()
                .expect("destroy() called outside of AvrTester's runtime")
                .sim
        })
    }

    pub fn with<T>(f: impl FnOnce(&mut Self) -> T) -> T {
        RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();

            let rt = rt
                .as_mut()
                .expect("with() called outside of AvrTester's runtime");

            f(rt)
        })
    }

    pub fn sim(&mut self) -> &mut AvrSimulator {
        &mut self.sim
    }

    pub fn clock_frequency(&self) -> u32 {
        self.clock_frequency
    }

    pub fn tt(&self) -> AvrDuration {
        self.tt
    }
}

thread_local! {
    static RUNTIME: RefCell<Option<ComponentRuntime>> = RefCell::new(None);
}
