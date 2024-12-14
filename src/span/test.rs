use std::cell::RefCell;
use std::rc::Rc;
use crate::span::{BranchSpan, Span};

#[test]
fn expand() {
    let source = "abc";
    let mut span = Rc::new(RefCell::new(Span::new(source)));
    
    let inner = span.branch();
    inner.borrow_mut().expand(4);
    
    let inner2 = span.branch();
    inner2.borrow_mut().expand(4);
    
    let inner3 = span.branch();
    inner3.borrow_mut().expand(2);

    dbg!(&inner);
    dbg!(&inner2);
    dbg!(&inner3);
}