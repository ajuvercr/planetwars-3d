use proc_macro2::Span;
use std::collections::HashMap;
use syn::parse::*;
use syn::{bracketed, Ident, LitFloat, LitStr, Token};

struct Spanned<T>(pub Span, pub T);
impl<T> Into<(Span, T)> for Spanned<T> {
    fn into(self) -> (Span, T) {
        (self.0, self.1)
    }
}
impl<T> From<(Span, T)> for Spanned<T> {
    fn from((span, t): (Span, T)) -> Self {
        Spanned(span, t)
    }
}

#[derive(Debug)]
pub enum MapValue {
    Str(String),
    Float(f32),
    Vector([f32; 3]),
}

impl Parse for Spanned<MapValue> {
    fn parse(input: ParseStream<'_>) -> Result<Spanned<MapValue>> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input
                .parse::<LitStr>()
                .map(|lit| (lit.span(), MapValue::Str(lit.value())).into())
        } else if lookahead.peek(LitFloat) {
            input.parse::<LitFloat>().and_then(|lit| {
                lit.base10_parse()
                    .map(|x| (lit.span(), MapValue::Float(x)).into())
            })
        } else {
            if input.fork().parse::<Spanned<MyVec>>().is_ok() {
                input
                    .parse::<Spanned<MyVec>>()
                    .map(|Spanned(span, MyVec(lit))| (span, MapValue::Vector(lit)).into())
            } else {
                Err(lookahead.error())
            }
        }
    }
}

#[derive(Debug)]
pub struct AttrParseMap(pub HashMap<String, (Span, MapValue)>);
impl Parse for AttrParseMap {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut map = HashMap::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let Spanned(span, value) = input.parse()?;
            input.parse::<Option<Token![,]>>()?;

            map.insert(ident.to_string(), (span, value));
        }

        Ok(Self(map))
    }
}

#[derive(Debug, Clone)]
pub struct MyVec([f32; 3]);
impl Parse for Spanned<MyVec> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        bracketed!(content in input);

        let x = content
            .parse::<LitFloat>()
            .and_then(|lit| lit.base10_parse())?;
        content.parse::<Token![,]>()?;

        let y = content
            .parse::<LitFloat>()
            .and_then(|lit| lit.base10_parse())?;
        content.parse::<Token![,]>()?;

        let z = content
            .parse::<LitFloat>()
            .and_then(|lit| lit.base10_parse())?;
        content.parse::<Option<Token![,]>>()?;

        Ok(Self(input.span(), MyVec([x, y, z])))
    }
}
