/// Describes the state of a key or button
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum ButtonState {
    /// The key/button is currently pressed.
    Pressed,
    /// The key/button was released.
    #[default]
    Released,
}
