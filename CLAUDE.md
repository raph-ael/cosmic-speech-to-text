# cosmic-speech-to-text

COSMIC panel applet for speech-to-text transcription. Record voice and have transcribed text typed into any application.

## Features
- Three transcription backends: Mistral API (voxtral), OpenAI API (whisper), local whisper.cpp
- Global hotkey (default Ctrl+Y) auto-registered in COSMIC system shortcuts
- Audio recording via cpal, WAV→MP3 via ffmpeg
- Text insertion via wtype (direct typing)
- One-click whisper.cpp build and model download
- Settings window with backend selection, hotkey config, dependency checker
- i18n: English, German, Spanish, French
- Config migration from old single-key format

## Build & Install
```bash
sudo apt install wl-clipboard wtype ffmpeg libasound2-dev
cargo build --release
sudo just install
```

## Development
```bash
just run          # Run with RUST_BACKTRACE=full
just check        # Clippy check
```

## Project Structure
- `src/main.rs` — Entry point, shortcut registration, CLI flags (--toggle, --settings)
- `src/app.rs` — Panel applet UI, recording state machine, toggle polling
- `src/settings.rs` — Settings window (backend selection, hotkey, deps)
- `src/config.rs` — Config struct with TranscribeMode enum, load/save, migration
- `src/audio.rs` — cpal recording, WAV writer, ffmpeg MP3 conversion
- `src/transcribe.rs` — Mistral/OpenAI/local whisper.cpp transcription
- `src/paste.rs` — Text insertion via wtype
- `src/shortcut.rs` — COSMIC system shortcut RON file management
- `src/toggle.rs` — File-based IPC for --toggle hotkey
- `src/whisper_models.rs` — Model definitions, download, whisper.cpp build
- `src/deps.rs` — System dependency checker (wl-copy, wtype, ffmpeg)
- `src/i18n.rs` — Localization with fl!() macro
- `i18n/` — Fluent translation files (en, de, es, fr)

## Key Patterns
- Toggle recording via file-based IPC: `--toggle` writes `/tmp/cosmic-stt-toggle`, applet polls every 50ms
- COSMIC shortcuts stored as RON in `~/.config/cosmic/com.system76.CosmicSettings.Shortcuts/v1/custom`
- Config at `~/.config/cosmic-speech-to-text/config.json`
- Whisper models at `~/.local/share/cosmic-speech-to-text/models/`

## GitHub
- Repo: https://github.com/raph-ael/cosmic-speech-to-text
- Listed in: https://github.com/cosmic-utils/cosmic-project-collection
