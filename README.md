# Velcro

A set of macros for conveniently initializing `Vec`, `HashMap`, `HashSet`, `BTreeMap`,
`BTreeSet` and iterators. All of the macros support the `..` operator which "spreads"
the values of another collection or iterator into the collection being initialized.

`velcro::vec!` is a drop-in replacement for `std::vec!`. All functionality of
the `std` macro is supported, but it also supports spreading values with `..`.

## Examples

```
use velcro::{hash_map, vec};

assert_eq!(vec![..(0..7)], vec![0, 1, 2, 3, 4, 5, 6]);
assert_eq!(vec![0, 1, ..(2..7)], vec![0, 1, 2, 3, 4, 5, 6]);

let other = vec![3, 4, 5];
assert_eq!(vec![0, 1, 2, ..other, 6], vec![0, 1, 2, 3, 4, 5, 6]);

let map = hash_map! {
    0: "zero",
    1: "one",
    ..other: "all of these keys have the same value",
    ..(10..20): "as do these",
};

assert_eq!(map.get(&0), Some(&"zero"));
assert_eq!(map.get(&3), Some(&"all of these keys have the same value"));
assert_eq!(map.get(&4), Some(&"all of these keys have the same value"));
assert_eq!(map.get(&10), Some(&"as do these"));
```
