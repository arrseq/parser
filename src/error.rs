use thiserror::Error;
use crate::span::{ArithmeticOverflow, Span};

#[cfg(test)]
mod test;

#[derive(Debug, Error, PartialEq)]
pub enum ErrorKind<SpecificError> {
    Specific(SpecificError),
    Unexpected
}

#[derive(Debug, Error, PartialEq)]
pub struct SyntaxError<SpecificError> {
    kind: ErrorKind<SpecificError>,
    span: Span
}

#[derive(Debug, Error, PartialEq)]
pub enum Error<SpecificError> {
    #[error("Cannot parse because it would result in an overflow")]
    ArithmeticOverflow(ArithmeticOverflow),
    #[error("Syntax error in parsing content")]
    SyntaxError(SyntaxError<SpecificError>),
}