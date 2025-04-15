//! This example shows how you can test TWIs (aka I2Cs).
//!
//! We're given an AVR that connects through TWI with a 64-byte RAM, simulated
//! within the Rust code below.
//!
//! See: [../../../avr-tester-fixtures/src/twi.rs].

use avr_tester::{AvrTester, Reader, TwiPacket, TwiSlave};

#[test]
fn test() {
    let mut avr = AvrTester::atmega164pa()
        .with_clock_of_16_mhz()
        .load("../avr-tester-fixtures/target/avr-none/release/twi.elf");

    avr.twi0().attach_slave(SimpleTwiRam::default());
    avr.run_for_ms(250);

    assert_eq!(avr.uart0().read_byte(), (0xca + 0xfe + 0xba + 0xbe) as u8);
}

#[derive(Default)]
struct SimpleTwiRam {
    addr: Option<usize>,
    cells: [u8; 32],
}

impl TwiSlave for SimpleTwiRam {
    fn recv(&mut self, packet: TwiPacket) -> Option<TwiPacket> {
        if packet.addr != 246 && packet.addr != 247 {
            return None;
        }

        if packet.is_start() || packet.is_stop() {
            return Some(packet.respond_ack());
        }

        if packet.is_write() {
            if let Some(addr) = self.addr.take() {
                self.cells[addr] = packet.data;
            } else {
                self.addr = Some(packet.data as usize);
            }

            return Some(packet.respond_ack());
        }

        if packet.is_read() {
            let addr = self.addr.take().expect("No RAM address selected");

            // For extra funkyness, reading from the address 255 actually
            // sums the entire contents of RAM and returns this number
            let resp = if addr == 255 {
                self.cells.iter().copied().sum()
            } else {
                self.cells[addr]
            };

            return Some(packet.respond_data(resp));
        }

        None
    }
}
