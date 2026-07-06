use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub extension_type: ExtensionType,
    pub entry: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub contributes: ExtensionContributes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionType {
    Theme,
    Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionContributes {
    #[serde(default)]
    pub preview_handlers: Vec<String>,
    #[serde(default)]
    pub context_menu_actions: Vec<ContextMenuAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMenuAction {
    pub label: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtensionInfo {
    pub manifest: ExtensionManifest,
    pub path: PathBuf,
    pub enabled: bool,
}

pub struct ExtensionManager {
    extensions_dir: PathBuf,
    extensions: Vec<ExtensionInfo>,
}

impl ExtensionManager {
    pub fn new(app_handle: &AppHandle) -> Self {
        let extensions_dir = app_handle
            .path()
            .app_data_dir()
            .expect("failed to get app data dir")
            .join("extensions");

        // Create extensions directory if it doesn't exist
        let _ = fs::create_dir_all(&extensions_dir);

        let mut manager = Self {
            extensions_dir,
            extensions: Vec::new(),
        };

        manager.scan_extensions();
        manager
    }

    pub fn scan_extensions(&mut self) {
        self.extensions.clear();

        if !self.extensions_dir.exists() {
            return;
        }

        for entry in WalkDir::new(&self.extensions_dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == "manifest.json" {
                let extension_dir = entry.path().parent().unwrap().to_path_buf();

                match self.load_manifest(entry.path()) {
                    Ok(manifest) => {
                        self.extensions.push(ExtensionInfo {
                            manifest,
                            path: extension_dir,
                            enabled: true,
                        });
                    }
                    Err(e) => {
                        log::error!("Failed to load extension manifest: {}", e);
                    }
                }
            }
        }
    }

    fn load_manifest(&self, path: &std::path::Path) -> Result<ExtensionManifest, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn get_extensions(&self) -> &[ExtensionInfo] {
        &self.extensions
    }

    pub fn get_themes(&self) -> Vec<&ExtensionInfo> {
        self.extensions
            .iter()
            .filter(|e| e.manifest.extension_type == ExtensionType::Theme && e.enabled)
            .collect()
    }

    pub fn get_plugins(&self) -> Vec<&ExtensionInfo> {
        self.extensions
            .iter()
            .filter(|e| e.manifest.extension_type == ExtensionType::Plugin && e.enabled)
            .collect()
    }

    pub fn get_extension_by_id(&self, id: &str) -> Option<&ExtensionInfo> {
        self.extensions.iter().find(|e| e.manifest.id == id)
    }

    pub fn enable_extension(&mut self, id: &str) -> bool {
        if let Some(ext) = self.extensions.iter_mut().find(|e| e.manifest.id == id) {
            ext.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_extension(&mut self, id: &str) -> bool {
        if let Some(ext) = self.extensions.iter_mut().find(|e| e.manifest.id == id) {
            ext.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn get_theme_css(&self, theme_id: &str) -> Result<String, String> {
        let ext = self
            .get_extension_by_id(theme_id)
            .ok_or_else(|| format!("Theme '{}' not found", theme_id))?;

        if ext.manifest.extension_type != ExtensionType::Theme {
            return Err("Extension is not a theme".to_string());
        }

        let css_path = ext.path.join(&ext.manifest.entry);
        fs::read_to_string(&css_path).map_err(|e| e.to_string())
    }

    pub fn get_plugin_js(&self, plugin_id: &str) -> Result<String, String> {
        let ext = self
            .get_extension_by_id(plugin_id)
            .ok_or_else(|| format!("Plugin '{}' not found", plugin_id))?;

        if ext.manifest.extension_type != ExtensionType::Plugin {
            return Err("Extension is not a plugin".to_string());
        }

        let js_path = ext.path.join(&ext.manifest.entry);
        fs::read_to_string(&js_path).map_err(|e| e.to_string())
    }
}
