use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

pub struct HashSetInput<V = Verbatim>(SeqInput<V>);

impl<V> Parse for HashSetInput<V>
where
    Value<V>: Parse,
{
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(HashSetInput(input.parse()?))
    }
}

impl<V> HashSetInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
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
