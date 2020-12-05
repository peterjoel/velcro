mod btree_map;
mod btree_set;
mod hash_map;
mod hash_set;
mod iter;
mod key_value;
mod seq;
mod value;
mod vector;

pub use crate::{
    btree_map::BTreeMapInput,
    btree_set::BTreeSetInput,
    hash_map::HashMapInput,
    hash_set::HashSetInput,
    iter::IterInput,
    value::{ConvertInto, Verbatim},
    vector::VecInput,
};
use proc_macro2::TokenStream;
use syn::parse::Result;

pub trait ParseRaw: Sized {
    fn parse_raw(input: TokenStream) -> Result<Self>;
}

#[macro_export]
macro_rules! parse_raw_macro_input {
    ($tokenstream: ident as $ty: ty) => {
        match <$ty as $crate::ParseRaw>::parse_raw($tokenstream.into()) {
            Ok(data) => data,
            Err(err) => {
                return ::syn::export::TokenStream::from(err.to_compile_error());
            }
        }
    };
}
