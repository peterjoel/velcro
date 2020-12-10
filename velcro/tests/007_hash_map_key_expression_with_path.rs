//! Tests for https://github.com/peterjoel/velcro/issues/7
use velcro::*;

#[test]
fn expressions_with_paths_should_be_permitted_as_hash_map_keys() {
    let map = hash_map! {
        String::from("foo"): 1,
    };

    assert_eq!(map.get(&String::from("foo")), Some(&1));
}
