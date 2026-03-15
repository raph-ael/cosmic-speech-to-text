use std::path::PathBuf;

fn toggle_path() -> PathBuf {
    std::env::temp_dir().join("cosmic-stt-toggle")
}

/// Called by `--toggle` CLI: creates a flag file the applet watches for.
pub fn send_toggle() {
    let path = toggle_path();
    if let Err(e) = std::fs::write(&path, "toggle") {
        eprintln!("Failed to send toggle signal: {e}");
    }
}

/// Called by the applet subscription: checks and consumes the flag file.
pub fn check_toggle() -> bool {
    let path = toggle_path();
    if path.exists() {
        let _ = std::fs::remove_file(&path);
        true
    } else {
        false
    }
}
