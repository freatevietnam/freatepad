mod recent;
mod associations;

pub use recent::RecentFiles;
pub use associations::FileAssociations;

#[derive(Debug, Clone, Default)]
pub struct FileHandle {
    path: Option<std::path::PathBuf>,
    is_new: bool,
}

impl FileHandle {
    pub fn set_path(&mut self, path: std::path::PathBuf) {
        self.path = Some(path);
        self.is_new = false;
    }

    pub fn path(&self) -> Option<&std::path::PathBuf> {
        self.path.as_ref()
    }

    pub fn clear(&mut self) {
        self.path = None;
        self.is_new = true;
    }

    pub fn filename(&self) -> &str {
        self.path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
    }
}
