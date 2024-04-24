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
    let mut shift_pressed = false;
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
        if e.event_type == EventType::KeyPress(Key::ShiftLeft)
            || e.event_type == EventType::KeyPress(Key::ShiftRight)
        {
            shift_pressed = true;
        }
        if e.event_type == EventType::KeyRelease(Key::ShiftLeft)
            || e.event_type == EventType::KeyRelease(Key::ShiftRight)
        {
            shift_pressed = false;
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
        if e.event_type == EventType::KeyPress(Key::KeyZ) && cmd_pressed {
            tokio::spawn(async {
                play_collision();
            });
        }
        if e.event_type == EventType::KeyPress(Key::KeyP) && cmd_pressed  {
            tokio::spawn(async {
                play_tele();
            });
        }
        if (e.event_type == EventType::KeyPress(Key::KeyK) && cmd_pressed && shift_pressed) || (e.event_type == EventType::KeyPress(Key::Backspace) && cmd_pressed) {
            tokio::spawn(async {
                play_poison();
            });
        }
        if e.event_type == EventType::KeyPress(Key::Escape) {
            tokio::spawn(async {
                play_fly();
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

macro_rules! play_sound {
    ($name:ident, $path:expr) => {
        fn $name() {
            play_bytes(
                &include_bytes!($path)
                    .to_vec(),
            )
        }
    };
}

play_sound!(play_doors, "./Pokémon Red_Blue_Yellow - Door Enter - Sound Effect-00rlTif_Kfg.flac");
play_sound!(play_pkmn_center, "./Pokémon Center Heal - Pokémon Red_Blue_Yellow Version-3IQSjLXfiPI.flac");
play_sound!(play_collision, "./Pokémon Red_Blue_Yellow - Collision - Sound Effect-TgOm3ewdXcc.flac");
play_sound!(play_tele, "./Pokémon Red_Blue_Yellow - Teleport - Sound Effect-wa6_3zkNGKI.flac");
play_sound!(play_fly, "Pokémon Red_Blue_ Yellow - Fly - Sound Effect-OUdD1Itsukc.flac");
play_sound!(play_poison, "Pokémon Red_Blue_Yellow - Poison - Sound Effect-09nSUB3QhlM.flac");
