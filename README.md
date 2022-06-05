# avr-tester

AvrTester provides a comfortable wrapper over [simavr](https://github.com/buserror/simavr)
that allows to test AVR binaries in isolation by simulating the behavior of
UARTs, GPIOs etc. -- get your code tested in seconds!

Status: alpha; work in progress; not yet released.

## Usage

Assuming your AVR code has been already compiled somewhere, plugging AvrTester
is as easy as creating a new crate with:

```toml
[dependencies]
avr-tester = { git = "https://github.com/Patryk27/avr-tester" }
```

... and then:

```rust
use avr_tester::AvrTester;

fn main() {
    //
}

// Assuming `firmware-uart.elf` implements some sort of rot13 encoder:
#[test]
fn test_uart() {
    let mut avr = AvrTester::atmega328p("firmware-uart.elf", 16_000_000);

    avr.run_for_ms(1);
    avr.uart0().send_string("Hello!");
    avr.run_for_ms(5);
    
    assert_eq!("Uryyb!", avr.uart0().recv_string());
}

// Assuming `firmware-pins.elf` implements some sort of `pc2 = !pc1` logic:
#[test]
fn test_pins() {
    let mut avr = AvrTester::atmega328p("firmware-pins.elf", 16_000_000);

    avr.pins().pc1().set_high();
    avr.run_for_ms(1);
    avr.pins().pc2().assert_low();

    avr.pins().pc1().set_low();
    avr.run_for_ms(1);
    avr.pins().pc2().assert_high();
}
```

Note that this crate doesn't provide any way to get the `*.elf` file itself
yet - usually you'll find it somewhere inside the `target` directory after
running `cargo build --release` on your AVR crate.

This means that the AvrTester-tests have to be provided somewhat _next to_ your
AVR application, you can't easily have just a single crate.

Hopefully https://github.com/rust-lang/cargo/issues/9096 will be able to
simplify this.

## Requirements

AvrTester depends on [simavr-ffi](https://github.com/Patryk27/simavr-ffi), so:

- clang (with `LIBCLANG_PATH` set),
- libelf,
- pkg-config.

## Testing

```shell
$ cd avr-tester
$ cargo test
```

## License

Copyright (c) 2022, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
