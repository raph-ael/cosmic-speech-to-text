use crate::config::{self, TranscribeMode};
use crate::deps;
use crate::fl;
use crate::shortcut;
use crate::whisper_models;

use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget;

pub struct SettingsModel {
    core: cosmic::Core,
    config: config::Config,
    // Input fields
    mistral_key_input: String,
    openai_key_input: String,
    whisper_cpp_input: String,
    hotkey_input: String,
    // Whisper model selection
    selected_model: usize,
    model_labels: Vec<String>,
    // Cached for dropdown
    mode_names: Vec<String>,
    // State
    saved: bool,
    downloading: bool,
    installing_cpp: bool,
    download_status: String,
    shortcut_status: ShortcutStatus,
    missing_deps: Vec<(&'static str, &'static str)>,
}

#[derive(Debug, Clone)]
enum ShortcutStatus {
    Ok(String),
    Conflict(String),
    NotRegistered,
    Error(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    ModeChanged(usize),
    MistralKeyChanged(String),
    OpenAIKeyChanged(String),
    WhisperCppChanged(String),
    WhisperModelSelected(usize),
    DownloadModel,
    DownloadDone(Result<String, String>),
    InstallWhisperCpp,
    InstallWhisperCppDone(Result<String, String>),
    HotkeyChanged(String),
    PasteApiKey,
    Save,
    Close,
}

fn check_shortcut_status(hotkey: &str) -> ShortcutStatus {
    if let Some(current) = shortcut::find_our_shortcut() {
        if current.to_lowercase() == hotkey.to_lowercase() {
            return ShortcutStatus::Ok(current);
        }
    }
    let parts: Vec<&str> = hotkey.split('+').map(|s| s.trim()).collect();
    if parts.len() < 2 {
        return ShortcutStatus::Error(fl!("shortcut-invalid", error = "Ctrl+Y"));
    }
    if let Some(conflict) = shortcut::check_conflict(hotkey) {
        return ShortcutStatus::Conflict(conflict);
    }
    ShortcutStatus::NotRegistered
}

impl cosmic::Application for SettingsModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.cosmic-speech-to-text.settings";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let cfg = config::load_config();

        // Find which model is currently selected based on model_path
        let selected_model = whisper_models::find_model_index(&cfg.whisper_model_path)
            .unwrap_or(1); // default: base

        let app = SettingsModel {
            core,
            mistral_key_input: cfg.mistral_api_key.clone(),
            openai_key_input: cfg.openai_api_key.clone(),
            whisper_cpp_input: cfg.whisper_cpp_path.clone(),
            hotkey_input: cfg.hotkey.clone(),
            selected_model,
            model_labels: whisper_models::model_labels(),
            mode_names: vec![
                fl!("mode-mistral"),
                fl!("mode-openai"),
                fl!("mode-local"),
            ],
            shortcut_status: check_shortcut_status(&cfg.hotkey),
            missing_deps: deps::check_missing(),
            saved: false,
            downloading: false,
            installing_cpp: false,
            download_status: String::new(),
            config: cfg,
        };
        (app, Task::none())
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        vec![]
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = widget::text::title3(fl!("settings-title"));

        // --- Mode selector ---
        let mode_label = widget::text(fl!("mode-label"));
        let mode_dropdown = widget::dropdown(
            &self.mode_names,
            Some(self.config.mode.index()),
            Message::ModeChanged,
        );

