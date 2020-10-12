use std::collections::{HashMap};
use syn::parse::*;
use syn::{Ident, Token, LitStr, LitFloat};
use proc_macro2::Span;

#[derive(Debug)]
pub enum MapValue {
    Str(String, Span),
    Float(f32, Span),
}

impl Parse for MapValue {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse::<LitStr>().map(|lit| MapValue::Str(lit.value(), lit.span()))
        } else if lookahead.peek(LitFloat) {
            input.parse::<LitFloat>().and_then(|lit| lit.base10_parse().map(|x| MapValue::Float(x, lit.span())))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
pub struct AttrParseMap(pub HashMap<String, MapValue>);
impl Parse for AttrParseMap {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut map = HashMap::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: MapValue = input.parse()?;
            input.parse::<Option<Token![,]>>()?;

            map.insert(ident.to_string(), value);
        }

        Ok(Self(map))
    }
}
