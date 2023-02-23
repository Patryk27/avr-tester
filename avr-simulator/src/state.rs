use simavr_ffi as ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AvrState {
    Limbo,
    Stopped,
    Running,
    Sleeping,
    Step,
    StepDone,
    Done,
    Crashed,
}

impl AvrState {
    pub(crate) fn from_ffi(val: i32) -> Self {
        match val as u32 {
            ffi::cpu_Limbo => Self::Limbo,
            ffi::cpu_Stopped => Self::Stopped,
            ffi::cpu_Running => Self::Running,
            ffi::cpu_Sleeping => Self::Sleeping,
            ffi::cpu_Step => Self::Step,
            ffi::cpu_StepDone => Self::StepDone,
            ffi::cpu_Done => Self::Done,
            ffi::cpu_Crashed => Self::Crashed,

            val => {
                panic!("Unknown AvrState: {}", val);
            }
        }
    }
}
