//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! NOT implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!

use failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableNot<T>(T);

impl<T> FailableNot<T> {

    pub fn new(a: T) -> FailableNot<T> {
        FailableNot(a)
    }

}

impl<N, E, T> FailableFilter<N, E> for FailableNot<T>
    where T: FailableFilter<N, E>
{
    fn filter(&self, e: &N) -> Result<bool, E> {
        self.0.filter(e).map(|b| !b)
    }
}
