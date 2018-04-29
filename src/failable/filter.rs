//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::borrow::Borrow;

pub use failable::ops::and::FailableAnd;
pub use failable::ops::bool::FailableBool;
pub use failable::ops::not::FailableNot;
pub use failable::ops::xor::FailableXOr;
pub use failable::ops::or::FailableOr;
pub use failable::ops::map::{FailableMapInput, FailableMapErr};

/// Trait for converting something into a Filter
pub trait IntoFailableFilter<N> {
    type IntoFilt: FailableFilter<N>;

    fn into_failable_filter(self) -> Self::IntoFilt;
}

/// All Filters can be turned into Filters
impl<N, I: FailableFilter<N>> IntoFailableFilter<N> for I {
    type IntoFilt = I;

    fn into_failable_filter(self) -> Self::IntoFilt {
        self
    }
}

pub trait FailableFilter<N> {
    type Error: Sized;

    /// The function which is used to filter something
    fn filter(&self, &N) -> Result<bool, Self::Error>;

    /// Helper to invert a filter.
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let f = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 1) }).not();
    ///
    /// assert!(f.filter(&2).unwrap());
    /// ```
    fn not(self) -> FailableNot<Self>
        where Self: Sized
    {
        FailableNot::new(self)
    }

    /// Helper to connect two filters via logical OR
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 1) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 2) });
    /// let c = a.or(b);
    ///
    /// assert!(c.filter(&1).unwrap());
    /// assert!(c.filter(&2).unwrap());
    /// assert!(!c.filter(&7).unwrap());
    /// ```
    fn or<F>(self, other: F) -> FailableOr<Self, F::IntoFilt>
        where Self: Sized,
              F: IntoFailableFilter<N> + Sized
    {
        FailableOr::new(self, other.into_failable_filter())
    }

    /// Helper to connect two filters via logical OR and NOT
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 1) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 2) });
    /// let c = a.or_not(b);
    ///
    /// assert!(c.filter(&1).unwrap());
    /// assert!(!c.filter(&2).unwrap());
    /// assert!(c.filter(&7).unwrap());
    /// ```
    fn or_not<F>(self, other: F) -> FailableOr<Self, FailableNot<F::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N> + Sized,
    {
        self.or(FailableNot::new(other.into_failable_filter()))
    }

    /// Helper to connect three filters via logical OR
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 1) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 2) });
    /// let c = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 3) });
    /// let d = a.or3(b, c);
    ///
    /// assert!(d.filter(&1).unwrap());
    /// assert!(d.filter(&2).unwrap());
    /// assert!(d.filter(&3).unwrap());
    /// assert!(!d.filter(&4).unwrap());
    /// ```
    fn or3<F, F2>(self, other: F, other2: F2) -> FailableOr<Self, FailableOr<F::IntoFilt, F2::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N> + Sized,
              F2: IntoFailableFilter<N> + Sized
    {
        FailableOr::new(self, FailableOr::new(other.into_failable_filter(), other2.into_failable_filter()))
    }

    /// Helper to connect two filters via logical NOR
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 1) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a == 2) });
    /// let c = a.nor(b); /* !(a == 1 || a == 2) */
    ///
    /// assert!(!c.filter(&1).unwrap());
    /// assert!(!c.filter(&2).unwrap());
    /// assert!(c.filter(&3).unwrap());
    /// assert!(c.filter(&4).unwrap());
    /// ```
    fn nor<F>(self, other: F) -> FailableNot<FailableOr<Self, F>>
        where Self: Sized,
    {
        FailableNot::new(FailableOr::new(self, other))
    }

    /// Helper to connect two filters via logical XOR
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a > 3) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a < 7) });
    /// let c = a.xor(b);
    ///
    /// assert!(c.filter(&1).unwrap());
    /// assert!(c.filter(&3).unwrap());
    /// assert!(!c.filter(&4).unwrap());
    /// assert!(!c.filter(&6).unwrap());
    /// assert!(c.filter(&9).unwrap());
    /// ```
    fn xor<F>(self, other: F) -> FailableXOr<Self, F>
        where Self: Sized,
    {
        FailableXOr::new(self, other)
    }

    /// Helper to connect two filters via logical AND
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a > 1) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a < 7) });
    /// let c = a.and(b);
    ///
    /// assert!(!c.filter(&1).unwrap());
    /// assert!(c.filter(&3).unwrap());
    /// assert!(c.filter(&4).unwrap());
    /// assert!(c.filter(&6).unwrap());
    /// assert!(!c.filter(&9).unwrap());
    /// ```
    fn and<F>(self, other: F) -> FailableAnd<Self, F::IntoFilt>
        where Self: Sized,
              F: IntoFailableFilter<N> + Sized
    {
        FailableAnd::new(self, other.into_failable_filter())
    }

    /// Helper to connect three filters via logical AND
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a > 1) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a < 20) });
    /// let c = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a % 2 == 0) });
    /// let d = a.and3(b, c);
    ///
    /// assert!(!d.filter(&1).unwrap());
    /// assert!(!d.filter(&3).unwrap());
    /// assert!(d.filter(&8).unwrap());
    /// assert!(d.filter(&10).unwrap());
    /// assert!(d.filter(&14).unwrap());
    /// assert!(!d.filter(&15).unwrap());
    /// assert!(!d.filter(&19).unwrap());
    /// ```
    fn and3<F, F2>(self, other: F, other2: F2) -> FailableAnd<Self, FailableAnd<F::IntoFilt, F2::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N> + Sized,
              F2: IntoFailableFilter<N> + Sized
    {
        FailableAnd::new(self, FailableAnd::new(other.into_failable_filter(), other2.into_failable_filter()))
    }

    /// Helper to connect two filters via logical AND and NOT
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a > 10) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a < 20) });
    /// let c = a.and_not(b);
    ///
    /// assert!(!c.filter(&1).unwrap());
    /// assert!(!c.filter(&3).unwrap());
    /// assert!(!c.filter(&8).unwrap());
    /// assert!(!c.filter(&11).unwrap());
    /// assert!(c.filter(&24).unwrap());
    /// assert!(c.filter(&25).unwrap());
    /// assert!(c.filter(&29).unwrap());
    /// ```
    fn and_not<F>(self, other: F) -> FailableAnd<Self, FailableNot<F::IntoFilt>>
        where Self: Sized,
              F: IntoFailableFilter<N> + Sized
    {
        self.and(FailableNot::new(other.into_failable_filter()))
    }

    /// Helper to connect two filters via logical NAND
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a > 10) });
    /// let b = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a < 20) });
    /// let c = a.nand(b);
    ///
    /// assert!(c.filter(&1).unwrap());
    /// assert!(c.filter(&3).unwrap());
    /// assert!(c.filter(&8).unwrap());
    /// assert!(!c.filter(&11).unwrap());
    /// assert!(!c.filter(&14).unwrap());
    /// assert!(c.filter(&25).unwrap());
    /// assert!(c.filter(&29).unwrap());
    /// ```
    fn nand<F>(self, other: F) -> FailableNot<FailableAnd<Self, F>>
        where Self: Sized,
    {
        FailableNot::new(FailableAnd::new(self, other))
    }

    /// Helper to transform the input of a filter
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a > 1) });
    /// let b = (|&a: &i64| -> Result<bool, ErrorStub> { Ok(a < 7) });
    /// let b = b.map_input(|&x: &usize| x as i64);
    /// let c = a.and(b);
    ///
    /// assert!(!c.filter(&1).unwrap());
    /// assert!(c.filter(&3).unwrap());
    /// assert!(c.filter(&4).unwrap());
    /// assert!(c.filter(&6).unwrap());
    /// assert!(!c.filter(&9).unwrap());
    /// ```
    fn map_input<O, B, T, M>(self, map: M) -> FailableMapInput<Self, M, O, B>
        where Self: Sized,
              M: Fn(&T) -> N,
              B: Borrow<O> + Sized
    {
        FailableMapInput::new(self, map)
    }

    /// Helper to transform the input of a filter
    ///
    /// ```
    /// # use std::fmt;
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| -> Result<bool, ErrorStub> { Ok(a > 1) });
    /// let b = (|&a: &usize| -> Result<bool, fmt::Error> { Ok(a < 7) });
    /// let b = b.map_err(|_: fmt::Error| ErrorStub {});
    /// let c = a.and(b);
    ///
    /// assert!(!c.filter(&1).unwrap());
    /// assert!(c.filter(&3).unwrap());
    /// assert!(c.filter(&4).unwrap());
    /// assert!(c.filter(&6).unwrap());
    /// assert!(!c.filter(&9).unwrap());
    /// ```
    fn map_err<M, OE>(self, map: M) -> FailableMapErr<Self, M, OE>
        where Self: Sized,
              M: Fn(Self::Error) -> OE
    {
        FailableMapErr::new(self, map)
    }

}

