//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! FailableBool Filter implementation, so we can insert this in filter construction
//!
//! Will be automatically included when including `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!

use crate::failable::filter::FailableFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FailableBool(bool);

impl FailableBool {
    pub fn new(b: bool) -> FailableBool {
        FailableBool(b)
    }
}

impl From<bool> for FailableBool {
    fn from(b: bool) -> FailableBool {
        FailableBool::new(b)
    }
}

impl<N> FailableFilter<N> for FailableBool {
    type Error = ();

    fn filter(&self, _: &N) -> Result<bool, Self::Error> {
        Ok(self.0)
    }
}
