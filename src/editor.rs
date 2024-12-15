use iced::widget::{ container, text_editor, Column};
use iced::{Element, Length, Sandbox};



use crate::state::EditorState;
use crate::file_handler::{FileHandler, show_open_dialog, show_save_dialog};
use crate::toolbar::create_toolbar;

pub struct Editor {
    state: EditorState,
}


#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    New,
    Open,
    Save,
    Undo,
    Redo,
}



impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: EditorState::new(),
        }
    }

    fn title(&self) -> String {
        let modified = if self.state.is_modified() { "*" } else { "" };
        format!("{}{} - Mirea notepad by John Drof",
                modified,
                self.state.current_file_name().unwrap_or("Untitled".to_string()))
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.state.content_mut().edit(action);
                self.state.record_change();
            }
            Message::New => {
                self.state.new_file();
            }
            Message::Open => {
                if let Some(path) = show_open_dialog() {
                    match FileHandler::open_file(path.clone()) {
                        Ok(content) => {
                            self.state.set_content(content, path);
                        }
                        Err(err) => {
                            // TODO: Show error dialog
                            println!("Error opening file: {}", err);
                        }
                    }
                }
            }
            Message::Save => {
                let path = if let Some(current_path) = self.state.current_file() {
                    Some(current_path.clone())
                } else {
                    show_save_dialog()
                };

                if let Some(path) = path {
                    match FileHandler::save_file(&self.state.get_content(), path.clone()) {
                        Ok(_) => {
                            self.state.set_current_file(path);
                        }
                        Err(err) => {
                            // TODO: Show error dialog
                            println!("Error saving file: {}", err);
                        }
                    }
                }
            }
            Message::Undo => {
                self.state.undo();
            }
            Message::Redo => {
                self.state.redo();
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let toolbar = create_toolbar(self.state.can_undo(), self.state.can_redo());

        let editor = text_editor(self.state.content()).on_edit(Message::Edit);

        Column::new()
            .push(toolbar)
            .push(
                container(editor)
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .into()
    }


}
