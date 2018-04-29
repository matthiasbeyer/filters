//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Filter -> FailableFilter implementations
//!
//! Will be automatically included when including `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!

use filter::Filter;
use failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct IntoFailable<F>(F);

impl<F> IntoFailable<F> {
    pub fn new(a: F) -> IntoFailable<F> {
        IntoFailable(a)
    }
}

impl<F, N> FailableFilter<N> for IntoFailable<F>
    where F: Filter<N>,
{
    type Error = ();

    fn filter(&self, e: &N) -> Result<bool, Self::Error> {
        Ok(self.0.filter(e))
    }
}

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct AsFailable<'a, F: 'a + ?Sized>(&'a F);

impl<'a, F: 'a + ?Sized> AsFailable<'a, F> {
    pub fn new(a: &'a F) -> AsFailable<F> {
        AsFailable(a)
    }
}

impl<'a, F, N> FailableFilter<N> for AsFailable<'a, F>
    where F: Filter<N> + 'a + ?Sized,
{
    type Error = ();

    fn filter(&self, e: &N) -> Result<bool, Self::Error> {
        Ok(self.0.filter(e))
    }
}
