# avr-tester &emsp; [![crates-badge]][crates-link] [![docs-badge]][docs-link]

[crates-badge]: https://img.shields.io/crates/v/avr-tester.svg
[crates-link]: https://crates.io/crates/avr-tester
[docs-badge]: https://img.shields.io/docsrs/avr-tester
[docs-link]: https://docs.rs/avr-tester

Framework for testing [AVR] binaries, powered by [simavr].

tl;dr get your microcontroller's firmware black-box-tested in seconds!

[AVR]: https://en.wikipedia.org/wiki/AVR_microcontrollers
[simavr]: https://github.com/buserror/simavr

## Getting started

Create a crate dedicated to your firmware's tests:

```shell
$ cargo new firmware-tests --lib
```

... add `avr-tester` as its dependency:

```toml
# firmware-tests/Cargo.toml

[dependencies]
avr-tester = "0.5"
```

... and start writing tests:

```rust
// firmware-tests/src/lib.rs

use avr_tester::*;

fn avr() -> AvrTester {
    AvrTester::atmega328p()
        .with_clock_of_16_mhz()
        .load("../../firmware/target/avr-none/release/firmware.elf")
}

// Assuming `firmware` implements a ROT-13 encoder:

#[test]
fn short_text() {
    let mut avr = avr();

    // Let's give our firmware a moment to initialize:
    avr.run_for_ms(1);

    // Now, let's send the string:
    avr.uart0().write("Hello, World!");

    // ... give the AVR a moment to retrieve it & send back, encoded:
    avr.run_for_ms(1);

    // ... and, finally, let's assert the outcome:
    assert_eq!("Uryyb, Jbeyq!", avr.uart0().read::<String>());
}

#[test]
fn long_text() {
    let mut avr = avr();

    avr.run_for_ms(1);
    avr.uart0().write("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    avr.run_for_ms(10);

    assert_eq!(
        "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg",
        avr.uart0().read::<String>(),
    );
}
```

... having the tests ready, just run `cargo test` inside `firmware-tests`.

Since AvrTester emulates an actual AVR, you don't have to modify your firmware
at all - it can use timers, GPIOs etc. and everything should just work ™.

In fact, your project doesn't even have to be written in Rust - you can create
Rust tests for a firmware written in C, Zig and anything else!

## Features

- [Analog pins](avr-tester/tests/examples/analog_pins.rs),
- [Custom components](avr-tester/tests/examples/shift_register.rs),
- [Digital pins](avr-tester/tests/examples/digital_pins.rs),
- [SPIs](avr-tester/tests/examples/spi.rs),
- [TWIs](avr-tester/tests/examples/twi.rs) (aka I2C),
- [Timeouts](avr-tester/tests/examples/timeout.rs),
- [UARTs](avr-tester/tests/examples/uart.rs).

See more: <./avr-tester/tests/examples>.

## Supported platforms

See: [simavr-ffi](https://github.com/Patryk27/simavr-ffi).

## Roadmap

Following features are supported by simavr, but haven't been yet exposed in
AvrTester:

- Interrupts,
- EEPROM,
- Watchdog,
- USB.

Your firmware can use those features, you just won't be able to test them.

## Caveats

- Triggering AVR's sleep mode will cause the Rust code to panic, because the
  only way to wake an AVR is to trigger an interrupt and those are not yet
  supported.

## Contributing

Pull requests are very much welcome!

### Tests

Use `just test` to test AvrTester (so meta!) -- note that you might need some
additional dependencies:

#### ... using Nix (Linux / Mac)

```shell
$ nix develop
# and then `just test`
```

#### ... on Ubuntu

```shell
$ sudo apt install avr-libc gcc-avr
# and then `just test`
```

## License

Copyright (c) 2022 Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
