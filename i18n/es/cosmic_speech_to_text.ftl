app-title = Voz a texto

start-recording = Iniciar grabación
stop-recording = Detener grabación
processing = Procesando...
settings = Ajustes
hotkey = Atajo: {$hotkey}
status = Estado: {$status}

status-ready = Listo
status-not-configured = No configurado
status-recording = Grabando...
status-transcribing = Transcribiendo...
status-set-api-key = ¡Configure primero la clave API o whisper local!
status-done = Listo: {$text}...
status-error = Error: {$error}
status-paste-failed = Transcrito pero no se pudo pegar: {$error}
status-saved = ¡Ajustes guardados!

settings-title = Ajustes de voz a texto
paste-from-clipboard = Pegar desde el portapapeles
save = Guardar
close = Cerrar

mode-label = Motor de transcripción
mode-mistral = Mistral (voxtral)
mode-openai = OpenAI (whisper)
mode-local = Local (whisper.cpp)

mistral-api-key-label = Clave API de Mistral
openai-api-key-label = Clave API de OpenAI
api-key-placeholder = Introducir clave API...

whisper-cpp-path-label = Ruta del binario whisper.cpp
whisper-cpp-path-placeholder = /usr/local/bin/whisper-cpp
whisper-model-select = Modelo Whisper
whisper-download = Descargar
whisper-downloading = Descargando...
whisper-downloaded = Descargado
whisper-download-error = Descarga fallida: {$error}
whisper-install-cpp = Instalar whisper.cpp
whisper-installing-cpp = Compilando whisper.cpp...
whisper-cpp-installed = whisper.cpp instalado
whisper-cpp-not-installed = whisper.cpp no instalado

hotkey-label = Atajo de teclado
hotkey-placeholder = p.ej. Ctrl+Y
shortcut-active = El atajo {$hotkey} está registrado y activo.
shortcut-conflict = Advertencia: Este atajo ya está en uso por: {$other}
shortcut-will-register = El atajo se registrará al guardar.
shortcut-invalid = Atajo no válido: {$error}

missing-deps = Dependencias faltantes:
missing-dep-entry = {$cmd} (paquete: {$pkg})
missing-deps-install = sudo apt install {$packages}
missing-deps-summary = Faltan: {$cmds} — Instalar: sudo apt install {$packages}
