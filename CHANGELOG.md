# 0.2.0

- Added support for SPI.
- Renamed `Uart::send()` to `Uart::write()` and `Uart::recv()` to `Uart::read()`.
- Renamed `UartSend` to `Writable` and `UartRecv` to `Readable`.
- Refactored simulator-like types into a separate crate.
