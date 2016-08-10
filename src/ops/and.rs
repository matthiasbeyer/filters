use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct And<T, U> {
    a: T,
    b: U
}

impl<T, U> And<T, U> {

    pub fn new(a: T, b: U) -> And<T, U> {
        And { a: a, b: b }
    }

}

impl<I, T: Filter<I>, U: Filter<I>> Filter<I> for And<T, U> {

    fn filter(&self, e: &I) -> bool {
        self.a.filter(e) && self.b.filter(e)
    }

}
