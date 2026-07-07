use eframe::egui;

use crate::app::FreatePad;
use crate::file::FileAssociations;

pub struct MenuBar;

impl MenuBar {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub fn show(&self, ui: &mut egui::Ui, app: &mut FreatePad) {
        egui::MenuBar::new().ui(ui, |ui| {
            // File menu
            ui.menu_button("File", |ui| {
                if ui
                    .add(egui::Button::new("New").shortcut_text("Ctrl+N"))
                    .clicked()
                {
                    app.new_file();
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Open...").shortcut_text("Ctrl+O"))
                    .clicked()
                {
                    app.open_file_dialog();
                    ui.close();
                }

                ui.separator();

                // Recent files
                let recent = app.get_recent_files();
                if !recent.is_empty() {
                    ui.menu_button("Recent Files", |ui| {
                        for path in &recent {
                            let name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown");
                            if ui.button(name).clicked() {
                                app.open_file(path);
                                ui.close();
                            }
                        }
                    });
                }

                ui.separator();

                if ui
                    .add(egui::Button::new("Save").shortcut_text("Ctrl+S"))
                    .clicked()
                {
                    app.save_file();
                    ui.close();
                }

                if ui
                    .add(
                        egui::Button::new("Save As...").shortcut_text("Ctrl+Shift+S"),
                    )
                    .clicked()
                {
                    app.save_file_as_dialog();
                    ui.close();
                }

                ui.separator();

                if ui.button("Export HTML...").clicked() {
                    // TODO: Export HTML
                    ui.close();
                }

                ui.separator();

                if ui.button("Install File Associations").clicked() {
                    if let Err(e) = FileAssociations::install() {
                        log::error!("Failed to install file associations: {}", e);
                    }
                    ui.close();
                }

                ui.separator();

                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });

            // Edit menu
            ui.menu_button("Edit", |ui| {
                if ui
                    .add(egui::Button::new("Undo").shortcut_text("Ctrl+Z"))
                    .clicked()
                {
                    app.undo();
                    ui.close();
                }

                if ui
                    .add(
                        egui::Button::new("Redo").shortcut_text("Ctrl+Shift+Z"),
                    )
                    .clicked()
                {
                    app.redo();
                    ui.close();
                }

                ui.separator();

                if ui
                    .add(egui::Button::new("Find...").shortcut_text("Ctrl+F"))
                    .clicked()
                {
                    app.toggle_find();
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Replace...").shortcut_text("Ctrl+H"))
                    .clicked()
                {
                    app.toggle_replace();
                    ui.close();
                }

                ui.separator();

                if ui
                    .add(egui::Button::new("Bold").shortcut_text("Ctrl+B"))
                    .clicked()
                {
                    app.wrap_selection("**", "**");
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Italic").shortcut_text("Ctrl+I"))
                    .clicked()
                {
                    app.wrap_selection("*", "*");
                    ui.close();
                }
            });

            // View menu
            ui.menu_button("View", |ui| {
                if ui
                    .add(egui::Button::new("Plain Mode").shortcut_text("Ctrl+1"))
                    .clicked()
                {
                    app.set_view_mode(crate::app::ViewMode::Plain);
                    ui.close();
                }

                if ui
                    .add(
                        egui::Button::new("Markdown Mode").shortcut_text("Ctrl+2"),
                    )
                    .clicked()
                {
                    app.set_view_mode(crate::app::ViewMode::Markdown);
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Split Mode").shortcut_text("Ctrl+3"))
                    .clicked()
                {
                    app.set_view_mode(crate::app::ViewMode::Split);
                    ui.close();
                }

                ui.separator();

                if ui.button("Zoom In").clicked() {
                    app.zoom_in();
                    ui.close();
                }

                if ui.button("Zoom Out").clicked() {
                    app.zoom_out();
                    ui.close();
                }

                ui.separator();

                if ui
                    .add(
                        egui::Checkbox::new(&mut app.show_status_bar, "Status Bar"),
                    )
                    .clicked()
                {
                    ui.close();
                }
            });

            // Help menu
            ui.menu_button("Help", |ui| {
                if ui.button("About FreatePad").clicked() {
                    // TODO: Show about dialog
                    ui.close();
                }

                if ui.button("Documentation").clicked() {
                    let _ = open::that("https://github.com/freatevietnam/freatepad");
                    ui.close();
                }
            });
        });
    }
}
