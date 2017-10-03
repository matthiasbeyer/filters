//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! OR implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!

use failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableOr<T, U>(T, U);

impl<T, U> FailableOr<T, U> {

    pub fn new(a: T, b: U) -> FailableOr<T, U> {
        FailableOr(a, b)
    }

}

impl<N, E, T, U> FailableFilter<N, E> for FailableOr<T, U>
    where T: FailableFilter<N, E>,
          U: FailableFilter<N, E>
{
    fn filter(&self, e: &N) -> Result<bool, E> {
        Ok(try!(self.0.filter(e)) || try!(self.1.filter(e)))
    }
}

