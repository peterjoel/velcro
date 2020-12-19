# Change Log

All notable changes by version.

## v0.4.4

- Add heuristic for calculating `HashSet` and `HashMap` capacity
- Fix(#7): Expressions containing paths are not accepted by macros

## v0.4.3

- Fix: Prevent warnings when a range is in parentheses
- Improve docs

## v0.4.2

- Update Cargo package description

## v0.4.0

- Add heuristic for calculating `Vec` capacity
- Add `btree_map!` and `hash_map!` macros
- Add `*_from` variants to all macros

## v0.3.0

- Add `vec!`, `btree_set!`, `hash_set!` and `iter!` macros

## v0.2.0

- Remove `splat_tts` macro and `Splat` trait.
- Rename `velcro!` to `vec!` and support all input patterns of `std::vec!`

## v0.1.2

- Add generalized `splat_tts`, a tt-muncher macro, and supporting `Splat` trait

## v0.1.1

## v0.1.0

- Add `velco!` macro for vector initialization with spread operator
