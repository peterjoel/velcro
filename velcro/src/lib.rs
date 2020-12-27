//! # Velcro
//!
//! [![Build Status](https://travis-ci.com/peterjoel/velcro.svg?branch=main)](https://travis-ci.com/peterjoel/velcro)
//!
//! A set of macros for conveniently initializing collections from Rust's `std` and iterators. All of the macros support the unary `..` operator which "spreads"
//! the values of another collection or iterator.
//!
//! `velcro::vec!` is a drop-in replacement for `std::vec!`. All functionality of
//! the `std` macro is supported without overhead, but it also supports spreading values with the `..` operator.
//!
//! ## Examples
//!
//! ```
//! use velcro::{hash_map, iter, vec};
//!
//! assert_eq!(vec![0, 1, ..(2..7)], vec![0, 1, 2, 3, 4, 5, 6]);
//!
//! let other = vec![3, 4, 5];
//! assert_eq!(vec![0, 1, 2, ..&other, 6], vec![0, 1, 2, 3, 4, 5, 6]);
//!
//! let whitespace = iter![' ', '\t', '\r', '\n'];
//! let map = hash_map! {
//!     ..('0'..='9'): "digit",
//!     ..('a'..='z'): "lower",
//!     ..('A'..='Z'): "upper",
//!     ..whitespace: "whitespace",
//!     '.': "punctuation",
//!     ',': "punctuation",
//! };
//!
//! assert_eq!(map[&'x'], "lower");
//! assert_eq!(map[&'\r'], "whitespace");
//! assert_eq!(map[&'.'], "punctuation");
//! ```
//!
//! ## Contributing
//!
//! Contributions are welcome! Check the [Github issue tracker](https://github.com/peterjoel/velcro/issues)
//! for issues marked with [`good first issue`](https://github.com/peterjoel/velcro/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
//! or [`help wanted`](https://github.com/peterjoel/velcro/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
//! for issues that are reasonably complete in their description. Feel free to ask for
//! help or clarification by leaving comments on the issue.
//!
//! This project uses Travis for continuous integration. Please check that your changes
//! build and all of the tests pass:
//!
//! - https://travis-ci.com/github/peterjoel/velcro
//!
//! ## Help
//!
//! For help, questions or to report an issue, please use the [Github issue tracker](https://github.com/peterjoel/velcro/issues).

/// A more flexible vector initialization macro. `velcro::vec!` is a
/// drop-in replacement for the built-in `std::vec!` macro, but with extra
/// functionality. In particular, it adds the `..` spread operator, which
/// can insert multiple elements at once, provided that the expression
/// implements `IntoIterator`, which is the case for all iterators and most
/// collections.
///
/// # Usage
///
/// ```rust
/// use velcro::vec;
///
/// assert_eq!(vec![..(0..7)], vec![0, 1, 2, 3, 4, 5, 6]);
/// assert_eq!(vec![0, 1, ..(2..7)], vec![0, 1, 2, 3, 4, 5, 6]);
///
/// let other = vec![3, 4, 5];
/// assert_eq!(vec![0, 1, 2, ..other, 6], vec![0, 1, 2, 3, 4, 5, 6]);
///
/// let mut it = (0..=3).into_iter().map(|x| x + 2);
/// assert_eq!(vec![0, 1, ..it, 6], vec![0, 1, 2, 3, 4, 5, 6]);
///
/// assert_eq!(vec![3; 5], vec![3, 3, 3, 3, 3]);
/// let range = 3..;
/// assert_eq!(vec![..range; 5], vec![3, 4, 5, 6, 7]);
/// // Note that this is different from not using `..`
/// let range = 3..;
/// assert_eq!(vec![range; 5], vec![3.., 3.., 3.., 3.., 3..]);
///
/// // If the requested length is less than the input length then the
/// // result will be shorter.
/// let range = 2..5;
/// assert_eq!(vec![..range; 5], vec![2, 3, 4]);
/// ```
///
/// # Performance
///
/// For syntax that is supported by `std::vec!`, `velco::vec!` performs the same,
/// since it delegates to `std::vec!` wherever the input is compatible. That is,
/// if you don't use the `..` spread operator, you don't pay for it.
pub use velcro_macros::vec;

