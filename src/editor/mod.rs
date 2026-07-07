mod history;

use egui::TextEdit;
use history::History;

pub struct Editor {
    content: String,
    history: History,
    show_find: bool,
    show_replace: bool,
    find_text: String,
    replace_text: String,
    font_size: f32,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            content: String::new(),
            history: History::new(),
            show_find: false,
            show_replace: false,
            find_text: String::new(),
            replace_text: String::new(),
            font_size: 14.0,
        }
    }
}

impl Editor {
    pub fn show(&mut self, ui: &mut egui::Ui, is_modified: &mut bool) {
        let font_size = self.font_size;
        let response = ui.add(
            TextEdit::multiline(&mut self.content)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_width(f32::INFINITY)
                .lock_focus(true)
                .layouter(&mut |ui: &egui::Ui, string: &dyn egui::TextBuffer, _wrap_width| {
                    let format = egui::text::TextFormat::simple(
                        egui::FontId::new(font_size, egui::FontFamily::Monospace),
                        ui.visuals().text_color(),
                    );
                    let job = egui::text::LayoutJob::single_section(
                        string.as_str().to_string(),
                        format,
                    );
                    ui.ctx().fonts_mut(|f| f.layout_job(job))
                }),
        );

        if response.changed() {
            *is_modified = true;
            self.history.push(self.content.clone());
        }

        // Find and Replace dialog
        if self.show_find || self.show_replace {
            self.show_find_dialog(ui);
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.history.clear();
        self.history.push(self.content.clone());
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.history.clear();
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.history.undo() {
            self.content = prev;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.history.redo() {
            self.content = next;
        }
    }

    pub fn toggle_find(&mut self) {
        self.show_find = !self.show_find;
        self.show_replace = false;
    }

    pub fn toggle_replace(&mut self) {
        self.show_replace = !self.show_replace;
        self.show_find = self.show_replace;
    }

    pub fn wrap_selection(&mut self, before: &str, after: &str) {
        // Simple wrapping - in a real implementation, this would use selection
        self.content = format!("{}{}{}", before, self.content, after);
    }

    fn show_find_dialog(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Find and Replace")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    ui.text_edit_singleline(&mut self.find_text);
                });

                if self.show_replace {
                    ui.horizontal(|ui| {
                        ui.label("Replace:");
                        ui.text_edit_singleline(&mut self.replace_text);
                    });
                }

                ui.horizontal(|ui| {
                    if ui.button("Find Next").clicked() {
                        self.find_next();
                    }

                    if self.show_replace {
                        if ui.button("Replace").clicked() {
                            self.replace_current();
                        }
                        if ui.button("Replace All").clicked() {
                            self.replace_all();
                        }
                    }

                    if ui.button("Close").clicked() {
                        self.show_find = false;
                        self.show_replace = false;
                    }
                });
            });
    }

    fn find_next(&self) {
        // TODO: Implement find next
    }

    fn replace_current(&mut self) {
        if let Some(pos) = self.content.find(&self.find_text) {
            let end = pos + self.find_text.len();
            self.content.replace_range(pos..end, &self.replace_text);
        }
    }

    fn replace_all(&mut self) {
        self.content = self.content.replace(&self.find_text, &self.replace_text);
    }
}
