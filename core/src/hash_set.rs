use crate::seq::SeqInput;
use crate::value::Value;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};

pub struct HashSetInput(SeqInput);

impl Parse for HashSetInput {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(HashSetInput(input.parse()?))
    }
}

impl HashSetInput {
    pub fn into_output(self) -> TokenStream {
        let target = Ident::new("set", Span::call_site());
        let updates = self.0.values().map(|value| match value {
            Value::One(expr) => quote! {
                #target.insert(#expr);
            },
            Value::Many(expr) => quote! {
                for item in #expr {
                    #target.insert(item);
                }
            },
        });
        quote! {{
            let mut #target = ::std::collections::HashSet::new();
            #(#updates)*
            #target
        }}
    }
}
