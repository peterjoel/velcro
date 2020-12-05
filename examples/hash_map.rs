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
}
