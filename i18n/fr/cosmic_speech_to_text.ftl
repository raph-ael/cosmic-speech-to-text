app-title = Reconnaissance vocale

start-recording = Démarrer l'enregistrement
stop-recording = Arrêter l'enregistrement
processing = Traitement...
settings = Paramètres
hotkey = Raccourci : {$hotkey}
status = État : {$status}

status-ready = Prêt
status-not-configured = Non configuré
status-recording = Enregistrement...
status-transcribing = Transcription...
status-set-api-key = Veuillez d'abord configurer la clé API ou whisper local !
status-done = Terminé : {$text}...
status-error = Erreur : {$error}
status-paste-failed = Transcrit mais le collage a échoué : {$error}
status-saved = Paramètres enregistrés !

settings-title = Paramètres de reconnaissance vocale
paste-from-clipboard = Coller depuis le presse-papiers
save = Enregistrer
close = Fermer

mode-label = Moteur de transcription
mode-mistral = Mistral (voxtral)
mode-openai = OpenAI (whisper)
mode-local = Local (whisper.cpp)

mistral-api-key-label = Clé API Mistral
openai-api-key-label = Clé API OpenAI
api-key-placeholder = Entrer la clé API...

whisper-cpp-path-label = Chemin du binaire whisper.cpp
whisper-cpp-path-placeholder = /usr/local/bin/whisper-cpp
whisper-model-select = Modèle Whisper
whisper-download = Télécharger
whisper-downloading = Téléchargement...
whisper-downloaded = Téléchargé
whisper-download-error = Échec du téléchargement : {$error}
whisper-install-cpp = Installer whisper.cpp
whisper-installing-cpp = Compilation de whisper.cpp...
whisper-cpp-installed = whisper.cpp installé
whisper-cpp-not-installed = whisper.cpp non installé

hotkey-label = Raccourci clavier
hotkey-placeholder = ex. Ctrl+Y
shortcut-active = Le raccourci {$hotkey} est enregistré et actif.
shortcut-conflict = Attention : Ce raccourci est déjà utilisé par : {$other}
shortcut-will-register = Le raccourci sera enregistré lors de la sauvegarde.
shortcut-invalid = Raccourci invalide : {$error}

missing-deps = Dépendances manquantes :
missing-dep-entry = {$cmd} (paquet : {$pkg})
missing-deps-install = sudo apt install {$packages}
missing-deps-summary = Manquant : {$cmds} — Installer : sudo apt install {$packages}
