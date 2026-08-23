use magma_app::{App, magma_ecs::component::Component, schedule::PreUpdate};
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
    app.world
        .component_store
        .register_component::<DummyWindow>();
    let entity = app
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
            window: entity,
        })
        .unwrap();

    app.world
        .event_manager
        .send_event(KeyboardInput {
            key: magma_input::keyboard::Key::Space,
            key_code: magma_input::keyboard::KeyCode::Space,
            state: magma_input::ButtonState::Released,
            repeat: false,
            window: entity,
        })
        .unwrap();

    app.world
        .event_manager
        .send_event(MouseButtonInput {
            button: magma_input::mouse::MouseButton::Left,
            state: magma_input::ButtonState::Pressed,
            window: entity,
        })
        .unwrap();

    app.world
        .event_manager
        .send_event(MouseButtonInput {
            button: magma_input::mouse::MouseButton::Left,
            state: magma_input::ButtonState::Released,
            window: entity,
        })
        .unwrap();

    app.run_schedule(PreUpdate);
    assert!(
        app.world
            .resource_store
            .get::<ButtonMap<KeyCode>>()
            .unwrap()
            .just_released(KeyCode::Space)
            || app
                .world
                .resource_store
                .get::<ButtonMap<MouseButton>>()
                .unwrap()
                .just_released(MouseButton::Left)
    );
}

#[derive(Component)]
struct DummyWindow;
