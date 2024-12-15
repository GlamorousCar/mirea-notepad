use crate::editor::Message;
use iced::widget::{button, row, Row};

pub fn create_toolbar<'a>(can_undo: bool, can_redo: bool) -> Row<'a, Message> {
    row![
        button("New").on_press(Message::New),
        button("Open").on_press(Message::Open),
        button("Save").on_press(Message::Save),
        button("Undo")
            .on_press(Message::Undo)
            .style(if can_undo { iced::theme::Button::Primary } else { iced::theme::Button::Secondary }),
        button("Redo")
            .on_press(Message::Redo)
            .style(if can_redo { iced::theme::Button::Primary } else { iced::theme::Button::Secondary }),
    ]
    .spacing(5)
    .padding(10)
}
