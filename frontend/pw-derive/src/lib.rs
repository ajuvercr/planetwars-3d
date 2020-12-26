#![allow(dead_code)]
extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::Span;

mod attrs_parse;
use attrs_parse::{MAttrs, MAttr};

fn to_cammel_case(orig: &str) -> String {
    let chars = orig.chars();
    let mut start_word = true;
    chars.filter_map(|c| {
        if c == '_' {
            start_word = true; None
        } else {
            if start_word {
                start_word = false;
                Some(c.to_uppercase().next().unwrap())
            } else {
                Some(c)
            }
        }
    }).collect()
}

fn parse_attrs(attrs: &Vec<syn::Attribute>) -> syn::Result<HashMap<syn::Ident, MAttr>> {
    attrs
        .iter()
        .find(|i| i.path.is_ident("settings"))
        .map(|attr| attr.parse_args::<MAttrs>().map(|x| x.0))
        .unwrap_or(Ok(HashMap::new()))
}

#[derive(Default)]
struct AliasGenerator {
    aliases: Vec<TokenStream2>,
    recurse: Vec<TokenStream2>,
    at: usize,
}

impl AliasGenerator {
    fn push<S: std::fmt::Display>(&mut self, i: S) -> syn::Ident {
        assert!(!self.recurse.is_empty(), "AliasGenerator was in an invalid state!");
        let ty = self.last();

        self.push_with_type(i, ty)
    }

    fn push_with_type<S: std::fmt::Display, T: quote::ToTokens>(&mut self, i: S, ty: T) -> syn::Ident {
        let orig = i.to_string();
        let alias = to_cammel_case(&orig);

        self.at += 1;

        let ident = syn::Ident::new(&format!("{}Alias{}", alias, self.at), Span::call_site());

        self.aliases.push(
            quote! {
                type #ident = <#ty as ::pw_settings::FieldTrait>::Config;
            }
        );
        self.recurse.push(quote!{#ident});

        ident
    }

    fn reset(&mut self, i: &syn::Ident, ty: &syn::Type) -> syn::Ident {
        self.recurse.clear();
        self.recurse.push(quote! { #ty });
        self.push(i)
    }

    fn last(&self) -> TokenStream2 {
        assert!(!self.recurse.is_empty(), "AliasGenerator was in an invalid state!");

        self.recurse.last().cloned().unwrap()
    }

    fn pop(&mut self) {
        self.recurse.pop();
    }

    fn into_aliases(self) -> Vec<TokenStream2> {
        self.aliases
    }
}

const IGNORE_KEYS: [&'static str;2] = ["name", "ty"];
fn gen_config_default_fields(alias_generator: &mut AliasGenerator, map: &HashMap<syn::Ident, MAttr>) -> syn::parse::Result<TokenStream2> {
    let mut out = Vec::new();

    for (key, value) in map {
        if IGNORE_KEYS[..].contains(&key.to_string().as_str()) { continue; }
        match value {
            MAttr::Lit(ref lit) => {
                out.push(quote!{
                    #key: (#lit).into(),
                });
            },
            MAttr::Stream(ref stream) => {
                let attrs = syn::parse2::<MAttrs>(stream.clone())?.0;
                let ty = attrs.get(&syn::Ident::new("ty", Span::call_site())).cloned().map(|x| x.token_stream());

                let alias = match ty {
                    Some(ref t) => {
                        alias_generator.push_with_type(key, t)
                    },
                    None => {
                        alias_generator.push(key)
                    }
                };

                let defaults = gen_config_default_fields(alias_generator, &attrs)?;
                out.push(quote! {
                    #key: #alias {
                        #defaults
                        ..Default::default()
                    },
                });

                alias_generator.pop();
            }
        }
    }

    Ok(out.into_iter().collect())
}

#[proc_macro_derive(Settings, attributes(settings))]
pub fn settings_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attrs = match parse_attrs(&input.attrs) {
        Ok(attrs) => attrs,
        Err(e) => return e.to_compile_error().into(),
    };

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut config_fields = Vec::new();
    let mut config_default_fields = Vec::new();
    let mut default_settings = Vec::new();
    let mut to_settings = Vec::new();

    let mut alias_generator = AliasGenerator::default();

    for field in fields {
        let ident = &field.ident.as_ref().expect("wtf no ident");
        let ty = &field.ty;


        let attrs = match parse_attrs(&field.attrs).or_else(|_|
            attrs.get(ident).cloned().map(|x| syn::parse2::<MAttrs>(x.stream()).map(|x| x.0)).unwrap_or(Ok(HashMap::new()))
        ) {
            Ok(e) => e,
            Err(e) => return e.to_compile_error().into(),
        };

        let alias_ident = alias_generator.reset(ident, ty);

        let id = ident.to_string();
        let name = attrs
            .get(&syn::Ident::new("name", Span::call_site())).map(|x| x.clone().lit())
            .unwrap_or(quote! { #id });

        config_fields.push(quote! {
            pub #ident: <#ty as ::pw_settings::FieldTrait>::Config,
        });

        let defaults = match gen_config_default_fields(&mut alias_generator, &attrs) {
            Ok(x) => x,
            Err(e) => return e.to_compile_error().into(),
        };

        config_default_fields.push(quote! {
            #ident: #alias_ident {
                #defaults
                ..Default::default()
            },
        });

        default_settings.push(quote! {
            #ident: <#ty as ::pw_settings::FieldTrait>::default_self(&config.#ident),
        });

        // if attrs.contains_key("data") {
        //     to_settings.push(quote! {
        //         settings.add_data(
        //             #id,
        //             &self.#ident
        //         );
        //     });
        // } else {
            to_settings.push(quote! {
                settings.add_field(
                    #id, #name,
                    self.#ident.to_field(&config.#ident)
                );
            });
        // }
    }

    let struct_stream: TokenStream2 = config_fields.into_iter().collect();
    let aliases: TokenStream2 = alias_generator.into_aliases().into_iter().collect();
    let struct_stream_default: TokenStream2 = config_default_fields.into_iter().collect();

    let default_stream: TokenStream2 = default_settings.into_iter().collect();
    let into_stream: TokenStream2 = to_settings.into_iter().collect();

    let generics = input.generics;
    let struct_name = input.ident;

    let type_ident = syn::Ident::new(&format!("{}Config", struct_name), Span::call_site());

    let inner = quote! {
        #[derive(Clone)]
        pub struct #type_ident {
            #struct_stream
        }

        impl Default for #type_ident {
            fn default() -> Self {
                #aliases
                Self {
                    #struct_stream_default
                }
            }
        }

        impl #generics ::pw_settings::SettingsTrait for #struct_name {
            type Config = #type_ident;

            fn default_settings_with(config: &Self::Config) -> Self {
                Self {
                    #default_stream
                }
            }

            fn to_settings_with(&self, config: &Self::Config) -> ::pw_settings::Settings {
                use ::pw_settings::FieldTrait;
                let mut settings = ::pw_settings::Settings::new();

                #into_stream

                settings
            }
        }
    };

    println!("{}", inner);

    TokenStream::from(inner)
}
