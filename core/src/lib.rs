mod arraylike;
mod btree_map;
mod btree_set;
mod hash_map;
mod hash_set;
mod iter;
mod key_value;
mod linked_list;
mod map_iter;
mod seq;
mod value;
mod vector;

pub use crate::{
    btree_map::BTreeMapInput,
    btree_set::BTreeSetInput,
    hash_map::HashMapInput,
    hash_set::HashSetInput,
    iter::IterInput,
    linked_list::LinkedListInput,
    map_iter::MapIterInput,
    value::{ConvertInto, Verbatim},
    vector::VecInput,
};
use proc_macro2::TokenStream;
use syn::parse::Result;

/// Trait to be implemented for parsing syntax that would be rejected by syn::Parse
pub trait ParseRaw: Sized {
    fn parse_raw(input: TokenStream) -> Result<Self>;
}

/// Used in a similar way to `syn::parse_macro_input` macro, but for types that implement
/// `ParseRaw` instead of `syn::parse::Parse`.
#[macro_export]
macro_rules! parse_raw_macro_input {
    ($tokenstream: ident as $ty: ty) => {
        match <$ty as $crate::ParseRaw>::parse_raw($tokenstream.into()) {
            Ok(data) => data,
            Err(err) => {
                return ::proc_macro::TokenStream::from(err.to_compile_error());
            }
        }
    };
}
