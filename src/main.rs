mod app;

use crate::app::TextEditor;

fn main() -> eframe::Result<()> {
    let app_name: &str = "Text Editor";
    let args: Vec<String> = std::env::args().collect();

    let editor = if args.len() > 1 {
        match TextEditor::from_file(&args[1]) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Could not open file: {}", err);
                std::process::exit(1);
            }
        }
    } else {
        TextEditor::new()
    };

    eframe::run_native(
        app_name,
        Default::default(),
        Box::new(|_cc| Ok(Box::new(editor))),
    )
}
