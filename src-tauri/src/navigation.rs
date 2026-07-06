use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_NAME: &str = "pinned-dirs.json";
const PINNED_KEY: &str = "pinned";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdgDirs {
    pub home: String,
    pub desktop: Option<String>,
    pub documents: Option<String>,
    pub downloads: Option<String>,
    pub music: Option<String>,
    pub pictures: Option<String>,
    pub videos: Option<String>,
    pub templates: Option<String>,
    pub public_share: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedDir {
    pub name: String,
    pub path: String,
}

pub fn get_xdg_dirs() -> XdgDirs {
    XdgDirs {
        home: dirs::home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string()),
        desktop: dirs::desktop_dir().map(|p| p.to_string_lossy().to_string()),
        documents: dirs::document_dir().map(|p| p.to_string_lossy().to_string()),
        downloads: Some(
            dirs::download_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| {
                    dirs::home_dir()
                        .map(|h| h.join("Downloads").to_string_lossy().to_string())
                        .unwrap_or_else(|| "/tmp".to_string())
                }),
        ),
        music: dirs::audio_dir().map(|p| p.to_string_lossy().to_string()),
        pictures: dirs::picture_dir().map(|p| p.to_string_lossy().to_string()),
        videos: dirs::video_dir().map(|p| p.to_string_lossy().to_string()),
        templates: dirs::template_dir().map(|p| p.to_string_lossy().to_string()),
        public_share: dirs::public_dir().map(|p| p.to_string_lossy().to_string()),
    }
}

pub fn get_pinned_dirs(app_handle: &AppHandle) -> Vec<PinnedDir> {
    match app_handle.store(STORE_NAME) {
        Ok(store) => {
            if let Some(value) = store.get(PINNED_KEY) {
                serde_json::from_value(value.clone()).unwrap_or_default()
            } else {
                default_pinned_dirs()
            }
        }
        Err(_) => default_pinned_dirs(),
    }
}

pub fn save_pinned_dirs(app_handle: &AppHandle, dirs: &[PinnedDir]) -> Result<(), String> {
    let store = app_handle
        .store(STORE_NAME)
        .map_err(|e| e.to_string())?;
    let value = serde_json::to_value(dirs).map_err(|e| e.to_string())?;
    store.set(PINNED_KEY.to_string(), value);
    store.save().map_err(|e| e.to_string())
}

pub fn add_pinned_dir(app_handle: &AppHandle, name: String, path: String) -> Result<Vec<PinnedDir>, String> {
    let mut dirs = get_pinned_dirs(app_handle);

    if dirs.iter().any(|d| d.path == path) {
        return Ok(dirs);
    }

    dirs.push(PinnedDir { name, path });
    save_pinned_dirs(app_handle, &dirs)?;
    Ok(dirs)
}

pub fn remove_pinned_dir(app_handle: &AppHandle, path: &str) -> Result<Vec<PinnedDir>, String> {
    let mut dirs = get_pinned_dirs(app_handle);
    dirs.retain(|d| d.path != path);
    save_pinned_dirs(app_handle, &dirs)?;
    Ok(dirs)
}

pub fn reorder_pinned_dirs(
    app_handle: &AppHandle,
    dirs: Vec<PinnedDir>,
) -> Result<Vec<PinnedDir>, String> {
    save_pinned_dirs(app_handle, &dirs)?;
    Ok(dirs)
}

fn default_pinned_dirs() -> Vec<PinnedDir> {
    let home = dirs::home_dir().map(|p| p.to_string_lossy().to_string());

    match home {
        Some(h) => vec![
            PinnedDir {
                name: "Home".to_string(),
                path: h.clone(),
            },
            PinnedDir {
                name: "Desktop".to_string(),
                path: format!("{}/Desktop", h),
            },
            PinnedDir {
                name: "Documents".to_string(),
                path: format!("{}/Documents", h),
            },
            PinnedDir {
                name: "Downloads".to_string(),
                path: format!("{}/Downloads", h),
            },
        ],
        None => vec![],
    }
}
