use proc_macro::TokenStream;
use syn::parse_macro_input;
use velcro_core::{
    parse_raw_macro_input, ArrInput, BTreeMapInput, BTreeSetInput, ConvertInto, HashMapInput,
    HashSetInput, IterInput, LinkedListInput, MapIterInput, VecInput,
};

#[proc_macro]
pub fn vec(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as VecInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn arr(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as ArrInput).into_output();
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
pub fn linked_list(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as LinkedListInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn hash_map(input: TokenStream) -> TokenStream {
    let output = parse_raw_macro_input!(input as HashMapInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn btree_map(input: TokenStream) -> TokenStream {
    let output = parse_raw_macro_input!(input as BTreeMapInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn iter(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as IterInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn map_iter(input: TokenStream) -> TokenStream {
    let output = parse_raw_macro_input!(input as MapIterInput).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn vec_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as VecInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn arr_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as ArrInput<ConvertInto>).into_output();
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
pub fn linked_list_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as LinkedListInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn iter_from(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as IterInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn map_iter_from(input: TokenStream) -> TokenStream {
    let output = parse_raw_macro_input!(input as MapIterInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn hash_map_from(input: TokenStream) -> TokenStream {
    let output = parse_raw_macro_input!(input as HashMapInput<ConvertInto>).into_output();
    TokenStream::from(output)
}

#[proc_macro]
pub fn btree_map_from(input: TokenStream) -> TokenStream {
    let output = parse_raw_macro_input!(input as BTreeMapInput<ConvertInto>).into_output();
    TokenStream::from(output)
}
