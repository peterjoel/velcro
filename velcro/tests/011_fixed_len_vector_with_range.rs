//! Tests for https://github.com/peterjoel/velcro/issues/11
use velcro::vec;

#[test]
fn fixed_len_vec_should_accept_range() {
    let v = vec![..(1..=4); 4];
    assert_eq!(v, vec![1, 2, 3, 4]);
}
