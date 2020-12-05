use crate::value::Value;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{Pair, Punctuated};
use syn::Token;

pub struct SeqInput {
    values: Punctuated<Value, Token![,]>,
}

impl Parse for SeqInput {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(SeqInput {
            values: input.parse_terminated(Value::parse)?,
        })
    }
}

impl SeqInput {
    pub fn is_simple(&self) -> bool {
        self.values.iter().all(Value::is_simple)
    }

    pub fn values(&self) -> impl ExactSizeIterator<Item = &Value> {
        self.values.iter()
    }

    pub fn simple_output(self) -> TokenStream {
        self.values
            .into_pairs()
            .map(Pair::into_tuple)
            .map(|(expr, delim)| Pair::new(expr.expr(), delim).into_token_stream())
            .collect()
    }
}
