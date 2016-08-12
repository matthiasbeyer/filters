# filters

A library crate to build predicates and filter.

[The docs are on github pages](https://matthiasbeyer.github.io/filters/).
[![Build Status](https://travis-ci.org/matthiasbeyer/filters.svg?branch=master)](https://travis-ci.org/matthiasbeyer/filters)

Examples explain it best:

```rust
use filters::filter::Filter;

let not_eq_to_one   = |&a: &usize| { a != 1 };
let not_eq_to_two   = |&a: &usize| { a != 2 };
let not_eq_to_three = |&a: &usize| { a != 3 };

let a = not_eq_to_one.and(not_eq_to_two).and(not_eq_to_three);

assert_eq!(a.filter(&21), true);
```

For more examples have a look at the tests in `./src/filters.rs`.

# License

    filters - A crate to build predicates/filters with the builder pattern
    Copyright (C) 2016 Matthias Beyer

    This library is free software; you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as published
    by the Free Software Foundation; either version 2.1 of the License, or
    (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with this library; if not, see <http://www.gnu.org/licenses/>.

For more information and the full license text, see
[the LICENSE file](./LICENSE).
