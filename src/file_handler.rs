use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;

pub struct FileHandler;

impl FileHandler {
    pub fn save_file(content: &str, path: PathBuf) -> std::io::Result<()> {
        fs::write(path, content)
    }

    pub fn open_file(path: PathBuf) -> std::io::Result<String> {
        fs::read_to_string(path)
    }
}

pub fn show_open_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Text Files", &["txt"])
        .add_filter("All Files", &["*"])
        .pick_file()
}

pub fn show_save_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Text Files", &["txt"])
        .add_filter("All Files", &["*"])
        .save_file()
}