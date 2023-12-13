//! This crate is an integration of the `kira` crate for the Magma3D engine

use kira::manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings};
use magma_app::module::Module;
use sounds::Sounds;

pub use kira;

/// sounds resource
pub mod sounds;

/// Can be added to an `App` for audio support
pub struct AudioModule;

impl Module for AudioModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.world.add_resource(
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
        );
        let sounds_resource: Sounds = vec![];
        app.world.add_resource(sounds_resource);
    }
}

#[cfg(test)]
mod tests {
    use kira::{
        manager::AudioManager,
        sound::static_sound::{StaticSoundData, StaticSoundSettings},
    };
    use magma_app::App;

    use crate::{sounds::Sounds, AudioModule};

    #[test]
    fn play_sound() {
        let mut app = App::new();
        app.add_module(AudioModule);
        app.world
            .get_resource_mut::<AudioManager>()
            .unwrap()
            .play(StaticSoundData::from_file("sound.ogg", StaticSoundSettings::default()).unwrap())
            .unwrap();
    }

    #[test]
    fn load_sound() {
        let mut app = App::new();
        app.add_module(AudioModule);
        let sounds = app.world.get_resource_mut::<Sounds>().unwrap();
        sounds
            .push(StaticSoundData::from_file("sound.ogg", StaticSoundSettings::default()).unwrap());
        let sound = app.world.get_resource::<Sounds>().unwrap()[0].clone();
        app.world
            .get_resource_mut::<AudioManager>()
            .unwrap()
            .play(sound)
            .unwrap();
    }
}
