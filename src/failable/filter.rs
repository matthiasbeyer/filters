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
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
              F: IntoFailableFilter<N, E> + Sized
    {
        FailableOr::new(self, other.into_failable_filter())
    }

    /// Helper to connect two filters via logical OR and NOT
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
              F: IntoFailableFilter<N, E> + Sized,
    {
        self.or(FailableNot::new(other.into_failable_filter()))
    }

    /// Helper to connect three filters via logical OR
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
              F: IntoFailableFilter<N, E> + Sized,
              F2: IntoFailableFilter<N, E> + Sized
    {
        FailableOr::new(self, FailableOr::new(other.into_failable_filter(), other2.into_failable_filter()))
    }

    /// Helper to connect two filters via logical NOR
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
              F: IntoFailableFilter<N, E> + Sized
    {
        FailableAnd::new(self, other.into_failable_filter())
    }

    /// Helper to connect three filters via logical AND
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
              F: IntoFailableFilter<N, E> + Sized,
              F2: IntoFailableFilter<N, E> + Sized
    {
        FailableAnd::new(self, FailableAnd::new(other.into_failable_filter(), other2.into_failable_filter()))
    }

    /// Helper to connect two filters via logical AND and NOT
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
              F: IntoFailableFilter<N, E> + Sized
    {
        self.and(FailableNot::new(other.into_failable_filter()))
    }

    /// Helper to connect two filters via logical NAND
    ///
    /// ```
    /// # #[derive(Debug)]
    /// # struct ErrorStub { }
    /// #
    /// # impl ::std::fmt::Display for ErrorStub {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl ::std::error::Error for ErrorStub {
    /// #     fn description(&self) -> &str { "stub" }
    /// # }
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
}

