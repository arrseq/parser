// #![no_std]
#![allow(clippy::result_unit_err)]

#![feature(round_char_boundary)]
#![feature(new_range_api)]
extern crate alloc;
extern crate core;

pub mod error;
pub mod span;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter::Peekable;
use core::ops::Range;
use core::str::Chars;
use std::ops::Deref;
use thiserror::Error;
use crate::error::Error;
use crate::span::Span;

#[derive(Debug)]
pub struct Node<Supplementary> {
    bounds: Span,
    source: Rc<str>,
    supplementary: Supplementary
}

impl<S> Node<S> {
    pub fn slice(&self) -> &'_ str {
        &self.source.deref()[self.bounds.byte_range()]
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    chars: Rc<RefCell<Peekable<Chars<'a>>>>,
    source: Rc<str>,
    span: Span
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
            span: self.span.at_end()
        }
    }
    
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.into(),
            chars: Rc::new(RefCell::new(source.chars().peekable())),
            span: Span::default()
        }
    }

    pub fn expect_char(&mut self, char: char) -> Result<(), ExpectError> {
        let mut chars = self.chars.borrow_mut();
        let peeked = *chars.peek().ok_or(ExpectError::Unexpected)?;
        let true = char == peeked else { return Err(ExpectError::Unexpected) };
        
        self.span.expand(peeked);
        chars.next();
        
        Ok(())
    }
    
    // pub fn parse_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> &'_ str {
    //     let mut chars = self.chars.borrow_mut();
    //     let mut slice_bounds = self.span.borrow().slice_bounds().start..self.span.borrow().slice_bounds().start;
    //     
    //     loop {
    //         let Some(peeked) = chars.peek() else { break };
    //         let peeked = *peeked;
    //         
    //         if !predicate(peeked) { break }
    //         let _ = chars.next();
    //         let Ok(_) = self.span.borrow_mut().expand(1) else { break };
    //         slice_bounds.end += peeked.len_utf8();
    //     }
    //     
    //     &self.source.deref()[slice_bounds]
    // }
    // 
    // pub fn parse_to_char(&mut self, char: char) -> &'_ str {
    //     self.parse_while(|peeked| peeked != char)
    // }
    
    pub fn parse<Type: Parsable + 'static>(&mut self) -> Result<Node<Type>, Error<Type::Error>> {
        let mut fork = self.derive();
        let supplementary = Type::parse(&mut fork)?;
        self.span.length += fork.span.length;
        self.span.byte_length += fork.span.byte_length;
        
        Ok(Node {
            bounds: fork.span,
            supplementary,
            source: self.source.clone()
        })
    }
}

pub trait Parsable: Sized {
    type Error;
    fn parse(parser: &mut Parser) -> Result<Self, Error<Self::Error>>;
}