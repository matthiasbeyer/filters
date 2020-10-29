# Changelog

# Next

# 0.4.0

## Breaking

* `Filter::as_failable()` was removed, as it prevented making a Filter into an
  trait object

## Other

* A cargo category ("rust-patterns") was added
* The (deprecated) `try!()` macro in the codebase was replaced by the `?`
  operator
* Unused variable warnings were fixed
* "Blacklisted" variable names were removed ("foo")


# 0.3.0

* A macro was added to quickly create `Filter` implementations
* An extension for iterators was added to filter the iterator with the filter
  object. This removes the need to write a closure.
* `FailableFilter` uses an associated type for the error output type now.

# 0.2.0

* The new "failable" filter API was introduced. Filters (more specifically
  `FailableFilter`s can be used to return a `Result<bool, E>` from a filter,
  if the filtering operation may fail due to some environmental reasons.
  All chaining operators (`and`, `or`,...) are supported, as well as
  transforming a `Filter` into a `FailableFilter`, as well as mapping error
  types from one type into another.

Versions prior to 0.2.0 are not documented in this file, have a look at the
git version history.

