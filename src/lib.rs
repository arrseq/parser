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
pub mod node;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::iter::Peekable;
use core::str::Chars;
use std::ops::{Deref, DerefMut, Range};
use bytestring::ByteString;
use indexmap::IndexMap;
use indexmap::map::Entry;
use thiserror::Error;
use crate::error::Error;
use crate::span::{ArithmeticOverflow, Span};

pub use string::String as ParserString;
use crate::string::{Strings};

#[derive(Debug)]
pub struct Node<Supplementary> {
    bounds: Span,
    source: ByteString,
    supplementary: Supplementary
}

impl<S> Node<S> {
    pub fn slice(&self) -> &'_ str {
        &self.source
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
    source: ByteString,
    span: Span,
    strings: Strings<Token>,
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

        self.span.overflowing_expand(peeked);
        chars.next();
        
        Ok(())
    }
    
    pub fn parse_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> ParserString<Token> {
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
        
            self.span.overflowing_expand(peeked);
            slice_bounds.end += peeked.len_utf8();
        }
        
        ParserString {
            strings: self.strings.clone(),
            // FIXME: Bytestring not being used correctly
            slice: self.derive_source(slice_bounds),
            index: None
        }
    }
    
    pub fn parse_till_char(&mut self, char: char) -> ParserString<Token> {
        self.parse_while(|peeked| peeked != char)
    }
    
    pub fn internalize(&mut self, slice: &str, token: Token) -> Result<ParserString<Token>, InternalizeError> {
        let mut borrow = self.strings.borrow_mut();
        let Entry::Vacant(entry) = borrow.entry(slice.into()) else { return Err(InternalizeError::EntryExists) };
        let index = entry.index();
        entry.insert(token);
        
        Ok(ParserString {
            strings: self.strings.clone(),
            index: Some(index),
            slice: slice.into()
        })
    }
    
    fn derive_source(&self, range: Range<usize>) -> ByteString {
        unsafe { ByteString::from_bytes_unchecked(self.source.clone().into_bytes().slice(range)) }
    }
    
    pub fn parse<Type: Parsable<Token=Token> + 'static>(&mut self, data: &mut Type::Data) -> Result<Node<Type>, Error<Type::Error>> {
        let mut fork = self.derive().map_err(Error::ArithmeticOverflow)?;
        let supplementary = Type::parse(&mut fork, data)?;
        
        self.span.length += fork.span.length;
        self.span.byte_length += fork.span.byte_length;
        
        Ok(Node {
            bounds: fork.span,
            supplementary,
            source: self.derive_source(fork.span.byte_range())
        })
    }
    
    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl<Token: Clone> Parser<'_, Token> {
    pub fn cloning_parse<Type: Parsable<Token=Token> + 'static>(&mut self, data: &mut Type::Data) -> Result<Node<Type>, Error<Type::Error>> {
        let strings = Rc::new(RefCell::clone(&self.strings));
        self.parse::<Type>(data).inspect_err(move |_| {
            self.strings = strings;
        })
    }
}

pub trait Parsable: Sized {
    type Error;
    type Token;
    type Data; 
    
    fn parse(parser: &mut Parser<Self::Token>, data: &mut Self::Data) -> Result<Self, Error<Self::Error>>;
}