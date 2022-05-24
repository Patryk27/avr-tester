# avr-tester

simavr meets `#[test]`

AvrTester provides a comfortable wrapper over [simavr](https://github.com/buserror/simavr)
allowing you to easily test your AVR code _end-to-end_ by simulating the
behavior of UARTs, GPIOs etc. -- test your code in seconds, not hours!

Status: work in progress, pretty alpha, not yet released.

## Usage

At the moment only the UART interface for Atmega328p is exposed, so:

```rust
// Somewhere in `tests/smoke.rs`:

#[test]
fn smoke() {
    let mut avr = AvrTester::atmega_328p(
        "./target/atmega328p/release/your-firmware.elf",
        16_000_000,
    );

    avr.run_for_ms(1);
    avr.uart0().send_string("Hello!");
    avr.run_for_ms(5);
    
    // Assuming `your-firmware.elf` implements a simple ROT13 UART encoder:
    assert_eq!("Uryyb!", avr.uart0().recv_string());
}
```

Note that this crate doesn't provide any way to get `your-firmware.elf` yet - 
it's assumed that your test will e.g. do `Command::new("cargo").arg("build")` or
something similar on its own.

Hopefully before the release I'll be able to come up with some neat wrapper!

## Testing

```shell
$ cd avr-tester
$ cargo test
```

## Requirements

AvrTester depends on [simavr-ffi](https://github.com/Patryk27/simavr-ffi), so:

- clang (with `LIBCLANG_PATH` set),
- libelf,
- pkg-config.

## License

Copyright (c) 2022, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
