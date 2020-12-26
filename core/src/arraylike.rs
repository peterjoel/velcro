//! Input for array-like structures; `Array` and `Vec`.

use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::TokenTree;
use quote::ToTokens;
use syn::parse::{self, Parse, ParseStream};
use syn::{Expr, Token};

#[allow(clippy::large_enum_variant)]
pub enum ArraylikeInput<V = Verbatim> {
    Seq(SeqInput<V>),
    Repeat(RepeatInput<V>),
}

impl<V> Parse for ArraylikeInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(if has_semicolon_separator(input) {
            ArraylikeInput::Repeat(input.parse()?)
        } else {
            ArraylikeInput::Seq(input.parse()?)
        })
    }
}

/// test if the input stream contains a semicolon at the top level. Semicolons
/// that are part of expressions do not count
fn has_semicolon_separator(input: ParseStream) -> bool {
    input
        .step(|cursor| {
            let mut rest = *cursor;
            while let Some((tt, next)) = rest.token_tree() {
                match &tt {
                    TokenTree::Punct(token) if token.as_char() == ';' => {
                        // keep the cursor where it was
                        return Ok((true, *cursor));
                    }
                    _ => rest = next,
                }
            }
            Err(cursor.error("';' not found"))
        })
        .unwrap_or(false)
}

pub struct RepeatInput<V> {
    value: Value<V>,
    len: Expr,
}

impl<V> RepeatInput<V> {
    pub fn value(&self) -> &Value<V> {
        &self.value
    }

    pub fn len(&self) -> &Expr {
        &self.len
    }
}

impl<V> Parse for RepeatInput<V>
where
    Value<V>: Parse,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(RepeatInput {
            value: input.parse()?,
            len: {
                let _: Token![;] = input.parse()?;
                input.parse()?
            },
        })
    }
}