/// Works the same as `vec!` except that values may be of any type that can be
/// converted into the item type via an implementation of `Into`.
///
/// The type of the item must be known at compile time, and usually this means an
/// explicit type annotation is required.
///
/// # Usage
///
/// ```rust
/// use velcro::vec_from;
///
/// #[derive(Debug, PartialEq)]
/// struct Foo(u64);
///
/// impl From<u64> for Foo {
///     fn from(other: u64) -> Self {
///         Foo(other)
///     }
/// }
///
/// let foos: Vec<Foo> = vec_from![1, 2, Foo(3), ..(4..=6), 7];
/// assert_eq!(foos, vec![Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7)]);
/// ```
pub use velcro_macros::vec_from;

/// An initializer for `BTreeSet`, allowing for items to be specified individually
/// or "spread" using the `..` operator.
///
/// # Usage
///
/// ```rust
/// # use std::collections::BTreeSet;
/// use velcro::btree_set;
/// let set: BTreeSet<_> = (0..7).into_iter().collect();
///
/// assert_eq!(btree_set![..(0..7)], set);
/// assert_eq!(btree_set![0, 1, ..(2..7)], set);
///```
pub use velcro_macros::btree_set;

/// An initializer for `BTreeSet` that works the same as `btree_set!` except that
/// values can be of any type that can be converted into the collection's item type.
///
/// The type of the item must be known at compile time, and usually this means an
/// explicit type annotation is required.
///
/// # Usage
///
/// ```rust
/// # use std::collections::BTreeSet;
/// use velcro::{btree_set, btree_set_from};
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct Foo(u64);
///
/// impl From<u64> for Foo {
///     fn from(other: u64) -> Self {
///         Foo(other)
///     }
/// }
///
/// let foos: BTreeSet<Foo> = btree_set_from![1, 2, Foo(3), ..(4..=6), 7];
/// assert_eq!(foos, btree_set![Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7)]);
///```
pub use velcro_macros::btree_set_from;

/// An initializer for `HashSet`, allowing for items to be specified individually
/// or "spread" using the `..` operator.
///
/// # Usage
///
/// ```rust
/// # use std::collections::HashSet;
/// use velcro::hash_set;
/// let set: HashSet<_> = (0..7).into_iter().collect();
///
/// assert_eq!(hash_set![..(0..7)], set);
/// assert_eq!(hash_set![0, 1, ..(2..7)], set);
///```
pub use velcro_macros::hash_set;

/// An initializer for `HashSet` that works the same as `hash_set!` except that
/// values can be of any type that can be converted into the collection's item
/// type via an `Into` implementation.
///
/// The type of the item must be known at compile time, and usually this means an
/// explicit type annotation is required.
///
/// # Usage
///
/// ```rust
/// # use std::collections::HashSet;
/// use velcro::{hash_set, hash_set_from};
///
/// #[derive(Debug, PartialEq, Eq, Hash)]
/// struct Foo(u64);
///
/// impl From<u64> for Foo {
///     fn from(other: u64) -> Self {
///         Foo(other)
///     }
/// }
///
/// let foos: HashSet<Foo> = hash_set_from![1, 2, Foo(3), ..(4..=6), 7];
/// assert_eq!(foos, hash_set![Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7)]);
///```
pub use velcro_macros::hash_set_from;

/// An initializer for `LinkedList`, allowing for items to be specified individually
/// or "spread" using the `..` operator.
///
/// # Usage
///
/// ```rust
/// # use std::collections::LinkedList;
/// use velcro::linked_list;
/// let list: LinkedList<_> = (0..7).into_iter().collect();
///
/// assert_eq!(linked_list![..(0..7)], list);
/// assert_eq!(linked_list![0, 1, ..(2..7)], list);
///```
pub use velcro_macros::linked_list;

