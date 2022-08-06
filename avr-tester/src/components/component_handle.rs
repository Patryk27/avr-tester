use super::*;
use std::{cell::RefCell, rc::Rc};

pub struct ComponentHandle {
    state: Rc<RefCell<ComponentState>>,
}

impl ComponentHandle {
    pub(super) fn new(state: Rc<RefCell<ComponentState>>) -> Self {
        Self { state }
    }

    /// Pauses component until [`Self::resume()`] is called.
    pub fn pause(&self) {
        *self.state.borrow_mut() = ComponentState::Paused;
    }

    /// Resumes component paused through [`Self::pause()`].
    pub fn resume(&self) {
        *self.state.borrow_mut() = ComponentState::Working;
    }

    /// Removes component, preventing it from running again.
    pub fn remove(self) {
        *self.state.borrow_mut() = ComponentState::Removed;
    }

    /// Returns component's state.
    pub fn state(&self) -> ComponentState {
        *self.state.borrow()
    }
}
