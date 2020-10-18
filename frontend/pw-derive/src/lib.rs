extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

use proc_macro2::TokenStream as TokenStream2;

mod attrs_parse;
use attrs_parse::MAttrs;

fn parse_attrs(attrs: &Vec<syn::Attribute>) -> syn::Result<HashMap<String, syn::Lit>> {
    attrs
        .iter()
        .find(|i| i.path.is_ident("settings"))
        .map(|attr| attr.parse_args::<MAttrs>().map(|x| x.0))
        .unwrap_or(Ok(HashMap::new()))
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

    let mut default_settings = Vec::new();
    let mut to_settings = Vec::new();

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

        let (value, min, max, inc) = (
            attrs
                .get("value")
                .map(|x| quote! { Some(#x) })
                .unwrap_or(quote! { None }),
            attrs
                .get("min")
                .map(|x| quote! { Some(#x) })
                .unwrap_or(quote! { None }),
            attrs
                .get("max")
                .map(|x| quote! { Some(#x) })
                .unwrap_or(quote! { None }),
            attrs
                .get("inc")
                .map(|x| quote! { Some(#x) })
                .unwrap_or(quote! { None }),
        );

        let ident = &field.ident;
        let ty = &field.ty;

        let settings = quote! {::pw_settings::FieldConfig { value: #value, min: #min, max: #max, inc: #inc, }
        };

        default_settings.push(quote! {
            #ident: <#ty as ::pw_settings::FieldTrait>::default_self(&#settings),
        });

        to_settings.push(quote! {
            settings.add_field(
                #id, #name,
                self.#ident.to_field(&#settings)
            );
        });
    }

    let default_stream: TokenStream2 = default_settings.into_iter().collect();
    let into_stream: TokenStream2 = to_settings.into_iter().collect();

    let generics = input.generics;
    let struct_name = input.ident;

    let inner = quote! {
        impl #generics ::pw_settings::SettingsTrait for #struct_name {
            fn default_settings() -> Self {
                use ::pw_settings::{SettingsTrait, FieldTrait};
                Self {
                    #default_stream
                }
            }

            fn to_settings(&self) -> ::pw_settings::Settings {
                use ::pw_settings::{SettingsTrait, FieldTrait};
                let mut settings = ::pw_settings::Settings::new();

                #into_stream

                settings
            }
        }
    };

    TokenStream::from(inner)
}
