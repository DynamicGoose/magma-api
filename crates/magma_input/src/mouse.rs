/// Mouse buttons
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,
    Forward,
    Back,
    Other(u16),
}

/// Indicates the unit of a mouse scroll event.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum MouseScrollUnit {
    /// The delta of the [`MouseScrollInput`](crate::input_event::MouseScrollInput) event corresponds to the amount of lines or rows to scroll.
    #[default]
    Line,
    /// The delta of the [`MouseScrollInput`](crate::input_event::MouseScrollInput) event corresponds to the amount of pixels to scroll.
    Pixel,
}
