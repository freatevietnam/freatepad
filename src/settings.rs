use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    pub editor: EditorSettings,
    pub preview: PreviewSettings,
    pub general: GeneralSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_size: f32,
    pub font_family: String,
    pub tab_size: u32,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub auto_save: bool,
    pub auto_save_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewSettings {
    pub zoom: f32,
    pub render_math: bool,
    pub render_code: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub theme: Theme,
    pub remember_window_size: bool,
    pub remember_recent_files: bool,
    pub max_recent_files: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            font_family: "Monospace".to_string(),
            tab_size: 4,
            word_wrap: true,
            line_numbers: true,
            auto_save: true,
            auto_save_interval: 60,
        }
    }
}

impl Default for PreviewSettings {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            render_math: true,
            render_code: true,
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            remember_window_size: true,
            remember_recent_files: true,
            max_recent_files: 10,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("freatepad");

        let settings_file = config_dir.join("settings.toml");

        if settings_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&settings_file) {
                if let Ok(settings) = toml::from_str(&content) {
                    return settings;
                }
            }
        }

        Self::default()
    }
}
