//! See: [../../avr-tester/tests/acceptance/fractal.rs].

#![no_std]
#![no_main]

#[cfg(feature = "custom-compiler-builtins")]
extern crate custom_compiler_builtins;

use atmega_hal::clock::MHz16;
use atmega_hal::usart::{BaudrateExt, Usart0};
use atmega_hal::{pins, Peripherals};
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut uart = Usart0::<MHz16>::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        115200u32.into_baudrate(),
    );

    mandelbrot(&mut uart, 40, 20, -2.05, -1.12, 0.47, 1.12, 50);

    loop {
        //
    }
}

fn mandelbrot(
    uart: &mut Usart0<MHz16>,
    width: i64,
    height: i64,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    max_iters: i64,
) {
    for viewport_y in 0..height {
        let y0 = y1 + (y2 - y1) * ((viewport_y as f32) / (height as f32));

        for viewport_x in 0..width {
            let x0 = x1 + (x2 - x1) * ((viewport_x as f32) / (width as f32));

            let mut x = 0.0;
            let mut y = 0.0;
            let mut iters = max_iters;

            while x * x + y * y <= 4.0 && iters > 0 {
                let xtemp = x * x - y * y + x0;

                y = 2.0 * x * y + y0;
                x = xtemp;
                iters -= 1;
            }

            let ch = (8.0 * ((iters as f32) / (max_iters as f32))) as usize;
            let ch = b"#%=-:,. "[ch];

            uart.write_byte(ch);
        }

        uart.write_byte(b'\n');
    }
}
