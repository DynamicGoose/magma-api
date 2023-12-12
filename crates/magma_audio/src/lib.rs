use audio::Audio;
use magma_app::module::Module;
use rodio::OutputStream;

pub mod audio;

pub struct AudioModule;

impl Module for AudioModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.world.register_component::<Audio>();
        app.world
            .add_resource(OutputStream::try_default().unwrap().1);
    }
}

#[cfg(test)]
mod tests {}
