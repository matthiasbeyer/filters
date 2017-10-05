//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Filter -> FailableFilter implementations
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use std::marker::PhantomData;

use filter::Filter;
use failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct IntoFailable<F, E>(F, PhantomData<E>);

impl<F, E> IntoFailable<F, E> {
    pub fn new(a: F) -> IntoFailable<F, E> {
        IntoFailable(a, PhantomData)
    }
}

impl<F, N, E> FailableFilter<N, E> for IntoFailable<F, E>
    where F: Filter<N>,
{
    fn filter(&self, e: &N) -> Result<bool, E> {
        Ok(self.0.filter(e))
    }
}

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct AsFailable<'a, F: 'a + ?Sized, E>(&'a F, PhantomData<E>);

impl<'a, F: 'a + ?Sized, E> AsFailable<'a, F, E> {
    pub fn new(a: &'a F) -> AsFailable<F, E> {
        AsFailable(a, PhantomData)
    }
}

impl<'a, F, N, E> FailableFilter<N, E> for AsFailable<'a, F, E>
    where F: Filter<N> + 'a + ?Sized,
{
    fn filter(&self, e: &N) -> Result<bool, E> {
        Ok(self.0.filter(e))
    }
}
