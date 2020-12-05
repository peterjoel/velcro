use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

pub struct IterInput<W = Verbatim>(SeqInput<W>);

impl<W> Parse for IterInput<W>
where
    Value<W>: Parse,
{
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(IterInput(input.parse()?))
    }
}

impl<V> IterInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        let target = Ident::new("it", Span::call_site());
        let updates = self.0.values().map(|value| match value {
            Value::One(expr) => quote! {
                let #target = #target.chain(std::iter::once(#expr));
            },
            Value::Many(expr) => quote! {
                let #target = #target.chain(#expr);
            },
        });
        quote! {{
            let #target = ::std::iter::empty();
            #(#updates)*
            #target
        }}
    }
}
