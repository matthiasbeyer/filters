//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::error::Error;

pub use failable::ops::and::FailableAnd;
pub use failable::ops::bool::FailableBool;
pub use failable::ops::not::FailableNot;
pub use failable::ops::xor::FailableXOr;
pub use failable::ops::or::FailableOr;

/// Trait for converting something into a Filter
pub trait IntoFailableFilter<N, E: Error + Sized> {
    type IntoFilt: FailableFilter<N, E>;

    fn into_failable_filter(self) -> Self::IntoFilt;
}

/// All Filters can be turned into Filters
impl<N, E: Error + Sized, I: FailableFilter<N, E>> IntoFailableFilter<N, E> for I {
    type IntoFilt = I;

    fn into_failable_filter(self) -> Self::IntoFilt {
        self
    }
}

pub trait FailableFilter<N, E: Error> {
    /// The function which is used to filter something
    fn filter(&self, &N) -> Result<bool, E>;

    /// Helper to invert a filter.
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let f = (|&a: &usize| { a == 1 }).not();
    ///
    /// assert!(f.filter(&2));
    /// ```
    fn not(self) -> FailableNot<Self>
        where Self: Sized
    {
        FailableNot::new(self)
    }

    /// Helper to connect two filters via logical OR
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a == 1 });
    /// let b = (|&a: &usize| { a == 2 });
    /// let c = a.or(b);
    ///
    /// assert!(c.filter(&1));
    /// assert!(c.filter(&2));
    /// assert!(!c.filter(&7));
    /// ```
    fn or<F>(self, other: F) -> FailableOr<Self, F::IntoFilt>
        where Self: Sized,
              F: IntoFailableFilter<N, E> + Sized
    {
        FailableOr::new(self, other.into_failable_filter())
    }

    /// Helper to connect two filters via logical OR and NOT
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a == 1 });
    /// let b = (|&a: &usize| { a == 2 });
    /// let c = a.or_not(b);
    ///
    /// assert!(c.filter(&1));
    /// assert!(!c.filter(&2));
    /// assert!(c.filter(&7));
    /// ```
    fn or_not<F>(self, other: F) -> FailableOr<Self, FailableNot<F::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N, E> + Sized,
    {
        self.or(FailableNot::new(other.into_failable_filter()))
    }

    /// Helper to connect three filters via logical OR
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a == 1 });
    /// let b = (|&a: &usize| { a == 2 });
    /// let c = (|&a: &usize| { a == 3 });
    /// let d = a.or3(b, c);
    ///
    /// assert!(d.filter(&1));
    /// assert!(d.filter(&2));
    /// assert!(d.filter(&3));
    /// assert!(!d.filter(&4));
    /// ```
    fn or3<F, F2>(self, other: F, other2: F2) -> FailableOr<Self, FailableOr<F::IntoFilt, F2::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N, E> + Sized,
              F2: IntoFailableFilter<N, E> + Sized
    {
        FailableOr::new(self, FailableOr::new(other.into_failable_filter(), other2.into_failable_filter()))
    }

    /// Helper to connect two filters via logical NOR
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a == 1 });
    /// let b = (|&a: &usize| { a == 2 });
    /// let c = a.nor(b); /* !(a == 1 || a == 2) */
    ///
    /// assert!(!c.filter(&1));
    /// assert!(!c.filter(&2));
    /// assert!(c.filter(&3));
    /// assert!(c.filter(&4));
    /// ```
    fn nor<F>(self, other: F) -> FailableNot<FailableOr<Self, F>>
        where Self: Sized,
    {
        FailableNot::new(FailableOr::new(self, other))
    }

    /// Helper to connect two filters via logical XOR
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a > 3 });
    /// let b = (|&a: &usize| { a < 7 });
    /// let c = a.xor(b);
    ///
    /// assert!(c.filter(&1));
    /// assert!(c.filter(&3));
    /// assert!(!c.filter(&4));
    /// assert!(!c.filter(&6));
    /// assert!(c.filter(&9));
    /// ```
    fn xor<F>(self, other: F) -> FailableXOr<Self, F>
        where Self: Sized,
    {
        FailableXOr::new(self, other)
    }

    /// Helper to connect two filters via logical AND
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a > 1 });
    /// let b = (|&a: &usize| { a < 7 });
    /// let c = a.and(b);
    ///
    /// assert!(!c.filter(&1));
    /// assert!(c.filter(&3));
    /// assert!(c.filter(&4));
    /// assert!(c.filter(&6));
    /// assert!(!c.filter(&9));
    /// ```
    fn and<F>(self, other: F) -> FailableAnd<Self, F::IntoFilt>
        where Self: Sized,
              F: IntoFailableFilter<N, E> + Sized
    {
        FailableAnd::new(self, other.into_failable_filter())
    }

    /// Helper to connect three filters via logical AND
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a > 1 });
    /// let b = (|&a: &usize| { a < 20 });
    /// let c = (|&a: &usize| { a % 2 == 0 });
    /// let d = a.and3(b, c);
    ///
    /// assert!(!d.filter(&1));
    /// assert!(!d.filter(&3));
    /// assert!(d.filter(&8));
    /// assert!(d.filter(&10));
    /// assert!(d.filter(&14));
    /// assert!(!d.filter(&15));
    /// assert!(!d.filter(&19));
    /// ```
    fn and3<F, F2>(self, other: F, other2: F2) -> FailableAnd<Self, FailableAnd<F::IntoFilt, F2::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N, E> + Sized,
              F2: IntoFailableFilter<N, E> + Sized
    {
        FailableAnd::new(self, FailableAnd::new(other.into_failable_filter(), other2.into_failable_filter()))
    }

    /// Helper to connect two filters via logical AND and NOT
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a > 10 });
    /// let b = (|&a: &usize| { a < 20 });
    /// let c = a.and_not(b);
    ///
    /// assert!(!c.filter(&1));
    /// assert!(!c.filter(&3));
    /// assert!(!c.filter(&8));
    /// assert!(!c.filter(&11));
    /// assert!(c.filter(&24));
    /// assert!(c.filter(&25));
    /// assert!(c.filter(&29));
    /// ```
    fn and_not<F>(self, other: F) -> FailableAnd<Self, FailableNot<F::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N, E> + Sized
    {
        self.and(FailableNot::new(other.into_failable_filter()))
    }

    /// Helper to connect two filters via logical NAND
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a > 10 });
    /// let b = (|&a: &usize| { a < 20 });
    /// let c = a.nand(b);
    ///
    /// assert!(c.filter(&1));
    /// assert!(c.filter(&3));
    /// assert!(c.filter(&8));
    /// assert!(!c.filter(&11));
    /// assert!(!c.filter(&14));
    /// assert!(c.filter(&25));
    /// assert!(c.filter(&29));
    /// ```
    fn nand<F>(self, other: F) -> FailableNot<FailableAnd<Self, F>>
        where Self: Sized,
    {
        FailableNot::new(FailableAnd::new(self, other))
    }

}

/// All closures that take a ref to something and return Result<bool, E> are failable filters
impl<I, E: Error, T: Fn(&I) -> Result<bool, E>> FailableFilter<I, E> for T {
    fn filter(&self, other: &I) -> Result<bool, E>{
        self(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct StupError { }

    impl ::std::fmt::Display for StupError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            Ok(())
        }
    }

    impl Error for StupError {
         fn description(&self) -> &str {
             "stub"
         }
    }

    #[test]
    fn compile_test() {
        let a = |r: &i32| -> Result<bool, StupError> { Ok(true) };

        assert!(a.filter(&1).unwrap());
    }
}

