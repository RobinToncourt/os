use core::fmt;

pub enum StackStringError {
    ExceedCapacity,
}

#[derive(Debug, Clone)]
pub struct StackString<const CAPACITY: usize> {
    size: usize,
    data: [char; CAPACITY],
}

impl<const CAPACITY: usize> Default for StackString<CAPACITY> {
    fn default() -> Self {
        Self {
            size: 0,
            data: ['\0'; CAPACITY],
        }
    }
}

impl<const CAPACITY: usize> StackString<CAPACITY> {
    pub fn clear(&mut self) {
        self.size = 0;
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.size > 0 {
            self.size -= 1;
            let result = self.data[self.size];
            Some(result)
        } else {
            None
        }
    }

    /// # Errors
    ///
    /// Will return 'Err' if capacity is reached.
    pub fn push(&mut self, c: char) -> Result<(), StackStringError> {
        if self.size >= CAPACITY {
            return Err(StackStringError::ExceedCapacity);
        }

        self.data[self.size] = c;
        self.size += 1;

        Ok(())
    }

    /// # Errors
    ///
    /// Will return 'Err' if the str len exceed remaining capacity,
    /// will still fill the remaining capacity.
    pub fn push_str(&mut self, s: &str) -> Result<usize, StackStringError> {
        let mut chars_written: usize = 0;

        for c in s.chars() {
            if self.size >= CAPACITY {
                return Err(StackStringError::ExceedCapacity);
            }

            self.data[self.size] = c;
            self.size += 1;

            chars_written += 1;
        }

        Ok(chars_written)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.size
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[must_use]
    pub fn get_data(&self) -> &[char] {
        &self.data[..self.size]
    }
}

impl<const CAPACITY: usize> core::fmt::Display for StackString<CAPACITY> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size {
            write!(f, "{}", self.data[i])?;
        }

        Ok(())
    }
}

impl<const CAPACITY: usize> core::ops::Index<usize> for StackString<CAPACITY> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const CAPACITY: usize> core::ops::IndexMut<usize> for StackString<CAPACITY> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod stack_string_test {
    use crate::assert_eq;

    use super::*;

    #[test_case]
    fn test_stack_string() {
        let mut s = StackString::<5>::default();
        assert!(s.push('a').is_ok());
        assert_eq!(s.pop(), Some('a'));
    }
}
