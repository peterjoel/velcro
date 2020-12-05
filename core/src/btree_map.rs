use crate::key_value::KeyValueSeq;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use crate::ParseRaw;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{self};

pub struct BTreeMapInput<V = Verbatim>(KeyValueSeq<V>);

impl<V> BTreeMapInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        let target = Ident::new("map", Span::call_site());
        let updates = self.0.key_values().map(|kv| {
            let key = kv.key();
            let value = kv.value();
            match key {
                Value::One(expr) => quote! {
                    #target.insert(#expr, #value);
                },
                Value::Many(expr) => quote! {
                    for key in #expr {
                        #target.insert(key, #value);
                    }
                },
            }
        });
        quote! {{
            let mut #target = ::std::collections::BTreeMap::new();
            #(#updates)*
            #target
        }}
    }
}

impl<V> ParseRaw for BTreeMapInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn parse_raw(input: TokenStream) -> parse::Result<Self> {
        Ok(BTreeMapInput(KeyValueSeq::parse_raw(input)?))
    }
}
