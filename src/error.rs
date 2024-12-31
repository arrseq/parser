use alloc::rc::Rc;
use thiserror::Error;
use crate::span::ArithmeticOverflow;

#[cfg(test)]
mod test;

#[derive(Debug, Error, PartialEq)]
pub struct SyntaxError<SpecificError> {
    string: Rc<str>,
    x: SpecificError
}

#[derive(Debug, Error, PartialEq)]
pub enum Error<SpecificError> {
    #[error("Cannot parse because it would result in an overflow")]
    ArithmeticOverflow(ArithmeticOverflow),
    #[error("Syntax error in parsing")]
    SyntaxError(SyntaxError<SpecificError>)
}