use crate::files::save_file;
use log::error;

pub struct TextEditor {
    content: String,
    file_path: Option<String>,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            content: String::new(),
            file_path: None,
        }
    }

    pub fn from_file(file_path: String, content: String) -> Self {
        TextEditor {
            content,
            file_path: Some(file_path),
        }
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl)
            && let Some(path) = &self.file_path
            && let Err(e) = save_file(path, &self.content)
        {
            error!("Failed to save file {}", e);
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.content),
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_content_and_no_path() {
        let editor = TextEditor::new();
        assert!(editor.content.is_empty());
        assert!(editor.file_path.is_none());
    }

    #[test]
    fn default_equals_new() {
        let a = TextEditor::new();
        let b = TextEditor::default();
        assert_eq!(a.content, b.content);
        assert_eq!(a.file_path, b.file_path);
    }

    #[test]
    fn from_file_stores_content_and_path() {
        let editor = TextEditor::from_file("foo.txt".to_string(), "bar".to_string());
        assert_eq!(editor.content, "bar");
        assert_eq!(editor.file_path, Some("foo.txt".to_string()));
    }
}
