pub struct Buffer {
    content: String,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn from_str(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn insert(&mut self, offset: usize, text: &str) {
        if offset > self.content.len() {
            self.content.push_str(text);
        } else {
            self.content.insert_str(offset, text);
        }
    }

    pub fn delete(&mut self, start: usize, end: usize) {
        let start = start.min(self.content.len());
        let end = end.min(self.content.len());
        if start < end {
            self.content.replace_range(start..end, "");
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
