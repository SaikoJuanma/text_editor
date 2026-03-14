use eframe;
use egui;

pub struct TextEditor {
    content: String,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            content: String::new(),
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
}
