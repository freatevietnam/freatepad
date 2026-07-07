use eframe::egui;

use crate::app::ViewMode;

pub struct Toolbar;

impl Toolbar {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub fn show(&self, ui: &mut egui::Ui, view_mode: &mut ViewMode) {
        ui.horizontal(|ui| {
            // View mode buttons
            ui.label("View:");

            if ui
                .selectable_label(*view_mode == ViewMode::Plain, "Plain")
                .clicked()
            {
                *view_mode = ViewMode::Plain;
            }

            if ui
                .selectable_label(*view_mode == ViewMode::Markdown, "Markdown")
                .clicked()
            {
                *view_mode = ViewMode::Markdown;
            }

            if ui
                .selectable_label(*view_mode == ViewMode::Split, "Split")
                .clicked()
            {
                *view_mode = ViewMode::Split;
            }

            ui.separator();

            // Quick actions
            if ui.button("📋 Copy").clicked() {
                // TODO: Copy
            }

            if ui.button("📄 Paste").clicked() {
                // TODO: Paste
            }

            ui.separator();

            // Search
            if ui.button("🔍 Find").clicked() {
                // TODO: Find
            }
        });
    }
}
