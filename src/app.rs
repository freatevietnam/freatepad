use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::editor::Editor;
use crate::file::{FileHandle, RecentFiles};
use crate::settings::Settings;
use crate::ui::StatusBar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    Plain,
    Markdown,
    #[default]
    Split,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuAction {
    None,
    NewFile,
    OpenFile,
    SaveFile,
    SaveAs,
    Undo,
    Redo,
    Find,
    Replace,
    Bold,
    Italic,
    SetViewMode(ViewMode),
    ZoomIn,
    ZoomOut,
    Exit,
}

pub struct FreatePad {
    #[allow(dead_code)]
    settings: Settings,
    editor: Editor,
    markdown_cache: CommonMarkCache,
    file_handle: Arc<Mutex<FileHandle>>,
    recent_files: RecentFiles,
    status_bar: StatusBar,
    view_mode: ViewMode,
    split_ratio: f32,
    pub show_status_bar: bool,
    word_count: usize,
    char_count: usize,
    currentzoom: f32,
    is_modified: bool,
}

impl Default for FreatePad {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            editor: Editor::default(),
            markdown_cache: CommonMarkCache::default(),
            file_handle: Arc::new(Mutex::new(FileHandle::default())),
            recent_files: RecentFiles::load(),
            status_bar: StatusBar::new(),
            view_mode: ViewMode::Split,
            split_ratio: 0.5,
            show_status_bar: true,
            word_count: 0,
            char_count: 0,
            currentzoom: 1.0,
            is_modified: false,
        }
    }
}

