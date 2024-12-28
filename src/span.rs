use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Span {
    pub start: usize,
    pub length: usize,
    pub byte_start: usize,
    pub byte_length: usize
}

impl Span {
    pub fn at_end(&self) -> Self {
        Self {
            // TODO: Check for overflow
            start: self.start + self.length,
            length: 0,
            byte_start: self.byte_end(),
            byte_length: 0
        }
    }
    
    pub fn byte_end(&self) -> usize {
        self.byte_start + self.byte_length
    }
    
    pub fn expand(&mut self, char: char) {
        self.length += 1;
        self.byte_length += char.len_utf8()
    }
    
    pub fn byte_range(&self) -> Range<usize> {
        Range {
            start: self.byte_start,
            end: self.byte_end()
        }
    }
}