use magma_app::App;
use magma_input::{
    InputModule,
    input_event::{KeyboardInput, MouseButtonInput, MouseMotionInput, MouseScrollInput},
};

#[test]
fn keyboard_input() {
    let mut app = App::new();
    app.add_module(InputModule);

    app.world.register_component::<DummyWindow>();
    let window = app.world.create_entity((DummyWindow,)).unwrap();

    app.world
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

    app.world.register_component::<DummyWindow>();
    let window = app.world.create_entity((DummyWindow,)).unwrap();

    app.world
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

    app.world.register_component::<DummyWindow>();
    let window = app.world.create_entity((DummyWindow,)).unwrap();

    app.world
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
        .send_event(MouseMotionInput {
            delta: magma_math::Vec2::new(-1.4, 3.3),
        })
        .unwrap();
}

struct DummyWindow;
