use magma_app::module::Module;

pub use window::Window;

mod window;

pub struct WindowModule;

impl Module for WindowModule {
    fn setup(self, app: &mut magma_app::App) {
        app.world.register_component::<Window>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magma_app::App;

    #[test]
    fn module_setup() {
        let mut app = App::new();
        app.add_module(WindowModule);
        app.world.create_entity((Window::default(),)).unwrap();
        app.world
            .query()
            .with_component::<Window>()
            .unwrap()
            .run(|entities| {
                entities[0]
                    .component_mut(|window: &mut Window| window.maximized = true)
                    .unwrap();
            });
    }
}
