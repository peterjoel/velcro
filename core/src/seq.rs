use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::marker::PhantomData;
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::{Pair, Punctuated};
use syn::Token;

/// A comma-delimited sequence of `Value`s, used for macros with list-like input.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr as _;
    use syn::Expr;

    #[test]
    fn parse_empty_input_as_empty_sequence() {
        let tokens = TokenStream::from_str("").unwrap();
        let seq: SeqInput<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(seq.values().next().is_none());
    }

    #[test]
    fn parse_single_expression_as_one_value_of_one() {
        let tokens = TokenStream::from_str("a + b").unwrap();
        let seq: SeqInput<Verbatim> = syn::parse2(tokens.clone()).unwrap();
        let expected_expr: Expr = syn::parse2(tokens).unwrap();
        let values: Vec<_> = seq.values().collect();
        assert_eq!(values.len(), 1);
        assert!(matches!(values[0], Value::One(ValueExpr { expr, .. }) if *expr == expected_expr));
    }

    #[test]
    fn parse_single_spread_expression_as_one_value_of_many() {
        let source = "..a + b";
        let tokens = TokenStream::from_str(source).unwrap();
        let seq: SeqInput<Verbatim> = syn::parse2(tokens).unwrap();
        let expected_expr: Expr = {
            let tokens = TokenStream::from_str(source.strip_prefix("..").unwrap()).unwrap();
            syn::parse2(tokens).unwrap()
        };
        let values: Vec<_> = seq.values().collect();
        assert_eq!(values.len(), 1);
        assert!(
            matches!(values[0], Value::Many(ValueIterExpr { expr, .. }) if *expr == expected_expr)
        );
    }

    #[test]
    fn empty_input_is_simple() {
        let tokens = TokenStream::from_str("").unwrap();
        let seq: SeqInput<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(seq.is_simple());
    }

    #[test]
    fn non_spread_input_is_simple() {
        let tokens = TokenStream::from_str("a, b, c").unwrap();
        let seq: SeqInput<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(seq.is_simple());
    }

    #[test]
    fn spread_input_is_not_simple() {
        let tokens = TokenStream::from_str("a, ..b, c").unwrap();
        let seq: SeqInput<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(!seq.is_simple());
    }
}
