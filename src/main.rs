use rdev::{listen, Event, EventType, Key};
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::io::Cursor;
use std::env;

const VOL: f32 = 0.35;
const PKMN_MODE: &str = "pkmn";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut mode: String = String::from(PKMN_MODE);

    println!("{:?}", args);

    for (idx, arg) in args.iter().enumerate() {
        if arg.to_lowercase() == "--mode" {
            mode = String::from(args[idx+1].to_lowercase());
        }
    }

    match mode.as_ref() {
        PKMN_MODE => {
            if let Err(error) = listen(pkmn_binds()) {
                println!("Error {:?}", error);
            }
        },
        _ => {
            println!("No matching mode found for {:?}. Exiting", mode);
        }
    }

    Ok(())
}

macro_rules! handle_key_press {
    ($event_type:expr, $key:expr, $action:expr) => {
        if $event_type == EventType::KeyPress($key) {
            tokio::spawn(async {
                $action();
            });
        }
    };
}

macro_rules! handle_key_state {
    ($event_type:expr, $key_press:expr, $key_release:expr, $state:ident) => {
        if $event_type == EventType::KeyPress($key_press)
            || $event_type == EventType::KeyPress($key_release)
        {
            $state = true;
        }
        if $event_type == EventType::KeyRelease($key_press)
            || $event_type == EventType::KeyRelease($key_release)
        {
            $state = false;
        }
    };
}

fn pkmn_binds() -> impl FnMut(Event) {
    let mut cmd_pressed = false;
    let mut shift_pressed = false;
    move |e: Event| {
        // println!("{:?}", e);
        handle_key_state!(e.event_type, Key::MetaLeft, Key::MetaRight, cmd_pressed);
        handle_key_state!(e.event_type, Key::ShiftLeft, Key::ShiftRight, shift_pressed);

        if cmd_pressed {
            handle_key_press!(e.event_type, Key::KeyW, play_doors);
            handle_key_press!(e.event_type, Key::KeyS, play_pkmn_center);
            handle_key_press!(e.event_type, Key::KeyZ, play_collision);
            handle_key_press!(e.event_type, Key::KeyP, play_tele);
            handle_key_press!(e.event_type, Key::Backspace, play_poison);
            if shift_pressed {
                handle_key_press!(e.event_type, Key::KeyK, play_poison);
            }
        }

        handle_key_press!(e.event_type, Key::Escape, play_fly);
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
