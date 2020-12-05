use proc_macro::TokenStream;
use syn::parse_macro_input;
use velcro_core::{BTreeSetInput, ConvertInto, HashSetInput, IterInput, VecInput};

#[proc_macro]
pub fn vec(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as VecInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn btree_set(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as BTreeSetInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn hash_set(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as HashSetInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn iter(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as IterInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn vec_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as VecInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn btree_set_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as BTreeSetInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn hash_set_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as HashSetInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn iter_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as IterInput<ConvertInto>).into_output();
    TokenStream::from(output)
}
