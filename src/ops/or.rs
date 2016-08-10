use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Or<T, U> {
    a: T,
    b: U
}

impl<T, U> Or<T, U> {

    pub fn new(a: T, b: U) -> Or<T, U> {
        Or { a: a, b: b }
    }

}

impl<I, T: Filter<I>, U: Filter<I>> Filter<I> for Or<T, U> {

    fn filter(&self, e: &I) -> bool {
        self.a.filter(e) || self.b.filter(e)
    }

}
