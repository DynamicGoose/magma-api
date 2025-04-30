use magma_app::{
    World,
    events::Events,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
};
use magma_window::{ClosingWindow, Window, window_event::WindowCloseRequested};

pub fn mark_closed_windows(world: &World) {
    let events = world.get_resource::<Events>().unwrap();
    let close_requests = events.get_events::<WindowCloseRequested>().unwrap();

    for close_request in close_requests {
        let close_request = close_request
            .downcast_ref::<WindowCloseRequested>()
            .unwrap();
        world
            .query::<(Window,)>()
            .unwrap()
            .par_iter()
            .for_each(|window| {
                if window.id() == close_request.window
                    && window
                        .get_component::<Window>()
                        .unwrap()
                        .default_event_handling()
                {
                    window.assign_components((ClosingWindow,)).unwrap();
                }
            });
    }
}
