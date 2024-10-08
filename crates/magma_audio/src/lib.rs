/*!
This crate is an integration of the [`kira`] crate for the Magma3D engine.
Usage example:
```
use magma_audio::kira::{
manager::AudioManager,
sound::static_sound::StaticSoundData,
};
use magma_app::App;
use magma_audio::{sounds::Sounds, AudioModule};

// add AudioModule
let mut app = App::new();
app.add_module(AudioModule);
{
    // load sound
    let mut resources = app.world.resources_write();
    let sounds = resources.get_mut::<Sounds>().unwrap();
    sounds.push(StaticSoundData::from_file("sound.ogg").unwrap());
}

// get sound
let sound = app.world.resources_read().get_ref::<Sounds>().unwrap()[0].clone();
//play sound
app.world
    .resources_write()
    .get_mut::<AudioManager>()
    .unwrap()
    .play(sound)
    .unwrap();
```
*/

use kira::manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings};
use magma_app::{module::Module, App};
use sounds::Sounds;

pub use kira;

/// Sounds resource
pub mod sounds;

/// Can be added to an [`App`] for audio support
pub struct AudioModule;

impl Module for AudioModule {
    fn setup(&self, app: &mut App) {
        app.world.add_resource(
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
        );
        let sounds_resource: Sounds = vec![];
        app.world.add_resource(sounds_resource);
    }
}

#[cfg(test)]
mod tests {
    use kira::{manager::AudioManager, sound::static_sound::StaticSoundData};
    use magma_app::App;

    use crate::{sounds::Sounds, AudioModule};

    #[test]
    fn play_sound() {
        let mut app = App::new();
        app.add_module(AudioModule);
        app.world
            .resources_write()
            .get_mut::<AudioManager>()
            .unwrap()
            .play(StaticSoundData::from_file("sound.ogg").unwrap())
            .unwrap();
    }

    #[test]
    fn load_sound() {
        let mut app = App::new();
        app.add_module(AudioModule);
        {
            let mut resources = app.world.resources_write();
            let sounds = resources.get_mut::<Sounds>().unwrap();
            sounds.push(StaticSoundData::from_file("sound.ogg").unwrap());
        }
        let sound = app.world.resources_read().get_ref::<Sounds>().unwrap()[0].clone();
        app.world
            .resources_write()
            .get_mut::<AudioManager>()
            .unwrap()
            .play(sound)
            .unwrap();
    }
}
