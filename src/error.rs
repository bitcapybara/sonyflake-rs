use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("start_time `{0}` is ahead of current time")]
    StartTimeAheadOfCurrentTime(DateTime<Utc>),
    #[error("machine_id returned an error: {0}")]
    MachineIdFailed(Box<dyn std::error::Error>),
    #[error("check_machine_id returned false")]
    CheckMachineIdFailed,
    #[error("over the time limit")]
    OverTimeLimit,
    #[error("could not find any private ipv4 address")]
    NoPrivateIPv4,
}
