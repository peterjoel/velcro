use crate::seq::SeqInput;
use crate::value::Value;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

pub enum VecInput {
    Seq(VecSeqInput),
    Repeat(VecRepeatInput),
}

impl VecInput {
    pub fn into_output(self) -> TokenStream {
        match self {
            VecInput::Seq(seq) => seq.into_output(),
            VecInput::Repeat(repeat) => repeat.into_output(),
        }
    }
}

impl Parse for VecInput {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(if find_semicolon_separator(input) {
            VecInput::Repeat(input.parse()?)
        } else {
            VecInput::Seq(input.parse()?)
        })
    }
}

fn find_semicolon_separator(input: ParseStream) -> bool {
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

pub struct VecRepeatInput {
    value: Value,
    len: Expr,
}

impl VecRepeatInput {
    fn into_output(self) -> TokenStream {
        if self.value.is_simple() {
            self.simple_output()
        } else {
            self.splatted_output()
        }
    }

    fn simple_output(self) -> TokenStream {
        let len = self.len;
        let expr = self.value.expr();
        quote! {
            ::std::vec![#expr; #len]
        }
    }

    fn splatted_output(self) -> TokenStream {
        let len = self.len;
        let expr = self.value.expr();
        quote! {
            #expr.into_iter().take(#len).collect::<::std::vec::Vec<_>>()
        }
    }
}

impl Parse for VecRepeatInput {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(VecRepeatInput {
            value: input.parse()?,
            len: {
                let _: Token![;] = input.parse()?;
                input.parse()?
            },
        })
    }
}

pub struct VecSeqInput(SeqInput);

impl Parse for VecSeqInput {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(VecSeqInput(input.parse()?))
    }
}

impl VecSeqInput {
    fn into_output(self) -> TokenStream {
        if self.0.is_simple() {
            let values = self.0.simple_output();
            quote! {
                ::std::vec![#values]
            }
        } else {
            let target = Ident::new("vec", Span::call_site());
            let values = self.0.values();
            // A simple heuristic for the initial capacity. At this point we can guess that
            // the output length is likely to be greater than the number of values, since
            // at least one of the values is an iterator. This will reduce the number of
            // allocations in common cases, while not massively over-allocating when the
            // collection is small.
            let initial_capacity = 16.max(values.len().next_power_of_two() * 2);
            let updates = self.0.values().map(|value| match value {
                Value::One(expr) => quote! {
                    #target.push(#expr);
                },
                Value::Many(expr) => quote! {
                    #target.extend(#expr);
                },
            });
            quote! {{
                let mut #target = ::std::vec::Vec::with_capacity(#initial_capacity);
                #(#updates)*
                #target
            }}
        }
    }
}
