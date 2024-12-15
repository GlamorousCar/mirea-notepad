use std::path::PathBuf;
use std::ptr::copy;
// use iced::futures::TryFutureExt;
use iced::widget::text_editor;
use crate::history::History;


pub struct EditorState {
    content: text_editor::Content,
    current_file: Option<PathBuf>,
    is_modified: bool,
    history: History,
}


impl EditorState {
    pub fn new() -> Self {
        let content = text_editor::Content::new();
        // Create history with a reference to content
        let history = History::new(&content);

        Self {
            content,
            current_file: None,
            is_modified: false,
            history,
        }
    }

    pub fn content(&self) -> &text_editor::Content {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut text_editor::Content {
        self.is_modified = true;
        &mut self.content
    }

    pub fn current_file_name(&self) -> Option<String> {
        self.current_file
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .map(String::from)
    }

    pub fn set_content(&mut self, content: String, path: PathBuf) {
        self.content = text_editor::Content::with(&content);
        self.current_file = Some(path);
        self.is_modified = false;
        self.history = History::new(&self.content);
    }

    pub fn get_content(&self) -> String {
        self.content.text()
    }

    pub fn new_file(&mut self) {
        self.content = text_editor::Content::new();
        self.current_file = None;
        self.is_modified = false;
        self.history = History::new(&self.content);
    }

    pub fn current_file(&self) -> Option<&PathBuf> {
        self.current_file.as_ref()
    }

    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = Some(path);
    }

    pub fn record_change(&mut self) {
        self.history.push(&self.content);
    }

    pub fn undo(&mut self) -> bool {
        if let Some(text) = self.history.undo() {
            self.content = text_editor::Content::with(&text);
            self.is_modified = true;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(text) = self.history.redo() {
            self.content = text_editor::Content::with(&text);
            self.is_modified = true;
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }
}