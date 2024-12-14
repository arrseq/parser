use alloc::rc::Rc;
use core::cell::RefCell;
use crate::span::{BranchSpan, Span};

#[test]
fn expand() {
    let mut span = Rc::new(RefCell::new(Span::new("abc".char_indices())));
}