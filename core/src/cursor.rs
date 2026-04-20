pub struct Cursor {
    pub position: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { position: 0 }
    }

    pub fn move_to(&mut self, position: usize) {
        self.position = position;
    }

    pub fn move_forward(&mut self, amount: usize) {
        self.position += amount;
    }

    pub fn move_backward(&mut self, amount: usize) {
        self.position = self.position.saturating_sub(amount);
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
