use velcro::vec;

// A (very!) inefficient implementation of Fibonacci sequence
fn fib(a: u64, b: u64, len: usize) -> Vec<u64> {
    if len == 0 {
        vec![]
    } else {
        vec![a, ..fib(b, a + b, len - 1)]
    }
}

fn main() {
    let fibs = fib(1, 1, 7);

    assert_eq!(fibs, vec![1, 1, 2, 3, 5, 8, 13]);
}
