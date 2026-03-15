app-title = Speech to Text

# Panel popup
start-recording = Start Recording
stop-recording = Stop Recording
processing = Processing...
settings = Settings
hotkey = Hotkey: {$hotkey}
status = Status: {$status}

# Status messages
status-ready = Ready
status-not-configured = Not configured
status-recording = Recording...
status-transcribing = Transcribing...
status-set-api-key = Configure API key or local whisper first!
status-done = Done: {$text}...
status-error = Error: {$error}
status-paste-failed = Transcribed but paste failed: {$error}
status-saved = Settings saved!

# Settings window
settings-title = Speech to Text Settings
paste-from-clipboard = Paste from clipboard
save = Save
close = Close

# Mode selection
mode-label = Transcription Backend
mode-mistral = Mistral (voxtral)
mode-openai = OpenAI (whisper)
mode-local = Local (whisper.cpp)

# API key fields
mistral-api-key-label = Mistral API Key
openai-api-key-label = OpenAI API Key
api-key-placeholder = Enter API key...

# Local whisper settings
whisper-cpp-path-label = whisper.cpp binary path
whisper-cpp-path-placeholder = /usr/local/bin/whisper-cpp
whisper-model-select = Whisper Model
whisper-download = Download
whisper-downloading = Downloading...
whisper-downloaded = Downloaded
whisper-download-error = Download failed: {$error}
whisper-install-cpp = Install whisper.cpp
whisper-installing-cpp = Building whisper.cpp...
whisper-cpp-installed = whisper.cpp installed
whisper-cpp-not-installed = whisper.cpp not installed

# Hotkey settings
hotkey-label = Keyboard Shortcut
hotkey-placeholder = e.g. Ctrl+Y
shortcut-active = Shortcut {$hotkey} is registered and active.
shortcut-conflict = Warning: This shortcut is already used by: {$other}
shortcut-will-register = Shortcut will be registered on save.
shortcut-invalid = Invalid shortcut: {$error}

# Dependencies
missing-deps = Missing dependencies:
missing-dep-entry = {$cmd} (package: {$pkg})
missing-deps-install = sudo apt install {$packages}
missing-deps-summary = Missing: {$cmds} — Install: sudo apt install {$packages}
