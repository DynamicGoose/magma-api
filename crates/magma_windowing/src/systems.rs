use magma_app::World;

use crate::{ClosingWindow, Window};

pub fn delete_pending_windows(world: &World) {
    world
        .query::<(ClosingWindow, Window)>()
        .unwrap()
        .iter()
        .for_each(|closing_window| {
            closing_window.delete();
        });
}
