use cursive::traits::*;
use cursive::views::{Dialog, NamedView, OnEventView, SelectView};
use cursive::Cursive;
use std::io::Read;
use std::path;
use std::path::PathBuf;

mod cursive_actions;
mod fsys;

const FILE_NAME: &str = "names.txt";

fn main() -> std::io::Result<()> {
    let err_msg = format!("Error accessing {}", FILE_NAME);
    let mut file = fsys::get_file_handle(FILE_NAME).expect(&err_msg);
    let mut file_content: String = String::new();
    file.read_to_string(&mut file_content)?;

    let dialog = if file_content.trim().is_empty() {
        let path = PathBuf::from(FILE_NAME);
        let display_path = format!("{:?}", path::absolute(path)?).replace(r"\\", r"\");
        let err_msg = format!("Please first add one name per line to the following file and restart:\n{}", display_path);
        Dialog::text(err_msg).title("names.txt is empty").button("Ok", Cursive::quit)
    } else {
        let title = "Select next (top = recommended)";
        let file_lines: Vec<&str> = file_content.lines().filter(|l| !l.trim().is_empty()).collect();
        let view: NamedView<OnEventView<SelectView>> = cursive_actions::create_view(file_lines);
        Dialog::around(view.scrollable()).title(title)
            .button("Save [CTRL+S]", cursive_actions::save_cursive)
            .button("Quit [ESC]", Cursive::quit)
    };

    let mut siv = cursive::default();
    siv.add_layer(dialog);
    siv.run();

    Ok(())
}



