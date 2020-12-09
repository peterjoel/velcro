use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{self, Parse, ParseStream};

pub struct HashSetInput<V = Verbatim>(SeqInput<V>);

impl<V> Parse for HashSetInput<V>
where
    Value<V>: Parse,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(HashSetInput(input.parse()?))
    }
}

impl<V> HashSetInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        let values = self.0.values();
        let initial_capacity = if self.0.is_simple() {
            values.len()
        } else {
            // A simple heuristic for the initial capacity. At this point we can guess that
            // the output length is likely to be greater than the number of values, since
            // at least one of the values is an iterator. This will reduce the number of
            // allocations in common cases, while not massively over-allocating when the
            // collection is small.
            16.max(values.len().next_power_of_two() * 2)
        };
        let target = Ident::new("set", Span::call_site());
        let updates = values.map(|value| match value {
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
            let mut #target = ::std::collections::HashSet::with_capacity(#initial_capacity);
            #(#updates)*
            #target
        }}
    }
}
