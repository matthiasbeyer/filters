use filter::Filter;

pub struct Or<T> {
    a: Box<Filter<T>>,
    b: Box<Filter<T>>
}

impl<T> Or<T> {

    pub fn new(a: Box<Filter<T>>, b: Box<Filter<T>>) -> Or<T> {
        Or { a: a, b: b }
    }

}

impl<T> Filter<T> for Or<T> {

    fn filter(&self, e: &T) -> bool {
        self.a.filter(e) || self.b.filter(e)
    }

}
