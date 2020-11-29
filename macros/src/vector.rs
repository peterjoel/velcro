use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{Pair, Punctuated};
use syn::{Expr, Token};

pub(crate) enum VecInput {
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

pub(crate) struct VecRepeatInput {
    value: VecValue,
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

pub(crate) struct VecSeqInput {
    values: Punctuated<VecValue, Token![,]>,
}

impl Parse for VecSeqInput {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        Ok(VecSeqInput {
            values: input.parse_terminated(VecValue::parse)?,
        })
    }
}

impl VecSeqInput {
    fn into_output(self) -> TokenStream {
        if self.values.iter().all(VecValue::is_simple) {
            self.simple_output()
        } else {
            self.splatted_output()
        }
    }

    fn splatted_output(self) -> TokenStream {
        let updates = self
            .values
            .iter()
            .map(|value| match value {
                VecValue::One(expr) => quote! {
                    vec.push(#expr);
                },
                VecValue::Many(expr) => quote! {
                    vec.extend(#expr);
                },
            })
            .collect::<TokenStream>();

        quote! {{
            let mut vec = ::std::vec::Vec::new();
            #updates
            vec
        }}
    }

    fn simple_output(self) -> TokenStream {
        let values = self
            .values
            .into_pairs()
            .map(Pair::into_tuple)
            .map(|(expr, delim)| Pair::new(expr.expr(), delim).into_token_stream())
            .collect::<TokenStream>();
        quote! {
            ::std::vec![#values]
        }
    }
}

enum VecValue {
    One(Expr),
    Many(Expr),
}

impl VecValue {
    fn is_simple(&self) -> bool {
        matches!(self, Self::One(_))
    }

    fn expr(&self) -> &Expr {
        match self {
            VecValue::One(expr) | VecValue::Many(expr) => expr,
        }
    }
}

impl Parse for VecValue {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        if input.peek(Token![..]) {
            let _: Token![..] = input.parse()?;
            Ok(VecValue::Many(input.parse()?))
        } else {
            Ok(VecValue::One(input.parse()?))
        }
    }
}