impl FreatePad {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            settings: Settings::load(),
            ..Default::default()
        };

        // Handle command line arguments
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            let path = PathBuf::from(&args[1]);
            if path.exists() && path.extension().is_some_and(|ext| ext == "md") {
                app.open_file(&path);
            }
        }

        app
    }

    // Public methods for menu access
    pub fn new_file(&mut self) {
        self.editor.clear();
        self.file_handle.lock().unwrap().clear();
        self.is_modified = false;
    }

    pub fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "txt"])
            .pick_file()
        {
            self.open_file(&path);
        }
    }

    pub fn open_file(&mut self, path: &PathBuf) {
        match std::fs::read(path) {
            Ok(bytes) => {
                let content = decode_with_bom_handling(&bytes);
                self.editor.set_content(content);
                self.file_handle.lock().unwrap().set_path(path.clone());
                self.recent_files.add(path.clone());
                self.is_modified = false;
                self.update_counts();
            }
            Err(e) => {
                log::error!("Failed to open file: {}", e);
            }
        }
    }

    pub fn save_file(&mut self) {
        let path = self.file_handle.lock().unwrap().path().cloned();
        if let Some(path) = path {
            let content = self.editor.get_content();
            if let Err(e) = std::fs::write(&path, content) {
                log::error!("Failed to save file: {}", e);
            } else {
                self.is_modified = false;
            }
        }
    }

    pub fn save_file_as_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md"])
            .save_file()
        {
            self.save_file_as(path);
        }
    }

    pub fn save_file_as(&mut self, path: PathBuf) {
        let content = self.editor.get_content();
        if let Err(e) = std::fs::write(&path, &content) {
            log::error!("Failed to save file: {}", e);
        } else {
            self.file_handle.lock().unwrap().set_path(path.clone());
            self.recent_files.add(path);
            self.is_modified = false;
        }
    }

    pub fn undo(&mut self) {
        self.editor.undo();
    }

    pub fn redo(&mut self) {
        self.editor.redo();
    }

    pub fn toggle_find(&mut self) {
        self.editor.toggle_find();
    }

    pub fn toggle_replace(&mut self) {
        self.editor.toggle_replace();
    }

    pub fn wrap_selection(&mut self, before: &str, after: &str) {
        self.editor.wrap_selection(before, after);
    }

    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }

    pub fn zoom_in(&mut self) {
        self.currentzoom = (self.currentzoom + 0.1).min(3.0);
    }

    pub fn zoom_out(&mut self) {
        self.currentzoom = (self.currentzoom - 0.1).max(0.3);
    }

    pub fn get_recent_files(&self) -> Vec<PathBuf> {
        self.recent_files.files().to_vec()
    }

    fn build_menu(&self, ui: &mut egui::Ui) -> MenuAction {
        let mut action = MenuAction::None;

        // File menu
        ui.menu_button("File", |ui| {
            if ui
                .add(egui::Button::new("New").shortcut_text("Ctrl+N"))
                .clicked()
            {
                action = MenuAction::NewFile;
                ui.close();
            }

            if ui
                .add(egui::Button::new("Open...").shortcut_text("Ctrl+O"))
                .clicked()
            {
                action = MenuAction::OpenFile;
                ui.close();
            }

            ui.separator();

            // Recent files
            let recent = self.recent_files.files().to_vec();
            if !recent.is_empty() {
                ui.menu_button("Recent Files", |ui| {
                    for path in &recent {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown");
                        if ui.button(name).clicked() {
                            action = MenuAction::OpenFile;
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
                action = MenuAction::SaveFile;
                ui.close();
            }

            if ui
                .add(
                    egui::Button::new("Save As...").shortcut_text("Ctrl+Shift+S"),
                )
                .clicked()
            {
                action = MenuAction::SaveAs;
                ui.close();
            }

            ui.separator();

            if ui.button("Exit").clicked() {
                action = MenuAction::Exit;
                ui.close();
            }
        });

        // Edit menu
        ui.menu_button("Edit", |ui| {
            if ui
                .add(egui::Button::new("Undo").shortcut_text("Ctrl+Z"))
                .clicked()
            {
                action = MenuAction::Undo;
                ui.close();
            }

            if ui
                .add(
                    egui::Button::new("Redo").shortcut_text("Ctrl+Shift+Z"),
                )
                .clicked()
            {
                action = MenuAction::Redo;
                ui.close();
            }

            ui.separator();

            if ui
                .add(egui::Button::new("Find...").shortcut_text("Ctrl+F"))
                .clicked()
            {
                action = MenuAction::Find;
                ui.close();
            }

            if ui
                .add(egui::Button::new("Replace...").shortcut_text("Ctrl+H"))
                .clicked()
            {
                action = MenuAction::Replace;
                ui.close();
            }

            ui.separator();

            if ui
                .add(egui::Button::new("Bold").shortcut_text("Ctrl+B"))
                .clicked()
            {
                action = MenuAction::Bold;
                ui.close();
            }

            if ui
                .add(egui::Button::new("Italic").shortcut_text("Ctrl+I"))
                .clicked()
            {
                action = MenuAction::Italic;
                ui.close();
            }
        });

        // View menu
        ui.menu_button("View", |ui| {
            if ui
                .add(egui::Button::new("Plain Mode").shortcut_text("Ctrl+1"))
                .clicked()
            {
                action = MenuAction::SetViewMode(ViewMode::Plain);
                ui.close();
            }

            if ui
                .add(
                    egui::Button::new("Markdown Mode").shortcut_text("Ctrl+2"),
                )
                .clicked()
            {
                action = MenuAction::SetViewMode(ViewMode::Markdown);
                ui.close();
            }

            if ui
                .add(egui::Button::new("Split Mode").shortcut_text("Ctrl+3"))
                .clicked()
            {
                action = MenuAction::SetViewMode(ViewMode::Split);
                ui.close();
            }

            ui.separator();

            if ui.button("Zoom In").clicked() {
                action = MenuAction::ZoomIn;
                ui.close();
            }

            if ui.button("Zoom Out").clicked() {
                action = MenuAction::ZoomOut;
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

        action
    }

    fn update_counts(&mut self) {
        let content = self.editor.get_content();
        self.char_count = content.chars().count();
        self.word_count = content.split_whitespace().count();
    }

    fn show_split_view(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let min_panel_width = 100.0;
        let divider_width = 4.0;
        let total_width = available.x - divider_width;

        let left_width = (total_width * self.split_ratio).max(min_panel_width);
        let right_width = total_width - left_width;

        let (left_width, right_width) = if left_width < min_panel_width {
            (min_panel_width, total_width - min_panel_width)
        } else if right_width < min_panel_width {
            (total_width - min_panel_width, min_panel_width)
        } else {
            (left_width, right_width)
        };

        ui.allocate_ui_with_layout(
            egui::vec2(available.x, available.y),
            egui::Layout::left_to_right(egui::Align::TOP),
            |ui| {
                ui.allocate_ui(egui::vec2(left_width, available.y), |ui| {
                    ui.set_min_size(egui::vec2(left_width, available.y));
                    self.editor.show(ui, &mut self.is_modified);
                    self.update_counts();
                });

                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(divider_width, available.y),
                    egui::Sense::click_and_drag(),
                );

                let divider_color = if response.hovered() || response.dragged() {
                    ui.visuals().strong_text_color()
                } else {
                    ui.visuals().weak_text_color()
                };
                ui.painter().rect_filled(rect, 0.0, divider_color);

                if response.dragged() {
                    let drag_delta = response.drag_delta().x;
                    self.split_ratio =
                        ((self.split_ratio * total_width + drag_delta) / total_width)
                            .clamp(0.1, 0.9);
                }

                ui.allocate_ui(egui::vec2(right_width, available.y), |ui| {
                    ui.set_min_size(egui::vec2(right_width, available.y));
                    self.render_preview(ui);
                });
            },
        );
    }

    fn render_preview(&mut self, ui: &mut egui::Ui) {
        let content = self.editor.get_content();

        CommonMarkViewer::new()
            .show(ui, &mut self.markdown_cache, &content);
    }

    fn handle_keyboard_shortcuts(&mut self, ui: &egui::Ui) {
        let input = ui.input(|i| i.clone());

        // Ctrl+O - Open file
        if input.modifiers.ctrl && input.key_pressed(egui::Key::O) {
            self.open_file_dialog();
        }

        // Ctrl+S - Save
        if input.modifiers.ctrl && !input.modifiers.shift && input.key_pressed(egui::Key::S) {
            self.save_file();
        }

        // Ctrl+Shift+S - Save As
        if input.modifiers.ctrl && input.modifiers.shift && input.key_pressed(egui::Key::S) {
            self.save_file_as_dialog();
        }

        // Ctrl+1 - Plain mode
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num1) {
            self.view_mode = ViewMode::Plain;
        }

        // Ctrl+2 - Markdown mode
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num2) {
            self.view_mode = ViewMode::Markdown;
        }

        // Ctrl+3 - Split mode
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num3) {
            self.view_mode = ViewMode::Split;
        }

        // Ctrl+Z - Undo
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Z) && !input.modifiers.shift {
            self.editor.undo();
        }

        // Ctrl+Shift+Z or Ctrl+Y - Redo
        if (input.modifiers.ctrl && input.modifiers.shift && input.key_pressed(egui::Key::Z))
            || (input.modifiers.ctrl && input.key_pressed(egui::Key::Y))
        {
            self.editor.redo();
        }

        // Ctrl+F - Find
        if input.modifiers.ctrl && input.key_pressed(egui::Key::F) {
            self.editor.toggle_find();
        }

        // Ctrl+H - Replace
        if input.modifiers.ctrl && input.key_pressed(egui::Key::H) {
            self.editor.toggle_replace();
        }

        // Ctrl+B - Bold
        if input.modifiers.ctrl && input.key_pressed(egui::Key::B) {
            self.editor.wrap_selection("**", "**");
        }

        // Ctrl+I - Italic
        if input.modifiers.ctrl && input.key_pressed(egui::Key::I) {
            self.editor.wrap_selection("*", "*");
        }

        // Ctrl+= - Zoom in
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Equals) {
            self.currentzoom = (self.currentzoom + 0.1).min(3.0);
        }

        // Ctrl+- - Zoom out
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Minus) {
            self.currentzoom = (self.currentzoom - 0.1).max(0.3);
        }
    }
}

