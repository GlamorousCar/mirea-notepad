use std::path::PathBuf;
use iced::widget::text_editor;

pub struct EditorState {
    content: text_editor::Content,
    current_file: Option<PathBuf>,
    is_modified: bool,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
            current_file: None,
            is_modified: false,
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
    }

    pub fn get_content(&self) -> String {
        self.content.text()
    }

    pub fn new_file(&mut self) {
        self.content = text_editor::Content::new();
        self.current_file = None;
        self.is_modified = false;
    }

    pub fn current_file(&self) -> Option<&PathBuf> {
        self.current_file.as_ref()
    }

    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = Some(path);
    }
}