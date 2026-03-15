mod app;
mod audio;
mod config;
mod deps;
mod i18n;
mod paste;
mod settings;
mod shortcut;
mod toggle;
mod transcribe;
mod whisper_models;

fn ensure_shortcut_registered() {
    if shortcut::find_our_shortcut().is_none() {
        let cfg = config::load_config();
        let hotkey = if cfg.hotkey.contains('+') {
            cfg.hotkey
        } else {
            "Ctrl+Y".to_string()
        };
        if let Err(e) = shortcut::set_shortcut(&hotkey) {
            eprintln!("Failed to register shortcut: {e}");
        }
    }
}

fn main() -> cosmic::iced::Result {
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);

    // Always ensure shortcut is registered, regardless of mode
    ensure_shortcut_registered();

    if std::env::args().any(|a| a == "--toggle") {
        toggle::send_toggle();
        std::process::exit(0);
    } else if std::env::args().any(|a| a == "--settings") {
        let settings = cosmic::app::Settings::default()
            .size(cosmic::iced::Size::new(500.0, 400.0));
        cosmic::app::run::<settings::SettingsModel>(settings, ())
    } else {
        cosmic::applet::run::<app::AppModel>(())
    }
}
