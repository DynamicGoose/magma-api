use magma_app::{App, events::Events};
use magma_math::IVec2;
use magma_windowing::{window_event::*, *};

#[test]
fn create_window() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();
    app.world
        .get_component_mut::<Window>(window)
        .unwrap()
        .set_title("Hello World!");

    assert_eq!(
        app.world
            .get_component::<Window>(window)
            .unwrap()
            .changed_attr,
        true
    );
}

#[test]
fn resize_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowResized {
            window,
            width: 1280,
            height: 720,
        })
        .unwrap();
}

#[test]
fn redraw_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(RedrawRequested)
        .unwrap();
}

#[test]
fn window_created_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowCreated { window })
        .unwrap();
}

#[test]
fn close_requested_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowCloseRequested { window })
        .unwrap();
}

#[test]
fn closed_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowClosed { window })
        .unwrap();
}

#[test]
fn destroyed_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowDestroyed)
        .unwrap();
}

#[test]
fn cursor_moved_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(CursorMoved {
            window,
            position: IVec2::new(16, 16),
        })
        .unwrap();
}

#[test]
fn cursor_entered_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(CursorEntered { window })
        .unwrap();
}

#[test]
fn cursor_left_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(CursorLeft { window })
        .unwrap();
}

#[test]
fn focused_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowFocused {
            window,
            focus: true,
        })
        .unwrap();
}

#[test]
fn occluded_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowOcclusion::Occluded { window })
        .unwrap();
}

#[test]
fn filednd_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(FileDragDrop::HoverCanceled { window })
        .unwrap();
}

#[test]
fn moved_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowMoved {
            window,
            position: IVec2::new(100, 200),
        })
        .unwrap();
}

#[test]
fn theme_changed_event() {
    let mut app = App::new();
    app.add_module(WindowModule);

    let window = app.world.create_entity((Window::new(),)).unwrap();

    // push event
    app.world
        .get_resource_mut::<Events>()
        .unwrap()
        .push_event(WindowThemeChanged {
            window,
            theme: window::WindowTheme::Dark,
        })
        .unwrap();
}
