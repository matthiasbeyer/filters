//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! AND implementation.
//!
//! Will be automatically included when including `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!

use failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableAnd<T, U>(T, U);

impl<T, U> FailableAnd<T, U> {
    pub fn new(a: T, b: U) -> FailableAnd<T, U> {
        FailableAnd(a, b)
    }
}

impl<N, T, U, E> FailableFilter<N> for FailableAnd<T, U>
where
    T: FailableFilter<N, Error = E>,
    U: FailableFilter<N, Error = E>,
{
    type Error = E;

    fn filter(&self, e: &N) -> Result<bool, Self::Error> {
        Ok(self.0.filter(e)? && self.1.filter(e)?)
    }
}
