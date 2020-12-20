# Velcro

[![Build Status](https://travis-ci.com/peterjoel/velcro.svg?branch=main)](https://travis-ci.com/peterjoel/velcro)

A set of macros for conveniently initializing collections from Rust's `std` and iterators. All of the macros support the unary `..` operator which "spreads"
the values of another collection or iterator.

`velcro::vec!` is a drop-in replacement for `std::vec!`. All functionality of
the `std` macro is supported without overhead, but it also supports spreading values with the `..` operator.

## Examples

```
use velcro::{hash_map, iter, vec};

assert_eq!(vec![0, 1, ..(2..7)], vec![0, 1, 2, 3, 4, 5, 6]);

let other = vec![3, 4, 5];
assert_eq!(vec![0, 1, 2, ..&other, 6], vec![0, 1, 2, 3, 4, 5, 6]);

let whitespace = iter![' ', '\t', '\r', '\n'];
let map = hash_map! {
    ..('0'..='9'): "digit",
    ..('a'..='z'): "lower",
    ..('A'..='Z'): "upper",
    ..whitespace: "whitespace",
    '.': "punctuation",
    ',': "punctuation",
};

assert_eq!(map[&'x'], "lower");
assert_eq!(map[&'\r'], "whitespace");
assert_eq!(map[&'.'], "punctuation");
```

## Contributing

Contributions are welcome! Check the [Github issue tracker](https://github.com/peterjoel/velcro/issues)
for issues marked with [`good first issue`](https://github.com/peterjoel/velcro/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
or [`help wanted`](https://github.com/peterjoel/velcro/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
for issues that are reasonably complete in their description. Feel free to ask for
help or clarification by leaving comments on the issue.

This project uses Travis for continuous integration. Please check that your changes
build and all of the tests pass:

- https://travis-ci.com/github/peterjoel/velcro

## Help

For help, questions or to report an issue, please use the [Github issue tracker](https://github.com/peterjoel/velcro/issues).
