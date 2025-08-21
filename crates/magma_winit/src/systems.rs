use magma_app::{
    World,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
};
use magma_windowing::{
    ClosingWindow, Window,
    window::WindowResolution,
    window_event::{WindowCloseRequested, WindowClosed, WindowFocused, WindowMoved, WindowResized},
};

pub fn mark_closed_windows(world: &World) {
    let close_requests = world.poll_events::<WindowCloseRequested>().unwrap();

    for close_request in close_requests {
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
    let resize_events = world.poll_events::<WindowResized>().unwrap();

    for resize_event in resize_events {
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
    let move_events = world.poll_events::<WindowMoved>().unwrap();

    for move_event in move_events {
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
    let focus_events = world.poll_events::<WindowFocused>().unwrap();

    for focus_event in focus_events {
        let mut window = world
            .get_component_mut::<Window>(focus_event.window)
            .unwrap();

        if window.default_event_handling() {
            window.set_focused(focus_event.focus);
            window.changed_attr = false;
        }
    }
}

pub fn delete_pending_windows(world: &World) {
    world
        .query::<(ClosingWindow, Window)>()
        .unwrap()
        .iter()
        .for_each(|closing_window| {
            closing_window.delete();
            world
                .send_event(WindowClosed {
                    window: closing_window.into(),
                })
                .unwrap();
        });
}
