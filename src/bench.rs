use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BenchmarkState {
    pub is_active: bool,
    pub target_clicks: u32,
    pub current_clicks: u32,
    pub timestamps: Vec<Instant>,
    pub last_key: Option<char>,
}

impl Default for BenchmarkState {
    fn default() -> Self {
        Self {
            is_active: false,
            target_clicks: 20,
            current_clicks: 0,
            timestamps: Vec::new(),
            last_key: None,
        }
    }
}

impl BenchmarkState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.current_clicks = 0;
        self.timestamps.clear();
        self.last_key = None;
    }

    pub fn register_click(&mut self, key: char) {
        if !self.is_active {
            return;
        }

        self.current_clicks += 1;
        self.timestamps.push(Instant::now());
        self.last_key = Some(key);

        if self.current_clicks >= self.target_clicks {
            self.is_active = false;
        }
    }

    pub fn result(&mut self) {
        let mut dt_i = Vec::with_capacity(19);
        let mut bpm_i = Vec::with_capacity(19);
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.current_clicks = 0;
        self.timestamps.clear();
        self.last_key = None;
    }
}
