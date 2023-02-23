mod component_controller;
mod component_handle;
mod component_runtime;
mod component_state;
mod futures;

use self::component_controller::*;
use crate::*;
use std::future::Future;

pub use self::{component_handle::*, component_state::*};
pub(crate) use self::{component_runtime::*, futures::*};

/// Manages components.
///
/// # Abstract
///
/// Components are _peripherals_ attached to the AVR - they allow to easily
/// simulate external devices, such as shift registers or screens, without
/// forcing you to think about those devices' timings with respect to other
/// attached peripherals.
///
/// For instance, let's say that we've got a firmware that provides some UART
/// functionality, but at the same time it requires for `PB1` and `PB2` to be
/// toggled in regular intervals (say, because they are attached to watchdog).
///
/// Assuming `PB1` has to be toggled each 5 ms and `PB2` each 15 ms, we could
/// write a test such as this:
///
/// ```no_run
/// # use avr_tester::*;
/// # fn avr() -> AvrTester { panic!() }
/// #
/// let mut avr = avr();
///
/// avr.uart0().write([0x01, 0x02, 0x03]);
///
/// for cycle in 0.. {
///     // Keep the watchdog happy:
///     if cycle % 5 == 0 {
///         avr.pins().pb1().toggle();
///     }
///
///     if cycle % 15 == 0 {
///         avr.pins().pb2().toggle();
///     }
///
///     // Check if the response has arrived:
///     if let Some(response) = avr.uart0().try_read_byte() {
///         assert_eq!(0x06, response);
///         break;
///     }
///
///     avr.run_for_ms(1);
/// }
/// ```
///
/// ... but that approach not only scales poorly (imagine having to handle
/// multiple devices, each with its own clock!), but also obfuscates the test -
/// if we're mostly interested in the UART part, then there shouldn't be any
/// reason to intertwine it with the pin-toggling.
///
/// Here come components - they are like background tasks that are polled after
/// each AVR's instruction:
///
/// ```no_run
/// # use avr_tester::*;
/// # fn avr() -> AvrTester { panic!() }
/// #
/// let mut avr = avr();
///
/// // Start the `PB1` toggler:
/// avr.components().add(async {
///     loop {
///         avr_rt().pins().pb1().toggle();
///         avr_rt().run_for_ms(5).await;
///     }
/// });
///
/// // Start the `PB2` toggler:
/// avr.components().add(async {
///     loop {
///         avr_rt().pins().pb2().toggle();
///         avr_rt().run_for_ms(15).await;
///     }
/// });
///
/// // Perform the test:
/// avr.uart0().write([0x01, 0x02, 0x03]);
/// avr.run_for_ms(100);
/// assert_eq!(Some(0x06), avr.uart0().try_read_byte());
/// ```
///
/// Components are handy, because AvrTester automatically takes care of
/// their scheduling - we don't have to worry about `PB1` and `PB2`'s timings
/// anymore: we just say "PB1 must be toggled every 5 ms", "PB2 must be toggled
/// every 15 ms" and that's it.
///
/// From AvrTester's perspective, what happens here is basically:
///
/// ```text
/// fn run_for_ms(ms):
///     /* run() in a loop */
///
/// fn run():
///     simavr.run_one_instruction()
///
///     for component in components:
///         component.poll(simavr)
/// ```
///
/// # Writing components
///
/// Writing components doesn't differ that much from writing regular tests - the
/// most important caveat is that components must be asynchronous, so that
/// AvrTester knows when a component has finished its "clock cycle".
///
/// This means that inside components you can't access regular [`AvrTester`] -
/// you have to call [`avr_rt()`], which returns [`AvrTesterAsync`] with its
/// own set of functions that operate on pins.
///
/// Similarly, instead of calling `thread::sleep()` you should invoke
/// `avr_rt().run_for_ms(...)`.
///
/// # Examples
///
/// ## `PB2 = !PB1`
///
/// This component implements a simple `PB2 = !PB1` real-time gate:
///
/// ```no_run
/// # use avr_tester::*;
/// # fn avr() -> AvrTester { panic!() }
/// # let mut avr = avr();
/// #
/// avr.components().add(async {
///     loop {
///         let is_high = avr_rt().pins().pb1().is_high();
///
///         avr_rt().pins().pb2().set(!is_high);
///         avr_rt().run().await;
///     }
/// });
/// ```
#[derive(Debug)]
pub struct Components {
    components: Vec<ComponentController>,
}

impl Components {
    pub(crate) fn new() -> Self {
        Self {
            components: Default::default(),
        }
    }

    /// Creates a new component and attaches it into the AVR.
    ///
    /// See [`Components`] for more details.
    pub fn add(&mut self, component: impl Future<Output = ()> + 'static) -> ComponentHandle {
        let (controller, handle) = ComponentController::new(component);

        self.components.push(controller);

        handle
    }

    pub(crate) fn run(
        &mut self,
        sim: &mut Option<AvrSimulator>,
        clock_frequency: u32,
        tt: AvrDuration,
    ) {
        if self.components.is_empty() {
            return;
        }

        ComponentRuntime::setup(sim.take().unwrap(), clock_frequency, tt);

        // ---

        let mut components_to_remove = Vec::new();

        for (component_idx, component) in self.components.iter_mut().enumerate() {
            match component.run() {
                ComponentControllerResult::KeepComponent => {
                    //
                }
                ComponentControllerResult::RemoveComponent => {
                    components_to_remove.push(component_idx);
                }
            }
        }

        for (removed_components, component_idx) in components_to_remove.into_iter().enumerate() {
            self.components.remove(component_idx - removed_components);
        }

        // ---

        *sim = Some(ComponentRuntime::destroy());
    }
}
