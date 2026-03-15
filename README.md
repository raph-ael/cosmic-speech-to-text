# cosmic-speech-to-text

A native speech-to-text panel applet for the [COSMIC](https://system76.com/cosmic) desktop environment.

Record your voice and have it transcribed and typed into any application — directly from the panel.

![COSMIC Panel Applet](https://img.shields.io/badge/COSMIC-Panel%20Applet-blue)
![License: MIT](https://img.shields.io/badge/License-MIT-green)

## Features

- **Panel applet** with microphone icon showing recording/processing state
- **Global hotkey** (default: `Ctrl+Y`) — auto-registered in COSMIC system shortcuts
- **Three transcription backends:**
  - **Mistral API** (voxtral-mini-2507)
  - **OpenAI API** (whisper-1)
  - **Local whisper.cpp** — fully offline, no API key needed
- **One-click model download** — choose from Tiny to Large v3
- **One-click whisper.cpp install** — builds from source automatically
- **Direct text typing** via `wtype` — works in terminals, editors, browsers
- **Settings window** with backend selection, hotkey config, dependency checker
- **Translations:** English, German, Spanish, French

## Installation

### Prerequisites

```bash
sudo apt install wl-clipboard wtype ffmpeg
```

For local whisper.cpp, you also need build tools:
```bash
sudo apt install cmake gcc g++ git
```

### Build & Install

```bash
git clone https://github.com/raphaelhuefner/cosmic-speech-to-text.git
cd cosmic-speech-to-text
cargo build --release
sudo just install
```

### Add to Panel

Right-click the COSMIC panel → Edit Panel → Applets → Add "Speech to Text"

## Usage

1. **Click the microphone icon** or press **Ctrl+Y** to start recording
2. **Click again** or press **Ctrl+Y** to stop — transcription runs automatically
3. The transcribed text is typed into the currently focused window

### First-time Setup

1. Open Settings (click applet → Settings)
2. Choose your transcription backend:
   - **Mistral/OpenAI:** Paste your API key
   - **Local whisper.cpp:** Click "Install whisper.cpp", then download a model
3. Save

## Configuration

Config is stored at `~/.config/cosmic-speech-to-text/config.json`

Whisper models are downloaded to `~/.local/share/cosmic-speech-to-text/models/`

## Architecture

```
Panel Icon ──► Popup (Record/Settings)
     │
  Ctrl+Y (--toggle via COSMIC shortcut)
     │
┌────▼────┐
│ Record  │  cpal → WAV → ffmpeg → MP3
└────┬────┘
     │
┌────▼────┐
│Transcribe│  Mistral API / OpenAI API / whisper.cpp
└────┬────┘
     │
┌────▼────┐
│  Type   │  wtype (direct keyboard input)
└─────────┘
```

## License

MIT
