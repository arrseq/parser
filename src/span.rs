use alloc::rc::{Rc, Weak};
use core::cell::RefCell;
use core::str::CharIndices;
use derive_getters::Getters;
use thiserror::Error;

#[cfg(test)]
mod test;

#[derive(Debug, Getters)]
pub struct Span<'a> {
    /// Bounds of bytes in source slice that correspond to the [`bounds`] field.
    slice_bounds: [usize; 2],
    #[getter(skip)]
    indices: Rc<RefCell<CharIndices<'a>>>,
    /// Position of span in respect to the source.
    /// This is the start and end indexes of the slice in reference to the source string.
    bounds: [usize; 2],
    /// Parent span that this current span was derived from. 
    /// 
    /// This is stored to expand the parent when the child expands because the parent must encompass
    /// the child spans.
    #[getter(skip)]
    parent: Option<Weak<RefCell<Self>>>,
    /// Last child of this span. 
    /// 
    /// This is stored so that it can be blocked when a new child is added to this span. it is 
    /// unwanted behavior to allow older spans to resize.
    #[getter(skip)]
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
    ParentDeallocated,
    #[error("Bound exceeds length of source")]
    OutOfBounds
}

impl<'a> Span<'a> {
    pub fn new(char_indices: CharIndices<'a>) -> Self {
        Self {
            slice_bounds: [0; 2],
            indices: Rc::new(RefCell::new(char_indices)),
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
        self.backpropagation_expand(amount, None)
    }
    
    fn backpropagation_expand(&mut self, amount: usize, parent_slice_end: Option<usize>) -> Result<(), Error> {
        if self.blocked { return Err(Error::BlockedResize) }
        
        if let Some(slice_end) = parent_slice_end {
            self.slice_bounds[1] = slice_end;
        } else {
            let mut indices = self.indices.borrow_mut();
            // Save snapshots of fields to reset this to its original state in case of an out of 
            // bounds error.
            let old_indices = indices.clone();
            let old_slice_end = self.slice_bounds[1];
            
            for _ in 0..amount { 
                let Some(next) = indices.next() else {
                    println!("failed");
                    
                    *indices = old_indices;
                    self.slice_bounds[1] = old_slice_end;
                    return Err(Error::OutOfBounds);
                };
                self.slice_bounds[1] += next.1.len_utf8();
            }
        }

        self.bounds[1] += amount;

        if let Some(parent) = &self.parent {
            let parent = parent.upgrade().ok_or(Error::ParentDeallocated)?;
            let result = parent.borrow_mut().backpropagation_expand(amount, Some(self.slice_bounds[1]));
            Err(match result {
                Err(Error::BlockedResize | Error::BlockedParentResize) => Error::BlockedParentResize,
                Err(Error::ParentDeallocated) => Error::ParentDeallocated,
                Err(Error::OutOfBounds) => unreachable!("Out of bounds is only given if called with 'parent_slice_end' being 'None'"),
                Ok(_) => return Ok(())
            })
        } else { Ok(()) }
    }

    pub fn as_bounds(&self) -> [usize; 2] {
        self.bounds
    }
}

pub trait BranchSpan {
    fn derive(&self) -> Self;
}

impl BranchSpan for Rc<RefCell<Span<'_>>> {
    fn derive(&self) -> Self {
        let mut self_span = self.borrow_mut();
        
        let start = if let Some(last_child) = &self_span.latest_child {
            let mut borrowed = last_child.borrow_mut();
            borrowed.blocked = true;
            borrowed.bounds[1]
        } else { self_span.bounds[1] };
        
        let child = Self::new(RefCell::new(Span {
            indices: self_span.indices.clone(),
            slice_bounds: [self_span.slice_bounds[1]; 2],
            bounds: [start; 2],
            parent: Some(Rc::downgrade(self)),
            latest_child: None,
            blocked: false
        }));
        self_span.latest_child = Some(child.clone());
        child
    }
}