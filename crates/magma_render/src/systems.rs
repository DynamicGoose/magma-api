use feufeu::RenderState;
use magma_app::World;
use magma_windowing::{ClosingWindow, window_event::WindowClosed};

use crate::extracted_windows::ExtractedWindows;

pub fn drop_windows(world: &World) {
    let render_state = world.get_resource::<RenderState>().unwrap();

    world
        .poll_events::<WindowClosed>()
        .unwrap()
        .iter()
        .for_each(|window_event| {
            render_state
                .render_world
                .assign_components(
                    (ClosingWindow,),
                    render_state
                        .render_world
                        .get_resource::<ExtractedWindows>()
                        .unwrap()
                        .get_render_entity(&window_event.window),
                )
                .unwrap();
        });
}
