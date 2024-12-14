#![no_std]
#![allow(clippy::result_unit_err)]

extern crate alloc;

pub mod span;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter::Peekable;
use core::str::Chars;
use crate::span::{BranchSpan, Span};

pub struct Node<'a, Supplementary> {
    span: Span<'a>,
    supplementary: Supplementary
}

pub struct Builder<'a> {
    chars: Rc<RefCell<Peekable<Chars<'a>>>>,
    span: Rc<RefCell<Span<'a>>>
}

impl<'a> Builder<'a> {
    pub fn derive(&self) -> Self {
        Self {
            chars: self.chars.clone(),
            span: self.span.derive()
        }
    }
    
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: Rc::new(RefCell::new(source.chars().peekable())),
            span: Rc::new(RefCell::new(Span::default()))
        }
    }

    pub fn test(&mut self, mut test: impl FnMut(char) -> bool) -> Option<char> {
        let mut chars = self.chars.borrow_mut();
        let peeked = chars.peek()?;
        let true = test(*peeked) else { return None };
        Some(chars.next().unwrap())
    }
    
    pub fn parse<Type: Parsable>(&mut self) {
        
    }
}

pub trait Parsable {
    fn parse(builder: Builder) -> Self;
}