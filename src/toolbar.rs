use crate::editor::Message;
use iced::widget::{button, row, Row};

pub fn create_toolbar<'a>() -> Row<'a, Message> {
    row![
        button("New").on_press(Message::New),
        button("Open").on_press(Message::Open),
        button("Save").on_press(Message::Save),
        button("Undo").on_press(Message::Undo),
        button("Redo").on_press(Message::Redo),
    ]
    .spacing(5)
    .padding(10)
}
