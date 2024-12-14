use alloc::rc::{Rc, Weak};
use core::cell::RefCell;
use core::str::CharIndices;
use thiserror::Error;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Span<'a> {
    slice_bounds: [usize; 2],
    indices: CharIndices<'a>,
    /// Position of span in respect to the source.
    /// This is the start and end indexes of the slice in reference to the source string.
    bounds: [usize; 2],
    parent: Option<Weak<RefCell<Self>>>,
    latest_child: Option<Rc<RefCell<Self>>>,
    /// Whether a sibling was created after this. If so, this span cannot be resized.
    blocked: bool
}

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Resizing of a blocked span")]
    BlockedResize,
    #[error("Resizing when one or more parents are blocked")]
    BlockedParentResize,
    #[error("At least one parent span was deallocated")]
    ParentDeallocated
}

impl<'a> Span<'a> {
    pub fn new(char_indices: CharIndices<'a>) -> Self {
        Self {
            slice_bounds: [0; 2],
            indices: char_indices,
            bounds: [0; 2],
            parent: None,
            latest_child: None,
            blocked: false
        }
    }
    
    /// Grow the span by `amount`.
    /// 
    /// # Result
    /// [`Err(())`] is returned if the span is blocked in from resizing, otherwise [`Ok(())`] is 
    /// returned.
    pub fn expand(&mut self, amount: usize) -> Result<(), Error> {
        if self.blocked { return Err(Error::BlockedResize) }
        
        self.bounds[1] += amount;
        if let Some(parent) = &self.parent {
            let parent = parent.upgrade().ok_or(Error::ParentDeallocated)?;
            let result = parent.borrow_mut().expand(amount);
            Err(match result {
                Err(Error::BlockedResize | Error::BlockedParentResize) => Error::BlockedParentResize,
                Err(Error::ParentDeallocated) => Error::ParentDeallocated,
                Ok(_) => return Ok(())
            })
        } else { Ok(()) }
    }
    
    pub fn as_bounds(&self) -> [usize; 2] {
        self.bounds
    }
}

pub(super) trait BranchSpan {
    fn derive(&self) -> Self;
}

impl BranchSpan for Rc<RefCell<Span<'_>>> {
    fn derive(&self) -> Self {
        let mut self_span = self.borrow_mut();
        
        let start = if let Some(last_child) = &self_span.latest_child {
            let mut borrowed = last_child.borrow_mut();
            borrowed.blocked = true;
            borrowed.bounds[1]
        } else { self_span.bounds[0] };
        
        let child = Self::new(RefCell::new(Span {
            indices: self_span.indices.clone(),
            slice_bounds: [self_span.slice_bounds[0]; 2],
            bounds: [start; 2],
            parent: Some(Rc::downgrade(self)),
            latest_child: None,
            blocked: false
        }));
        self_span.latest_child = Some(child.clone());
        child
    }
}