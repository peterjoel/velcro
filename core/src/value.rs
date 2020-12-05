use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use std::marker::PhantomData;
use syn::parse::{self, Parse, ParseStream};
use syn::{spanned::Spanned, Expr, Token};

pub enum Value<V> {
    One(ValueExpr<V>),
    Many(ValueIterExpr<V>),
}

impl<V> Value<V> {
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
    pub expr: Expr,
    pub _phantom: PhantomData<V>,
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
    expr: Expr,
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

pub struct Verbatim;
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

impl ToTokens for ValueIterExpr<Verbatim> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ValueIterExpr<ConvertInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        let output = quote_spanned! {
            expr.span() =>
            std::iter::IntoIterator::into_iter(#expr).map(|value| std::convert::Into::into(value))

        };
        tokens.append_all(output);
    }
}
