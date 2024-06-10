# 0.2.3

- Bumped [simavr-ffi](https://github.com/Patryk27/simavr-ffi/pull/4).

# 0.2.2

- Added support for asynchronous SPI and UART, for usage in components.

# 0.2.1

- Added `DigitalPin::wait_while_low_timeout()` and `DigitalPin::wait_while_high_timeout()` ([pull request](https://github.com/Patryk27/avr-tester/pull/4); thanks, nilclass!)
- Fixed a bug where sometimes SPI and UART wouldn't receive first transmitted bytes ([issue](https://github.com/Patryk27/avr-tester/issues/3)).

# 0.2.0

- Added support for SPI.
- Renamed `Uart::send()` to `Uart::write()` and `Uart::recv()` to `Uart::read()`.
- Renamed `UartSend` to `Writable` and `UartRecv` to `Readable`.
- Refactored simulator-like types into a separate crate.

