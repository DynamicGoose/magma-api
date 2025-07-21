use magma_app::App;
use magma_input::{ButtonMap, InputModule, keyboard::KeyCode, mouse::MouseButton};

#[test]
fn button_map_keyboard_resource() {
    let mut app = App::new();
    app.add_module(InputModule);

    app.world
        .get_resource_mut::<ButtonMap<KeyCode>>()
        .unwrap()
        .press(KeyCode::Comma);

    assert!(
        app.world
            .get_resource::<ButtonMap<KeyCode>>()
            .unwrap()
            .just_pressed(KeyCode::Comma)
    );
}

#[test]
fn button_map_mouse_resource() {
    let mut app = App::new();
    app.add_module(InputModule);

    app.world
        .get_resource_mut::<ButtonMap<MouseButton>>()
        .unwrap()
        .press(MouseButton::Left);

    assert!(
        app.world
            .get_resource::<ButtonMap<MouseButton>>()
            .unwrap()
            .just_pressed(MouseButton::Left)
    );
}