/// An initializer for `LinkedList` that works the same as `linked_list!` except that
/// values can be of any type that can be converted into the collection's item
/// type via an `Into` implementation.
///
/// The type of the item must be known at compile time, and usually this means an
/// explicit type annotation is required.
///
/// # Usage
///
/// ```rust
/// # use std::collections::LinkedList;
/// use velcro::{linked_list, linked_list_from};
///
/// #[derive(Debug, PartialEq, Eq, Hash)]
/// struct Foo(u64);
///
/// impl From<u64> for Foo {
///     fn from(other: u64) -> Self {
///         Foo(other)
///     }
/// }
///
/// let foos: LinkedList<Foo> = linked_list_from![1, 2, Foo(3), ..(4..=6), 7];
/// assert_eq!(foos, linked_list![Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7)]);
///```
pub use velcro_macros::linked_list_from;

/// An initializer for `HashMap`, allowing for entries to be specified individually
/// or for the same value to be given to multiple keys using the `..` operator.
///
/// # Usage
///
/// ```rust
/// # use std::collections::HashMap;
/// use velcro::hash_map;
/// let mut map1 = HashMap::new();
/// map1.insert('a', 0);
/// map1.insert('b', 1);
/// map1.insert('c', 1);
/// map1.insert('d', 1);
/// map1.insert('e', 1);
/// map1.insert('f', 2);
///
/// let map2 = hash_map! {
///     'a': 0,
///     ..('b'..='e'): 1,
///     'f': 2
/// };
///
/// assert_eq!(map1, map2);
///```
pub use velcro_macros::hash_map;

/// An initializer for `HashMap` that works the same as `hash_map!` except that
/// values can be of any type that can be converted into the collection's item
/// type via an `Into` implementation.
///
/// The type of the item must be known at compile time, and usually this means an
/// explicit type annotation is required.
///
/// # Usage
///
/// ```rust
/// # use std::collections::HashMap;
/// use velcro::hash_map_from;
///
/// #[derive(Debug, PartialEq, Eq, Hash)]
/// struct Foo(u64);
///
/// impl From<u64> for Foo {
///     fn from(other: u64) -> Self {
///         Foo(other)
///     }
/// }
///
/// let mut map1 = HashMap::new();
/// map1.insert('a', Foo(0));
/// map1.insert('b', Foo(1));
/// map1.insert('c', Foo(1));
/// map1.insert('d', Foo(1));
/// map1.insert('e', Foo(1));
/// map1.insert('f', Foo(2));
///
/// let map2: HashMap<char, Foo> = hash_map_from! {
///     'a': 0,
///     ..('b'..='e'): 1,
///     'f': 2
/// };
///
/// assert_eq!(map1, map2);
///```
pub use velcro_macros::hash_map_from;

/// An initializer for `BTreeMap`, allowing for entries to be specified individually
/// or for the same value to be given to multiple keys using the `..` operator.
///
/// # Usage
///
/// ```rust
/// # use std::collections::BTreeMap;
/// use velcro::btree_map;
///
/// let mut map1 = BTreeMap::new();
/// map1.insert('a', 0);
/// map1.insert('b', 1);
/// map1.insert('c', 1);
/// map1.insert('d', 1);
/// map1.insert('e', 1);
/// map1.insert('f', 2);
///
/// let map2 = btree_map! {
///     'a': 0,
///     ..('b'..='e'): 1,
///     'f': 2
/// };
///
/// assert_eq!(map1, map2);
///```
pub use velcro_macros::btree_map;

/// An initializer for `BTreeMap` that works the same as `btree_map!` except that
/// values can be of any type that can be converted into the collection's item
/// type via an `Into` implementation.
///
/// The type of the item must be known at compile time, and usually this means an
/// explicit type annotation is required.
///
/// # Usage
///
/// ```rust
/// # use std::collections::BTreeMap;
/// use velcro::btree_map_from;
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct Foo(u64);
///
/// impl From<u64> for Foo {
///     fn from(other: u64) -> Self {
///         Foo(other)
///     }
/// }
///
/// let mut map1 = BTreeMap::new();
/// map1.insert('a', Foo(0));
/// map1.insert('b', Foo(1));
/// map1.insert('c', Foo(1));
/// map1.insert('d', Foo(1));
/// map1.insert('e', Foo(1));
/// map1.insert('f', Foo(2));
///
/// let map2: BTreeMap<char, Foo> = btree_map_from! {
///     'a': 0,
///     ..('b'..='e'): 1,
///     'f': 2
/// };
///
/// assert_eq!(map1, map2);
///```
pub use velcro_macros::btree_map_from;

