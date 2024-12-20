use alloc::rc::Rc;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Error<SpecificError> {
    source: Rc<str>,
    x: SpecificError
}