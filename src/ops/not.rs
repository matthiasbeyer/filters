use filter::Filter;

pub struct Not<T> {
    a: Box<Filter<T>>
}

impl<T> Not<T> {

    pub fn new(a: Box<Filter<T>>) -> Not<T> {
        Not { a: a }
    }

}

impl<T> Filter<T> for Not<T> {

    fn filter(&self, e: &T) -> bool {
        !self.a.filter(e)
    }

}
