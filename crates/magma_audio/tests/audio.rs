use std::io::BufReader;

use magma_app::{App, World};
use magma_audio::{audio::Audio, AudioModule};
use rodio::{source, Decoder, OutputStream, OutputStreamHandle, Source};

#[test]
fn play_audio() {
    let mut app = App::new();
    app.add_module(AudioModule);
}

fn play_sound(world: &mut World) {
    let output_stream = world.get_resource_mut::<OutputStreamHandle>().unwrap();
    let mut source = world.query();
    let source = source.with_component::<Audio>().unwrap().run_entity();
}