impl eframe::App for FreatePad {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_keyboard_shortcuts(ui);

        // Menu bar at the top - use Panel for proper spanning
        egui::Panel::top("menu_bar").show(ui, |ui| {
            let mut action = MenuAction::None;
            egui::MenuBar::new().ui(ui, |ui| {
                action = self.build_menu(ui);
            });
            // Execute menu action
            match action {
                MenuAction::NewFile => self.new_file(),
                MenuAction::OpenFile => self.open_file_dialog(),
                MenuAction::SaveFile => self.save_file(),
                MenuAction::SaveAs => self.save_file_as_dialog(),
                MenuAction::Undo => self.undo(),
                MenuAction::Redo => self.redo(),
                MenuAction::Find => self.toggle_find(),
                MenuAction::Replace => self.toggle_replace(),
                MenuAction::Bold => self.wrap_selection("**", "**"),
                MenuAction::Italic => self.wrap_selection("*", "*"),
                MenuAction::SetViewMode(mode) => self.set_view_mode(mode),
                MenuAction::ZoomIn => self.zoom_in(),
                MenuAction::ZoomOut => self.zoom_out(),
                MenuAction::Exit => std::process::exit(0),
                MenuAction::None => {}
            }
        });

        // Status bar at the bottom - use Panel for proper spanning
        if self.show_status_bar {
            egui::Panel::bottom("status_bar").show(ui, |ui| {
                self.status_bar.show(
                    ui,
                    &self.view_mode,
                    self.word_count,
                    self.char_count,
                    &self.file_handle.lock().unwrap(),
                    self.is_modified,
                );
            });
        }

