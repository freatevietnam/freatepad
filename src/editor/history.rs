pub struct History {
    states: Vec<String>,
    current_index: usize,
    max_states: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            current_index: 0,
            max_states: 100,
        }
    }

    pub fn push(&mut self, state: String) {
        // Remove any redo states
        self.states.truncate(self.current_index);

        // Add new state
        self.states.push(state);

        // Limit history size
        if self.states.len() > self.max_states {
            self.states.remove(0);
        } else {
            self.current_index += 1;
        }
    }

    pub fn undo(&mut self) -> Option<String> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(self.states[self.current_index].clone())
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<String> {
        if self.current_index < self.states.len() - 1 {
            self.current_index += 1;
            Some(self.states[self.current_index].clone())
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.states.clear();
        self.current_index = 0;
    }
}
