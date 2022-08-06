use crate::*;

pub trait IntoCycles {
    fn into_cycles(self) -> u64;
}

impl IntoCycles for u64 {
    fn into_cycles(self) -> u64 {
        self
    }
}

impl IntoCycles for CpuDuration {
    fn into_cycles(self) -> u64 {
        self.as_cycles()
    }
}
