use rdev::{listen, Event, EventType};
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = listen(cb) {
        println!("Error {:?}", error);
    }

    Ok(())
}

fn cb(e: Event) {
    if e.event_type == EventType::ButtonPress(rdev::Button::Left) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let slice = Cursor::new(
            include_bytes!(
                "./Pokémon Red_Blue_Yellow - Door Enter - Sound Effect-00rlTif_Kfg.flac"
            )
            .as_ref(),
        );
        // Decode that sound file into a source
        let source = Decoder::new(slice).unwrap();

        sink.append(source);
        sink.sleep_until_end();
    }
}
