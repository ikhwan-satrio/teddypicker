use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct OpenWithApp {
    pub name: String,
    pub exec: String,
    pub path: String,
}

fn extension_to_mime(ext: &str) -> &str {
    match ext.to_lowercase().as_str() {
        "txt" | "md" | "log" | "csv" | "conf" | "ini" | "cfg" => "text/plain",
        "html" | "htm" | "xhtml" => "text/html",
        "css" | "scss" | "less" => "text/css",
        "js" | "ts" | "tsx" | "jsx" | "mjs" | "cjs" => "text/javascript",
        "json" | "jsonc" => "application/json",
        "xml" | "xsl" | "xslt" => "application/xml",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",
        "mp4" => "video/mp4",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        "webm" => "video/webm",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" | "oga" => "audio/ogg",
        "flac" => "audio/flac",
        "aac" => "audio/aac",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" | "tgz" => "application/gzip",
        "7z" => "application/x-7z-compressed",
        "rar" => "application/vnd.rar",
        "xz" => "application/x-xz",
        "py" => "text/x-python",
        "rs" => "text/x-rust",
        "go" => "text/x-go",
        "java" => "text/x-java",
        "c" | "h" => "text/x-csrc",
        "cpp" | "cxx" | "cc" | "hpp" => "text/x-c++src",
        "toml" => "text/x-toml",
        "yaml" | "yml" => "application/x-yaml",
        "sh" | "bash" | "zsh" => "application/x-shellscript",
        _ => "application/octet-stream",
    }
}

fn find_desktop_dirs() -> Vec<String> {
    let mut dirs = vec!["/usr/share/applications".to_string()];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(format!("{}/.local/share/applications", home));
        dirs.push(format!("{}/.local/share/flatpak/exports/share/applications", home));
    }
    dirs.push("/var/lib/flatpak/exports/share/applications".to_string());
    dirs
}

fn parse_exec_template(exec: &str) -> String {
    // Strip field codes (%U, %F, %f, %u, etc.) for display
    let cleaned = exec
        .replace("%U", "")
        .replace("%u", "")
        .replace("%F", "")
        .replace("%f", "")
        .replace("%d", "")
        .replace("%D", "")
        .replace("%n", "")
        .replace("%N", "")
        .replace("%i", "")
        .replace("%c", "")
        .replace("%k", "")
        .replace("%m", "");
    cleaned.trim().to_string()
}

fn resolve_exec_template(exec: &str, file_path: &str) -> String {
    let escaped = shell_escape(file_path);

    if exec.contains("%f") {
        exec.replacen("%f", &escaped, 1)
    } else if exec.contains("%F") {
        exec.replacen("%F", &escaped, 1)
    } else if exec.contains("%u") {
        exec.replacen("%u", &escaped, 1)
    } else if exec.contains("%U") {
        exec.replacen("%U", &escaped, 1)
    } else {
        format!("{} {}", exec.trim(), escaped)
    }
}

fn shell_escape(s: &str) -> String {
    if s.contains(char::is_whitespace) || s.contains(['\'', '"', '\\', '$', '`']) {
        format!("'{}'", s.replace('\'', "'\\''"))
    } else {
        s.to_string()
    }
}

#[derive(Default)]
struct DesktopEntry {
    name: String,
    exec: String,
    mime_types: Vec<String>,
    no_display: bool,
    terminal: bool,
}

fn parse_desktop_file(path: &str) -> Option<DesktopEntry> {
    let content = fs::read_to_string(path).ok()?;
    let mut entry = DesktopEntry::default();
    let mut in_desktop = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[Desktop Entry]" {
            in_desktop = true;
            continue;
        }

        if !in_desktop || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Stop at next section
        if trimmed.starts_with('[') {
            break;
        }

        if let Some(val) = trimmed.strip_prefix("Name=") {
            if entry.name.is_empty() {
                entry.name = val.to_string();
            }
        } else if let Some(val) = trimmed.strip_prefix("Exec=") {
            entry.exec = val.to_string();
        } else if let Some(val) = trimmed.strip_prefix("MimeType=") {
            entry.mime_types = val.split(';').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        } else if let Some(val) = trimmed.strip_prefix("NoDisplay=") {
            entry.no_display = val.trim() == "true";
        } else if let Some(val) = trimmed.strip_prefix("Terminal=") {
            entry.terminal = val.trim() == "true";
        }
    }

    if entry.name.is_empty() || entry.exec.is_empty() {
        return None;
    }

    Some(entry)
}

/// Scan .desktop files in all known application directories.
fn scan_desktop_files() -> Vec<DesktopEntry> {
    let mut apps = Vec::new();
    let mut seen_names: std::collections::HashSet<String> = std::collections::HashSet::new();

    let dirs = find_desktop_dirs();
    for dir_path in dirs {
        let dir = Path::new(&dir_path);
        if !dir.is_dir() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) != Some("desktop") {
                    continue;
                }
                if let Some(de) = parse_desktop_file(&p.to_string_lossy()) {
                    if de.no_display || de.terminal {
                        continue;
                    }
                    // Deduplicate by name (user dir takes priority)
                    let key = de.name.to_lowercase();
                    if !seen_names.contains(&key) {
                        seen_names.insert(key);
                        apps.push(de);
                    }
                }
            }
        }
    }

    apps
}

pub fn find_apps_for_file(path: &str) -> Vec<OpenWithApp> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let mime = extension_to_mime(ext);

    let all_apps = scan_desktop_files();
    let mut matched: Vec<OpenWithApp> = all_apps
        .into_iter()
        .filter(|a| a.mime_types.iter().any(|mt| mt == mime))
        .map(|a| OpenWithApp {
            name: a.name,
            exec: parse_exec_template(&a.exec),
            path: a.exec, // store raw exec template for later execution
        })
        .collect();

    matched.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    matched
}

pub fn open_file_with(path: &str, exec_template: &str) -> Result<(), String> {
    let cmd = resolve_exec_template(exec_template, path);
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    let program = parts[0];
    let args = &parts[1..];

    std::process::Command::new(program)
        .args(args)
        .spawn()
        .map_err(|e| format!("Failed to launch app: {}", e))?;

    Ok(())
}
