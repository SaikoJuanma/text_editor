mod app;
mod files;

use crate::app::TextEditor;
use crate::files::open_or_make_file;

fn main() -> eframe::Result<()> {
    let app_name: &str = "Text Editor";
    let args: Vec<String> = std::env::args().collect();

    let path = args.get(1).map(|s| s.as_str());
    let content = open_or_make_file(path).expect("Failed to open or create file");
    let editor = TextEditor::from_content(content);

    eframe::run_native(
        app_name,
        Default::default(),
        Box::new(|_cc| Ok(Box::new(editor))),
    )
}
