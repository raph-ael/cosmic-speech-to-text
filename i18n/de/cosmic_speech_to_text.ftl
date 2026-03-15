app-title = Sprache zu Text

# Panel popup
start-recording = Aufnahme starten
stop-recording = Aufnahme stoppen
processing = Verarbeitung...
settings = Einstellungen
hotkey = Tastenkürzel: {$hotkey}
status = Status: {$status}

# Status messages
status-ready = Bereit
status-not-configured = Nicht konfiguriert
status-recording = Aufnahme läuft...
status-transcribing = Transkribiere...
status-set-api-key = Bitte zuerst API-Schlüssel oder lokales Whisper konfigurieren!
status-done = Fertig: {$text}...
status-error = Fehler: {$error}
status-paste-failed = Transkribiert, aber Einfügen fehlgeschlagen: {$error}
status-saved = Einstellungen gespeichert!

# Settings window
settings-title = Sprache-zu-Text Einstellungen
paste-from-clipboard = Aus Zwischenablage einfügen
save = Speichern
close = Schließen

# Mode selection
mode-label = Transkriptions-Backend
mode-mistral = Mistral (voxtral)
mode-openai = OpenAI (whisper)
mode-local = Lokal (whisper.cpp)

# API key fields
mistral-api-key-label = Mistral API-Schlüssel
openai-api-key-label = OpenAI API-Schlüssel
api-key-placeholder = API-Schlüssel eingeben...

# Local whisper settings
whisper-cpp-path-label = whisper.cpp Binary-Pfad
whisper-cpp-path-placeholder = /usr/local/bin/whisper-cpp
whisper-model-select = Whisper Modell
whisper-download = Herunterladen
whisper-downloading = Wird heruntergeladen...
whisper-downloaded = Heruntergeladen
whisper-download-error = Download fehlgeschlagen: {$error}
whisper-install-cpp = whisper.cpp installieren
whisper-installing-cpp = whisper.cpp wird kompiliert...
whisper-cpp-installed = whisper.cpp installiert
whisper-cpp-not-installed = whisper.cpp nicht installiert

# Hotkey settings
hotkey-label = Tastenkombination
hotkey-placeholder = z.B. Ctrl+Y
shortcut-active = Tastenkürzel {$hotkey} ist registriert und aktiv.
shortcut-conflict = Warnung: Dieses Tastenkürzel wird bereits verwendet von: {$other}
shortcut-will-register = Tastenkürzel wird beim Speichern registriert.
shortcut-invalid = Ungültiges Tastenkürzel: {$error}

# Dependencies
missing-deps = Fehlende Abhängigkeiten:
missing-dep-entry = {$cmd} (Paket: {$pkg})
missing-deps-install = sudo apt install {$packages}
missing-deps-summary = Fehlt: {$cmds} — Installieren: sudo apt install {$packages}
