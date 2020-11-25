# Velcro

A simple macro for initializing vectors in a variety of interesting ways. The
macro supports a superset of the input of the built-in `vec!` macro, adding
sequences of values which can be "spread" into the vector by preceding them with
the `..` operator. In order to be spread, values must implement `IntoIterator`.

## Examples

```
use velcro::velcro;

assert_eq!(velcro![..0..7], vec![0, 1, 2, 3, 4, 5, 6]);
assert_eq!(velcro![0, 1, ..2..7], vec![0, 1, 2, 3, 4, 5, 6]);

let other = vec![3, 4, 5];
assert_eq!(velcro![0, 1, 2, ..other, 6], vec![0, 1, 2, 3, 4, 5, 6]);

let mut it = (0..=3).into_iter().map(|x| x + 2);
assert_eq!(velcro![0, 1, ..it, 6], vec![0, 1, 2, 3, 4, 5, 6]);
```
