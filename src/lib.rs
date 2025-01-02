// #![no_std]
#![allow(clippy::result_unit_err)]

#![feature(round_char_boundary)]
#![feature(new_range_api)]
#![feature(let_chains)]
extern crate alloc;
extern crate core;

pub mod error;
pub mod span;
pub mod string;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter::Peekable;
use core::str::Chars;
use std::ops::{Deref, DerefMut};
use indexmap::IndexMap;
use indexmap::map::Entry;
use thiserror::Error;
use crate::error::Error;
use crate::span::{ArithmeticOverflow, Span};

pub use string::String as ParserString;
pub use string::Intern as InternString;

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

impl<S> Deref for Node<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.supplementary
    }
}

impl<S> DerefMut for Node<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.supplementary
    }
}

#[derive(Debug)]
pub struct Parser<'a, Token> {
    chars: Rc<RefCell<Peekable<Chars<'a>>>>,
    source: Rc<str>,
    span: Span,
    strings: Rc<RefCell<IndexMap<String, Token>>>,
}

impl<Token> Clone for Parser<'_, Token> {
    fn clone(&self) -> Self {
        Self {
            chars: Rc::new(RefCell::clone(&self.chars)),
            source: self.source.clone(),
            span: self.span,
            strings: self.strings.clone()
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

#[derive(Debug, Error, PartialEq)]
pub enum InternalizeError {
    #[error("Entry already exists")]
    EntryExists,
    
}

impl<'a, Token> Parser<'a, Token> {
    pub fn derive(&self) -> Result<Self, ArithmeticOverflow> {
        Ok(Self {
            source: self.source.clone(),
            chars: self.chars.clone(),
            span: self.span.at_end()?,
            strings: self.strings.clone()
        })
    }
    
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.into(),
            chars: Rc::new(RefCell::new(source.chars().peekable())),
            span: Span::default(),
            strings: Rc::new(RefCell::new(IndexMap::new()))
        }
    }

    pub fn expect_char(&mut self, char: char) -> Result<(), ExpectError> {
        let mut chars = self.chars.borrow_mut();
        let peeked = *chars.peek().ok_or(ExpectError::Unexpected)?;
        let true = char == peeked else { return Err(ExpectError::Unexpected) };

        Self::expand_internal_character(&mut self.span, peeked);
        chars.next();
        
        Ok(())
    }

    /// Expand the span by a character that was read from this string that want used to expand this 
    /// string previously.
    /// 
    /// This is ok to do only if the specific instance of that character was used by this function 
    /// once because
    /// - The byte length of the span will always be valid because it uses the same type to 
    ///   index the string
    /// - The character length will never be larger than the byte length because every characters 
    ///   byte length is greater than one
    #[inline]
    fn expand_internal_character(span: &mut Span, char: char) {
        span.expand(char).unwrap()
    }
    
    pub fn parse_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> ParserString<'_, Token> {
        let mut chars = self.chars.borrow_mut();
        let mut slice_bounds = {
            let byte_end = self.span.byte_end();
            byte_end..byte_end
        };
        
        loop {
            let Some(peeked) = chars.peek() else { break };
            let peeked = *peeked;
            
            if !predicate(peeked) { break }
            let _ = chars.next();
            
            Self::expand_internal_character(&mut self.span, peeked);
            slice_bounds.end += peeked.len_utf8();
        }
        
        ParserString {
            strings: self.strings.clone(),
            slice: (&self.source.deref()[slice_bounds]).into()
        }
    }
    
    pub fn parse_till_char(&mut self, char: char) -> ParserString<'_, Token> {
        self.parse_while(|peeked| peeked != char)
    }
    
    pub fn internalize(&mut self, slice: &str, token: Token) -> Result<InternString<Token>, InternalizeError> {
        let mut borrow = self.strings.borrow_mut();
        let Entry::Vacant(entry) = borrow.entry(slice.to_string()) else { return Err(InternalizeError::EntryExists) };
        let index = entry.index();
        entry.insert(token);
        
        Ok(InternString {
            strings: self.strings.clone(),
            index
        })
    }
    
    pub fn parse<Type: Parsable<Token=Token> + 'static>(&mut self, data: &mut Type::Data) -> Result<Node<Type>, Error<Type::Error>> {
        let mut fork = self.derive().map_err(Error::ArithmeticOverflow)?;
        let supplementary = Type::parse(&mut fork, data)?;
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
    type Token;
    type Data; 
    
    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>>;
}