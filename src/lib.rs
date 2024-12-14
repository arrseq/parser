// #![no_std]
#![allow(clippy::result_unit_err)]

#![feature(round_char_boundary)]

extern crate alloc;

pub mod span;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter::Peekable;
use core::str::Chars;
use crate::span::{BranchSpan, Span};

pub struct Node<'a, Supplementary> {
    bounds: [usize; 2],
    slice: &'a str,
    supplementary: Supplementary
}

pub struct Parser<'a> {
    chars: Rc<RefCell<Peekable<Chars<'a>>>>,
    span: Rc<RefCell<Span<'a>>>
}

impl<'a> Parser<'a> {
    pub fn derive(&self) -> Self {
        Self {
            chars: self.chars.clone(),
            span: self.span.derive()
        }
    }
    
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: Rc::new(RefCell::new(source.chars().peekable())),
            span: Rc::new(RefCell::new(Span::new(source.char_indices())))
        }
    }

    pub fn test(&mut self, mut test: impl FnMut(char) -> bool) -> Option<char> {
        let mut chars = self.chars.borrow_mut();
        let peeked = chars.peek()?;
        let true = test(*peeked) else { return None };
        Some(chars.next().unwrap())
    }
    
    pub fn parse<Type: Parsable>(&mut self) -> Node<'_, Type> {
        let supplementary = Type::parse(self);
        // Node {
        //     bounds: self.span.borrow().as_bounds(),
        //     supplementary
        // }
        
        todo!()
    }
}

pub trait Parsable {
    fn parse(parser: &mut Parser) -> Self;
}