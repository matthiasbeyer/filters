//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Map implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use std::marker::PhantomData;
use std::borrow::Borrow;
use std::error::Error;

use failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableMapInput<F, M, FT, B>(F, M, PhantomData<FT>, PhantomData<B>);

impl<F, M, FT, B> FailableMapInput<F, M, FT, B> {
    pub fn new(a: F, m: M) -> FailableMapInput<F, M, FT, B> {
        FailableMapInput(a, m, PhantomData, PhantomData)
    }
}

impl<FT, E, F, T, B, M> FailableFilter<T, E> for FailableMapInput<F, M, FT, B>
    where E: Error,
          F: FailableFilter<FT, E>,
          B: Borrow<FT> + Sized,
          M: Fn(&T) -> B
{
    fn filter(&self, e: &T) -> Result<bool, E> {
        self.0.filter(self.1(e).borrow())
    }
}

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableMapErr<F, M, FE, E>(F, M, PhantomData<FE>, PhantomData<E>);

impl<F, M, FE, E> FailableMapErr<F, M, FE, E> {
    pub fn new(a: F, m: M) -> FailableMapErr<F, M, FE, E> {
        FailableMapErr(a, m, PhantomData, PhantomData)
    }
}

impl<FE, E, F, T, M> FailableFilter<T, E> for FailableMapErr<F, M, FE, E>
    where E: Error,
          FE: Error,
          F: FailableFilter<T, FE>,
          M: Fn(FE) -> E
{
    fn filter(&self, e: &T) -> Result<bool, E> {
        self.0.filter(e).map_err(&self.1)
    }
}
