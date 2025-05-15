use magma_app::{
    World,
    events::Events,
    rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
};
use magma_math::IVec2;
use magma_window::{
    ClosingWindow, Window,
    window_event::{WindowCloseRequested, WindowMoved, WindowResized},
};

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
                if window.id() == close_request.window.id()
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

pub fn resized(world: &World) {
    let events = world.get_resource::<Events>().unwrap();
    let resize_events = events.get_events::<WindowResized>().unwrap();

    for resize_event in resize_events {
        let resize_event = resize_event.downcast_ref::<WindowResized>().unwrap();
        // TODO: use world.get_component_mut() when released
        world
            .query::<(Window,)>()
            .unwrap()
            .par_iter()
            .for_each(|window| {
                if window.id() == resize_event.window.id() {
                    let mut component = window.get_component_mut::<Window>().unwrap();
                    if component.default_event_handling() {
                        component.set_resolution(magma_window::window::WindowResolution {
                            width: resize_event.width,
                            height: resize_event.height,
                        });
                        component.changed_attr = false;
                    }
                }
            });
    }
}

pub fn moved(world: &World) {
    let events = world.get_resource::<Events>().unwrap();
    let move_events = events.get_events::<WindowMoved>().unwrap();

    for move_event in move_events {
        let move_event = resize_event.downcast_ref::<WindowMoved>().unwrap();
        // TODO: use world.get_component_mut() when released
        world
            .query::<(Window,)>()
            .unwrap()
            .par_iter()
            .for_each(|window| {
                if window.id() == move_event.window.id() {
                    let mut component = window.get_component_mut::<Window>().unwrap();
                    if component.default_event_handling() {
                        component.set_position(magma_window::window::WindowPosition::Pos(
                            move_event.position,
                        ));
                        component.changed_attr = false;
                    }
                }
            });
    }
}
