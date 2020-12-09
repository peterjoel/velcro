use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::marker::PhantomData;
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::{Pair, Punctuated};
use syn::Token;

pub struct SeqInput<V = Verbatim> {
    values: Punctuated<Value<V>, Token![,]>,
    _phantom: PhantomData<V>,
}

impl<V> Parse for SeqInput<V>
where
    Value<V>: Parse,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(SeqInput {
            values: input.parse_terminated(Value::parse)?,
            _phantom: PhantomData,
        })
    }
}

impl<V> SeqInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    /// Returns true if the sequence contains no spread values
    pub fn is_simple(&self) -> bool {
        self.values.iter().all(Value::is_simple)
    }

    pub fn values(&self) -> impl ExactSizeIterator<Item = &Value<V>> {
        self.values.iter()
    }

    pub fn simple_output(self) -> TokenStream {
        self.values
            .into_pairs()
            .map(Pair::into_tuple)
            .map(|(value, delim)| Pair::new(value.into_token_stream(), delim).into_token_stream())
            .collect()
    }
}
