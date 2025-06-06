use magma_app::{
    World,
    events::Events,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
};
use magma_windowing::{
    ClosingWindow, Window,
    window::WindowResolution,
    window_event::{WindowCloseRequested, WindowFocused, WindowMoved, WindowResized},
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
        let mut window = world
            .get_component_mut::<Window>(resize_event.window)
            .unwrap();

        if window.default_event_handling() {
            window.set_resolution(WindowResolution::new(
                resize_event.width,
                resize_event.height,
            ));
            window.changed_attr = false;
        }
    }
}

pub fn moved(world: &World) {
    let events = world.get_resource::<Events>().unwrap();
    let move_events = events.get_events::<WindowMoved>().unwrap();

    for move_event in move_events {
        let move_event = move_event.downcast_ref::<WindowMoved>().unwrap();
        let mut window = world
            .get_component_mut::<Window>(move_event.window)
            .unwrap();

        if window.default_event_handling() {
            window.set_position(magma_windowing::window::WindowPosition::Pos(
                move_event.position,
            ));
            window.changed_attr = false;
        }
    }
}

pub fn focused(world: &World) {
    let events = world.get_resource::<Events>().unwrap();
    let focus_events = events.get_events::<WindowFocused>().unwrap();

    for focus_event in focus_events {
        let focus_event = focus_event.downcast_ref::<WindowFocused>().unwrap();
        let mut window = world
            .get_component_mut::<Window>(focus_event.window)
            .unwrap();

        if window.default_event_handling() {
            window.set_focused(focus_event.focus);
            window.changed_attr = false;
        }
    }
}
