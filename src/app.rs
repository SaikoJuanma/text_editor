pub struct TextEditor {
    content: String,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            content: String::new(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        Ok(TextEditor { content })
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
    use std::io::Write;

    #[test]
    fn new_creates_empty_content() {
        let editor = TextEditor::new();
        assert!(editor.content.is_empty());
    }

    #[test]
    fn default_equals_new() {
        let a = TextEditor::new();
        let b = TextEditor::default();
        assert_eq!(a.content, b.content);
    }

    #[test]
    fn from_file_loads_content() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "hello from file").unwrap();
        let editor = TextEditor::from_file(file.path().to_str().unwrap()).unwrap();
        assert_eq!(editor.content, "hello from file");
    }

    #[test]
    fn from_file_returns_error_for_missing_file() {
        let result = TextEditor::from_file("/nonexistent/path/file.txt");
        assert!(result.is_err());
    }
}