        // --- Mode-specific fields ---
        let mode_fields: Element<'_, Self::Message> = match self.config.mode {
            TranscribeMode::Mistral => {
                let label = widget::text(fl!("mistral-api-key-label"));
                let field =
                    widget::text_input(fl!("api-key-placeholder"), &self.mistral_key_input)
                        .on_input(Message::MistralKeyChanged)
                        .password()
                        .width(Length::Fill);
                let paste = widget::button::standard(fl!("paste-from-clipboard"))
                    .on_press(Message::PasteApiKey);
                widget::column::with_children(vec![label.into(), field.into(), paste.into()])
                    .spacing(8)
                    .into()
            }
            TranscribeMode::OpenAI => {
                let label = widget::text(fl!("openai-api-key-label"));
                let field =
                    widget::text_input(fl!("api-key-placeholder"), &self.openai_key_input)
                        .on_input(Message::OpenAIKeyChanged)
                        .password()
                        .width(Length::Fill);
                let paste = widget::button::standard(fl!("paste-from-clipboard"))
                    .on_press(Message::PasteApiKey);
                widget::column::with_children(vec![label.into(), field.into(), paste.into()])
                    .spacing(8)
                    .into()
            }
            TranscribeMode::LocalWhisper => {
                // whisper.cpp binary install
                let cpp_install_btn = if self.installing_cpp {
                    widget::button::standard(fl!("whisper-installing-cpp"))
                } else if whisper_models::is_whisper_cpp_installed()
                    || !self.whisper_cpp_input.is_empty()
                {
                    widget::button::suggested(fl!("whisper-cpp-installed"))
                } else {
                    widget::button::suggested(fl!("whisper-install-cpp"))
                        .on_press(Message::InstallWhisperCpp)
                };

                let cpp_label = widget::text(fl!("whisper-cpp-path-label")).size(12);
                let cpp_field = widget::text_input(
                    fl!("whisper-cpp-path-placeholder"),
                    &self.whisper_cpp_input,
                )
                .on_input(Message::WhisperCppChanged)
                .size(12)
                .width(Length::Fill);

                // Model dropdown
                let model_label = widget::text(fl!("whisper-model-select"));
                let model_dropdown = widget::dropdown(
                    &self.model_labels,
                    Some(self.selected_model),
                    Message::WhisperModelSelected,
                );

                // Download button
                let download_btn = if self.downloading {
                    widget::button::standard(fl!("whisper-downloading"))
                } else if whisper_models::is_downloaded(
                    &whisper_models::MODELS[self.selected_model],
                ) {
                    widget::button::suggested(fl!("whisper-downloaded"))
                } else {
                    widget::button::suggested(fl!("whisper-download"))
                        .on_press(Message::DownloadModel)
                };

                let mut items: Vec<Element<'_, Self::Message>> = vec![
                    cpp_install_btn.into(),
                    cpp_label.into(),
                    cpp_field.into(),
                    widget::divider::horizontal::default().into(),
                    model_label.into(),
                    model_dropdown.into(),
                    download_btn.into(),
                ];

                if !self.download_status.is_empty() {
                    items.push(widget::text(&self.download_status).size(12).into());
                }

