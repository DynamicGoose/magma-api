use magma_app::magma_ecs::ComponentStore;

use crate::ClosingWindow;

pub fn delete_pending_windows(component_store: &mut ComponentStore) {
    let closing_windows = component_store
        .get_components_ref::<ClosingWindow>()
        .unwrap()
        .iter()
        .map(|(id, _)| id)
        .collect::<Vec<usize>>();

    for window in closing_windows {
        component_store.delete_entity(window);
    }
}
