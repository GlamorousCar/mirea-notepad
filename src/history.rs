use iced::widget::text_editor::Content;

#[derive(Clone)]
struct HistoryState {
    text: String,
    cursor_position: Option<usize>,
}

impl HistoryState {
    fn new(content: &Content) -> Self {
        Self {
            text: content.text(),
            cursor_position: None, // TODO: Implement cursor position tracking
        }
    }
}

pub struct History {
    states: Vec<HistoryState>,
    current_index: usize,
    max_history: usize,
}

impl History {
    pub fn new(initial_content: &Content) -> Self {
        Self {
            states: vec![HistoryState::new(initial_content)],
            current_index: 0,
            max_history: 100, // Maximum number of states to keep
        }
    }

    pub fn push(&mut self, content: &Content) {
        // Remove any future states if we're not at the latest state
        if self.current_index < self.states.len() - 1 {
            self.states.truncate(self.current_index + 1);
        }

        // Add new state
        self.states.push(HistoryState::new(content));
        self.current_index += 1;

        // Remove oldest states if we exceed max_history
        if self.states.len() > self.max_history {
            self.states.remove(0);
            self.current_index -= 1;
        }
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_index < self.states.len() - 1
    }

    pub fn undo(&mut self) -> Option<String> {
        if self.can_undo() {
            self.current_index -= 1;
            Some(self.states[self.current_index].text.clone())
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<String> {
        if self.can_redo() {
            self.current_index += 1;
            Some(self.states[self.current_index].text.clone())
        } else {
            None
        }
    }
}