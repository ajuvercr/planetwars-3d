use proc_macro2::{Delimiter, Group, Span};
use quote::{quote, TokenStreamExt};
use std::collections::HashMap;
use syn::parse::*;
use syn::{bracketed, Ident, LitFloat, LitStr, Token};

#[derive(Debug)]
pub enum MapValue {
    Str(String, Span),
    Float(f32, Span),
    Vector(MyVec, Span),
}

impl Parse for MapValue {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input
                .parse::<LitStr>()
                .map(|lit| MapValue::Str(lit.value(), lit.span()))
        } else if lookahead.peek(LitFloat) {
            input
                .parse::<LitFloat>()
                .and_then(|lit| lit.base10_parse().map(|x| MapValue::Float(x, lit.span())))
        } else {
            if input.fork().parse::<MyVec>().is_ok() {
                input
                    .parse::<MyVec>()
                    .map(|MyVec(lit, span)| MapValue::Vector(MyVec(lit, span.clone()), span))
            } else {
                Err(lookahead.error())
            }
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

#[derive(Debug, Clone)]
pub struct MyVec([f32; 3], Span);
impl MyVec {
    pub fn new(inner: [f32; 3]) -> Self {
        Self(inner, Span::call_site())
    }
}
impl Parse for MyVec {
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

        Ok(Self([x, y, z], input.span()))
    }
}

impl quote::ToTokens for MyVec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let [x, y, z] = self.0;

        tokens.append(Group::new(Delimiter::Bracket, quote! {#x, #y, #z}));
    }
}
