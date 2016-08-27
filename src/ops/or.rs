//! OR implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Or<T, U> {
    a: T,
    b: U
}

impl<T, U> Or<T, U> {

    pub fn new(a: T, b: U) -> Or<T, U> {
        Or { a: a, b: b }
    }

}

impl_operators!(Or, self e { self.a.filter(e) || self.b.filter(e) }, T, U);
