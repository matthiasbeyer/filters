//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use filter::Filter;

pub struct FilteredIterator<T, F, I>(F, I)
    where F: Filter<T>,
          I: Iterator<Item = T>;

impl<T, F, I> Iterator for FilteredIterator<T, F, I>
    where F: Filter<T>,
          I: Iterator<Item = T>
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

pub trait FilterWith<T, F: Filter<T>> : Iterator<Item = T> + Sized {
    fn filter_with(self, f: F) -> FilteredIterator<T, F, Self>;
}

impl<I, T, F: Filter<T>> FilterWith<T, F> for I
    where I: Iterator<Item = T>
{
    fn filter_with(self, f: F) -> FilteredIterator<T, F, Self> {
        FilteredIterator(f, self)
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

        let foo = Foo;

        let v : Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
            .into_iter()
            .filter_with(foo)
            .collect();

        assert_eq!(v, vec![6, 7, 8, 9]);
    }
}
