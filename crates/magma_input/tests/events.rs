use magma_app::{App, magma_ecs::component::Component};
use magma_input::{
    InputModule,
    input_event::{KeyboardInput, MouseButtonInput, MouseMotionInput, MouseScrollInput},
};

#[test]
fn keyboard_input() {
    let mut app = App::new();
    app.add_module(InputModule);

    app.world
        .component_store
        .register_component::<DummyWindow>();
    let window = app
        .world
        .component_store
        .create_entity((DummyWindow,))
        .unwrap();

    app.world
        .event_manager
        .send_event(KeyboardInput {
            key: magma_input::keyboard::Key::Space,
            key_code: magma_input::keyboard::KeyCode::Space,
            state: magma_input::ButtonState::Pressed,
            repeat: false,
            window,
        })
        .unwrap();
}

#[test]
fn mouse_button_input() {
    let mut app = App::new();
    app.add_module(InputModule);

    app.world
        .component_store
        .register_component::<DummyWindow>();
    let window = app
        .world
        .component_store
        .create_entity((DummyWindow,))
        .unwrap();

    app.world
        .event_manager
        .send_event(MouseButtonInput {
            button: magma_input::mouse::MouseButton::Left,
            state: magma_input::ButtonState::Pressed,
            window,
        })
        .unwrap();
}

#[test]
fn mouse_scroll_input() {
    let mut app = App::new();
    app.add_module(InputModule);

    app.world
        .component_store
        .register_component::<DummyWindow>();
    let window = app
        .world
        .component_store
        .create_entity((DummyWindow,))
        .unwrap();

    app.world
        .event_manager
        .send_event(MouseScrollInput {
            unit: magma_input::mouse::MouseScrollUnit::Line,
            x: 0.0,
            y: 2.0,
            window,
        })
        .unwrap();
}

#[test]
fn mouse_motion_input() {
    let mut app = App::new();
    app.add_module(InputModule);

    app.world
        .event_manager
        .send_event(MouseMotionInput {
            delta: glam::Vec2::new(-1.4, 3.3),
        })
        .unwrap();
}

#[derive(Component)]
struct DummyWindow;
