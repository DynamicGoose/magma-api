/// Mouse buttons
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Forward,
    Back,
    Other(u16),
}

/// Indicates the unit of a mouse scroll event.
pub enum MouseScrollUnit {
    /// The delta of the [`MouseScrollInput`](crate::input_event::MouseScrollInput) event corresponds to the amount of lines or rows to scroll.
    Line,
    /// The delta of the [`MouseScrollInput`](crate::input_event::MouseScrollInput) event corresponds to the amount of pixels to scroll.
    Pixel,
}
