use crate::seq::SeqInput;
use crate::value::Value;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};

pub struct BTreeSetInput(SeqInput);

impl Parse for BTreeSetInput {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(BTreeSetInput(input.parse()?))
    }
}

impl BTreeSetInput {
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
            let mut #target = ::std::collections::BTreeSet::new();
            #(#updates)*
            #target
        }}
    }
}
