use velcro::*;

fn main() {
    let a = arr![1, 2, 3, 4];
    assert_eq!([1, 2, 3, 4], a);

    let a: [i32; 4] = arr![..(1..); 4];
    assert_eq!([1, 2, 3, 4], a);

    let a: [i64; 4] = arr_from![..(1_i32..); 4];
    assert_eq!([1, 2, 3, 4], a);
}
