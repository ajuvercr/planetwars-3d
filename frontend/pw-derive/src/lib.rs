extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::Span;

mod attrs_parse;
use attrs_parse::MAttrs;

fn parse_attrs(attrs: &Vec<syn::Attribute>) -> syn::Result<HashMap<String, syn::Lit>> {
    attrs
        .iter()
        .find(|i| i.path.is_ident("settings"))
        .map(|attr| attr.parse_args::<MAttrs>().map(|x| x.0))
        .unwrap_or(Ok(HashMap::new()))
}

#[derive(Default)]
struct AliasGenerator {
    at: usize,
}

impl AliasGenerator {
    fn alias_ident(&mut self, i: &syn::Ident) -> syn::Ident {
        let orig = i.to_string();
        let mut c = orig.chars();
        let alias = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        };

        self.at += 1;

        syn::Ident::new(&format!("{}Alias{}", alias, self.at), Span::call_site())
    }
}

const IGNORE_KEYS: [&'static str;1] = ["name"];
fn gen_config_default_fields(map: &HashMap<String, syn::Lit>) -> TokenStream2 {
    let mut out = Vec::new();

    for (key, value) in map {
        if IGNORE_KEYS[..].contains(&key.as_str()) { continue; }
        let ident = syn::Ident::new(key, Span::call_site());
        out.push(quote!{
            #ident: #value.into(),
        });
    }

    out.into_iter().collect()
}


#[proc_macro_derive(Settings, attributes(settings))]
pub fn settings_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut config_fields = Vec::new();
    let mut config_aliases = Vec::new();
    let mut config_default_fields = Vec::new();
    let mut default_settings = Vec::new();
    let mut to_settings = Vec::new();

    let mut alias_generator = AliasGenerator::default();

    for field in fields {
        let attrs = match parse_attrs(&field.attrs) {
            Ok(attrs) => attrs,
            Err(e) => return e.to_compile_error().into(),
        };
        let id = field
            .ident
            .as_ref()
            .expect("Wtf this field has no Ient")
            .to_string();
        let name = attrs
            .get("name")
            .map(|x| quote! { #x })
            .unwrap_or(quote! { #id });

        // let (value, min, max, inc) = (
        //     attrs
        //         .get("value")
        //         .map(|x| quote! { Some(#x.into()) })
        //         .unwrap_or(quote! { None }),
        //     attrs
        //         .get("min")
        //         .map(|x| quote! { Some(#x) })
        //         .unwrap_or(quote! { None }),
        //     attrs
        //         .get("max")
        //         .map(|x| quote! { Some(#x) })
        //         .unwrap_or(quote! { None }),
        //     attrs
        //         .get("inc")
        //         .map(|x| quote! { Some(#x) })
        //         .unwrap_or(quote! { None }),
        // );

        let ident = &field.ident.as_ref().expect("wtf no ident");
        let ty = &field.ty;

        config_fields.push(quote! {
            pub #ident: <#ty as ::pw_settings::FieldTrait>::Config,
        });

        let alias_ident = alias_generator.alias_ident(ident);
        config_aliases.push(
            quote! {
                type #alias_ident = <#ty as ::pw_settings::FieldTrait>::Config;
            }
        );

        let defaults = gen_config_default_fields(&attrs);
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
    let aliases: TokenStream2 = config_aliases.into_iter().collect();
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
                use ::pw_settings::{SettingsTrait, FieldTrait};
                let mut settings = ::pw_settings::Settings::new();

                #into_stream

                settings
            }
        }
    };

    println!("Final struct\n{}", inner);

    TokenStream::from(inner)
}
