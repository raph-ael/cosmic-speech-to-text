use crate::audio::{self, Recorder};
use crate::config;
use crate::paste;
use crate::transcribe;

use cosmic::iced::window::Id;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Idle,
    Recording,
    Processing,
}

pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    state: AppState,
    config: config::Config,
    api_key_input: String,
    status_text: String,
    recorder: Recorder,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    ToggleRecording,
    ApiKeyChanged(String),
    SaveConfig,
    TranscriptionDone(Result<String, String>),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.cosmic-speech-to-text";

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
        let api_key_input = cfg.api_key.clone();
        let app = AppModel {
            core,
            popup: None,
            state: AppState::Idle,
            config: cfg,
            api_key_input,
            status_text: "Ready".to_string(),
            recorder: Recorder::new(),
        };
        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let icon_name = match self.state {
            AppState::Idle => "audio-input-microphone-symbolic",
            AppState::Recording => "media-record-symbolic",
            AppState::Processing => "content-loading-symbolic",
        };

        self.core
            .applet
            .icon_button(icon_name)
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        // Record toggle button
        let record_label = match self.state {
            AppState::Idle => "Start Recording",
            AppState::Recording => "Stop Recording",
            AppState::Processing => "Processing...",
        };

        let record_icon = match self.state {
            AppState::Idle => "media-record-symbolic",
            AppState::Recording => "media-playback-stop-symbolic",
            AppState::Processing => "content-loading-symbolic",
        };

        let record_btn = cosmic::applet::menu_button(
            widget::row::with_children(vec![
                widget::icon::from_name(record_icon).size(16).icon().into(),
                widget::text(record_label).size(14).into(),
            ])
            .spacing(8),
        );

        let record_btn = if self.state != AppState::Processing {
            record_btn.on_press(Message::ToggleRecording)
        } else {
            record_btn
        };

        // API Key input
        let api_key_row = widget::column::with_children(vec![
            widget::text("Mistral API Key").size(12).into(),
            widget::text_input("Enter API key...", &self.api_key_input)
                .on_input(Message::ApiKeyChanged)
                .password()
                .size(14)
                .into(),
        ])
        .spacing(4);

        // Save button
        let save_btn = cosmic::applet::menu_button(
            widget::row::with_children(vec![
                widget::icon::from_name("document-save-symbolic")
                    .size(16)
                    .icon()
                    .into(),
                widget::text("Save API Key").size(14).into(),
            ])
            .spacing(8),
        )
        .on_press(Message::SaveConfig);

        // Hotkey display
        let hotkey_text =
            widget::text(format!("Hotkey: {}", self.config.hotkey)).size(12);

        // Status
        let status_text =
            widget::text(format!("Status: {}", self.status_text)).size(12);

        let content = widget::column::with_children(vec![
            record_btn.into(),
            widget::divider::horizontal::default().into(),
            api_key_row.into(),
            save_btn.into(),
            widget::divider::horizontal::default().into(),
            hotkey_text.into(),
            status_text.into(),
        ])
        .spacing(4);

        let cosmic = self.core.system_theme().cosmic();
        let pad = cosmic::iced::Padding::from([cosmic.space_xxs() as u16, cosmic.space_xs() as u16]);

        self.core
            .applet
            .popup_container(widget::container(content).padding(pad))
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::ToggleRecording => {
                match self.state {
                    AppState::Idle => {
                        if self.config.api_key.is_empty() {
                            self.status_text = "Set API key first!".to_string();
                            return Task::none();
                        }
                        self.recorder.start_recording();
                        self.state = AppState::Recording;
                        self.status_text = "Recording...".to_string();
                    }
                    AppState::Recording => {
                        self.state = AppState::Processing;
                        self.status_text = "Transcribing...".to_string();

                        let wav_path = self.recorder.stop_recording();
                        let api_key = self.config.api_key.clone();

                        return Task::perform(
                            async move {
                                let Some(wav) = wav_path else {
                                    return Err("No audio file recorded".to_string());
                                };

                                let mp3 = audio::convert_to_mp3(&wav)
                                    .map_err(|e| {
                                        audio::cleanup_temp_files(&[&wav]);
                                        e
                                    })?;

                                let result =
                                    transcribe::transcribe_mistral(&mp3, &api_key).await;

                                audio::cleanup_temp_files(&[&wav, &mp3]);
                                result
                            },
                            |result| cosmic::Action::App(Message::TranscriptionDone(result)),
                        );
                    }
                    AppState::Processing => {}
                }
            }
            Message::ApiKeyChanged(key) => {
                self.api_key_input = key;
            }
            Message::SaveConfig => {
                self.config.api_key = self.api_key_input.clone();
                config::save_config(&self.config);
                self.status_text = "API key saved".to_string();
            }
            Message::TranscriptionDone(result) => {
                match result {
                    Ok(text) => {
                        self.status_text = format!("Done: {}...", &text[..text.len().min(30)]);
                        if let Err(e) = paste::paste_text(&text) {
                            eprintln!("Paste error: {e}");
                            self.status_text = format!("Transcribed but paste failed: {e}");
                        }
                    }
                    Err(e) => {
                        self.status_text = format!("Error: {e}");
                        eprintln!("Transcription error: {e}");
                    }
                }
                self.state = AppState::Idle;
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
