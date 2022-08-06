mod waker;

use self::waker::*;
use super::*;
use std::{cell::RefCell, pin::Pin, rc::Rc, task::Context};

pub struct ComponentController {
    component: Pin<Box<dyn Future<Output = ()>>>,
    state: Rc<RefCell<ComponentState>>,
}

impl ComponentController {
    pub fn new(component: impl Future<Output = ()> + 'static) -> (Self, ComponentHandle) {
        let state = Rc::new(RefCell::new(ComponentState::Working));

        let controller = Self {
            component: Box::pin(component),
            state: state.clone(),
        };

        let handle = ComponentHandle::new(state);

        (controller, handle)
    }

    pub fn run(&mut self) -> ComponentControllerResult {
        match *self.state.borrow() {
            ComponentState::Working => {
                //
            }
            ComponentState::Paused => {
                return ComponentControllerResult::KeepComponent;
            }
            ComponentState::Removed => {
                return ComponentControllerResult::RemoveComponent;
            }
        }

        let waker = waker();
        let mut cx = Context::from_waker(&waker);

        let poll = self.component.as_mut().poll(&mut cx);

        if poll.is_ready() {
            ComponentControllerResult::RemoveComponent
        } else {
            ComponentControllerResult::KeepComponent
        }
    }
}

pub enum ComponentControllerResult {
    KeepComponent,
    RemoveComponent,
}
