use crate::audio::{self, Recorder};
use crate::config;
use crate::deps;
use crate::fl;
use crate::paste;
use crate::shortcut;
use crate::toggle;
use crate::transcribe;

use cosmic::iced::window::Id;
use cosmic::iced::Subscription;
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
    status_text: String,
    missing_deps: Vec<(&'static str, &'static str)>,
    recorder: Recorder,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    ToggleRecording,
    OpenSettings,
    CheckToggle,
    TranscriptionDone(Result<String, String>),
}

fn idle_status(missing: &[(&str, &str)], config: &config::Config) -> String {
    if !missing.is_empty() {
        deps::format_missing_i18n(missing)
    } else if !config.is_configured() {
        fl!("status-not-configured")
    } else {
        fl!("status-ready")
    }
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
        let mut cfg = config::load_config();

        // Sync config hotkey with actual registered shortcut
        if let Some(current) = shortcut::find_our_shortcut() {
            if cfg.hotkey != current {
                cfg.hotkey = current;
                config::save_config(&cfg);
            }
        }

        let missing = deps::check_missing();
        let status = idle_status(&missing, &cfg);
        let app = AppModel {
            core,
            popup: None,
            state: AppState::Idle,
            config: cfg,
            status_text: status,
            missing_deps: missing,
            recorder: Recorder::new(),
        };
        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(std::time::Duration::from_millis(50))
            .map(|_| Message::CheckToggle)
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
        let record_label = match self.state {
            AppState::Idle => fl!("start-recording"),
            AppState::Recording => fl!("stop-recording"),
            AppState::Processing => fl!("processing"),
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

        let settings_btn = cosmic::applet::menu_button(
            widget::row::with_children(vec![
                widget::icon::from_name("emblem-system-symbolic")
                    .size(16)
                    .icon()
                    .into(),
                widget::text(fl!("settings")).size(14).into(),
            ])
            .spacing(8),
        )
        .on_press(Message::OpenSettings);

        let hotkey_text =
            widget::text(fl!("hotkey", hotkey = self.config.hotkey.as_str())).size(12);

        let status_text =
            widget::text(fl!("status", status = self.status_text.as_str())).size(12);

        let mut items: Vec<Element<'_, Self::Message>> = Vec::new();

        if !self.missing_deps.is_empty() {
            let warning = widget::text(deps::format_missing_i18n(&self.missing_deps)).size(12);
            items.push(warning.into());
            items.push(widget::divider::horizontal::default().into());
        }

        items.push(record_btn.into());
        items.push(widget::divider::horizontal::default().into());
        items.push(settings_btn.into());
        items.push(widget::divider::horizontal::default().into());
        items.push(hotkey_text.into());
        items.push(status_text.into());

        let content = widget::column::with_children(items).spacing(4);

        let cosmic = self.core.system_theme().cosmic();
        let pad =
            cosmic::iced::Padding::from([cosmic.space_xxs() as u16, cosmic.space_xs() as u16]);

        self.core
            .applet
            .popup_container(widget::container(content).padding(pad))
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::CheckToggle => {
                if toggle::check_toggle() {
                    return self.update(Message::ToggleRecording);
                }
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    self.config = config::load_config();
                    self.missing_deps = deps::check_missing();
                    if self.state == AppState::Idle {
                        self.status_text =
                            idle_status(&self.missing_deps, &self.config);
                    }

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
            Message::OpenSettings => {
                let _ = std::process::Command::new("cosmic-speech-to-text")
                    .arg("--settings")
                    .spawn();
                if let Some(p) = self.popup.take() {
                    return destroy_popup(p);
                }
            }
            Message::ToggleRecording => match self.state {
                AppState::Idle => {
                    self.config = config::load_config();
                    let missing = deps::check_missing();
                    if !missing.is_empty() {
                        self.missing_deps = missing.clone();
                        self.status_text = deps::format_missing_i18n(&missing);
                        return Task::none();
                    }
                    if !self.config.is_configured() {
                        self.status_text = fl!("status-set-api-key");
                        return Task::none();
                    }
                    self.recorder.start_recording();
                    self.state = AppState::Recording;
                    self.status_text = fl!("status-recording");
                }
                AppState::Recording => {
                    self.state = AppState::Processing;
                    self.status_text = fl!("status-transcribing");

                    let wav_path = self.recorder.stop_recording();
                    let cfg = self.config.clone();

                    return Task::perform(
                        async move {
                            let Some(wav) = wav_path else {
                                return Err("No audio file recorded".to_string());
                            };

                            let mp3 = audio::convert_to_mp3(&wav).map_err(|e| {
                                audio::cleanup_temp_files(&[&wav]);
                                e
                            })?;

                            let result = transcribe::transcribe(
                                &mp3,
                                &wav,
                                &cfg.mode,
                                cfg.active_api_key(),
                                &cfg.whisper_cpp_path,
                                &cfg.whisper_model_path,
                            )
                            .await;

                            audio::cleanup_temp_files(&[&wav, &mp3]);
                            result
                        },
                        |result| cosmic::Action::App(Message::TranscriptionDone(result)),
                    );
                }
                AppState::Processing => {}
            },
            Message::TranscriptionDone(result) => {
                match result {
                    Ok(text) => {
                        let preview = &text[..text.len().min(30)];
                        self.status_text = fl!("status-done", text = preview);

                        let close_task = if let Some(p) = self.popup.take() {
                            Some(destroy_popup(p))
                        } else {
                            None
                        };

                        let text_clone = text.clone();
                        std::thread::spawn(move || {
                            std::thread::sleep(std::time::Duration::from_millis(300));
                            if let Err(e) = paste::paste_text(&text_clone) {
                                eprintln!("Paste error: {e}");
                            }
                        });

                        self.state = AppState::Idle;
                        if let Some(task) = close_task {
                            return task;
                        }
                        return Task::none();
                    }
                    Err(e) => {
                        self.status_text = fl!("status-error", error = e.as_str());
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
