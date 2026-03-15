use directories::ProjectDirs;
use std::path::PathBuf;

pub struct WhisperModel {
    pub label: &'static str,
    pub size: &'static str,
    pub url: &'static str,
    pub filename: &'static str,
}

pub const MODELS: &[WhisperModel] = &[
    WhisperModel {
        label: "Tiny",
        size: "~75 MB",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
        filename: "ggml-tiny.bin",
    },
    WhisperModel {
        label: "Base",
        size: "~142 MB",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
        filename: "ggml-base.bin",
    },
    WhisperModel {
        label: "Small",
        size: "~466 MB",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
        filename: "ggml-small.bin",
    },
    WhisperModel {
        label: "Medium",
        size: "~1.5 GB",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
        filename: "ggml-medium.bin",
    },
    WhisperModel {
        label: "Large v3",
        size: "~2.9 GB",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3.bin",
        filename: "ggml-large-v3.bin",
    },
];

pub fn models_dir() -> PathBuf {
    let dirs = ProjectDirs::from("", "", "cosmic-speech-to-text")
        .expect("Could not determine data dir");
    dirs.data_dir().join("models")
}

pub fn model_path(model: &WhisperModel) -> PathBuf {
    models_dir().join(model.filename)
}

pub fn is_downloaded(model: &WhisperModel) -> bool {
    let path = model_path(model);
    path.exists() && path.metadata().map(|m| m.len() > 1000).unwrap_or(false)
}

pub fn model_labels() -> Vec<String> {
    MODELS
        .iter()
        .map(|m| {
            let status = if is_downloaded(m) { " ✓" } else { "" };
            format!("{} ({}){}", m.label, m.size, status)
        })
        .collect()
}

pub fn find_model_index(model_path_str: &str) -> Option<usize> {
    MODELS
        .iter()
        .position(|m| model_path_str.contains(m.filename))
}

pub fn whisper_cpp_bin_path() -> PathBuf {
    models_dir().join("whisper-cpp")
}

pub fn is_whisper_cpp_installed() -> bool {
    let path = whisper_cpp_bin_path();
    path.exists() && path.metadata().map(|m| m.len() > 1000).unwrap_or(false)
}

/// Build whisper.cpp from source and install the binary.
pub async fn install_whisper_cpp() -> Result<PathBuf, String> {
    let dir = models_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create dir: {e}"))?;

    let src_dir = dir.join("whisper.cpp-src");

    // Clone or update
    if src_dir.join("CMakeLists.txt").exists() {
        tokio::process::Command::new("git")
            .args(["pull"])
            .current_dir(&src_dir)
            .status()
            .await
            .map_err(|e| format!("git pull failed: {e}"))?;
    } else {
        let _ = std::fs::remove_dir_all(&src_dir);
        let status = tokio::process::Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "https://github.com/ggerganov/whisper.cpp.git",
            ])
            .arg(&src_dir)
            .status()
            .await
            .map_err(|e| format!("git clone failed: {e}"))?;
        if !status.success() {
            return Err("git clone failed".into());
        }
    }

    // Build
    let build_dir = src_dir.join("build");
    let _ = std::fs::create_dir_all(&build_dir);

    let status = tokio::process::Command::new("cmake")
        .args(["..", "-DCMAKE_BUILD_TYPE=Release"])
        .current_dir(&build_dir)
        .status()
        .await
        .map_err(|e| format!("cmake failed: {e}"))?;
    if !status.success() {
        return Err("cmake configure failed".into());
    }

    let status = tokio::process::Command::new("cmake")
        .args(["--build", ".", "--config", "Release", "-j"])
        .current_dir(&build_dir)
        .status()
        .await
        .map_err(|e| format!("cmake build failed: {e}"))?;
    if !status.success() {
        return Err("cmake build failed".into());
    }

    // Find the built binary
    let built = build_dir.join("bin").join("whisper-cli");
    let built_alt = build_dir.join("bin").join("main");

    let source = if built.exists() {
        built
    } else if built_alt.exists() {
        built_alt
    } else {
        return Err("Could not find built whisper binary".into());
    };

    // Copy to our models dir
    let dest = whisper_cpp_bin_path();
    std::fs::copy(&source, &dest)
        .map_err(|e| format!("Failed to copy binary: {e}"))?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755));
    }

    Ok(dest)
}

/// Download a model using curl (runs as external process for progress).
/// Returns the path to the downloaded file.
pub async fn download_model(model_index: usize) -> Result<PathBuf, String> {
    let model = MODELS
        .get(model_index)
        .ok_or("Invalid model index")?;

    let dir = models_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create models dir: {e}"))?;

    let dest = model_path(model);

    // Use curl for download (shows progress in logs, handles redirects)
    let status = tokio::process::Command::new("curl")
        .args(["-L", "-o"])
        .arg(&dest)
        .arg(model.url)
        .status()
        .await
        .map_err(|e| format!("Failed to run curl: {e}"))?;

    if !status.success() {
        let _ = std::fs::remove_file(&dest);
        return Err("Download failed".to_string());
    }

    // Verify file is not empty/error page
    let size = std::fs::metadata(&dest)
        .map(|m| m.len())
        .unwrap_or(0);

    if size < 10_000 {
        let _ = std::fs::remove_file(&dest);
        return Err("Downloaded file too small — check URL".to_string());
    }

    Ok(dest)
}
