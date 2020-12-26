use crate::arraylike::{ArraylikeInput, RepeatInput};
use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{self, Parse, ParseStream};

#[allow(clippy::large_enum_variant)]
pub enum VecInput<V = Verbatim> {
    Seq(VecSeqInput<V>),
    Repeat(VecRepeatInput<V>),
}

impl<V> VecInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        match self {
            VecInput::Seq(seq) => seq.into_output(),
            VecInput::Repeat(repeat) => repeat.into_output(),
        }
    }
}

impl<V> From<ArraylikeInput<V>> for VecInput<V> {
    fn from(input: ArraylikeInput<V>) -> Self {
        match input {
            ArraylikeInput::Seq(seq) => VecInput::Seq(VecSeqInput(seq)),
            ArraylikeInput::Repeat(repeat) => VecInput::Repeat(VecRepeatInput(repeat)),
        }
    }
}

impl<V> Parse for VecInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(input.parse::<ArraylikeInput<V>>()?.into())
    }
}

pub struct VecRepeatInput<V>(RepeatInput<V>);

impl<V> VecRepeatInput<V>
where
    Value<V>: ToTokens,
{
    fn into_output(self) -> TokenStream {
        if self.0.value().is_simple() {
            self.simple_output()
        } else {
            self.splatted_output()
        }
    }

    fn simple_output(self) -> TokenStream {
        let len = self.0.len();
        let expr = &self.0.value();
        quote! {
            ::std::vec![#expr; #len]
        }
    }

    fn splatted_output(self) -> TokenStream {
        let len = self.0.len();
        let expr = &self.0.value();
        quote! {
            std::iter::IntoIterator::into_iter(#expr).take(#len).collect::<::std::vec::Vec<_>>()
        }
    }
}

pub struct VecSeqInput<V>(SeqInput<V>);

impl<V> VecSeqInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
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
            let updates = values.map(|value| match value {
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
