use rdev::{listen, Event, EventType, Key};
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::io::Cursor;

const VOL: f32 = 0.35;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = listen(cb()) {
        println!("Error {:?}", error);
    }

    Ok(())
}

fn cb() -> impl FnMut(Event) {
    let mut cmd_pressed = false;
    move |e: Event| {
        // println!("{:?}", e);
        if e.event_type == EventType::KeyPress(Key::MetaLeft)
            || e.event_type == EventType::KeyPress(Key::MetaRight)
        {
            cmd_pressed = true;
        }
        if e.event_type == EventType::KeyRelease(Key::MetaLeft)
            || e.event_type == EventType::KeyRelease(Key::MetaRight)
        {
            cmd_pressed = false;
        }
        if e.event_type == EventType::KeyPress(Key::KeyW) && cmd_pressed {
            tokio::spawn(async {
                play_doors();
            });
        }
        if e.event_type == EventType::KeyPress(Key::KeyS) && cmd_pressed {
            tokio::spawn(async {
                play_pkmn_center();
            });
        }
    }
}

fn play_bytes(bytes: &Vec<u8>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(VOL);
    // Load a sound from a file, using a path relative to Cargo.toml
    let slice = Cursor::new(bytes.to_vec());
    // Decode that sound file into a source
    let source = Decoder::new(slice).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

fn play_doors() {
    play_bytes(
        &include_bytes!("./Pokémon Red_Blue_Yellow - Door Enter - Sound Effect-00rlTif_Kfg.flac")
            .to_vec(),
    )
}

fn play_pkmn_center() {
    play_bytes(
        &include_bytes!("./Pokémon Center Heal - Pokémon Red_Blue_Yellow Version-3IQSjLXfiPI.flac")
            .to_vec(),
    )
}
