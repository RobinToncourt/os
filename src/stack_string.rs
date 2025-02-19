#[derive(Debug, Clone)]
pub struct StackString<const CAPACITY: usize> {
    size: usize,
    data: [u8; CAPACITY],
}

impl<const CAPACITY: usize> Default for StackString<CAPACITY> {
    fn default() -> Self {
        Self {
            size: 0,
            data: [0; CAPACITY],
        }
    }
}

impl<const CAPACITY: usize> StackString<CAPACITY> {
    pub fn clear(&mut self) {
        self.size = 0;
    }

    pub fn push_str(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.data[self.size] = *b;
            self.size += 1;
        }
    }

    #[must_use]
    pub fn get_buffer(&self) -> &[u8] {
        &self.data
    }
}
