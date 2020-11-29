/// A more flexible vector initialization macro. `velcro::vec!` is a
/// drop-in replacement for `std`'s built-in `vec!` macro, but with extra
/// functionality. In particular, it adds the `..` spread operator, which
/// can insert multiple elements at once, provided that the expression
/// can be iterated over (i.e. it implements `IntoIterator`).
///
/// # Usage
///
/// ```rust
/// use velcro::vec;
///
/// assert_eq!(vec![..0..7], vec![0, 1, 2, 3, 4, 5, 6]);
/// assert_eq!(vec![0, 1, ..2..7], vec![0, 1, 2, 3, 4, 5, 6]);
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
pub use velcro_macros::velcro_vec as vec;
