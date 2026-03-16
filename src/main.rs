mod app;
mod files;

use crate::app::TextEditor;
use crate::files::open_or_make_file;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let app_name: &str = "Text Editor";
    let args: Vec<String> = std::env::args().collect();

    let editor = if let Some(path) = args.get(1).map(|s| s.as_str()) {
        let content = open_or_make_file(Some(path)).expect("Failed to open or create file");
        TextEditor::from_file(path.to_string(), content)
    } else {
        //BUG: this deletes the previous untitled file, if untitled exists (1) (2) etc
        let content = open_or_make_file(None).expect("Failed to open or create file");
        TextEditor::from_file("untitled.txt".to_string(), content)
    };

    eframe::run_native(
        app_name,
        Default::default(),
        Box::new(|_cc| Ok(Box::new(editor))),
    )
}
