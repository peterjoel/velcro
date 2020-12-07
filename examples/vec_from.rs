use velcro::vec_from;

#[derive(Debug, PartialEq)]
struct Foo(i32);

impl From<i32> for Foo {
    fn from(other: i32) -> Self {
        Foo(other)
    }
}

fn main() {
    let foos: Vec<Foo> = vec_from![1, 2, Foo(3), ..(4..=6), 7];
    assert_eq!(
        foos,
        vec![Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7)]
    );
}
