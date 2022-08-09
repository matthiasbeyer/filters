# filters

A library crate to build predicates and filter.

[_Documentation_](https://docs.rs/filters/).

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

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

For more information and the full license text, see
[the LICENSE file](./LICENSE).
