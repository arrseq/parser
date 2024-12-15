// #![no_std]
#![allow(clippy::result_unit_err)]

#![feature(round_char_boundary)]
#![feature(new_range_api)]
extern crate alloc;
extern crate core;

mod span;
mod snapshot;

pub use snapshot::Snapshot;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter::Peekable;
use core::ops::Range;
use core::str::Chars;
use std::ops::Deref;
use std::ptr;
use thiserror::Error;
use crate::span::{BranchSpan, Span};

#[derive(Debug)]
pub struct Node<Supplementary> {
    bounds: Range<usize>,
    slice_bounds: Range<usize>,
    source: Rc<str>,
    supplementary: Supplementary
}

impl<S> Node<S> {
    pub fn slice(&self) -> &'_ str {
        &self.source.deref()[self.slice_bounds.clone()]
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    chars: Rc<RefCell<Peekable<Chars<'a>>>>,
    source: Rc<str>,
    span: Rc<RefCell<Span<'a>>>
}

impl Clone for Parser<'_> {
    fn clone(&self) -> Self {
        Self {
            chars: Rc::new(RefCell::clone(&self.chars)),
            source: self.source.clone(),
            span: self.span.clone()
        }
    }
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
            source: self.source.clone(),
            chars: self.chars.clone(),
            span: self.span.derive()
        }
    }
    
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.into(),
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
    
    pub fn parse_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> &'_ str {
        let mut chars = self.chars.borrow_mut();
        let mut slice_bounds = self.span.borrow().slice_bounds().start..self.span.borrow().slice_bounds().start;
        
        loop {
            let Some(peeked) = chars.peek() else { break };
            let peeked = *peeked;
            
            if !predicate(peeked) { break }
            let _ = chars.next();
            let Ok(_) = self.span.borrow_mut().expand(1) else { break };
            slice_bounds.end += peeked.len_utf8();
        }
        
        &self.source.deref()[slice_bounds]
    }
    
    pub fn parse_to_char(&mut self, char: char) -> &'_ str {
        self.parse_while(|peeked| peeked != char)
    }
    
    pub fn parse<Type: Parsable + 'static>(&mut self) -> Result<Node<Type>, Type::Error> {
        let supplementary = Type::parse(self)?;
        let span = self.span.borrow();
        Ok(Node {
            bounds: span.bounds().clone(),
            slice_bounds: span.slice_bounds().clone(),
            supplementary,
            source: self.source.clone()
        })
    }
}

pub trait Parsable: Sized {
    type Error;
    fn parse(parser: &mut Parser) -> Result<Self, Self::Error>;
}