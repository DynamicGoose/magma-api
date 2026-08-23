use crate::App;

pub trait Module {
    fn init(self, app: &mut App);
}