/// Creates an iterator, over the given values. Other collections and iterators
/// may also be interspersed, or "spread", using the `..` operator.
///
/// # Usage
///
/// ```rust
/// use velcro::iter;
/// let vec = vec![0, 1, 2, 3];
///
/// assert_eq!(iter![..vec, 4, 5, 6].collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5, 6]);
///
/// for x in iter![0, 1, ..(2..=5), 6] {
///    println!("x = {}", x);   
/// }
///
/// assert_eq!(iter![0, 1, ..(2..=5), 6].collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5, 6]);
///```
pub use velcro_macros::iter;

/// Creates an iterator, over the given values. Works the same as `iter` except that values
/// may be any type that can be converted to the iterator item type via an `Into`
/// implementation.
///
/// # Usage
///
/// ```rust
/// use velcro::iter;
///
/// #[derive(Debug, PartialEq)]
/// struct Foo(u64);
///
/// impl From<u64> for Foo {
///     fn from(other: u64) -> Self {
///         Foo(other)
///     }
/// }
///
/// assert_eq!(iter![0, 1, ..(2..=5), 6].collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5, 6]);
///```
pub use velcro_macros::iter_from;

/// Creates an iterator over pairs of values, expressed with map-like syntax.
/// Other collections and iterators may also be interspersed, or "spread", using the
/// `..` operator.
///
/// # Usage
///
/// ```rust
/// use velcro::map_iter;
///
/// let vec = vec![0, 1, 2, 3];
///
/// for (key, value) in map_iter![0: "a", 1: "b", ..(2..=5): "c"] {
///    println!("{} = {}", key, value);
/// }
///
/// assert_eq!(
///     map_iter! {
///         0: "a",
///         1: "b",
///         ..(2..=5): "c"
///     }.collect::<Vec<_>>(),
///     vec![(0, "a"), (1, "b"), (2, "c"), (3, "c"), (4, "c"), (5, "c")]
/// );
///```
///
/// A typical use-case for `map_iter` is to collect into a third party map
/// implementation, not supported by velcro, while still being able to use the
/// velcro spread operator.
///
/// For example, `IndexMap` from the `indexmap` crate:
/// ```
/// use velcro::map_iter;
/// use indexmap::map::IndexMap;
///
/// let map: IndexMap<_, _> = map_iter! {
///         ..(0..10): 100,
///         1: 200,
///         ..(3..5): 300
///     }
///     .collect();
/// assert_eq!(map.get(&0).unwrap(), &100);
/// assert_eq!(map.get(&1).unwrap(), &200);
/// assert_eq!(map.get(&2).unwrap(), &100);
/// assert_eq!(map.get(&3).unwrap(), &300);
/// ```
pub use velcro_macros::map_iter;

/// Creates an iterator over pairs of values in the same way as `map_iter` except
/// that values are converted into the expected type using an `Into` implementation.
///
/// # Usage
///
/// ```rust
/// use velcro::map_iter_from;
///
/// assert_eq!(
///     map_iter_from! {
///         0: "a",
///         1: "b",
///         ..(2..=5): "c"
///     }.collect::<Vec<(i64, String)>>(),
///     vec![
///         (0, String::from("a")),
///         (1, String::from("b")),
///         (2, String::from("c")),
///         (3, String::from("c")),
///         (4, String::from("c")),
///         (5, String::from("c"))
///     ]);
///```
pub use velcro_macros::map_iter_from;

pub use velcro_macros::arr;

pub use velcro_macros::arr_from;
