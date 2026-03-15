use std::fs;
use std::path::PathBuf;

const OUR_COMMAND: &str = "cosmic-speech-to-text --toggle";

fn shortcuts_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home)
        .join(".config/cosmic/com.system76.CosmicSettings.Shortcuts/v1/custom")
}

/// A parsed shortcut entry from the COSMIC RON config.
#[derive(Debug, Clone)]
struct Entry {
    modifiers: Vec<String>,
    key: String,
    description: Option<String>,
    command: String,
}

/// Read and parse the COSMIC custom shortcuts file.
fn read_entries() -> Vec<Entry> {
    let content = match fs::read_to_string(shortcuts_path()) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut entries = Vec::new();

    // Split on ): Spawn( to get each entry block
    // Each entry looks like:
    //   (modifiers: [...], key: "x", description: Some/None): Spawn("cmd"),
    let blocks: Vec<&str> = content.split("Spawn(\"").collect();

    for i in 0..blocks.len().saturating_sub(1) {
        let before = blocks[i]; // contains the key definition
        let after = blocks[i + 1]; // starts with command"...

        // Extract command from after
        let command = match after.find('"') {
            Some(end) => after[..end].to_string(),
            None => continue,
        };

        // Extract key definition from before - find last '(' that starts the entry
        let key_block = before;

        // Extract modifiers
        let modifiers = if let (Some(start), Some(end)) = (key_block.rfind('['), key_block.rfind(']'))
        {
            if start < end {
                key_block[start + 1..end]
                    .split(',')
                    .map(|m| m.trim().to_string())
                    .filter(|m| !m.is_empty())
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Extract key
        let key = extract_between(key_block, "key: \"", "\"").unwrap_or_default();
        if key.is_empty() {
            continue;
        }

        // Extract description
        let description = extract_between(key_block, "Some(\"", "\")");

        entries.push(Entry {
            modifiers,
            key,
            description,
            command,
        });
    }

    entries
}

fn extract_between(text: &str, start_marker: &str, end_marker: &str) -> Option<String> {
    let start = text.rfind(start_marker)? + start_marker.len();
    let rest = &text[start..];
    let end = rest.find(end_marker)?;
    Some(rest[..end].to_string())
}

/// Write entries back as COSMIC RON format.
fn write_entries(entries: &[Entry]) -> Result<(), String> {
    let path = shortcuts_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut ron = String::from("{\n");
    for entry in entries {
        ron.push_str("    (\n");
        ron.push_str("        modifiers: [\n");
        for m in &entry.modifiers {
            ron.push_str(&format!("            {},\n", m));
        }
        ron.push_str("        ],\n");
        ron.push_str(&format!("        key: \"{}\",\n", entry.key));
        match &entry.description {
            Some(desc) => ron.push_str(&format!("        description: Some(\"{}\"),\n", desc)),
            None => ron.push_str("        description: None,\n"),
        }
        ron.push_str(&format!("    ): Spawn(\"{}\"),\n", entry.command));
    }
    ron.push('}');

    fs::write(&path, &ron).map_err(|e| format!("Failed to write shortcuts: {e}"))
}

fn format_entry(entry: &Entry) -> String {
    let mut parts: Vec<String> = entry.modifiers.clone();
    parts.push(entry.key.to_uppercase());
    parts.join("+")
}

/// Parse "Ctrl+Shift+Y" into (["Ctrl", "Shift"], "y")
fn parse_hotkey(hotkey: &str) -> Result<(Vec<String>, String), String> {
    let parts: Vec<&str> = hotkey.split('+').map(|s| s.trim()).collect();
    if parts.len() < 2 {
        return Err("Need at least modifier+key (e.g. Ctrl+Y)".to_string());
    }
    let key = parts.last().unwrap().to_lowercase();
    let modifiers: Vec<String> = parts[..parts.len() - 1]
        .iter()
        .map(|m| match m.to_lowercase().as_str() {
            "ctrl" | "control" => "Ctrl".to_string(),
            "shift" => "Shift".to_string(),
            "alt" => "Alt".to_string(),
            "super" | "logo" | "win" => "Super".to_string(),
            other => {
                let mut c = other.chars();
                match c.next() {
                    Some(first) => first.to_uppercase().to_string() + c.as_str(),
                    None => other.to_string(),
                }
            }
        })
        .collect();
    Ok((modifiers, key))
}

// --- Public API ---

/// Find our shortcut if registered. Returns display string like "Ctrl+Y".
pub fn find_our_shortcut() -> Option<String> {
    let entries = read_entries();
    entries
        .iter()
        .find(|e| e.command == OUR_COMMAND)
        .map(format_entry)
}

/// Check if a hotkey combo is already used by something else.
/// Returns the description/command of the conflicting shortcut.
pub fn check_conflict(hotkey: &str) -> Option<String> {
    let (mods, key) = parse_hotkey(hotkey).ok()?;
    let entries = read_entries();
    let mods_lower: Vec<String> = mods.iter().map(|m| m.to_lowercase()).collect();

    for entry in &entries {
        if entry.command == OUR_COMMAND {
            continue;
        }
        let entry_mods: Vec<String> = entry.modifiers.iter().map(|m| m.to_lowercase()).collect();
        if entry_mods == mods_lower && entry.key.to_lowercase() == key {
            return Some(
                entry
                    .description
                    .clone()
                    .unwrap_or_else(|| entry.command.clone()),
            );
        }
    }
    None
}

/// Register or update our shortcut in COSMIC system shortcuts.
pub fn set_shortcut(hotkey: &str) -> Result<(), String> {
    let (modifiers, key) = parse_hotkey(hotkey)?;
    let mut entries = read_entries();

    // Remove any existing entry for our command
    entries.retain(|e| e.command != OUR_COMMAND);

    // Add new entry
    entries.push(Entry {
        modifiers,
        key,
        description: Some("Speech to Text".to_string()),
        command: OUR_COMMAND.to_string(),
    });

    write_entries(&entries)
}
