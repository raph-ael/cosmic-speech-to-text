use std::process::Command;

pub fn paste_text(text: &str) -> Result<(), String> {
    Command::new("wtype")
        .args(["-s", "300"])
        .arg(text)
        .status()
        .map_err(|e| format!("Failed to run wtype: {e}"))?;

    Ok(())
}
