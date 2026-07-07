use eframe::egui;

use crate::app::ViewMode;
use crate::file::FileHandle;

pub struct StatusBar;

impl StatusBar {
    pub fn new() -> Self {
        Self
    }

    pub fn show(
        &self,
        ui: &mut egui::Ui,
        view_mode: &ViewMode,
        word_count: usize,
        char_count: usize,
        file_handle: &FileHandle,
        is_modified: bool,
    ) {
        ui.horizontal(|ui| {
            // View mode
            let mode_text = match view_mode {
                ViewMode::Plain => "Plain",
                ViewMode::Markdown => "Markdown",
                ViewMode::Split => "Split",
            };
            ui.label(format!("View: {}", mode_text));

            ui.separator();

            // Word and character count
            ui.label(format!("Words: {} | Chars: {}", word_count, char_count));

            ui.separator();

            // File path
            let path_text = if is_modified {
                format!("*{}", file_handle.filename())
            } else {
                file_handle.filename().to_string()
            };
            ui.label(path_text);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // File status
                if is_modified {
                    ui.label("● Modified");
                } else {
                    ui.label("○ Saved");
                }
            });
        });
    }
}
