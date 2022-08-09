//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! XOR implementation.
//!
//! Will be automatically included when including `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!

use crate::failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableXOr<T, U>(T, U);

impl<T, U> FailableXOr<T, U> {
    pub fn new(a: T, b: U) -> FailableXOr<T, U> {
        FailableXOr(a, b)
    }
}

impl<N, T, U, E> FailableFilter<N> for FailableXOr<T, U>
where
    T: FailableFilter<N, Error = E>,
    U: FailableFilter<N, Error = E>,
{
    type Error = E;

    fn filter(&self, e: &N) -> Result<bool, Self::Error> {
        Ok(self.0.filter(e)? ^ self.1.filter(e)?)
    }
}
