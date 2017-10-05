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
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct XOr<T, U>(T, U);

impl<T, U> XOr<T, U> {

    pub fn new(a: T, b: U) -> XOr<T, U> {
        XOr(a, b)
    }

}

impl_operators!(XOr, self e { self.0.filter(e) ^ self.1.filter(e) }, T, U);
