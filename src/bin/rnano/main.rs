
mod editor;
mod config;
mod terminal;
//reexport
pub use terminal::Terminal;

use std::process;

use rtools::parser::parse_args;
fn main() {
    let editor_config : config::EditorConfig = parse_args();

    //errors in editor initialization are propagated upwards.
    let mut editor = editor::Editor::build(editor_config).unwrap_or_else(|error| {
        eprintln!("There was an error in initiating the editor : {}", error);
        process::exit(1);
    });

    //errors in editor operation are handled within the run function itself (shutdown).
    editor.run_editor();
}