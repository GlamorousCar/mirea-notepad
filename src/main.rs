mod editor;
mod state;
mod file_handler;
mod toolbar;

use iced::{Sandbox, Settings};
use editor::{Editor};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}