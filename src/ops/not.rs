//! NOT implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Not<T> {
    a: T
}

impl<T> Not<T> {

    pub fn new(a: T) -> Not<T> {
        Not { a: a }
    }

}

impl_operators!(Not, self e { !self.a.filter(e) }, T);
