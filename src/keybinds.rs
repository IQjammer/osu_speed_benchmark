use crossterm::event::KeyCode;

#[derive(Default, PartialEq, Eq)]
pub enum BindState {
    #[default]
    Idle,
    WaitingFirst,
    WaitingSecond,
}

pub struct Keys {
    pub key_primary: char,
    pub key_secondary: char,
    pub state: BindState,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            key_primary: 'a',
            key_secondary: 's',
            state: BindState::Idle,
        }
    }
}

impl Keys {
    pub fn selection(&mut self) {
        self.state = BindState::WaitingFirst;
    }

    pub fn is_rebinding(&self) -> bool {
        self.state != BindState::Idle
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        if let KeyCode::Char(c) = code {
            match self.state {
                BindState::WaitingFirst => {
                    self.key_primary = c;
                    self.state = BindState::WaitingSecond;
                }
                BindState::WaitingSecond => {
                    self.key_secondary = c;
                    self.state = BindState::Idle;
                }
                BindState::Idle => {}
            }
        }
    }
}