                widget::column::with_children(items).spacing(8).into()
            }
        };

        // --- Hotkey ---
        let hotkey_label = widget::text(fl!("hotkey-label"));
        let hotkey_field = widget::text_input(fl!("hotkey-placeholder"), &self.hotkey_input)
            .on_input(Message::HotkeyChanged)
            .width(Length::Fill);

        let shortcut_info: Element<'_, Self::Message> = match &self.shortcut_status {
            ShortcutStatus::Ok(key) => widget::text(fl!("shortcut-active", hotkey = key.as_str()))
                .size(12)
                .into(),
            ShortcutStatus::Conflict(other) => {
                widget::text(fl!("shortcut-conflict", other = other.as_str()))
                    .size(12)
                    .into()
            }
            ShortcutStatus::NotRegistered => {
                widget::text(fl!("shortcut-will-register")).size(12).into()
            }
            ShortcutStatus::Error(e) => widget::text(fl!("shortcut-invalid", error = e.as_str()))
                .size(12)
                .into(),
        };

        // --- Buttons ---
        let is_busy = self.downloading || self.installing_cpp;
        let save_btn = if is_busy {
            widget::button::suggested(fl!("save"))
        } else {
            widget::button::suggested(fl!("save")).on_press(Message::Save)
        };
        let close_btn = if is_busy {
            widget::button::standard(fl!("close"))
        } else {
            widget::button::standard(fl!("close")).on_press(Message::Close)
        };

        let status: Element<'_, Self::Message> = if self.saved {
            widget::text(fl!("status-saved")).into()
        } else {
            widget::text("").into()
        };

        let mut items: Vec<Element<'_, Self::Message>> = vec![
            title.into(),
            widget::divider::horizontal::default().into(),
            mode_label.into(),
            mode_dropdown.into(),
            mode_fields,
            widget::divider::horizontal::default().into(),
            hotkey_label.into(),
            hotkey_field.into(),
            shortcut_info,
            widget::divider::horizontal::default().into(),
            widget::row::with_children(vec![save_btn.into(), close_btn.into()])
                .spacing(8)
                .into(),
            status,
        ];

        // --- Missing deps ---
        if !self.missing_deps.is_empty() {
            items.push(widget::divider::horizontal::default().into());

            let install_cmd = self
                .missing_deps
                .iter()
                .map(|(_, pkg)| *pkg)
                .collect::<Vec<_>>()
                .join(" ");

            let mut warning_items: Vec<Element<'_, Self::Message>> =
                vec![widget::text(fl!("missing-deps")).size(14).into()];

            for &(cmd, pkg) in &self.missing_deps {
                let entry = fl!("missing-dep-entry", cmd = cmd, pkg = pkg);
                warning_items.push(widget::text(entry).size(13).into());
            }

            warning_items.push(
                widget::text(fl!("missing-deps-install", packages = install_cmd.as_str()))
                    .size(13)
                    .into(),
            );

            items.push(
                widget::container(widget::column::with_children(warning_items).spacing(4))
                    .padding(12)
                    .width(Length::Fill)
                    .class(cosmic::theme::Container::List)
                    .into(),
            );
        }

        let content = widget::column::with_children(items)
            .spacing(12)
            .padding(24)
            .width(Length::Fill);

        widget::scrollable(
            widget::container(content)
                .width(Length::Fill)
                .height(Length::Shrink),
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::ModeChanged(idx) => {
                self.config.mode = TranscribeMode::from_index(idx);
                self.saved = false;
            }
            Message::MistralKeyChanged(key) => {
                self.mistral_key_input = key;
                self.saved = false;
            }
            Message::OpenAIKeyChanged(key) => {
                self.openai_key_input = key;
                self.saved = false;
            }
            Message::WhisperCppChanged(path) => {
                self.whisper_cpp_input = path;
                self.saved = false;
            }
            Message::WhisperModelSelected(idx) => {
                self.selected_model = idx;
                // Auto-set model path if already downloaded
                let model = &whisper_models::MODELS[idx];
                if whisper_models::is_downloaded(model) {
                    self.config.whisper_model_path =
                        whisper_models::model_path(model).to_string_lossy().to_string();
                }
                self.saved = false;
            }
            Message::DownloadModel => {
                self.downloading = true;
                self.download_status = fl!("whisper-downloading");
                let idx = self.selected_model;
                return Task::perform(
                    async move { whisper_models::download_model(idx).await },
                    |result| {
                        cosmic::Action::App(Message::DownloadDone(
                            result.map(|p| p.to_string_lossy().to_string()),
                        ))
                    },
                );
            }
            Message::DownloadDone(result) => {
                self.downloading = false;
                self.model_labels = whisper_models::model_labels();
                match result {
                    Ok(path) => {
                        self.config.whisper_model_path = path;
                        self.download_status = fl!("whisper-downloaded");
                    }
                    Err(e) => {
                        self.download_status = fl!("whisper-download-error", error = e.as_str());
                    }
                }
            }
            Message::InstallWhisperCpp => {
                self.installing_cpp = true;
                self.download_status = fl!("whisper-installing-cpp");
                return Task::perform(
                    async { whisper_models::install_whisper_cpp().await },
                    |result| {
                        cosmic::Action::App(Message::InstallWhisperCppDone(
                            result.map(|p| p.to_string_lossy().to_string()),
                        ))
                    },
                );
            }
            Message::InstallWhisperCppDone(result) => {
                self.installing_cpp = false;
                match result {
                    Ok(path) => {
                        self.whisper_cpp_input = path.clone();
                        self.config.whisper_cpp_path = path;
                        self.download_status = fl!("whisper-cpp-installed");
                    }
                    Err(e) => {
                        self.download_status = fl!("whisper-download-error", error = e.as_str());
                    }
                }
            }
            Message::HotkeyChanged(key) => {
                self.hotkey_input = key.clone();
                self.saved = false;
                self.shortcut_status = check_shortcut_status(&key);
            }
            Message::PasteApiKey => {
                if let Ok(output) = std::process::Command::new("wl-paste")
                    .arg("--no-newline")
                    .output()
                {
                    if output.status.success() {
                        if let Ok(text) = String::from_utf8(output.stdout) {
                            let trimmed = text.trim().to_string();
                            match self.config.mode {
                                TranscribeMode::Mistral => self.mistral_key_input = trimmed,
                                TranscribeMode::OpenAI => self.openai_key_input = trimmed,
                                TranscribeMode::LocalWhisper => {}
                            }
                            self.saved = false;
                        }
                    }
                }
            }
            Message::Save => {
                self.config.mistral_api_key = self.mistral_key_input.clone();
                self.config.openai_api_key = self.openai_key_input.clone();
                self.config.whisper_cpp_path = self.whisper_cpp_input.clone();
                // Model path is set automatically on download/select
                self.config.hotkey = self.hotkey_input.clone();
                config::save_config(&self.config);

                match shortcut::set_shortcut(&self.config.hotkey) {
                    Ok(()) => {
                        self.shortcut_status = ShortcutStatus::Ok(self.config.hotkey.clone());
                        self.saved = true;
                    }
                    Err(e) => {
                        self.shortcut_status = ShortcutStatus::Error(e);
                    }
                }
            }
            Message::Close => {
                std::process::exit(0);
            }
        }
        Task::none()
    }
}
