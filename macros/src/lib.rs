mod btree_set;
mod hash_set;
mod seq;
mod value;
mod vector;

use crate::btree_set::BTreeSetInput;
use crate::hash_set::HashSetInput;
use crate::vector::VecInput;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn velcro_vec(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as VecInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn velcro_btree_set(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as BTreeSetInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn velcro_hash_set(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as HashSetInput).into_output();
    TokenStream::from(output)
}
