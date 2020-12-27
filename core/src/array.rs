use crate::arraylike::{ArraylikeInput, RepeatInput};
use crate::seq::SeqInput;
use crate::value::{Value, ValueExpr, ValueIterExpr, Verbatim};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{self, Parse, ParseStream};

#[allow(clippy::large_enum_variant)]
pub enum ArrInput<V = Verbatim> {
    Seq(ArrSeqInput<V>),
    Repeat(ArrRepeatInput<V>),
}

impl<V> ArrInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    pub fn into_output(self) -> TokenStream {
        match self {
            ArrInput::Seq(seq) => seq.into_output(),
            ArrInput::Repeat(repeat) => repeat.into_output(),
        }
    }
}

impl<V> From<ArraylikeInput<V>> for ArrInput<V> {
    fn from(input: ArraylikeInput<V>) -> Self {
        match input {
            ArraylikeInput::Seq(seq) => ArrInput::Seq(ArrSeqInput(seq)),
            ArraylikeInput::Repeat(repeat) => ArrInput::Repeat(ArrRepeatInput(repeat)),
        }
    }
}

impl<V> Parse for ArrInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Ok(input.parse::<ArraylikeInput<V>>()?.into())
    }
}

pub struct ArrRepeatInput<V>(RepeatInput<V>);

impl<V> ArrRepeatInput<V>
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
            [#expr; #len]
        }
    }

    fn splatted_output(self) -> TokenStream {
        let len = self.0.len();
        let values = self.0.value();
        quote! {{
            use std::mem::MaybeUninit;
            const len: usize = #len;
            unsafe {
                let mut arr: [MaybeUninit<_>; len] = MaybeUninit::uninit().assume_init();
                let mut count = 0;
                for (value, element) in std::iter::IntoIterator::into_iter(#values).zip(arr.iter_mut()) {
                    *element = MaybeUninit::new(value);
                    count += 1;
                }
                if count < len {
                    // Not enough elements: clean up the half-initialized array and panic
                    for element in &mut arr[0..count] {
                        std::ptr::drop_in_place(element.as_mut_ptr());
                    }
                    panic!("Expected {} items but found {}", len, count);
                }
                unsafe fn arr_init_transmute<T>(arr: [MaybeUninit<T>; #len]) -> [T; #len] {
                    arr.as_ptr().cast::<[T; #len]>().read()
                }
                arr_init_transmute(arr)
            }
        }}
    }
}

pub struct ArrSeqInput<V>(SeqInput<V>);

impl<V> ArrSeqInput<V>
where
    ValueExpr<V>: ToTokens,
    ValueIterExpr<V>: ToTokens,
{
    fn into_output(self) -> TokenStream {
        if self.0.is_simple() {
            let values = self.0.simple_output();
            quote! {
                [#values]
            }
        } else {
            quote_spanned! {
                Span::call_site()=>
                compile_error!("Length must be provided")
            }
        }
    }
}
