pub trait Filter<T> {

    fn filter(&self, &T) -> bool;

}
