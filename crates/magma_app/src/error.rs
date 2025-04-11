use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Error)]
pub enum EventError {
    /// The type of event requested is not registered
    #[error("attempted to access unregistered event")]
    EventNotRegistered,
}
