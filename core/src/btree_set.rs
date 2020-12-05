use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{self, Parse, ParseStream};

pub struct BTreeSetInput<V = Verbatim>(SeqInput<V>);

impl<V> Parse for BTreeSetInput<V>
where
    Value<V>: Parse,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(BTreeSetInput(input.parse()?))
    }
}

impl<V> BTreeSetInput<V>
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
            let mut #target = ::std::collections::BTreeSet::new();
            #(#updates)*
            #target
        }}
    }
}
