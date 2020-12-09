use crate::key_value::KeyValueSeq;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use crate::ParseRaw;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{self};

pub struct HashMapInput<V = Verbatim>(KeyValueSeq<V>);

impl<V> HashMapInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        let target = Ident::new("map", Span::call_site());
        let key_values = self.0.key_values();
        let initial_capacity = if self.0.is_simple() {
            key_values.len()
        } else {
            // A simple heuristic for the initial capacity. At this point we can guess that
            // the output length is likely to be greater than the number of values, since
            // at least one of the values is an iterator. This will reduce the number of
            // allocations in common cases, while not massively over-allocating when the
            // collection is small.
            16.max(key_values.len().next_power_of_two() * 2)
        };
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
            let mut #target = ::std::collections::HashMap::with_capacity(#initial_capacity);
            #(#updates)*
            #target
        }}
    }
}

impl<V> ParseRaw for HashMapInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn parse_raw(input: TokenStream) -> parse::Result<Self> {
        Ok(HashMapInput(KeyValueSeq::parse_raw(input)?))
    }
}
