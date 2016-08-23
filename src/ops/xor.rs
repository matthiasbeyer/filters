//! XOR implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct XOr<T, U> {
    a: T,
    b: U
}

impl<T, U> XOr<T, U> {

    pub fn new(a: T, b: U) -> XOr<T, U> {
        XOr { a: a, b: b }
    }

}

impl_operators!(XOr, self e { self.a.filter(e) ^ self.b.filter(e) }, T, U);
