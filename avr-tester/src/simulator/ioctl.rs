#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoCtl {
    UartSetFlags { uart_id: u8 },
    UartGetFlags { uart_id: u8 },
    UartGetIrq { uart_id: u8 },
}

impl IoCtl {
    pub fn into_ffi(self) -> u32 {
        let ctl = match self {
            IoCtl::UartSetFlags { uart_id } => [b'u', b'a', b's', b'0' + uart_id],
            IoCtl::UartGetFlags { uart_id } => [b'u', b'a', b'g', b'0' + uart_id],
            IoCtl::UartGetIrq { uart_id } => [b'u', b'a', b'r', b'0' + uart_id],
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

        const TEST_UART_SET_FLAGS_0: TestCase = TestCase {
            given: IoCtl::UartSetFlags { uart_id: 0 },
            expected: [b'u', b'a', b's', b'0'],
        };

        const TEST_UART_SET_FLAGS_1: TestCase = TestCase {
            given: IoCtl::UartSetFlags { uart_id: 1 },
            expected: [b'u', b'a', b's', b'1'],
        };

        const TEST_UART_GET_FLAGS_0: TestCase = TestCase {
            given: IoCtl::UartGetFlags { uart_id: 0 },
            expected: [b'u', b'a', b'g', b'0'],
        };

        const TEST_UART_GET_FLAGS_1: TestCase = TestCase {
            given: IoCtl::UartGetFlags { uart_id: 1 },
            expected: [b'u', b'a', b'g', b'1'],
        };

        const TEST_UART_GET_IRQ_0: TestCase = TestCase {
            given: IoCtl::UartGetIrq { uart_id: 0 },
            expected: [b'u', b'a', b'r', b'0'],
        };

        const TEST_UART_GET_IRQ_1: TestCase = TestCase {
            given: IoCtl::UartGetIrq { uart_id: 1 },
            expected: [b'u', b'a', b'r', b'1'],
        };

        #[test_case(TEST_UART_SET_FLAGS_0)]
        #[test_case(TEST_UART_SET_FLAGS_1)]
        #[test_case(TEST_UART_GET_FLAGS_0)]
        #[test_case(TEST_UART_GET_FLAGS_1)]
        #[test_case(TEST_UART_GET_IRQ_0)]
        #[test_case(TEST_UART_GET_IRQ_1)]
        fn test(case: TestCase) {
            let actual = case.given.into_ffi().to_be_bytes();

            assert_eq!(case.expected, actual);
        }
    }
}
