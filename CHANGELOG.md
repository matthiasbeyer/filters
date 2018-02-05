# Changelog

# Next

* A macro was added to quickly create `Filter` implementations
* An extension for iterators was added to filter the iterator with the filter
  object. This removes the need to write a closure.

# 0.2.0

* The new "failable" filter API was introduced. Filters (more specifically
  `FailableFilter`s can be used to return a `Result<bool, E>` from a filter,
  if the filtering operation may fail due to some environmental reasons.
  All chaining operators (`and`, `or`,...) are supported, as well as
  transforming a `Filter` into a `FailableFilter`, as well as mapping error
  types from one type into another.

Versions prior to 0.2.0 are not documented in this file, have a look at the
git version history.

