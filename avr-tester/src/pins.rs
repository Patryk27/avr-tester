mod analog_pin;
mod digital_pin;

use crate::*;
use simavr_ffi as ffi;

pub use self::{analog_pin::*, digital_pin::*};

pub struct Pins<'a> {
    avr: &'a mut AvrTester,
}

impl<'a> Pins<'a> {
    pub(crate) fn new(avr: &'a mut AvrTester) -> Self {
        Self { avr }
    }
}

macro_rules! analog_pins {
    ( $( $fn:ident($irq:expr) ),* $(,)? ) => {
        impl<'a> Pins<'a> {
            $(
                pub fn $fn(&mut self) -> AnalogPin<'_> {
                    AnalogPin::new(self.avr, $irq)
                }
            )*
        }
    }
}

analog_pins! {
    adc0(ffi::ADC_IRQ_ADC0),
    adc1(ffi::ADC_IRQ_ADC1),
    adc2(ffi::ADC_IRQ_ADC2),
    adc3(ffi::ADC_IRQ_ADC3),
    adc4(ffi::ADC_IRQ_ADC4),
    adc5(ffi::ADC_IRQ_ADC5),
    adc6(ffi::ADC_IRQ_ADC6),
    adc7(ffi::ADC_IRQ_ADC7),
    adc8(ffi::ADC_IRQ_ADC8),
    adc9(ffi::ADC_IRQ_ADC9),
    adc10(ffi::ADC_IRQ_ADC10),
    adc11(ffi::ADC_IRQ_ADC11),
    adc12(ffi::ADC_IRQ_ADC12),
    adc13(ffi::ADC_IRQ_ADC13),
    adc14(ffi::ADC_IRQ_ADC14),
    adc15(ffi::ADC_IRQ_ADC15),
    temp(ffi::ADC_IRQ_TEMP),
}

macro_rules! digital_pins {
    ( $( $fn:ident($port:expr, $pin:expr) ),* $(,)? ) => {
        impl<'a> Pins<'a> {
            $(
                pub fn $fn(&mut self) -> DigitalPin<'_> {
                    DigitalPin::new(self.avr, $port, $pin)
                }
            )*
        }
    }
}

digital_pins! {
    pa0('A', 0),
    pa1('A', 1),
    pa2('A', 2),
    pa3('A', 3),
    pa4('A', 4),
    pa5('A', 5),
    pa6('A', 6),
    pa7('A', 7),
    //
    pb0('B', 0),
    pb1('B', 1),
    pb2('B', 2),
    pb3('B', 3),
    pb4('B', 4),
    pb5('B', 5),
    pb6('B', 6),
    pb7('B', 7),
    //
    pc0('C', 0),
    pc1('C', 1),
    pc2('C', 2),
    pc3('C', 3),
    pc4('C', 4),
    pc5('C', 5),
    pc6('C', 6),
    pc7('C', 7),
    //
    pd0('D', 0),
    pd1('D', 1),
    pd2('D', 2),
    pd3('D', 3),
    pd4('D', 4),
    pd5('D', 5),
    pd6('D', 6),
    pd7('D', 7),
    //
    pe0('E', 0),
    pe1('E', 1),
    pe2('E', 2),
    pe3('E', 3),
    pe4('E', 4),
    pe5('E', 5),
    pe6('E', 6),
    pe7('E', 7),
    //
    pf0('F', 0),
    pf1('F', 1),
    pf2('F', 2),
    pf3('F', 3),
    pf4('F', 4),
    pf5('F', 5),
    pf6('F', 6),
    pf7('F', 7),
    //
    pg0('G', 0),
    pg1('G', 1),
    pg2('G', 2),
    pg3('G', 3),
    pg4('G', 4),
    pg5('G', 5),
    pg6('G', 6),
    pg7('G', 7),
    //
    ph0('H', 0),
    ph1('H', 1),
    ph2('H', 2),
    ph3('H', 3),
    ph4('H', 4),
    ph5('H', 5),
    ph6('H', 6),
    ph7('H', 7),
}
