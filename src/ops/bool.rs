//! Bool Filter implementation, so we can insert this in filter construction
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Bool {
    b: bool
}

impl Bool {

    pub fn new(b: bool) -> Bool {
        Bool { b: b }
    }

}

impl From<bool> for Bool {

    fn from(b: bool) -> Bool {
        Bool::new(b)
    }

}

impl_operators!(Bool, self e { self.b }, );