/// All closures that take a ref to something and return Result<bool, E> are failable filters
impl<I, E, T> FailableFilter<I> for T
    where T: Fn(&I) -> Result<bool, E>
{
    type Error = E;

    fn filter(&self, other: &I) -> Result<bool, Self::Error> {
        self(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct StupError { }

    #[test]
    fn compile_test() {
        let a = |r: &i32| -> Result<bool, StupError> { Ok(true) };

        assert!(a.filter(&1).unwrap());
    }

    #[test]
    fn test_error_return() {
        let a = |_: &i32| -> Result<bool, StupError> { Err(StupError {}) };

        assert!(a.filter(&1).is_err());
    }

    #[test]
    fn test_error_return_and_chained() {
        let a = |_: &i32| -> Result<bool, StupError> { Err(StupError {}) };
        let b = |_: &i32| -> Result<bool, StupError> { Ok(true) };
        let c = |_: &i32| -> Result<bool, StupError> { Ok(true) };
        let d = |_: &i32| -> Result<bool, StupError> { Ok(true) };

        let e = d.and(b).and(c).and(a);

        assert!(e.filter(&1).is_err());
    }

    #[test]
    fn test_error_return_or_chained() {
        let a = |_: &i32| -> Result<bool, StupError> { Err(StupError {}) };
        let b = |_: &i32| -> Result<bool, StupError> { Ok(true) };
        let c = |_: &i32| -> Result<bool, StupError> { Ok(true) };
        let d = |_: &i32| -> Result<bool, StupError> { Ok(true) };

        let e = a.or(b).or(c).or(d);

        assert!(e.filter(&1).is_err());
    }

    #[test]
    fn test_both_filter_types() {
        use filter::Filter;

        let a = |_: &i32| -> Result<bool, StupError> { Ok(true) };
        let b = |_: &i32| -> bool { true };
        let c = |_: &i32| -> Result<bool, StupError> { Ok(true) };
        let d = |_: &i32| -> bool { false };

        let e = a                                               // true
            .and(b.into_failable().map_err(|_| StupError {}))   // true
            .xor(c)                                             // true
            .or(d.into_failable().map_err(|_| StupError {}));   // true

        assert!(!e.filter(&1).unwrap());
    }


    #[test]
    fn test_both_filter_types_in_one_scope() {
        use filter::Filter;
        use failable::filter::FailableFilter;

        let failable   = |_: &i32| -> Result<bool, StupError> { Ok(true) };
        let unfailable = |_: &i32| -> bool { true };

        assert!(failable.filter(&1).unwrap());
        assert!(unfailable.filter(&1));

    }
}

