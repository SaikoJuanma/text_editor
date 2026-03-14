mod app;

use crate::app::TextEditor;

fn main() -> eframe::Result<()> {
    let app_name: &str = "Text Editor";
    eframe::run_native(
        &app_name,
        Default::default(),
        Box::new(|_cc| Ok(Box::new(TextEditor::new()))),
    )
}
