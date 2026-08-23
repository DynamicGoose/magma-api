use glam::IVec2;
use magma_app::App;
use magma_windowing::{window_event::*, *};

#[test]
fn create_window() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();
    app.world
        .component_store
        .get_components_mut::<Window>()
        .unwrap()
        .get_mut(window)
        .unwrap()
        .title = "Hello World!".to_string();

    assert_eq!(
        app.world
            .component_store
            .get_components_ref::<Window>()
            .unwrap()
            .get(window)
            .unwrap()
            .title,
        "Hello World!".to_string(),
    );
}

#[test]
fn resize_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowResized {
            window,
            width: 1280,
            height: 720,
        })
        .unwrap();
}

#[test]
fn redraw_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    // push event
    app.world.event_manager.send_event(RedrawRequested).unwrap();
}

#[test]
fn window_created_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowCreated { window })
        .unwrap();
}

#[test]
fn close_requested_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowCloseRequested { window })
        .unwrap();
}

#[test]
fn closed_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowClosed { window })
        .unwrap();
}

#[test]
fn destroyed_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowDestroyed { window })
        .unwrap();
}

#[test]
fn cursor_moved_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(CursorMoved {
            window,
            position: window::CursorPosition { x: 16.0, y: 16.0 },
        })
        .unwrap();
}

#[test]
fn cursor_entered_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(CursorEntered { window })
        .unwrap();
}

#[test]
fn cursor_left_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(CursorLeft { window })
        .unwrap();
}

#[test]
fn focused_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowFocused {
            window,
            focus: true,
        })
        .unwrap();
}

#[test]
fn occluded_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowOcclusion::Occluded { window })
        .unwrap();
}

#[test]
fn filednd_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(FileDragDrop::HoverCanceled { window })
        .unwrap();
}

#[test]
fn moved_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowMoved {
            window,
            position: IVec2::new(100, 200),
        })
        .unwrap();
}

#[test]
fn theme_changed_event() {
    let mut app = App::new();
    app.add_module(WindowingModule);

    let window = app
        .world
        .component_store
        .create_entity((Window::new(),))
        .unwrap();

    // push event
    app.world
        .event_manager
        .send_event(WindowThemeChanged {
            window,
            theme: window::WindowTheme::Dark,
        })
        .unwrap();
}
