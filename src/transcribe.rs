use std::path::PathBuf;

pub async fn transcribe_mistral(
    mp3_path: &PathBuf,
    api_key: &str,
) -> Result<String, String> {
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
        Err(format!("Mistral API error: {error_text}"))
    }
}
