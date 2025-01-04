use std::ops::Range;
use thiserror::Error;

/// A bound of characters with respect to the source string.
/// 
/// # Usage
/// Used by the parser for attributing text to a syntax node.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Span {
    /// Index of the character where this span's bound starts.
    pub start: usize,
    /// Length in characters of this span relative to the start position.
    pub length: usize,
    /// Byte index in the source string where this span' bound starts.
    pub byte_start: usize,
    /// Quantity of bytes the span's bound covers relative to the byte start position.
    pub byte_length: usize
}

#[derive(Debug, Error, PartialEq)]
#[error("An arithmetic operation resulted in an overflow")]
pub struct ArithmeticOverflow;

impl Span {
    /// Create a new span that starts at the end of the current span.
    /// 
    /// # Error
    /// Results in an error if the calculation to determine the new start position results in an 
    /// overflow.
    pub fn at_end(&self) -> Result<Self, ArithmeticOverflow> {
        // Check to see if the byte range will overflow.
        self.byte_start.checked_add(self.byte_length).ok_or(ArithmeticOverflow)?;
        
        Ok(Self {
            start: self.start.checked_add(self.length).ok_or(ArithmeticOverflow)?,
            length: 0,
            byte_start: self.byte_end(),
            byte_length: 0
        })
    }
    
    /// Calculates the byte index that this spans bound ends at.
    /// 
    /// # Usage
    /// Used by this span function to find where a new span could start at if the [Span::at_end] 
    /// method is used.
    pub fn byte_end(&self) -> usize {
        self.byte_start + self.byte_length
    }
    
    /// Expand the span to cover another character.
    /// 
    /// # Error
    /// Results in an error if expanding the span results in an overflow.
    /// 
    /// # Usage
    /// Used to include a character that was parsed.
    pub fn expand(&mut self, char: char) -> Result<(), ArithmeticOverflow> {
        let lengths = if let Some(byte_length) = self.byte_length.checked_add(char.len_utf8())
            && let Some(length) = self.length.checked_add(1) {
            [length, byte_length]
        } else {
            return Err(ArithmeticOverflow);
        };
        
        self.add_lengths(lengths);
        Ok(())
    }

    /// Same method as [Self::expand] but with no error checking and a risk of an overflow
    /// 
    /// # Usage
    /// This is ok to do only if the specific instance of that character was used by this function 
    /// once because
    /// - The byte length of the span will always be valid because it uses the same type to 
    ///   index the string
    /// - The character length will never be larger than the byte length because every characters 
    ///   byte length is greater than one
    pub(super) fn overflowing_expand(&mut self, char: char) {
        let lengths = [
            self.byte_length.overflowing_add(char.len_utf8()).0,
            self.length.overflowing_add(1).0
        ];

        self.add_lengths(lengths);
    }
    
    /// Add the byte and character length to this span. The result of the calculations are 
    /// unchecked.
    /// 
    /// # Privacy
    /// This method is kept private because it may result in an overflow and the lengths may become 
    /// meaningless.
    pub(super) fn add_lengths(&mut self, lengths: [usize; 2]) {
        self.length = self.length.overflowing_add(lengths[0]).0;
        self.byte_length = self.byte_length.overflowing_add(lengths[1]).0;
    }
    
    /// Constructs a range type from the byte start and end fields in this span.
    /// 
    /// # Usage
    /// Typically used by the parser internally to get a range to be used to index a string slice, 
    /// particularly the source string. This can be used to get the string that the span bounds.
    pub fn byte_range(&self) -> Range<usize> {
        Range {
            start: self.byte_start,
            // The call to calculate the byte end position should not fail because
            // - expanding this type will cause an error if the length can result in an overflow.
            // - when newly initialized, the lengths are 0 which cannot result in an overflow.
            end: self.byte_end()
        }
    }
}