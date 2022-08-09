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
use std::borrow::Borrow;
use std::marker::PhantomData;

use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct MapInput<F, M, FT, B>(F, M, PhantomData<FT>, PhantomData<B>);

impl<F, M, FT, B> MapInput<F, M, FT, B> {
    pub fn new(a: F, m: M) -> MapInput<F, M, FT, B> {
        MapInput(a, m, PhantomData, PhantomData)
    }
}

impl<FT, F, T, B, M> Filter<T> for MapInput<F, M, FT, B>
where
    F: Filter<FT>,
    B: Borrow<FT> + Sized,
    M: Fn(&T) -> B,
{
    fn filter(&self, e: &T) -> bool {
        self.0.filter(self.1(e).borrow())
    }
}
