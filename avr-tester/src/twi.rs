use crate::*;

/// Provides access to the TWI (aka I2C).
///
/// See: [`Twi::attach_slave()`] and [`Twi::detach_slave()`].
#[doc(alias = "i2c")]
pub struct Twi<'a> {
    mgr: &'a RefCell<TwiManager>,
}

impl<'a> Twi<'a> {
    pub(crate) fn new(tester: &'a mut AvrTester, id: u8) -> Self {
        let mgr = tester.twis.entry(id).or_insert_with(|| {
            let mgr = Rc::new(RefCell::new(TwiManager::default()));

            tester.sim.as_mut().unwrap().set_twi_slave(id, {
                let mgr = mgr.clone();

                move |packet: TwiPacket| -> Option<TwiPacket> {
                    for slave in mgr.borrow_mut().slaves.values_mut() {
                        if let Some(packet) = slave.recv(packet) {
                            return Some(packet);
                        }
                    }

                    None
                }
            });

            mgr
        });

        Self { mgr }
    }

    /// Attaches given slave into this TWI, executing it for each packet
    /// received on this interface - slave can then decide whether to respond or
    /// ignore the packet.
    ///
    /// When multiple slaves are attached, they all get executed in the order of
    /// the attachment until any slave responds (if any).
    ///
    /// See the `twi.rs` example for usage.
    ///
    /// See also: [`Self::attach_slave_fn()`].
    pub fn attach_slave(
        &mut self,
        slave: impl TwiSlave + 'static,
    ) -> TwiSlaveId {
        let mut mgr = self.mgr.borrow_mut();
        let id = TwiSlaveId(mgr.next_slave_id);

        mgr.slaves.insert(id, Box::new(slave));

        mgr.next_slave_id = mgr
            .next_slave_id
            .checked_add(1)
            .expect("Too many TWI slaves got attached, ran out of indices");

        id
    }

    /// Shortcut for [`Self::attach_slave()`] that allows to create a device
    /// with just a function:
    ///
    /// ```no_run
    /// # use avr_tester::*;
    /// #
    /// let mut avr = AvrTester::test();
    ///
    /// avr.twi0().attach_slave_fn(|packet| {
    ///     if packet.addr != 0x33 {
    ///         return None;
    ///     }
    ///
    ///     if packet.is_start() || packet.is_stop() {
    ///         return Some(packet.respond_ack());
    ///     }
    ///
    ///     if packet.is_write() {
    ///         todo!();
    ///     }
    ///
    ///     if packet.is_read() {
    ///         todo!();
    ///     }
    ///
    ///     None
    /// });
    /// ```
    ///
    /// See [`Self::attach_slave()`] for details.
    pub fn attach_slave_fn(
        &mut self,
        slave: impl FnMut(TwiPacket) -> Option<TwiPacket> + 'static,
    ) -> TwiSlaveId {
        self.attach_slave(slave)
    }

    /// Detaches given slave from this TWI, preventing it from being executed
    /// again.
    ///
    /// See: [`Self::attach_slave()`].
    pub fn detach_slave(&mut self, id: TwiSlaveId) {
        self.mgr.borrow_mut().slaves.remove(&id);
    }
}

#[derive(Default)]
pub(crate) struct TwiManager {
    slaves: BTreeMap<TwiSlaveId, Box<dyn TwiSlave>>,
    next_slave_id: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TwiSlaveId(u32);
