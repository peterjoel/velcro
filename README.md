# Velcro

[![Build Status](https://travis-ci.com/peterjoel/velcro.svg?branch=main)](https://travis-ci.com/peterjoel/velcro)

A set of macros for conveniently initializing collections from Rust's `std` and iterators. All of the macros support the unary `..` operator which "spreads"
the values of another collection or iterator.

`velcro::vec!` is a drop-in replacement for `std::vec!`. All functionality of
the `std` macro is supported without overhead, but it also supports spreading values with the `..` operator.

## Examples

```rust
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

## Help

For help, questions or to report an issue, please use the [Github issue tracker](https://github.com/peterjoel/velcro/issues).
