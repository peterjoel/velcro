# Velcro

A drop-in replacement for the `vec!` macro provided by `std`. All functionality of
the `std` macro is supported, but multiple values may be added in a single expression,
preceding expressions with the `..` operator. In order to be spread with this operator,
values must implement `IntoIterator`.

## Examples

```
use velcro::vec;

assert_eq!(vec![..0..7], vec![0, 1, 2, 3, 4, 5, 6]);
assert_eq!(vec![0, 1, ..2..7], vec![0, 1, 2, 3, 4, 5, 6]);

let other = vec![3, 4, 5];
assert_eq!(vec![0, 1, 2, ..other, 6], vec![0, 1, 2, 3, 4, 5, 6]);

let mut it = (0..=3).into_iter().map(|x| x + 2);
assert_eq!(vec![0, 1, ..it, 6], vec![0, 1, 2, 3, 4, 5, 6]);
```
