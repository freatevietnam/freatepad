use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const MAX_RECENT_FILES: usize = 10;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecentFiles {
    files: Vec<PathBuf>,
}

impl RecentFiles {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("freatepad");

        let recent_file = config_dir.join("recent.json");

        if recent_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&recent_file) {
                if let Ok(recent) = serde_json::from_str(&content) {
                    return recent;
                }
            }
        }

        Self::default()
    }

    pub fn save(&self) {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("freatepad");

        if let Err(e) = std::fs::create_dir_all(&config_dir) {
            log::error!("Failed to create config directory: {}", e);
            return;
        }

        let recent_file = config_dir.join("recent.json");

        if let Ok(content) = serde_json::to_string_pretty(self) {
            if let Err(e) = std::fs::write(&recent_file, content) {
                log::error!("Failed to save recent files: {}", e);
            }
        }
    }

    pub fn add(&mut self, path: PathBuf) {
        // Remove if already exists
        self.files.retain(|p| p != &path);

        // Add to front
        self.files.insert(0, path);

        // Limit size
        self.files.truncate(MAX_RECENT_FILES);

        self.save();
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }
}
