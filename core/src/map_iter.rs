use crate::key_value::KeyValueSeq;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use crate::ParseRaw;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse;

pub struct MapIterInput<V = Verbatim>(KeyValueSeq<V>);

impl<V> ParseRaw for MapIterInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn parse_raw(input: TokenStream) -> parse::Result<Self> {
        Ok(MapIterInput(KeyValueSeq::parse_raw(input)?))
    }
}

impl<V> MapIterInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        let target = Ident::new("it", Span::call_site());
        let updates = self.0.key_values().map(|kv| {
            let key = kv.key();
            let value = kv.value();
            match key {
                Value::One(expr) => quote! {
                    let #target = #target.chain(::std::iter::once((#expr, #value)));
                },
                Value::Many(expr) => quote! {
                    let #target = #target.chain(
                        ::std::iter::IntoIterator::into_iter(#expr)
                            .zip(::std::iter::repeat(#value))
                    );
                },
            }
        });
        quote! {{
            let #target = ::std::iter::empty();
            #(#updates)*
            #target
        }}
    }
}
