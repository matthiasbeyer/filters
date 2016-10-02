//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! A library for constructing predicates and filters for `Iterator::filter`
//!
//! This library offers types and traits for creating filters and combining them with logical
//! operators such as AND, OR, NOT and others.
//!
//! # Example
//!
//! ```
//! use filters::filter::Filter;
//! let a = (|&a: &usize|{ a > 5 }).and_not(|&a: &usize| a < 20).or(|&a: &usize| a == 10);
//! // We now have ((a > 5) && !(a < 20) ) || a == 10
//!
//! assert_eq!(a.filter(&21), true);
//! assert_eq!(a.filter(&10), true);
//! assert_eq!(a.filter(&11), false);
//! assert_eq!(a.filter(&5), false);
//! ```
//!
//! # Ops
//!
//! Some basic operators are shipped with the library:
//!
//!  * And
//!  * Or
//!  * Not
//!
//! others are build by combining them. The operators can be used from the filter instance
//! directly, as shown in the example above.
//!
//! # Implementing own filters
//!
//! One can implement own filters by implementing the `Filter` trait. Example:
//!
//! ```
//! use filters::filter::Filter;
//! struct EqTo {
//!     pub i: usize,
//! }
//!
//! impl Filter<usize> for EqTo {
//!     fn filter(&self, n: &usize) -> bool {
//!         self.i == *n
//!     }
//! }
//!
//! fn filter_with_eqto() {
//!     let eq = EqTo { i: 0 };
//!     assert_eq!(eq.filter(&0),  true);
//!     assert_eq!(eq.filter(&1),  false);
//!     assert_eq!(eq.filter(&17), false);
//!     assert_eq!(eq.filter(&42), false);
//! }
//! ```
//!
//! # Using in Iterator::filter()
//!
//! The library is intended to be used in the `Iterator::filter()` function, but can be used in
//! other places, of course. An example for `Iterator::filter()`:
//!
//! ```
//! use filters::filter::Filter;
//! let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
//! let inrange        = (|&a: &usize| { a > 5 }).and(|&a: &usize| { a < 15 });
//! let r : Vec<usize> = v.into_iter().filter(|x| inrange.filter(x)).collect();
//! assert_eq!(r, vec![6, 7, 8, 9, 10, 11, 12, 13, 14]);
//! ```

#![doc(html_root_url = "https://matthiasbeyer.github.io/filters/")]
#![cfg_attr(feature = "unstable-filter-as-fn", feature(unboxed_closures, fn_traits))]

pub mod filter;
#[macro_use]
pub mod impl_traits;
pub mod ops;

