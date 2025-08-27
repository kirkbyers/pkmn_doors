use rdev::{listen, Event, EventType, Key};
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::{cmp::max, env, fs, io::Cursor, path::Path, sync::Arc, time::Duration};
use tokio::sync::Mutex;

const PKMN_MODE: &str = "pkmn";
const ACID_MODE: &str = "acid";
const FILES_MODE: &str = "files";
const VOL_DEFAULT: f32 = 0.35;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut mode: String = String::from(PKMN_MODE);
    let mut sounds_path: Option<String> = None;

    for (idx, arg) in args.iter().enumerate() {
        if arg.to_lowercase() == "--mode" && idx + 1 < args.len() {
            mode = String::from(args[idx + 1].to_lowercase());
        }
        if (arg.to_lowercase() == "--path" || arg.to_lowercase() == "-p") && idx + 1 < args.len() {
            sounds_path = Some(args[idx + 1].clone());
            mode = String::from(FILES_MODE);
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
        FILES_MODE => {
            match sounds_path {
                Some(path) => {
                    if let Err(error) = listen(bind_from_files(&path)?) {
                        println!("Error {:?}", error);
                    }
                }
                None => {
                    println!("Files mode requires --path argument. Exiting");
                    return Ok(());
                }
            }
        }
        _ => {
            println!("No matching mode found for {:?}. Exiting", mode);
        }
    }

    Ok(())
}

fn get_volume_arg() -> f32 {
    let args: Vec<String> = env::args().collect();
    for (idx, arg) in args.iter().enumerate() {
        if ["--volume".to_string(), "-v".to_string()].contains(&arg.to_lowercase()) {
            let volume = match args[idx+1].to_lowercase().parse::<f32>() {
                Ok(v) => v,
                Err(_) => VOL_DEFAULT,
            };
            return volume;
        }
    }
    return VOL_DEFAULT;
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
    let vol = get_volume_arg();
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

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "a" => Some(Key::KeyA),
        "b" => Some(Key::KeyB),
        "c" => Some(Key::KeyC),
        "d" => Some(Key::KeyD),
        "e" => Some(Key::KeyE),
        "f" => Some(Key::KeyF),
        "g" => Some(Key::KeyG),
        "h" => Some(Key::KeyH),
        "i" => Some(Key::KeyI),
        "j" => Some(Key::KeyJ),
        "k" => Some(Key::KeyK),
        "l" => Some(Key::KeyL),
        "m" => Some(Key::KeyM),
        "n" => Some(Key::KeyN),
        "o" => Some(Key::KeyO),
        "p" => Some(Key::KeyP),
        "q" => Some(Key::KeyQ),
        "r" => Some(Key::KeyR),
        "s" => Some(Key::KeyS),
        "t" => Some(Key::KeyT),
        "u" => Some(Key::KeyU),
        "v" => Some(Key::KeyV),
        "w" => Some(Key::KeyW),
        "x" => Some(Key::KeyX),
        "y" => Some(Key::KeyY),
        "z" => Some(Key::KeyZ),
        "escape" => Some(Key::Escape),
        "backspace" => Some(Key::Backspace),
        "space" => Some(Key::Space),
        "enter" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct KeyBinding {
    cmd: bool,
    shift: bool,
    ctrl: bool,
    key: Key,
    file_path: String,
}

fn parse_filename(filename: &str) -> Option<KeyBinding> {
    let stem = Path::new(filename).file_stem()?.to_str()?;
    
    let parts: Vec<&str> = stem.split('-').collect();
    
    if parts.is_empty() {
        return None;
    }
    
    let (modifiers, key_str) = if parts.len() == 1 {
        // No modifiers, just key
        (Vec::new(), parts[0])
    } else {
        // Last part is the key, everything before are modifiers
        let key_str = parts.last()?;
        let modifiers = parts[..parts.len()-1].to_vec();
        (modifiers, *key_str)
    };
    
    let key = string_to_key(key_str)?;
    
    let mut cmd = false;
    let mut shift = false;
    let mut ctrl = false;
    
    for modifier in modifiers {
        match modifier.to_lowercase().as_str() {
            "cmd" => cmd = true,
            "shift" => shift = true,
            "ctrl" => ctrl = true,
            _ => return None, // Unknown modifier
        }
    }
    
    Some(KeyBinding {
        cmd,
        shift,
        ctrl,
        key,
        file_path: filename.to_string(),
    })
}

fn play_file(file_path: &str) {
    match fs::read(file_path) {
        Ok(bytes) => play_bytes(&bytes),
        Err(e) => println!("Error reading file {}: {:?}", file_path, e),
    }
}

fn bind_from_files(sounds_dir: &str) -> Result<impl FnMut(Event), Box<dyn std::error::Error>> {
    let mut bindings: Vec<KeyBinding> = Vec::new();
    
    for entry in fs::read_dir(sounds_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    match ext.to_lowercase().as_str() {
                        "flac" | "wav" | "mp3" | "ogg" => {
                            if let Some(mut binding) = parse_filename(filename) {
                                binding.file_path = path.to_string_lossy().to_string();
                                bindings.push(binding);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    println!("Loaded {} key bindings from {}", bindings.len(), sounds_dir);
    
    let mut cmd_pressed = false;
    let mut shift_pressed = false;
    let mut ctrl_pressed = false;
    
    Ok(move |e: Event| {
        handle_key_state!(e.event_type, Key::MetaLeft, Key::MetaRight, cmd_pressed);
        handle_key_state!(e.event_type, Key::ShiftLeft, Key::ShiftRight, shift_pressed);
        handle_key_state!(e.event_type, Key::ControlLeft, Key::ControlRight, ctrl_pressed);
        
        if let EventType::KeyPress(key) = e.event_type {
            for binding in &bindings {
                if binding.key == key && 
                   binding.cmd == cmd_pressed && 
                   binding.shift == shift_pressed && 
                   binding.ctrl == ctrl_pressed {
                    let file_path = binding.file_path.clone();
                    tokio::spawn(async move {
                        play_file(&file_path);
                    });
                    break;
                }
            }
        }
    })
}

fn play_bytes(bytes: &Vec<u8>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let vol = get_volume_arg();
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
