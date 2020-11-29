use std::collections::HashMap;
use syn::parse::*;
use syn::{Ident, Token};
use quote::{quote};

use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;

enum MAttrPriv {
    Lit(Ident, syn::Lit),
    Stream(Ident, TokenStream2),
}

impl Parse for MAttrPriv {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            Ok(MAttrPriv::Stream(ident, content.parse()?))
        } else {
            let value = input.parse()?;
            Ok(MAttrPriv::Lit(ident, value))
        }
    }
}

#[derive(Clone)]
pub enum MAttr {
    Lit(syn::Lit),
    Stream(TokenStream2),
}

impl MAttr {
    pub fn token_stream(self) -> TokenStream2 {
        match self {
            MAttr::Lit(x) => quote!({#x}),
            MAttr::Stream(s) => s,
        }
    }

    pub fn lit(self) -> syn::Lit {
        match self {
            MAttr::Lit(x) => x,
            MAttr::Stream(_) => panic!("Expected literal, found bracketed"),
        }
    }

    pub fn stream(self) -> TokenStream2 {
        match self {
            MAttr::Stream(x) => x,
            MAttr::Lit(_) => panic!("Expected bracketed, found literal"),
        }
    }

    pub fn is_lit(&self) -> bool {
        match self {
            MAttr::Stream(_) => false,
            MAttr::Lit(_) => true,
        }
    }

    pub fn is_stream(&self) -> bool {
        match self {
            MAttr::Stream(_) => true,
            MAttr::Lit(_) => false,
        }
    }
}

pub struct MAttrs(pub HashMap<Ident, MAttr>);
impl Parse for MAttrs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let punc: Punctuated<MAttrPriv, Token![,]> = Punctuated::parse_separated_nonempty(input)?;
        let mut inner = HashMap::new();

        punc.into_pairs().for_each(|p| {
            match p.into_value() {
                MAttrPriv::Lit(key, value) => inner.insert(key, MAttr::Lit(value)),
                MAttrPriv::Stream(key, stream) => inner.insert(key, MAttr::Stream(stream)),
            };
        });

        Ok(Self(inner))
    }
}