        // Main content area - use CentralPanel which fills remaining space
        egui::CentralPanel::default().show(ui, |ui| {
            match self.view_mode {
                ViewMode::Plain => {
                    self.editor.show(ui, &mut self.is_modified);
                    self.update_counts();
                }
                ViewMode::Markdown => {
                    self.render_preview(ui);
                }
                ViewMode::Split => {
                    self.show_split_view(ui);
                }
            }
        });

        // Handle drag and drop
        handle_drag_and_drop(ui, self);
    }
}

fn handle_drag_and_drop(ui: &egui::Ui, app: &mut FreatePad) {
    let hovered_files = ui.input(|i| i.raw.hovered_files.clone());
    let dropped_files = ui.input(|i| i.raw.dropped_files.clone());

    if !dropped_files.is_empty() {
        for file in dropped_files {
            if let Some(path) = file.path {
                if path.extension().is_some_and(|ext| ext == "md") {
                    app.open_file(&path);
                }
            }
        }
    }

    // Show drop indicator
    if !hovered_files.is_empty() {
        if let Some(rect) = ui.input(|i| i.raw.screen_rect) {
            ui.ctx().layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("drop_target"),
            ))
            .rect_filled(rect, 0.0, egui::Color32::from_black_alpha(80));
        }
    }
}

/// BOM (Byte Order Mark) constants
const BOM_UTF8: [u8; 3] = [0xEF, 0xBB, 0xBF];
const BOM_UTF16_LE: [u8; 2] = [0xFF, 0xFE];
const BOM_UTF16_BE: [u8; 2] = [0xFE, 0xFF];

/// Decode bytes to string with BOM handling
fn decode_with_bom_handling(bytes: &[u8]) -> String {
    // Check for BOM and decode accordingly
    if bytes.starts_with(&BOM_UTF8) {
        // UTF-8 with BOM - strip BOM
        String::from_utf8_lossy(&bytes[3..]).to_string()
    } else if bytes.starts_with(&BOM_UTF16_LE) {
        // UTF-16 LE with BOM
        decode_utf16_le(&bytes[2..])
    } else if bytes.starts_with(&BOM_UTF16_BE) {
        // UTF-16 BE with BOM
        decode_utf16_be(&bytes[2..])
    } else if bytes.starts_with(&BOM_UTF16_LE) {
        // UTF-16 LE without BOM (heuristic: starts with null bytes)
        decode_utf16_le(bytes)
    } else if bytes.starts_with(&BOM_UTF16_BE) {
        // UTF-16 BE without BOM (heuristic: starts with null bytes)
        decode_utf16_be(bytes)
    } else {
        // Try UTF-8 first, then fall back to lossy conversion
        match std::str::from_utf8(bytes) {
            Ok(s) => s.to_string(),
            Err(_) => String::from_utf8_lossy(bytes).to_string(),
        }
    }
}

/// Decode UTF-16 LE bytes to String
fn decode_utf16_le(bytes: &[u8]) -> String {
    let chunks: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    String::from_utf16_lossy(&chunks)
}

/// Decode UTF-16 BE bytes to String
fn decode_utf16_be(bytes: &[u8]) -> String {
    let chunks: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .collect();
    String::from_utf16_lossy(&chunks)
}
