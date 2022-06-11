# avr-tester

Functional testing framework for [AVR] binaries, powered by [simavr].

tl;dr get your microcontroller's firmware black-box-tested in seconds!

Status: alpha; work in progress.

[AVR]: https://en.wikipedia.org/wiki/AVR_microcontrollers
[simavr]: https://github.com/buserror/simavr

## Getting Started

First, create a separate crate, dedicated only to your project's tests:

```shell
$ cargo new yourproject-tests --lib
```

... then add `avr-tester` as its dependency:

```toml
# yourproject-tests/Cargo.toml

[dependencies]
avr-tester = { git = "https://github.com/Patryk27/avr-tester" }
```

... and, just like that, you can start writing tests:

```rust
// yourproject-tests/src/lib.rs

use avr_tester::AvrTester;

fn avr() -> AvrTester {
    AvrTester::atmega328p()
        .with_clock_of_16_mhz()
        .load("../../yourproject/target/atmega328p/release/your-project.elf")
}

// Assuming `your-project` is a ROT-13 encoder, one could write tests such as
// those:

#[test]
fn short_text() {
    let mut avr = avr(); 

    avr.run_for_ms(1);
    avr.uart0().send_string("Hello, World!");
    avr.run_for_ms(1);
 
    assert_eq!("Uryyb, Jbeyq!", avr.uart0().recv_string());
}

#[test]
fn long_text() {
    let mut avr = avr(); 

    avr.run_for_ms(1);
    avr.uart0().send_string("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    avr.run_for_ms(1);

    assert_eq!(
        "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg",
        avr.uart0().recv_string(),
    );
}
```

... having the tests ready (and [requirements](#requirements) met!), just run
`cargo test` inside `yourproject-tests` :-)

Note that because AvrTester _simulates an actual AVR_, you don't have to modify
`yourproject` at all - it's free to use timers, GPIOs etc. and everything should
just work.

In fact, `yourproject` doesn't even have to be written in Rust - you can create
Rust-based tests for a firmware written in C, Zig or anything else!

## Usage

- [Testing analog pins](avr-tester/tests/tests/pins-analog.rs),
- [Testing digital pins](avr-tester/tests/tests/pins-digital.rs),
- [Testing UARTs](avr-tester/tests/tests/uart.rs).

## Requirements & supported platforms

See: [simavr-ffi](https://github.com/Patryk27/simavr-ffi).

## Roadmap

Following features seem to be supported by simavr, but haven't been yet exposed
in AvrTester:

- interrupts,
- EEPROM,
- SPI,
- I2C,
- watchdog,
- TWI,
- <https://lib.rs/crates/simavr-section>,
- USB.

(i.e. your firmware can use those features, but you won't be able to test them.)

## Caveats

- triggering AVR's sleep mode will cause the Rust code to gracefully `panic!()`,
  because the only way to wake an AVR is to trigger an interrupt and those are
  not yet supported.

## Contributing

Pull requests are very much welcome!

### Tests

AvrTester's integration tests lay in `avr-tester/tests` - you can run them with:

```shell
$ cd avr-tester
$ cargo test
```

Note that for those tests to work, you might need some additional
dependencies:

#### ... on Nix

```shell
$ nix-shell
# and then `cargo test`
```

#### ... on Ubuntu

```shell
$ sudo apt install avr-libc gcc-avr
# and then `cargo test`
```

## License

Copyright (c) 2022, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
