/// A more flexible vector initialization macro
///
/// # Usage
///
/// ```rust
/// use velcro::velcro;
///
/// assert_eq!(velcro![..0..7], vec![0, 1, 2, 3, 4, 5, 6]);
/// assert_eq!(velcro![0, 1, ..2..7], vec![0, 1, 2, 3, 4, 5, 6]);
///
/// let other = vec![3, 4, 5];
/// assert_eq!(velcro![0, 1, 2, ..other, 6], vec![0, 1, 2, 3, 4, 5, 6]);
///
/// let mut it = (0..=3).into_iter().map(|x| x + 2);
/// assert_eq!(velcro![0, 1, ..it, 6], vec![0, 1, 2, 3, 4, 5, 6]);
/// ```
///
#[macro_export]
macro_rules! velcro {
    ($v: tt @ .. $e: expr $(,$($rem: tt)*)?) => {
        $v.extend($e);
        $($crate::velcro!($v @ $($rem)*);)?
    };
    ($v: tt @ $e: expr $(,$($rem: tt)*)?) => {
        $v.push($e);
        $($crate::velcro!($v @ $($rem)*);)?
    };
    ($v: tt @) => {
    };
    ($($e: tt)*) => {{
        let mut v = Vec::new();
        $crate::velcro!(v @ $($e)*);
        v
    }};
}
