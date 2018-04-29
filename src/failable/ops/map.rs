//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Map implementation.
//!
//! Will be automatically included when including `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use std::marker::PhantomData;
use std::borrow::Borrow;

use failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableMapInput<F, M, FT, B>(F, M, PhantomData<FT>, PhantomData<B>);

impl<F, M, FT, B> FailableMapInput<F, M, FT, B> {
    pub fn new(a: F, m: M) -> FailableMapInput<F, M, FT, B> {
        FailableMapInput(a, m, PhantomData, PhantomData)
    }
}

impl<FT, F, T, B, M> FailableFilter<T> for FailableMapInput<F, M, FT, B>
    where F: FailableFilter<FT>,
          B: Borrow<FT> + Sized,
          M: Fn(&T) -> B
{
    type Error = F::Error;

    fn filter(&self, e: &T) -> Result<bool, Self::Error> {
        self.0.filter(self.1(e).borrow())
    }
}

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableMapErr<F, M, E>(F, M, PhantomData<E>);

impl<F, M, E> FailableMapErr<F, M, E> {
    pub fn new(a: F, m: M) -> FailableMapErr<F, M, E> {
        FailableMapErr(a, m, PhantomData)
    }
}

impl<E, F, T, M> FailableFilter<T> for FailableMapErr<F, M, E>
    where F: FailableFilter<T>,
          M: Fn(F::Error) -> E
{
    type Error = E;

    fn filter(&self, e: &T) -> Result<bool, Self::Error> {
        self.0.filter(e).map_err(&self.1)
    }
}
