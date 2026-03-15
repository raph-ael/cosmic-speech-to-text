use std::process::Command;

const REQUIRED: &[(&str, &str)] = &[
    ("wl-copy", "wl-clipboard"),
    ("wtype", "wtype"),
    ("ffmpeg", "ffmpeg"),
];

pub fn check_missing() -> Vec<(&'static str, &'static str)> {
    REQUIRED
        .iter()
        .filter(|(cmd, _)| Command::new("which").arg(cmd).output().map_or(true, |o| !o.status.success()))
        .copied()
        .collect()
}

pub fn format_missing_i18n(missing: &[(&str, &str)]) -> String {
    use crate::fl;
    let cmds: Vec<&str> = missing.iter().map(|(cmd, _)| *cmd).collect();
    let pkgs: Vec<&str> = missing.iter().map(|(_, pkg)| *pkg).collect();
    let cmds_str = cmds.join(", ");
    let pkgs_str = pkgs.join(" ");
    fl!("missing-deps-summary",
        cmds = cmds_str.as_str(),
        packages = pkgs_str.as_str()
    )
}
