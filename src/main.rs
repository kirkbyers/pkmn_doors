use lazy_static::lazy_static;
use rdev::{listen, Event, EventType, Key};
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::cmp::max;
use std::env;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

const PKMN_MODE: &str = "pkmn";
const ACID_MODE: &str = "acid";

lazy_static! {
    static ref VOL: Mutex<f32> = Mutex::new(0.35);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut mode: String = String::from(PKMN_MODE);

    for (idx, arg) in args.iter().enumerate() {
        if arg.to_lowercase() == "--mode" {
            mode = String::from(args[idx + 1].to_lowercase());
        }
        if arg.to_lowercase() == "--vol"
            || arg.to_lowercase() == "--volume"
            || arg.to_lowercase() == "-v"
        {
            let volume = match args[idx + 1].to_lowercase().parse::<f32>() {
                Ok(v) => v,
                Err(_) => {
                    println!("Volume must be between 0 and 1");
                    return Ok(());
                }
            };
            if volume < 0.0 || volume > 1.0 {
                println!("Volume must be between 0 and 1");
                return Ok(());
            }
            *VOL.lock().await = volume;
        }
    }

    match mode.as_ref() {
        PKMN_MODE => {
            if let Err(error) = listen(pkmn_binds()) {
                println!("Error {:?}", error);
            }
        }
        ACID_MODE => {
            let mut output_stream = OutputStream::try_default().unwrap().0;
            if let Err(error) = listen(acid_binds(&mut output_stream)) {
                println!("Error {:?}", error);
            }
        }
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

fn acid_binds(output_stream: &mut OutputStream) -> impl FnMut(Event) {
    let consec_keys_counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let consec_keys_counter_in_future = Arc::clone(&consec_keys_counter);
    let should_play = Arc::new(Mutex::new(false));
    let should_play_in_future = Arc::clone(&should_play);
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    *output_stream = stream;
    let sink = Sink::try_new(&stream_handle).unwrap();
    let vol = match VOL.try_lock() {
        Ok(vol) => *vol,
        Err(_) => 0.35,
    };
    sink.set_volume(vol);
    tokio::spawn(async move {
        loop {
            if sink.len() <= 5 {
                for _ in vec![""; 5] {
                    let slice = Cursor::new(
                        include_bytes!(
                            "../sounds/KLOUD - DISCIPLE (Official Music Video)-wT5xVFlavP4.flac"
                        )
                        .to_vec(),
                    );
                    sink.append(Decoder::new(slice).unwrap());
                }
                sink.pause();
            }

            let mut should_play = should_play_in_future.lock().await;
            let mut consec_keys_counter = consec_keys_counter_in_future.lock().await;
            if *should_play {
                sink.play();
                *should_play = false;
            } else {
                if !sink.empty() && !sink.is_paused() {
                    *consec_keys_counter = 0;
                    sink.pause()
                }
            }
            let sleep_in_ms = 500 + max(*consec_keys_counter * 10, 500);
            drop(should_play);
            drop(consec_keys_counter);
            tokio::time::sleep(Duration::from_millis(sleep_in_ms)).await;
        }
    });

    let should_play_in_event = Arc::clone(&should_play);
    let consec_keys_counter_in_event = Arc::clone(&consec_keys_counter);
    move |e: Event| {
        // println!("{:?}", e);
        match e.name {
            Some(_name) => match should_play_in_event.try_lock() {
                Ok(mut should_play) => {
                    *should_play = true;
                    match consec_keys_counter_in_event.try_lock() {
                        Ok(mut consec_key_counter) => {
                            *consec_key_counter += 1;
                        }
                        Err(_) => {}
                    }
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            },
            None => {}
        }
    }
}

fn pkmn_binds() -> impl FnMut(Event) {
    let mut cmd_pressed = false;
    let mut shift_pressed = false;
    let mut ctrl_pressed = false;
    move |e: Event| {
        // println!("{:?}", e);
        handle_key_state!(e.event_type, Key::MetaLeft, Key::MetaRight, cmd_pressed);
        handle_key_state!(e.event_type, Key::ShiftLeft, Key::ShiftRight, shift_pressed);
        handle_key_state!(
            e.event_type,
            Key::ControlLeft,
            Key::ControlRight,
            ctrl_pressed
        );

        if cmd_pressed {
            handle_key_press!(e.event_type, Key::KeyW, play_doors);
            handle_key_press!(e.event_type, Key::KeyS, play_pkmn_center);
            handle_key_press!(e.event_type, Key::KeyZ, play_collision);
            handle_key_press!(e.event_type, Key::KeyP, play_tele);
            handle_key_press!(e.event_type, Key::KeyC, play_catching);
            handle_key_press!(e.event_type, Key::KeyV, play_catching_fail);
            handle_key_press!(e.event_type, Key::KeyR, play_pkdex);
            handle_key_press!(e.event_type, Key::KeyM, play_save);
            handle_key_press!(e.event_type, Key::Backspace, play_poison);
            if shift_pressed {
                handle_key_press!(e.event_type, Key::KeyK, play_poison);
            }
            if ctrl_pressed {
                handle_key_press!(e.event_type, Key::KeyC, play_wrong);
            }
        }

        handle_key_press!(e.event_type, Key::Escape, play_fly);
    }
}

fn play_bytes(bytes: &Vec<u8>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let vol = match VOL.try_lock() {
        Ok(vol) => *vol,
        Err(_) => 0.35,
    };
    sink.set_volume(vol);
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
            play_bytes(&include_bytes!($path).to_vec())
        }
    };
}

play_sound!(
    play_doors,
    "../sounds/Pokémon Red_Blue_Yellow - Door Enter - Sound Effect-00rlTif_Kfg.flac"
);
play_sound!(
    play_pkmn_center,
    "../sounds/Pokémon Center Heal - Pokémon Red_Blue_Yellow Version-3IQSjLXfiPI.flac"
);
play_sound!(
    play_collision,
    "../sounds/Pokémon Red_Blue_Yellow - Collision - Sound Effect-TgOm3ewdXcc.flac"
);
play_sound!(
    play_tele,
    "../sounds/Pokémon Red_Blue_Yellow - Teleport - Sound Effect-wa6_3zkNGKI.flac"
);
play_sound!(
    play_fly,
    "../sounds/Pokémon Red_Blue_ Yellow - Fly - Sound Effect-OUdD1Itsukc.flac"
);
play_sound!(
    play_poison,
    "../sounds/Pokémon Red_Blue_Yellow - Poison - Sound Effect-09nSUB3QhlM.flac"
);
play_sound!(
    play_catching,
    "../sounds/Pokémon Red_Blue_Yellow - Pokéball Throw - Sound Effect-tXY4u4OM4w4.flac"
);
play_sound!(
    play_catching_fail,
    "../sounds/Pokémon Red_Blue_Yellow - Catching Attempt Fail 1 - Sound Effect-T5DLcfFVsKY.flac"
);
play_sound!(
    play_pkdex,
    "../sounds/Pokémon Red_Blue_Yellow - Pokémon Data Pokédex - Sound Effect-rduVgdbO4B0.flac"
);
play_sound!(
    play_wrong,
    "../sounds/Pokémon Red_Blue_Yellow - Wrong Answer - Sound Effect-K4sgpVUOCZw.flac"
);
play_sound!(
    play_save,
    "../sounds/Pokémon Red_Blue_Yellow - Save Game - Sound Effect-59luvyf9SYI.flac"
);
