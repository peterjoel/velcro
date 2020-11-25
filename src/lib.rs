/// A more flexible vector initialization macro. `velcro!` can be used as a
/// drop-in replacement for `vec!`, but with extra functionality.
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
///
/// // There is basic support for `vec![a; size]` syntax, for compatibility
/// assert_eq!(velcro![3; 5], vec![3; 5]);
/// // With support for iterators, but currently the input cannot be mixed
/// let range = 3..;
/// assert_eq!(velcro![..range; 5], vec![3, 4, 5, 6, 7]);
/// ```
///
/// # Performance
///
/// Performance is mostly good however, in the general case, the macro does
/// not know the size of the collection ahead of time, so may re-allocate
/// while adding elements.
///
/// * `velcro![x; size]` performs the same as `vec![x; size]`
/// * `velcro![..it; size]` will perform the same as collecting an iterator
///    into a `Vec`
/// * `velcro![a, b, c, d]` is likely to be a bit slower than `vec![a, b, c, d]`
/// * Forms which combine other collections and iterators (like
///   `velcro![a, ..b]`) will grow the `Vec` as items are added. This could
///   potentially be improved in a future version, using a procedural macro.
#[macro_export]
macro_rules! velcro {
    (.. $iter: expr; $size: expr) => {
        IntoIterator::into_iter($iter).take($size).collect::<Vec<_>>()
     };
    ($val: expr; $size: expr) => {
       vec![$val; $size]
    };
    ($($e: tt)*) => {{
        let v: Vec<_> = $crate::splat_tts![ $($e)* ];
        v
    }};
}

/// Macro which can be used by declarative macro authors to apply
/// velcro-style splatting to its arguments. It expects a raw input of only `tt`
/// tokens. The type of the output can be changed by implementing the `Splat`
/// trait for your type, and then relying on type inference.
///
/// **This macro is internal and highly likely to change. Use at your own risk.**
#[macro_export]
macro_rules! splat_tts {
    (@ $v: tt .. $e: expr $(,$($rem: tt)*)?) => {
        $crate::Splat::<_>::splat_extend(&mut $v, $e);
        $($crate::splat_tts!(@ $v $($rem)*);)?
    };
    (@ $v: tt $e: expr $(,$($rem: tt)*)?) => {
        $crate::Splat::<_>::splat_push(&mut $v, $e);
        $($crate::splat_tts!(@ $v $($rem)*);)?
    };
    (@ $v: tt) => {
    };
    ($($e: tt)*) => {{
        let mut v = $crate::Splat::<_>::splat_new(0);
        $crate::splat_tts!(@ v $($e)*);
        v
    }};
}

/// A helper trait, used for customising the `splat_tts!` macro.
///
/// **This trait is internal and highly likely to change. Use at your own risk.**
pub trait Splat<T> {
    fn splat_new(size_hint: usize) -> Self;
    fn splat_push(&mut self, value: T);
    fn splat_extend(&mut self, value: impl IntoIterator<Item = T>);
}

impl<T> Splat<T> for Vec<T> {
    fn splat_new(size_hint: usize) -> Self {
        Vec::with_capacity(size_hint)
    }

    fn splat_push(&mut self, value: T) {
        self.push(value);
    }

    fn splat_extend(&mut self, values: impl IntoIterator<Item = T>) {
        self.extend(values);
    }
}
