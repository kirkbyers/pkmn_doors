# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust application that plays Pokémon Game Boy Color sound effects based on keyboard events. It's a global key listener that triggers audio playback for various key combinations, creating an immersive nostalgic experience.

## Build and Development Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the application
cargo run

# Run with specific mode (pkmn or acid)
cargo run -- --mode pkmn
cargo run -- --mode acid

# Run with custom volume (0.0 to 1.0)
cargo run -- --volume 0.5
cargo run -- -v 0.8

# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

## Architecture

The application is structured around two main operating modes:

### Core Components

- **Event Listening**: Uses `rdev` crate for global keyboard event monitoring
- **Audio System**: Uses `rodio` crate for audio playback with embedded sound files
- **Async Runtime**: Built on `tokio` for handling concurrent audio playback

### Operating Modes

1. **Pokémon Mode (`pkmn`)**: Default mode with specific key bindings for various Pokémon sounds
2. **Acid Mode (`acid`)**: Plays continuous electronic music with typing-responsive behavior

### Key Binding System

The application uses macros for clean key event handling:
- `handle_key_press!`: Triggers actions on key press events
- `handle_key_state!`: Tracks modifier key states (Cmd, Shift, Ctrl)

### Sound System

Sound files are embedded in the binary using `include_bytes!` macro. The `play_sound!` macro generates sound playback functions for each audio file.

## Key Bindings (Pokémon Mode)

- Cmd+W: Door sound
- Cmd+S: Pokémon Center heal
- Cmd+Z: Collision sound
- Cmd+P: Teleport sound
- Cmd+C: Pokéball throw
- Cmd+V: Catching fail
- Cmd+R: Pokédex sound
- Cmd+M: Save game sound
- Cmd+Backspace: Poison sound
- Cmd+Shift+K: Poison sound (alternative)
- Cmd+Ctrl+C: Wrong answer sound
- Escape: Fly sound

## Audio Files

Sound files are stored in the `sounds/` directory as FLAC files. All audio is embedded at compile time, making the binary self-contained.

## macOS Installation

The README mentions using launchctl for system-wide installation:
```bash
launchctl submit -l pkmn_doors -- /path/to/pkmn_doors
```