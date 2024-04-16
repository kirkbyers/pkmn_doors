use rdev::{listen, Event, EventType};
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    

    if let Err(error) = listen(cb) {
        println!("Error {:?}", error);
    }

    Ok(())
}

fn cb(e: Event) {
    if e.event_type == EventType::ButtonPress(rdev::Button::Left) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("src/Pokémon Red_Blue_Yellow - Door Enter - Sound Effect-00rlTif_Kfg.flac").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();

        stream_handle.play_raw(source.convert_samples()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}