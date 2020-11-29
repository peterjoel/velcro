use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

pub enum Value {
    One(Expr),
    Many(Expr),
}

impl Value {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::One(_))
    }

    pub fn expr(&self) -> &Expr {
        match self {
            Value::One(expr) | Value::Many(expr) => expr,
        }
    }
}

impl Parse for Value {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        if input.peek(Token![..]) {
            let _: Token![..] = input.parse()?;
            Ok(Value::Many(input.parse()?))
        } else {
            Ok(Value::One(input.parse()?))
        }
    }
}
