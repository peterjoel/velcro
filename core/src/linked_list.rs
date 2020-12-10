use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{self, Parse, ParseStream};

pub struct LinkedListInput<V = Verbatim>(SeqInput<V>);

impl<V> Parse for LinkedListInput<V>
where
    Value<V>: Parse,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(LinkedListInput(input.parse()?))
    }
}

impl<V> LinkedListInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        let values = self.0.values();
        let target = Ident::new("list", Span::call_site());
        let updates = values.map(|value| match value {
            Value::One(expr) => quote! {
                #target.push_back(#expr);
            },
            Value::Many(expr) => quote! {
                for item in #expr {
                    #target.push_back(item);
                }
            },
        });
        quote! {{
            let mut #target = ::std::collections::LinkedList::new();
            #(#updates)*
            #target
        }}
    }
}
