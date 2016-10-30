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
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Not<T>(T);

impl<T> Not<T> {

    pub fn new(a: T) -> Not<T> {
        Not(a)
    }

}

impl_operators!(Not, self e { !self.0.filter(e) }, T);
