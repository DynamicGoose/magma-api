use magma_app::World;

use crate::{
    ClosingWindow, Window,
    window::WindowResolution,
    window_event::{WindowCloseRequested, WindowFocused, WindowMoved, WindowResized},
};

pub fn mark_closed_windows(world: &World) {
    let close_requests = world.poll_events::<WindowCloseRequested>().unwrap();

    for close_request in close_requests {
        if world
            .get_component::<Window>(close_request.window)
            .unwrap()
            .default_event_handling
        {
            world
                .assign_components((ClosingWindow,), close_request.window)
                .unwrap();
        }
    }
}

pub fn resized(world: &World) {
    let resize_events = world.poll_events::<WindowResized>().unwrap();

    for resize_event in resize_events {
        let mut window = world
            .get_component_mut::<Window>(resize_event.window)
            .unwrap();

        if window.default_event_handling {
            window.resolution = WindowResolution::new(resize_event.width, resize_event.height);
        }
    }
}

pub fn moved(world: &World) {
    let move_events = world.poll_events::<WindowMoved>().unwrap();

    for move_event in move_events {
        let mut window = world
            .get_component_mut::<Window>(move_event.window)
            .unwrap();

        if window.default_event_handling {
            window.position = crate::window::WindowPosition::Pos(move_event.position);
        }
    }
}

pub fn focused(world: &World) {
    let focus_events = world.poll_events::<WindowFocused>().unwrap();

    for focus_event in focus_events {
        let mut window = world
            .get_component_mut::<Window>(focus_event.window)
            .unwrap();

        if window.default_event_handling {
            window.focused = focus_event.focus;
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
        });
}
