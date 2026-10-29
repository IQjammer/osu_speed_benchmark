use crossterm::event::KeyCode


#[derive(Default)]
pub struct Keys {
    pub KeyPrimary: KeyCode::char
    pub KeySecondary: KeyCode::char
}


impl Keys {
    pub fn selection(&mut self, key: KeyEvent) {
        

    }
}
