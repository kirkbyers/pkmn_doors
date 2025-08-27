# Pkmn doors

## What is this?

Have you ever wanted to have the doors sound from your favorite Pokemon GBC game play on every window close? Well here you go!

Included are other sounds on various key bindings. Because why shouldn't your keyboard sound like a Game Boy?

## Usage

### Pkmn Mode (Default)
Just run it and enjoy:
```bash
cargo run
# or
cargo run -- --mode pkmn
```

### Acid Mode
For when you want to get weird with it:
```bash
cargo run -- --mode acid
```

Type faster = longer music.

### Files Mode
Want to make your own sound bindings? Throw some audio files in a folder:

```bash
cargo run -- --path /path/to/your/sounds/
# or
cargo run -- --mode files --path /path/to/your/sounds/
```

**File naming convention:**
- `w.flac` - W key (no modifier)
- `cmd-s.wav` - Cmd+S
- `shift-ctrl-a.mp3` - Shift+Ctrl+A
- `escape.ogg` - Escape key

Supports: .flac, .wav, .mp3, .ogg

**Volume control:**
```bash
cargo run -- --volume 0.8
# or
cargo run -- -v 0.3
```

## Setup

- Download a build or build
- On MacOS, if you want it running all the time:
```bash
# for Mac systemctl
launchctl submit -l pkmn_doors -- /path/to/pkmn_doors
# or with nohup
nohup pkmn_doors -v 0.5 &
```
