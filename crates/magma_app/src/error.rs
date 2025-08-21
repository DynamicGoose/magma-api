use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Error)]
pub enum ScheduleError {
    /// The schedule requested is not registered
    #[error("attempted to access unregistered schedule")]
    ScheduleNotRegistered,
}
