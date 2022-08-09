//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! The filter implementation
//!
use std::borrow::Borrow;

pub use crate::ops::and::And;
pub use crate::ops::bool::Bool;
pub use crate::ops::failable::{AsFailable, IntoFailable};
pub use crate::ops::map::MapInput;
pub use crate::ops::not::Not;
pub use crate::ops::or::Or;
pub use crate::ops::xor::XOr;

/// Trait for converting something into a Filter
pub trait IntoFilter<N> {
    type IntoFilt: Filter<N>;

    fn into_filter(self) -> Self::IntoFilt;
}

/// All Filters can be turned into Filters
impl<N, I: Filter<N>> IntoFilter<N> for I {
    type IntoFilt = I;

    fn into_filter(self) -> I {
        self
    }
}

/// All closures that take a ref to something and return bool are filters
impl<I, T: Fn(&I) -> bool> Filter<I> for T {
    fn filter(&self, other: &I) -> bool {
        self(other)
    }
}

/// The filter trait
pub trait Filter<N> {
    /// The function which is used to filter something
    fn filter(&self, _: &N) -> bool;

    /// Helper to invert a filter.
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let f = (|&a: &usize| { a == 1 }).not();
    ///
    /// assert!(f.filter(&2));
    /// ```
    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not::new(self)
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
    fn or<F>(self, other: F) -> Or<Self, F::IntoFilt>
    where
        Self: Sized,
        F: IntoFilter<N> + Sized,
    {
        Or::new(self, other.into_filter())
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
    fn or_not<F>(self, other: F) -> Or<Self, Not<F::IntoFilt>>
    where
        Self: Sized,
        F: IntoFilter<N> + Sized,
    {
        self.or(Not::new(other.into_filter()))
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
    fn or3<F, F2>(self, other: F, other2: F2) -> Or<Self, Or<F::IntoFilt, F2::IntoFilt>>
    where
        Self: Sized,
        F: IntoFilter<N> + Sized,
        F2: IntoFilter<N> + Sized,
    {
        Or::new(self, Or::new(other.into_filter(), other2.into_filter()))
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
    fn nor<F>(self, other: F) -> Not<Or<Self, F>>
    where
        Self: Sized,
    {
        Not::new(Or::new(self, other))
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
    fn xor<F>(self, other: F) -> XOr<Self, F>
    where
        Self: Sized,
    {
        XOr::new(self, other)
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
    fn and<F>(self, other: F) -> And<Self, F::IntoFilt>
    where
        Self: Sized,
        F: IntoFilter<N> + Sized,
    {
        And::new(self, other.into_filter())
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
    fn and3<F, F2>(self, other: F, other2: F2) -> And<Self, And<F::IntoFilt, F2::IntoFilt>>
    where
        Self: Sized,
        F: IntoFilter<N> + Sized,
        F2: IntoFilter<N> + Sized,
    {
        And::new(self, And::new(other.into_filter(), other2.into_filter()))
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
    fn and_not<F>(self, other: F) -> And<Self, Not<F::IntoFilt>>
    where
        Self: Sized,
        F: IntoFilter<N> + Sized,
    {
        self.and(Not::new(other.into_filter()))
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
    fn nand<F>(self, other: F) -> Not<And<Self, F>>
    where
        Self: Sized,
    {
        Not::new(And::new(self, other))
    }

    /// Helper to transform the input of a filter
    ///
    /// ```
    /// use filters::filter::Filter;
    ///
    /// let a = (|&a: &usize| { a > 1 });
    /// let b = (|&a: &i64| { a < 7 });
    /// let b = b.map_input(|&x: &usize| { x as i64 });
    /// let c = a.and(b);
    ///
    /// assert!(!c.filter(&1));
    /// assert!(c.filter(&3));
    /// assert!(c.filter(&4));
    /// assert!(c.filter(&6));
    /// assert!(!c.filter(&9));
    /// ```
    fn map_input<O, B, T, M>(self, map: M) -> MapInput<Self, M, O, B>
    where
        Self: Sized,
        M: Fn(&T) -> N,
        B: Borrow<O> + Sized,
    {
        MapInput::new(self, map)
    }

    /// Helper to transform a filter into a FailableFilter
    ///
    /// ```
    /// use filters::filter::Filter;
    /// use filters::failable::filter::FailableFilter;
    ///
    /// let a = (|&a: &usize| { a > 5 });
    /// let a = a.into_failable();
    ///
    /// assert_eq!(a.filter(&3), Ok(false));
    /// assert_eq!(a.filter(&5), Ok(false));
    /// assert_eq!(a.filter(&7), Ok(true));
    /// assert_eq!(a.filter(&9), Ok(true));
    /// ```
    fn into_failable(self) -> IntoFailable<Self>
    where
        Self: Sized,
    {
        IntoFailable::new(self)
    }
}

#[macro_export]
macro_rules! make_filter {
    ($type:ty => $over:ty => $expression:expr) => {
        impl Filter<$over> for $type {
            fn filter(&self, element: &$over) -> bool {
                $expression(self, element)
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::filter::Filter;
    use crate::ops::and::And;
    use crate::ops::bool::Bool;

    #[test]
    fn closures() {
        let a = (|&a: &usize| a < 3).and(|&a: &usize| a > 1);

        assert_eq!(a.filter(&0), false);
        assert_eq!(a.filter(&2), true);
        assert_eq!(a.filter(&3), false);
    }

    #[test]
    fn and_filter() {
        let a = And::new(|&a: &usize| a > 0, |&a: &usize| a == 3);

        assert_eq!(a.filter(&3), true);
        assert_eq!(a.filter(&5), false);
        assert_eq!(a.filter(&0), false);
    }

    #[test]
    fn xor_filter() {
        let a = (|&a: &usize| a == 0).xor(|&a: &usize| a == 3);

        assert_eq!(a.filter(&3), true);
        assert_eq!(a.filter(&5), false);
        assert_eq!(a.filter(&0), true);
    }

    #[test]
    fn complex_filter() {
        let a = (|&a: &usize| a > 5)
            .and_not(|&a: &usize| a < 20)
            .or(|&a: &usize| a == 10);
        // We now have ((a > 5) && !(a < 20) ) || a == 10

        assert_eq!(a.filter(&21), true);
        assert_eq!(a.filter(&10), true);
        assert_eq!(a.filter(&11), false);
        assert_eq!(a.filter(&5), false);
    }

    #[test]
    fn complex_filter_closured() {
        let a = (|&a: &usize| (|&a: &usize| a > 5).and_not(|&a: &usize| a < 20).filter(&a))
            .or(|&a: &usize| a == 10);
        // We now have ((a > 5) && !(a < 20)) || a == 10

        assert_eq!(a.filter(&21), true);
        assert_eq!(a.filter(&10), true);
        assert_eq!(a.filter(&11), false);
        assert_eq!(a.filter(&5), false);
    }

    #[test]
    fn complex_filter_named_closures() {
        let not_eq_to_one = |&a: &usize| a != 1;
        let not_eq_to_two = |&a: &usize| a != 2;
        let not_eq_to_three = |&a: &usize| a != 3;

        let a = not_eq_to_one.and(not_eq_to_two).and(not_eq_to_three);
        // We now have ((a > 5) && !(a < 20)) || a == 10

        assert_eq!(a.filter(&21), true);
        assert_eq!(a.filter(&10), true);
        assert_eq!(a.filter(&1), false);
        assert_eq!(a.filter(&3), false);
    }

    #[test]
    fn filter_with_bool() {
        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.and(Bool::new(true)).filter(&0), false);

        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.and(Bool::new(true)).filter(&1), true);

        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.xor(Bool::new(true)).filter(&1), false);

        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.or(Bool::new(true)).filter(&42), true);
    }

    struct EqTo {
        pub i: usize,
    }

    impl Filter<usize> for EqTo {
        fn filter(&self, n: &usize) -> bool {
            self.i == *n
        }
    }

    #[test]
    fn filter_with_eqto() {
        let eq = EqTo { i: 0 };
        assert_eq!(eq.filter(&0), true);
        assert_eq!(eq.filter(&1), false);
        assert_eq!(eq.filter(&17), false);
        assert_eq!(eq.filter(&42), false);
    }

    #[test]
    fn filter_with_combined_eqto() {
        let aeq = EqTo { i: 1 }.not().and_not(EqTo { i: 17 });

        assert_eq!(aeq.filter(&0), true);
        assert_eq!(aeq.filter(&1), false);
        assert_eq!(aeq.filter(&2), true);
        assert_eq!(aeq.filter(&17), false);
    }

    #[test]
    fn filter_iterator() {
        let v = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];

        let inrange = (|&a: &usize| a > 5).and(|&a: &usize| a < 15);

        let r: Vec<usize> = v.into_iter().filter(|x| inrange.filter(x)).collect();

        assert_eq!(r, vec![6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }

    #[test]
    fn filter_macro_generated() {
        struct LowerThan(u64);
        make_filter! {
            LowerThan => u64 => |this: &LowerThan, e| e < &this.0
        };

        let lt = LowerThan(10);
        assert_eq!(lt.filter(&0), true);
        assert_eq!(lt.filter(&1), true);
        assert_eq!(lt.filter(&17), false);
        assert_eq!(lt.filter(&42), false);
    }
}

#[cfg(test)]
#[cfg(feature = "unstable-filter-as-fn")]
mod test_unstable {
    use filter::Filter;
    use ops::bool::Bool;

    #[test]
    fn closures() {
        let a = (|&a: &usize| a < 3).and(|&a: &usize| a > 1);

        assert_eq!(a(&0), false);
        assert_eq!(a(&2), true);
        assert_eq!(a(&3), false);
    }

    #[test]
    fn xor_filter() {
        let a = (|&a: &usize| a == 0).xor(|&a: &usize| a == 3);

        assert_eq!(a(&3), true);
        assert_eq!(a(&5), false);
        assert_eq!(a(&0), true);
    }

    #[test]
    fn complex_filter() {
        let a = (|&a: &usize| a > 5)
            .and_not(|&a: &usize| a < 20)
            .or(|&a: &usize| a == 10);
        // We now have ((a > 5) && !(a < 20) ) || a == 10

        assert_eq!(a(&21), true);
        assert_eq!(a(&11), false);
    }

    #[test]
    fn filter_with_bool() {
        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.and(Bool::new(true))(&0), false);

        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.and(Bool::new(true))(&1), true);

        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.xor(Bool::new(true))(&1), false);

        let eq = |&a: &usize| a == 1;
        assert_eq!(eq.or(Bool::new(true))(&42), true);
    }

    #[test]
    fn filter_iterator() {
        let v = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];

        let inrange = (|&a: &usize| a > 5).and(|&a: &usize| a < 15);

        let r: Vec<usize> = v.into_iter().filter(inrange).collect();

        assert_eq!(r, vec![6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }
}
