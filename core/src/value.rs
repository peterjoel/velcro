use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use std::marker::PhantomData;
use syn::parse::{self, Parse, ParseStream};
use syn::{spanned::Spanned, Expr, ExprParen, Token};

/// An input to a velcro macro, which can be an expression representing one value or
/// an expression preceded by `..`, representing many values.
///
/// The type parameter is a marker to control the code generation. It can be either
/// `Verbatim` or `ConvertInto`.
pub enum Value<V> {
    One(ValueExpr<V>),
    Many(ValueIterExpr<V>),
}

impl<V> Value<V> {
    /// Returns true if the value does not use the spread operator
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::One(_))
    }
}

impl<V> Parse for Value<V>
where
    ValueExpr<V>: Parse,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        if input.peek(Token![..]) {
            let _: Token![..] = input.parse()?;
            Ok(Value::Many(input.parse()?))
        } else {
            Ok(Value::One(input.parse()?))
        }
    }
}

impl<V> ToTokens for Value<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Value::One(expr) => expr.to_tokens(tokens),
            Value::Many(expr) => expr.to_tokens(tokens),
        }
    }
}

pub struct ValueExpr<V> {
    pub(crate) expr: Expr,
    _phantom: PhantomData<V>,
}

impl<V> Parse for ValueExpr<V> {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(ValueExpr {
            expr: input.parse()?,
            _phantom: PhantomData,
        })
    }
}

pub struct ValueIterExpr<V> {
    pub(crate) expr: Expr,
    _phantom: PhantomData<V>,
}

impl<V> Parse for ValueIterExpr<V> {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(ValueIterExpr {
            expr: input.parse()?,
            _phantom: PhantomData,
        })
    }
}

/// A marker type indicating that input variables should be passed to the result
/// collection as-is.
pub struct Verbatim;

/// A marker type indicating that input variables need to be converted into the
/// item (or key) type of the collection
pub struct ConvertInto;

impl ToTokens for ValueExpr<Verbatim> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ValueExpr<ConvertInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        let output = quote_spanned! {
            expr.span() =>
            std::convert::Into::into(#expr)
        };
        tokens.append_all(output);
    }
}

// If there is a Range in parentheses, strip the parentheses to avoid compiler warnings.
// Leave the parentheses for other types of expression.
fn remove_range_parens(expression: &Expr) -> &Expr {
    match expression {
        Expr::Paren(ExprParen { expr, .. }) if matches!(**expr, Expr::Range(_)) => expr,
        other => other,
    }
}

impl ToTokens for ValueIterExpr<Verbatim> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = remove_range_parens(&self.expr);
        expr.to_tokens(tokens);
    }
}

impl ToTokens for ValueIterExpr<ConvertInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = remove_range_parens(&self.expr);
        let output = quote_spanned! {
            expr.span() =>
            ::std::iter::IntoIterator::into_iter(#expr).map(|value| ::std::convert::Into::into(value))
        };
        tokens.append_all(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr as _;

    #[test]
    fn parse_literal_as_one() {
        let tokens = TokenStream::from_str("1").unwrap();
        let expected_expr: Expr = syn::parse2(tokens.clone()).unwrap();
        let value: Value<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(matches!(value, Value::One(ValueExpr { expr, .. } ) if expr == expected_expr));
    }

    #[test]
    fn parse_complex_expression_as_one() {
        let tokens = TokenStream::from_str("a..foo::bar(1 + {x::<i32>(&bas)}).ok() / y").unwrap();
        let expected_expr: Expr = syn::parse2(tokens.clone()).unwrap();
        let value: Value<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(matches!(value, Value::One(ValueExpr { expr, .. }) if expr == expected_expr));
    }

    #[test]
    fn parse_spread_simple_expression_as_many() {
        let source = "..foo";
        let tokens = TokenStream::from_str(source).unwrap();
        let value: Value<Verbatim> = syn::parse2(tokens).unwrap();
        let expected_expr: Expr = {
            let tokens = TokenStream::from_str(source.strip_prefix("..").unwrap()).unwrap();
            syn::parse2(tokens).unwrap()
        };
        assert!(matches!(value, Value::Many(ValueIterExpr { expr, .. }) if expr == expected_expr));
    }

    #[test]
    fn spread_complex_expression_as_many() {
        let source = "..a..foo::bar(1 + {x::<i32>(&bas)}).ok() / y";
        let tokens = TokenStream::from_str(source).unwrap();
        let value: Value<Verbatim> = syn::parse2(tokens).unwrap();
        let expected_expr: Expr = {
            let tokens = TokenStream::from_str(source.strip_prefix("..").unwrap()).unwrap();
            syn::parse2(tokens).unwrap()
        };
        assert!(matches!(value, Value::Many(ValueIterExpr { expr, .. }) if expr == expected_expr));
    }

    #[test]
    fn spread_with_no_expression_is_error() {
        let source = "..";
        let tokens = TokenStream::from_str(source).unwrap();
        let parse_result: Result<Value<Verbatim>, _> = syn::parse2(tokens);
        assert!(parse_result.is_err());
    }

    #[test]
    fn expression_without_spread_is_simple() {
        let tokens = TokenStream::from_str("foo").unwrap();
        let value: Value<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(value.is_simple());
    }

    #[test]
    fn expression_with_spread_is_not_simple() {
        let tokens = TokenStream::from_str("..foo").unwrap();
        let value: Value<Verbatim> = syn::parse2(tokens).unwrap();
        assert!(!value.is_simple());
    }
}
