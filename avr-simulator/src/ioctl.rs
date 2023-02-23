#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoCtl {
    AdcGetIrq,
    IoPortGetIrq { port: char },
    IoPortGetState { port: char },
    SpiGetIrq { spi: u8 },
    UartGetFlags { uart: char },
    UartGetIrq { uart: char },
    UartSetFlags { uart: char },
}

impl IoCtl {
    pub fn into_ffi(self) -> u32 {
        let ctl = match self {
            IoCtl::AdcGetIrq => [b'a', b'd', b'c', b'0'],
            IoCtl::IoPortGetIrq { port } => [b'i', b'o', b'g', port as u8],
            IoCtl::IoPortGetState { port } => [b'i', b'o', b's', port as u8],
            IoCtl::SpiGetIrq { spi } => [b's', b'p', b'i', spi],
            IoCtl::UartGetFlags { uart } => [b'u', b'a', b'g', uart as u8],
            IoCtl::UartGetIrq { uart } => [b'u', b'a', b'r', uart as u8],
            IoCtl::UartSetFlags { uart } => [b'u', b'a', b's', uart as u8],
        };

        u32::from_be_bytes(ctl)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod into_ffi {
        use super::*;
        use test_case::test_case;

        struct TestCase {
            given: IoCtl,
            expected: [u8; 4],
        }

        const TEST_ADC_GET_IRQ: TestCase = TestCase {
            given: IoCtl::AdcGetIrq,
            expected: [b'a', b'd', b'c', b'0'],
        };

        const TEST_IOPORT_GET_STATE: TestCase = TestCase {
            given: IoCtl::IoPortGetState { port: 'C' },
            expected: [b'i', b'o', b's', b'C'],
        };

        const TEST_IOPORT_GET_IRQ: TestCase = TestCase {
            given: IoCtl::IoPortGetIrq { port: 'C' },
            expected: [b'i', b'o', b'g', b'C'],
        };

        const TEST_SPI_GET_IRQ: TestCase = TestCase {
            given: IoCtl::SpiGetIrq { spi: 3 },
            expected: [b's', b'p', b'i', 3],
        };

        const TEST_UART_SET_FLAGS: TestCase = TestCase {
            given: IoCtl::UartSetFlags { uart: '3' },
            expected: [b'u', b'a', b's', b'3'],
        };

        const TEST_UART_GET_FLAGS: TestCase = TestCase {
            given: IoCtl::UartGetFlags { uart: '3' },
            expected: [b'u', b'a', b'g', b'3'],
        };

        const TEST_UART_GET_IRQ: TestCase = TestCase {
            given: IoCtl::UartGetIrq { uart: '3' },
            expected: [b'u', b'a', b'r', b'3'],
        };

        #[test_case(TEST_ADC_GET_IRQ)]
        #[test_case(TEST_IOPORT_GET_STATE)]
        #[test_case(TEST_IOPORT_GET_IRQ)]
        #[test_case(TEST_SPI_GET_IRQ)]
        #[test_case(TEST_UART_SET_FLAGS)]
        #[test_case(TEST_UART_GET_FLAGS)]
        #[test_case(TEST_UART_GET_IRQ)]
        fn test(case: TestCase) {
            let actual = case.given.into_ffi().to_be_bytes();

            assert_eq!(case.expected, actual);
        }
    }
}
