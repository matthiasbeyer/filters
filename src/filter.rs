pub use ops::and::And;
pub use ops::not::Not;
pub use ops::or::Or;

pub trait IntoFilter<N> {
    type IntoFilt: Filter<N>;

    fn into_filter(self) -> Self::IntoFilt;
}

impl<N, I: Filter<N>> IntoFilter<N> for I {
    type IntoFilt = I;

    fn into_filter(self) -> I {
        self
    }
}

impl<I, T: Fn(&I) -> bool> Filter<I> for T {
    fn filter(&self, other: &I) -> bool {
        self(other)
    }
}

pub trait Filter<N> {

    fn filter(&self, &N) -> bool;

    fn not(self) -> Not<Self>
        where Self: Sized
    {
        Not::new(self)
    }

    fn or<F>(self, other: F) -> Or<Self, F::IntoFilt>
        where Self: Sized,
              F: IntoFilter<N> + Sized
    {
        Or::new(self, other.into_filter())
    }

    fn or_not<F>(self, other: F) -> Or<Self, Not<F::IntoFilt>>
        where Self: Sized,
              F: IntoFilter<N> + Sized,
    {
        self.or(Not::new(other.into_filter()))
    }

    fn or3<F, F2>(self, other: F, other2: F2) -> Or<Self, Or<F::IntoFilt, F2::IntoFilt>>
        where Self: Sized,
              F: IntoFilter<N> + Sized,
              F2: IntoFilter<N> + Sized
    {
        Or::new(self, Or::new(other.into_filter(), other2.into_filter()))
    }

    fn and<F>(self, other: F) -> And<Self, F::IntoFilt>
        where Self: Sized,
              F: IntoFilter<N> + Sized
    {
        And::new(self, other.into_filter())
    }

    fn and3<F, F2>(self, other: F, other2: F2) -> And<Self, And<F::IntoFilt, F2::IntoFilt>>
        where Self: Sized,
              F: IntoFilter<N> + Sized,
              F2: IntoFilter<N> + Sized
    {
        And::new(self, And::new(other.into_filter(), other2.into_filter()))
    }

    fn and_not<F>(self, other: F) -> And<Self, Not<F::IntoFilt>>
        where Self: Sized,
              F: IntoFilter<N> + Sized
    {
        self.and(Not::new(other.into_filter()))
    }


}


#[cfg(test)]
mod test {
    use filter::Filter;
    use ops::and::And;

    #[test]
    fn closures() {
        let a = (|&a: &usize|{ a < 3 }).and(|&a: &usize| a > 1);

        assert_eq!(a.filter(&2), true);
    }

    #[test]
    fn and_filter() {
        let a = And::new(|&a: &usize| a > 0, |&a: &usize| a == 3);

        assert_eq!(a.filter(&3),  true);
        assert_eq!(a.filter(&5),  false);
        assert_eq!(a.filter(&0), false);
    }

    #[test]
    fn complex_filter() {
        let a = (|&a: &usize|{ a > 5 }).and_not(|&a: &usize| a < 20).or(|&a: &usize| a == 10);
        // We now have ((a > 5) && !(a < 20) ) || a == 10

        assert_eq!(a.filter(&21), true);
        assert_eq!(a.filter(&10), true);
        assert_eq!(a.filter(&11), false);
        assert_eq!(a.filter(&5), false);
    }

    #[test]
    fn complex_filter_closured() {
        let a = (|&a: &usize| (|&a: &usize|{ a > 5 }).and_not(|&a: &usize| a < 20).filter(&a)).or(|&a: &usize| a == 10);
        // We now have ((a > 5) && !(a < 20)) || a == 10

        assert_eq!(a.filter(&21), true);
        assert_eq!(a.filter(&10), true);
        assert_eq!(a.filter(&11), false);
        assert_eq!(a.filter(&5), false);
    }

    #[test]
    fn complex_filter_named_closures() {
        let not_eq_to_one   = |&a: &usize| { a != 1 };
        let not_eq_to_two   = |&a: &usize| { a != 2 };
        let not_eq_to_three = |&a: &usize| { a != 3 };

        let a = not_eq_to_one.and(not_eq_to_two).and(not_eq_to_three);
        // We now have ((a > 5) && !(a < 20)) || a == 10

        assert_eq!(a.filter(&21), true);
        assert_eq!(a.filter(&10), true);
        assert_eq!(a.filter(&1), false);
        assert_eq!(a.filter(&3), false);
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
        assert_eq!(eq.filter(&0),  true);
        assert_eq!(eq.filter(&1),  false);
        assert_eq!(eq.filter(&17), false);
        assert_eq!(eq.filter(&42), false);
    }

    #[test]
    fn filter_with_combined_eqto() {
        let aeq = EqTo { i: 1 }.not().and_not(EqTo { i: 17 });

        assert_eq!(aeq.filter(&0),  true);
        assert_eq!(aeq.filter(&1),  false);
        assert_eq!(aeq.filter(&2),  true);
        assert_eq!(aeq.filter(&17), false);
    }

    #[test]
    fn filter_iterator() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        let inrange        = (|&a: &usize| { a > 5 }).and(|&a: &usize| { a < 15 });

        let r : Vec<usize> = v.into_iter().filter(|x| inrange.filter(x)).collect();

        assert_eq!(r, vec![6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }
}
