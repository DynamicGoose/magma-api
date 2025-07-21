use magma_app::{App, World, events::Events};
use magma_input::{
    ButtonMap, InputModule,
    input_event::{KeyboardInput, MouseButtonInput},
    keyboard::KeyCode,
    mouse::MouseButton,
};

#[test]
fn keyboard_mouse_systems() {
    let mut app = App::new();
    app.add_module(InputModule);
    app.add_systems(
        magma_app::SystemType::Update,
        &[(send_events, "send_events", &[])],
    );
    app.world.register_component::<DummyWindow>();
    app.world.create_entity((DummyWindow,)).unwrap();
    app.update();
    assert!(
        app.world
            .get_resource::<ButtonMap<KeyCode>>()
            .unwrap()
            .just_released(KeyCode::Space)
            || app
                .world
                .get_resource::<ButtonMap<MouseButton>>()
                .unwrap()
                .just_released(MouseButton::Left)
    );
}

fn send_events(world: &World) {
    let mut events = world.get_resource_mut::<Events>().unwrap();
    let window = world.query::<(DummyWindow,)>().unwrap()[0];

    events
        .push_event(KeyboardInput {
            key: magma_input::keyboard::Key::Space,
            key_code: magma_input::keyboard::KeyCode::Space,
            state: magma_input::ButtonState::Pressed,
            repeat: false,
            window: window.into(),
        })
        .unwrap();

    events
        .push_event(KeyboardInput {
            key: magma_input::keyboard::Key::Space,
            key_code: magma_input::keyboard::KeyCode::Space,
            state: magma_input::ButtonState::Released,
            repeat: false,
            window: window.into(),
        })
        .unwrap();

    events
        .push_event(MouseButtonInput {
            button: magma_input::mouse::MouseButton::Left,
            state: magma_input::ButtonState::Pressed,
            window: window.into(),
        })
        .unwrap();

    events
        .push_event(MouseButtonInput {
            button: magma_input::mouse::MouseButton::Left,
            state: magma_input::ButtonState::Released,
            window: window.into(),
        })
        .unwrap();
}

struct DummyWindow;
