use super::*;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Twi {
    state: NonNull<TwiState>,
}

impl Twi {
    /// Initializes the subsystem; returns `None` if current AVR doesn't have
    /// this TWI.
    ///
    /// # Safety
    ///
    /// - Because this function registers an IRQ notification, the object
    ///   returned from here must be kept alive for at least as long as `avr`.
    pub unsafe fn new(id: u8, avr: &Avr) -> Option<Self> {
        let ioctl = IoCtl::TwiGetIrq { id };
        let irq_input = avr.try_io_getirq(ioctl, ffi::TWI_IRQ_INPUT)?;
        let irq_output = avr.try_io_getirq(ioctl, ffi::TWI_IRQ_OUTPUT)?;

        let this = Self {
            state: NonNull::from(Box::leak(Box::new(TwiState {
                slave: None,
                irq_input,
            }))),
        };

        unsafe {
            Avr::irq_register_notify(
                irq_output,
                Some(Self::on_output),
                this.state.as_ptr(),
            );
        }

        Some(this)
    }

    pub fn set_slave(&mut self, slave: impl TwiSlave + 'static) {
        self.state_mut().slave = Some(Box::new(slave));
    }

    fn state_mut(&mut self) -> &mut TwiState {
        // Safety: `state` points to a valid object; nothing else is writing
        // there at the moment, as guarded by `&mut self` here and on
        // `Avr::run()`
        unsafe { self.state.as_mut() }
    }

    unsafe extern "C" fn on_output(
        _: NonNull<ffi::avr_irq_t>,
        value: u32,
        mut state: NonNull<TwiState>,
    ) {
        unsafe {
            let state = state.as_mut();

            if let Some(slave) = &mut state.slave {
                if let Some(packet) = slave.recv(TwiPacket::decode(value)) {
                    ffi::avr_raise_irq(
                        state.irq_input.as_ptr(),
                        packet.encode(),
                    );
                }
            }
        }
    }
}

impl Drop for Twi {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.state.as_ptr()));
        }
    }
}

struct TwiState {
    slave: Option<Box<dyn TwiSlave>>,
    irq_input: NonNull<ffi::avr_irq_t>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TwiPacket {
    pub msg: u8,
    pub addr: u8,
    pub data: u8,
}

impl TwiPacket {
    fn decode(packet: u32) -> Self {
        let [_, msg, addr, data] = packet.to_le_bytes();

        Self { msg, addr, data }
    }

    fn encode(self) -> u32 {
        u32::from_le_bytes([0, self.msg, self.addr, self.data])
    }

    pub const MSG_START: u8 = ffi::TWI_COND_START as u8;
    pub const MSG_STOP: u8 = ffi::TWI_COND_STOP as u8;
    pub const MSG_ADDR: u8 = ffi::TWI_COND_ADDR as u8;
    pub const MSG_ACK: u8 = ffi::TWI_COND_ACK as u8;
    pub const MSG_WRITE: u8 = ffi::TWI_COND_WRITE as u8;
    pub const MSG_READ: u8 = ffi::TWI_COND_READ as u8;

    pub fn is_start(&self) -> bool {
        self.msg & Self::MSG_START > 0
    }

    pub fn is_stop(&self) -> bool {
        self.msg & Self::MSG_STOP > 0
    }

    pub fn is_addr(&self) -> bool {
        self.msg & Self::MSG_ADDR > 0
    }

    pub fn is_ack(&self) -> bool {
        self.msg & Self::MSG_ACK > 0
    }

    pub fn is_write(&self) -> bool {
        self.msg & Self::MSG_WRITE > 0
    }

    pub fn is_read(&self) -> bool {
        self.msg & Self::MSG_READ > 0
    }

    pub fn respond_ack(&self) -> Self {
        Self {
            msg: Self::MSG_ACK,
            addr: self.addr,
            data: 1,
        }
    }

    pub fn respond_data(&self, data: u8) -> Self {
        Self {
            msg: Self::MSG_READ,
            addr: self.addr,
            data,
        }
    }
}

pub trait TwiSlave {
    fn recv(&mut self, packet: TwiPacket) -> Option<TwiPacket>;
}

impl<T> TwiSlave for T
where
    T: FnMut(TwiPacket) -> Option<TwiPacket>,
{
    fn recv(&mut self, packet: TwiPacket) -> Option<TwiPacket> {
        (self)(packet)
    }
}
