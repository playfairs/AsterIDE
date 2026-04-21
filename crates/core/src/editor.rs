use crate::buffer::Buffer;
use crate::cursor::Cursor;

pub struct Editor {
    pub buffer: Buffer,
    pub cursor: Cursor,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn insert(&mut self, text: &str) {
        self.buffer.insert(self.cursor.position, text);
        self.cursor.move_forward(text.len());
    }

    pub fn backspace(&mut self) {
        if self.cursor.position > 0 {
            self.buffer
                .delete(self.cursor.position - 1, self.cursor.position);
            self.cursor.move_backward(1);
        }
    }

    pub fn delete(&mut self) {
        self.buffer
            .delete(self.cursor.position, self.cursor.position + 1);
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
