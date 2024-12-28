use alloc::borrow::Cow;
use alloc::rc::Rc;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use indexmap::IndexMap;

pub type StringsMap<Token> = IndexMap<alloc::string::String, Token>;
pub type Strings<Token> = Rc<RefCell<StringsMap<Token>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct String<'a, Token> {
    pub(super) strings: Strings<Token>,
    pub(super) slice: Cow<'a, str>
}

impl<'a, Token> String<'a, Token> {
    pub fn try_internalize(mut self, on_create: impl FnOnce() -> Option<Token>) -> Option<Intern<Token>> {
        let cloned_strings = self.strings.clone();
        let mut strings = cloned_strings.borrow_mut();
        
        self.set_owned();
        let Cow::Owned(string) = self.slice else { unreachable!() };

        Some(match strings.entry(string) {
            indexmap::map::Entry::Occupied(mapping) => Intern {
                strings: self.strings.clone(),
                index: mapping.index()
            },
            indexmap::map::Entry::Vacant(mapping) => {
                let index = mapping.index();

                let Some(intern) = on_create() else { return None };
                let _ = mapping.insert(intern);

                Intern {
                    strings: self.strings.clone(),
                    index
                }
            }
        })
    }
    
    fn set_owned(&mut self) {
        if let Cow::Borrowed(slice) = self.slice {
            self.slice = slice.to_string().into();
        }
    }
    
    pub fn token(&mut self) -> Option<TokenGuard<'a, Token>> {
        self.set_owned();
        // self.strings.borrow().entry(self.slice.owne)
        todo!()
    }
}

impl<Token> Deref for String<'_, Token> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}

impl<Token> AsRef<str> for String<'_, Token> {
    fn as_ref(&self) -> &str {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intern<Token> {
    pub(super) strings: Strings<Token>,
    pub(super) index: usize
}

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

impl<Token> Intern<Token> {
    pub fn token(&self) -> TokenGuard<'_, Token> {
        TokenGuard {
            borrow: self.strings.borrow(),
            index: self.index
        }
    }
}