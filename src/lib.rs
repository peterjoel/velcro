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
///     ..'b'..='e': 1,
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
///     ..'b'..='e': 1,
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
///     ..'b'..='e': 1,
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
///     ..'b'..='e': 1,
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
