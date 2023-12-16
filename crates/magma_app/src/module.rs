use crate::App;

/// Must be implemented on your own modules for them to work
pub trait Module {
    fn setup(&self, app: &mut App);
}
