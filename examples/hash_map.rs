use std::collections::HashMap;
use velcro::hash_map;

fn main() {
    let mut map = HashMap::new();
    map.insert('a', 0);
    map.insert('b', 1);
    map.insert('c', 1);
    map.insert('d', 1);
    map.insert('e', 0);
    map.insert('f', 0);
    map.insert('g', 0);
    map.insert('h', 0);
    let lit = hash_map! {
        'a': 0,
        ..'b'..='d': 1,
        ..'e'..='h': 0,
    };
    assert_eq!(map, lit);

    let other = vec![3, 4, 5];
    let map2 = hash_map! {
        0: "zero",
        1: "one",
        ..other: "all of these keys have the same value",
        ..10..20: "as do these",
    };

    assert_eq!(map2.get(&0).unwrap(), &"zero");
    assert_eq!(
        map2.get(&3).unwrap(),
        &"all of these keys have the same value"
    );
    assert_eq!(
        map2.get(&4).unwrap(),
        &"all of these keys have the same value"
    );
    assert_eq!(map2.get(&10).unwrap(), &"as do these");
}
