use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Not<T> {
    a: T
}

impl<T> Not<T> {

    pub fn new(a: T) -> Not<T> {
        Not { a: a }
    }

}

impl<I, T: Filter<I>> Filter<I> for Not<T> {

    fn filter(&self, e: &I) -> bool {
        !self.a.filter(e)
    }

}
