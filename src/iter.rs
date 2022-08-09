//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use crate::filter::Filter;

pub struct FilteredIterator<T, F, I>(F, I)
where
    F: Filter<T>,
    I: Iterator<Item = T>;

impl<T, F, I> Iterator for FilteredIterator<T, F, I>
where
    F: Filter<T>,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.1.next() {
            if self.0.filter(&next) {
                return Some(next);
            }
        }

        None
    }
}

pub trait FilterWith<T, F: Filter<T>>: Iterator<Item = T> + Sized {
    fn filter_with(self, f: F) -> FilteredIterator<T, F, Self>;
}

impl<I, T, F: Filter<T>> FilterWith<T, F> for I
where
    I: Iterator<Item = T>,
{
    fn filter_with(self, f: F) -> FilteredIterator<T, F, Self> {
        FilteredIterator(f, self)
    }
}

pub struct FilterOksIter<T, E, I, F>(I, F)
where
    F: Filter<T>,
    I: Iterator<Item = Result<T, E>>;

impl<T, E, I, F> Iterator for FilterOksIter<T, E, I, F>
where
    F: Filter<T>,
    I: Iterator<Item = Result<T, E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            match next {
                Err(e) => return Some(Err(e)),
                Ok(t) => {
                    if self.1.filter(&t) {
                        return Some(Ok(t));
                    }
                }
            }
        }

        None
    }
}

pub trait FilterOks<T, E, I, F>: Iterator<Item = Result<T, E>>
where
    I: Iterator<Item = Result<T, E>>,
    F: Filter<T>,
{
    fn filter_oks(self, _: F) -> FilterOksIter<T, E, I, F>;
}

impl<T, E, I, F> FilterOks<T, E, I, F> for I
where
    I: Iterator<Item = Result<T, E>>,
    F: Filter<T>,
{
    fn filter_oks(self, f: F) -> FilterOksIter<T, E, I, F> {
        FilterOksIter(self, f)
    }
}

pub struct FilterErrIter<T, E, I, F>(I, F)
where
    F: Filter<E>,
    I: Iterator<Item = Result<T, E>>;

impl<T, E, I, F> Iterator for FilterErrIter<T, E, I, F>
where
    F: Filter<E>,
    I: Iterator<Item = Result<T, E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            match next {
                Ok(t) => return Some(Ok(t)),
                Err(e) => {
                    if self.1.filter(&e) {
                        return Some(Err(e));
                    }
                }
            }
        }

        None
    }
}

pub trait FilterErr<T, E, I, F>: Iterator<Item = Result<T, E>>
where
    I: Iterator<Item = Result<T, E>>,
    F: Filter<E>,
{
    fn filter_errs(self, _: F) -> FilterErrIter<T, E, I, F>;
}

impl<T, E, I, F> FilterErr<T, E, I, F> for I
where
    I: Iterator<Item = Result<T, E>>,
    F: Filter<E>,
{
    fn filter_errs(self, f: F) -> FilterErrIter<T, E, I, F> {
        FilterErrIter(self, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filter_with() {
        struct Foo;
        impl Filter<u64> for Foo {
            fn filter(&self, u: &u64) -> bool {
                *u > 5
            }
        }

        let f = Foo;

        let v: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
            .into_iter()
            .filter_with(f)
            .collect();

        assert_eq!(v, vec![6, 7, 8, 9]);
    }

    #[test]
    fn test_filter_oks() {
        struct Foo;
        impl Filter<u64> for Foo {
            fn filter(&self, u: &u64) -> bool {
                *u > 5
            }
        }

        let f = Foo;

        let v: Vec<Result<u64, u64>> = vec![
            Ok(1),
            Err(2),
            Ok(3),
            Err(4),
            Ok(5),
            Err(6),
            Ok(7),
            Err(8),
            Ok(9),
            Err(0),
        ]
        .into_iter()
        .filter_oks(f)
        .collect();

        assert_eq!(
            v,
            vec![Err(2), Err(4), Err(6), Ok(7), Err(8), Ok(9), Err(0)]
        );
    }

    #[test]
    fn test_filter_errs() {
        struct Foo;
        impl Filter<u64> for Foo {
            fn filter(&self, u: &u64) -> bool {
                *u > 5
            }
        }

        let f = Foo;

        let v: Vec<Result<u64, u64>> = vec![
            Ok(1),
            Err(2),
            Ok(3),
            Err(4),
            Ok(5),
            Err(6),
            Ok(7),
            Err(8),
            Ok(9),
            Err(0),
        ]
        .into_iter()
        .filter_errs(f)
        .collect();

        assert_eq!(v, vec![Ok(1), Ok(3), Ok(5), Err(6), Ok(7), Err(8), Ok(9)]);
    }
}
