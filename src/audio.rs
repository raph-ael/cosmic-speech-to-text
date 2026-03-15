use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavWriter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct Recorder {
    recording: Arc<Mutex<bool>>,
    audio_file: Arc<Mutex<Option<PathBuf>>>,
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            recording: Arc::new(Mutex::new(false)),
            audio_file: Arc::new(Mutex::new(None)),
        }
    }

    pub fn is_recording(&self) -> bool {
        *self.recording.lock().unwrap()
    }

    pub fn start_recording(&self) {
        let mut rec = self.recording.lock().unwrap();
        if *rec {
            return;
        }
        *rec = true;
        drop(rec);

        let recording = Arc::clone(&self.recording);
        let audio_file = Arc::clone(&self.audio_file);

        std::thread::spawn(move || {
            if let Err(e) = record_audio(recording, audio_file) {
                eprintln!("Recording error: {e}");
            }
        });
    }

    pub fn stop_recording(&self) -> Option<PathBuf> {
        {
            let mut rec = self.recording.lock().unwrap();
            if !*rec {
                return None;
            }
            *rec = false;
        }
        // Give the recording thread time to finalize
        std::thread::sleep(std::time::Duration::from_millis(200));
        self.audio_file.lock().unwrap().take()
    }
}

fn record_audio(
    recording: Arc<Mutex<bool>>,
    audio_file: Arc<Mutex<Option<PathBuf>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or("No input device available")?;

    eprintln!("Using audio device: {}", device.name().unwrap_or_default());

    let supported_config = device.default_input_config()?;
    let sample_rate = supported_config.sample_rate().0;
    let channels = supported_config.channels() as u16;
    let stream_config: cpal::StreamConfig = supported_config.into();

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("cosmic_stt_{}.wav", Uuid::new_v4()));

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = Arc::new(Mutex::new(Some(WavWriter::create(&file_path, spec)?)));
    let writer_clone = Arc::clone(&writer);

    let stream = device.build_input_stream(
        &stream_config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if let Ok(mut guard) = writer_clone.lock() {
                if let Some(w) = guard.as_mut() {
                    if channels >= 2 {
                        for chunk in data.chunks_exact(channels as usize) {
                            let mono = chunk.iter().sum::<f32>() / channels as f32;
                            let _ = w.write_sample((mono * i16::MAX as f32) as i16);
                        }
                    } else {
                        for &sample in data {
                            let _ = w.write_sample((sample * i16::MAX as f32) as i16);
                        }
                    }
                }
            }
        },
        |err| eprintln!("Stream error: {err}"),
        None,
    )?;

    stream.play()?;

    while *recording.lock().unwrap() {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    drop(stream);

    if let Ok(mut guard) = writer.lock() {
        if let Some(w) = guard.take() {
            w.finalize()?;
        }
    }

    *audio_file.lock().unwrap() = Some(file_path);
    Ok(())
}

pub fn convert_to_mp3(wav_path: &PathBuf) -> Result<PathBuf, String> {
    let mp3_path = wav_path.with_extension("mp3");
    let status = std::process::Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(wav_path)
        .args(["-b:a", "64k"])
        .arg(&mp3_path)
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("Failed to run ffmpeg: {e}"))?;

    if !status.success() {
        return Err("ffmpeg conversion failed".into());
    }
    Ok(mp3_path)
}

pub fn cleanup_temp_files(paths: &[&PathBuf]) {
    for path in paths {
        if path.exists() {
            let _ = std::fs::remove_file(path);
        }
    }
}
