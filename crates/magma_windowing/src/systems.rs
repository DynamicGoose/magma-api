use magma_app::magma_ecs::query::{QueryMut, With};

use crate::{ClosingWindow, Window};

pub fn delete_pending_windows(query: QueryMut<(With<Window>, With<ClosingWindow>)>) {
    for closind_window in query {
        closind_window.delete();
    }
}
