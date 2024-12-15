// #![no_std]
#![allow(clippy::result_unit_err)]

#![feature(round_char_boundary)]

extern crate alloc;

mod span;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter::Peekable;
use core::str::Chars;
use thiserror::Error;
use crate::span::{BranchSpan, Span};

#[derive(Debug)]
pub struct Node<'a, Supplementary> {
    bounds: [usize; 2],
    slice: &'a str,
    supplementary: Supplementary
}

pub struct Parser<'a> {
    chars: Rc<RefCell<Peekable<Chars<'a>>>>,
    source: &'a str,
    span: Rc<RefCell<Span<'a>>>
}

#[derive(Debug, Error, PartialEq)]
pub enum ExpectError {
    #[error("Resizing of a blocked span in a parser")]
    BlockedSpan,
    #[error("Received an unexpected response")]
    Unexpected
}

impl<'a> Parser<'a> {
    pub fn derive(&self) -> Self {
        Self {
            source: self.source,
            chars: self.chars.clone(),
            span: self.span.derive()
        }
    }
    
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: Rc::new(RefCell::new(source.chars().peekable())),
            span: Rc::new(RefCell::new(Span::new(source.char_indices())))
        }
    }

    pub fn expect_char(&mut self, char: char) -> Result<(), ExpectError> {
        let mut chars = self.chars.borrow_mut();
        let peeked = chars.peek().ok_or(ExpectError::Unexpected)?;
        let true = char == *peeked else { return Err(ExpectError::Unexpected) };
        
        if let Err(span::Error::BlockedResize) = self.span.borrow_mut().expand(1) { return Err(ExpectError::BlockedSpan) }
        chars.next();
        
        Ok(())
    }
    
    pub fn parse_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> &'a str {
        let mut chars = self.chars.borrow_mut();
        let mut slice_bounds = [self.span.borrow().slice_bounds()[0]; 2];
        
        loop {
            let Some(peeked) = chars.peek() else { break };
            let peeked = *peeked;
            
            if !predicate(peeked) { break }
            let _ = chars.next();
            slice_bounds[1] += peeked.len_utf8();
        }
        
        &self.source[slice_bounds[0]..slice_bounds[1]]
    }
    
    pub fn parse_to_char(&mut self, char: char) -> &'a str {
        self.parse_while(|peeked| peeked != char)
    }
    
    pub fn parse<Type: Parsable>(&mut self) -> Node<'_, Type> {
        let supplementary = Type::parse(self);
        let span = self.span.borrow();
        Node {
            bounds: span.as_bounds(),
            slice: &self.source[span.slice_bounds()[0]..span.slice_bounds()[1]],
            supplementary
        }
    }
}

pub trait Parsable {
    fn parse(parser: &mut Parser) -> Self;
}