/// Number of cycles it took to execute the instruction, somewhat approximate¹.
///
/// ¹ <https://github.com/buserror/simavr/blob/b3ea4f93e18fa5059f76060ff718dc544ca48009/simavr/sim/sim_core.c#L621>
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CpuCyclesTaken(u64);

impl CpuCyclesTaken {
    pub fn new(value: u64) -> Self {
        assert!(value > 0);

        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}
