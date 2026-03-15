use std::io::Write;
use std::process::{Command, Stdio};

pub fn paste_text(text: &str) -> Result<(), String> {
    // 1. Copy text to clipboard via wl-copy
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run wl-copy: {e}"))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| format!("Failed to write to wl-copy: {e}"))?;
    }
    child.wait().map_err(|e| format!("wl-copy failed: {e}"))?;

    // 2. Small delay to let clipboard settle
    std::thread::sleep(std::time::Duration::from_millis(100));

    // 3. Simulate Ctrl+V via wtype
    Command::new("wtype")
        .args(["-M", "ctrl", "v", "-m", "ctrl"])
        .status()
        .map_err(|e| format!("Failed to run wtype: {e}"))?;

    Ok(())
}
