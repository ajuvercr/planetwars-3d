use std::collections::HashMap;
use syn::parse::*;
use syn::{Ident, Token};

// use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;

struct MAttr(String, syn::Lit);
impl Parse for MAttr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: syn::Lit = input.parse()?;
        Ok(Self(ident.to_string(), value))
    }
}

pub struct MAttrs(pub HashMap<String, syn::Lit>);
impl Parse for MAttrs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let punc: Punctuated<MAttr, Token![,]> = Punctuated::parse_separated_nonempty(input)?;
        let mut inner = HashMap::new();

        punc.into_pairs().for_each(|p| {
            let v = p.into_value();
            inner.insert(v.0, v.1);
        });

        Ok(Self(inner))
    }
}
