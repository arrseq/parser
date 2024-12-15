use core::str::Chars;
use std::iter::Peekable;
use crate::span::Span;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Snapshot<'a> {
    pub(super) source: &'a str,
    pub(super) chars: Peekable<Chars<'a>>,
    pub(super) span: Span<'a>
}