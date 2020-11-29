mod vector;

use crate::vector::VecInput;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn velcro_vec(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as VecInput).into_output();
    TokenStream::from(output)
}
