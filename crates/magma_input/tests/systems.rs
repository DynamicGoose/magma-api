use magma_app::{
    App,
    entities::Entity,
    magma_ecs::{
        component::Component,
        events::EventSender,
        query::{Query, With},
    },
    schedule::Update,
};
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
    app.add_system(Update, send_keyboard_events, vec![])
        .unwrap();
    app.add_system(Update, send_mouse_events, vec![]).unwrap();
    app.world.register_component::<DummyWindow>();
    app.world.create_entity((DummyWindow,)).unwrap();
    app.run_schedule::<Update>().unwrap();
    app.process_events();
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

fn send_keyboard_events(
    query: Query<Entity, With<DummyWindow>>,
    mut keyboard_events: EventSender<KeyboardInput>,
) {
    let window = query.into_iter().nth(0).unwrap();

    keyboard_events.send(KeyboardInput {
        key: magma_input::keyboard::Key::Space,
        key_code: magma_input::keyboard::KeyCode::Space,
        state: magma_input::ButtonState::Pressed,
        repeat: false,
        window: window,
    });

    keyboard_events.send(KeyboardInput {
        key: magma_input::keyboard::Key::Space,
        key_code: magma_input::keyboard::KeyCode::Space,
        state: magma_input::ButtonState::Released,
        repeat: false,
        window: window,
    });
}

fn send_mouse_events(
    query: Query<Entity, With<DummyWindow>>,
    mut mouse_events: EventSender<MouseButtonInput>,
) {
    let window = query.into_iter().nth(0).unwrap();
    mouse_events.send(MouseButtonInput {
        button: magma_input::mouse::MouseButton::Left,
        state: magma_input::ButtonState::Pressed,
        window: window.into(),
    });

    mouse_events.send(MouseButtonInput {
        button: magma_input::mouse::MouseButton::Left,
        state: magma_input::ButtonState::Released,
        window: window.into(),
    });
}

#[derive(Component)]
struct DummyWindow;
