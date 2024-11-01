#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoCtl {
    AdcGetIrq,
    IoPortGetIrq { port: char },
    IoPortGetState { port: char },
    SpiGetIrq { id: u8 },
    TwiGetIrq { id: u8 },
    UartGetFlags { id: char },
    UartGetIrq { id: char },
    UartSetFlags { id: char },
}

impl IoCtl {
    pub fn into_ffi(self) -> u32 {
        u32::from_be_bytes(match self {
            IoCtl::AdcGetIrq => [b'a', b'd', b'c', b'0'],
            IoCtl::IoPortGetIrq { port } => [b'i', b'o', b'g', port as u8],
            IoCtl::IoPortGetState { port } => [b'i', b'o', b's', port as u8],
            IoCtl::SpiGetIrq { id } => [b's', b'p', b'i', id],
            IoCtl::TwiGetIrq { id } => [b't', b'w', b'i', id],
            IoCtl::UartGetFlags { id } => [b'u', b'a', b'g', id as u8],
            IoCtl::UartGetIrq { id } => [b'u', b'a', b'r', id as u8],
            IoCtl::UartSetFlags { id } => [b'u', b'a', b's', id as u8],
        })
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

        const TEST_IOPORT_GET_IRQ: TestCase = TestCase {
            given: IoCtl::IoPortGetIrq { port: 'C' },
            expected: [b'i', b'o', b'g', b'C'],
        };

        const TEST_IOPORT_GET_STATE: TestCase = TestCase {
            given: IoCtl::IoPortGetState { port: 'C' },
            expected: [b'i', b'o', b's', b'C'],
        };

        const TEST_SPI_GET_IRQ: TestCase = TestCase {
            given: IoCtl::SpiGetIrq { id: 3 },
            expected: [b's', b'p', b'i', 3],
        };

        const TEST_TWI_GET_IRQ: TestCase = TestCase {
            given: IoCtl::TwiGetIrq { id: 3 },
            expected: [b't', b'w', b'i', 3],
        };
        const TEST_UART_GET_FLAGS: TestCase = TestCase {
            given: IoCtl::UartGetFlags { id: '3' },
            expected: [b'u', b'a', b'g', b'3'],
        };

        const TEST_UART_GET_IRQ: TestCase = TestCase {
            given: IoCtl::UartGetIrq { id: '3' },
            expected: [b'u', b'a', b'r', b'3'],
        };

        const TEST_UART_SET_FLAGS: TestCase = TestCase {
            given: IoCtl::UartSetFlags { id: '3' },
            expected: [b'u', b'a', b's', b'3'],
        };

        #[test_case(TEST_ADC_GET_IRQ)]
        #[test_case(TEST_IOPORT_GET_IRQ)]
        #[test_case(TEST_IOPORT_GET_STATE)]
        #[test_case(TEST_SPI_GET_IRQ)]
        #[test_case(TEST_TWI_GET_IRQ)]
        #[test_case(TEST_UART_GET_FLAGS)]
        #[test_case(TEST_UART_GET_IRQ)]
        #[test_case(TEST_UART_SET_FLAGS)]
        fn test(case: TestCase) {
            assert_eq!(case.expected, case.given.into_ffi().to_be_bytes());
        }
    }
}
