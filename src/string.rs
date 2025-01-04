use alloc::rc::Rc;
use bytestring::ByteString;
use indexmap::IndexMap;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use thiserror::Error;

pub type StringsMap<Token> = IndexMap<ByteString, Token>;
pub type Strings<Token> = Rc<RefCell<StringsMap<Token>>>;

#[derive(Debug)]
pub struct TokenGuard<'a, Token> {
    borrow: Ref<'a, StringsMap<Token>>,
    index: usize
}

impl<Token> Deref for TokenGuard<'_, Token> {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.borrow[self.index]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct String<Token> {
    pub(super) strings: Strings<Token>,
    pub(super) slice: ByteString,
    pub(super) index: Option<usize>
}

#[derive(Debug, Error, PartialEq)]
#[error("Cannot re-internalize a string")]
pub struct ReInternalizationError;

impl<Token> String<Token> {
    pub fn try_internalize(&mut self, on_create: impl for<'a> FnOnce(&'a str) -> Option<Token>) -> Result<Option<TokenGuard<Token>>, ReInternalizationError> {
        let None = self.index else { return Err(ReInternalizationError) };
        
        let mut strings = self.strings.borrow_mut();
        let index = match strings.entry(self.slice.clone()) {
            indexmap::map::Entry::Occupied(mapping) => {
                mapping.index()
            },
            indexmap::map::Entry::Vacant(mapping) => {
                let index = mapping.index();
                let Some(token) = on_create(&self.slice) else { return Ok(None) };
                let _ = mapping.insert(token);
                index
            }
        };
        drop(strings);
        
        self.index = Some(index);
        Ok(Some(TokenGuard {
            index,
            borrow: self.strings.borrow()
        }))
    }
    
    pub fn token(&self) -> Option<TokenGuard<'_, Token>> {
        let borrow = RefCell::borrow(&self.strings);
        
        let index = if let Some(index) = self.index {
            index
        } else {
            let index = borrow.get_full(&self.slice)?.0;
            index
        };
        
        Some(TokenGuard {
            borrow,
            index
        })
    }
}

impl<Token> Deref for String<Token> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}

impl<Token> AsRef<str> for String<Token> {
    fn as_ref(&self) -> &str {
        self
    }
}