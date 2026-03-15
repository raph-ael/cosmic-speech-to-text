use crate::config::TranscribeMode;
use std::path::PathBuf;

pub async fn transcribe(
    mp3_path: &PathBuf,
    wav_path: &PathBuf,
    mode: &TranscribeMode,
    api_key: &str,
    whisper_cpp_path: &str,
    whisper_model_path: &str,
) -> Result<String, String> {
    match mode {
        TranscribeMode::Mistral => transcribe_mistral(mp3_path, api_key).await,
        TranscribeMode::OpenAI => transcribe_openai(mp3_path, api_key).await,
        TranscribeMode::LocalWhisper => {
            transcribe_local(wav_path, whisper_cpp_path, whisper_model_path).await
        }
    }
}

async fn transcribe_mistral(mp3_path: &PathBuf, api_key: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let file_content =
        std::fs::read(mp3_path).map_err(|e| format!("Failed to read audio file: {e}"))?;

    let form = reqwest::multipart::Form::new()
        .text("model", "voxtral-mini-2507")
        .part(
            "file",
            reqwest::multipart::Part::bytes(file_content)
                .file_name("audio.mp3")
                .mime_str("audio/mpeg")
                .map_err(|e| format!("MIME error: {e}"))?,
        );

    let response = client
        .post("https://api.mistral.ai/v1/audio/transcriptions")
        .header("x-api-key", api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    parse_json_response(response, "Mistral").await
}

async fn transcribe_openai(mp3_path: &PathBuf, api_key: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let file_content =
        std::fs::read(mp3_path).map_err(|e| format!("Failed to read audio file: {e}"))?;

    let form = reqwest::multipart::Form::new()
        .text("model", "whisper-1")
        .part(
            "file",
            reqwest::multipart::Part::bytes(file_content)
                .file_name("audio.mp3")
                .mime_str("audio/mpeg")
                .map_err(|e| format!("MIME error: {e}"))?,
        );

    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {api_key}"))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    parse_json_response(response, "OpenAI").await
}

async fn parse_json_response(
    response: reqwest::Response,
    provider: &str,
) -> Result<String, String> {
    if response.status().is_success() {
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {e}"))?;
        Ok(json["text"].as_str().unwrap_or("").to_string())
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".into());
        Err(format!("{provider} API error: {error_text}"))
    }
}

async fn transcribe_local(
    wav_path: &PathBuf,
    whisper_cpp_path: &str,
    model_path: &str,
) -> Result<String, String> {
    // whisper.cpp needs 16kHz mono WAV — convert first
    let converted = wav_path.with_extension("16k.wav");
    let ffmpeg_status = std::process::Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(wav_path)
        .args(["-ar", "16000", "-ac", "1", "-c:a", "pcm_s16le"])
        .arg(&converted)
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("Failed to run ffmpeg: {e}"))?;

    if !ffmpeg_status.success() {
        return Err("ffmpeg conversion to 16kHz failed".into());
    }

    let output = std::process::Command::new(whisper_cpp_path)
        .args(["-m", model_path])
        .args(["-f", converted.to_str().unwrap_or("")])
        .args(["--no-timestamps"])
        .args(["--language", "auto"])
        .output()
        .map_err(|e| format!("Failed to run whisper.cpp: {e}"))?;

    // Clean up converted file
    let _ = std::fs::remove_file(&converted);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("whisper.cpp error: {stderr}"));
    }

    // whisper.cpp outputs to stdout
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Also check for .txt output file (some versions write there)
    if text.is_empty() {
        let txt_path = converted.with_extension("txt");
        if txt_path.exists() {
            let file_text = std::fs::read_to_string(&txt_path).unwrap_or_default();
            let _ = std::fs::remove_file(&txt_path);
            return Ok(file_text.trim().to_string());
        }
    }

    Ok(text)
}
