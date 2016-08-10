use filter::Filter;

pub struct And<T> {
    a: Box<Filter<T>>,
    b: Box<Filter<T>>
}

impl<T> And<T> {

    pub fn new(a: Box<Filter<T>>, b: Box<Filter<T>>) -> And<T> {
        And { a: a, b: b }
    }

}

impl<T> Filter<T> for And<T> {

    fn filter(&self, e: &T) -> bool {
        self.a.filter(e) && self.b.filter(e)
    }

}
