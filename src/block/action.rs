use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockAction {
    Open,
    Close,
    StartCrack { break_time: Duration },
    ContinueCrack { break_time: Duration },
    StopCrack,
}
