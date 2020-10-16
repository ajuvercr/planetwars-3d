extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::{quote, ToTokens};
use std::collections::{HashMap, HashSet};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields};

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;

mod attrs_parse;
use attrs_parse::{AttrParseMap, MapValue};
use syn::spanned::Spanned;

static FLOAT_DEFAULT: f32 = 0.0;
static FLOAT_MIN: f32 = 0.0;
static FLOAT_MAX: f32 = 1.0;
static FLOAT_INC: f32 = 0.1;


macro_rules! unpack_field {
    (String: $map:ident, $id:expr, $default:expr) => {
        if let Some((span, value)) = $map.get($id) {
            match value {
                attrs_parse::MapValue::Str(s) => Ok(s.clone()),
                _ => Err(syn::Error::new(span.clone(), "Expected a String value")),
            }
        } else {
            Ok($default)
        }
    };
    (Float: $map:ident, $id:expr, $default:expr) => {
        if let Some((span, value)) = $map.get($id) {
            match value {
                attrs_parse::MapValue::Float(s) => Ok(s.clone()),
                _ => Err(syn::Error::new(span.clone(), "Expected a Float value")),
            }
        } else {
            Ok($default)
        }
    };
    (Vector: $map:ident, $id:expr, $default:expr) => {
        if let Some((span, value)) = $map.get($id) {
            match value {
                attrs_parse::MapValue::Vector(s) => Ok(s.clone()),
                _ => Err(syn::Error::new(span.clone(), "Expected a Vector value")),
            }
        } else {
            Ok($default)
        }
    };
}

fn parse_attrs(attrs: &Vec<syn::Attribute>) -> syn::Result<HashMap<String, (Span, MapValue)>> {
    attrs
        .iter()
        .find(|i| i.path.is_ident("settings"))
        .map(|attr| attr.parse_args::<AttrParseMap>().map(|x| x.0))
        .unwrap_or(Ok(HashMap::new()))
}

fn map_field(
    field: &Field,
    ids: &mut HashSet<String>,
) -> syn::Result<(TokenStream2, TokenStream2, TokenStream2)> {
    let id = field.ident.as_ref().unwrap().to_string();
    let name = id.clone();
    let map = parse_attrs(&field.attrs)?;
    let id = unpack_field!(String: map, "id", id)?;
    let name = unpack_field!(String: map, "name", name)?;

    if !ids.insert(id.clone()) {
        return Err(syn::Error::new(
            field.span(),
            format!("Can't add id '{}' twice", id.to_string()),
        ));
    }

    let ident = &field.ident;

    match &field.ty.to_token_stream().to_string()[..] {
        "f32" => {
            let value = unpack_field!(Float: map, "value", FLOAT_DEFAULT)?;
            let min = unpack_field!(Float: map, "min", FLOAT_MIN)?;
            let max = unpack_field!(Float: map, "max", FLOAT_MAX)?;
            let inc = unpack_field!(Float: map, "inc", FLOAT_INC)?;

            Ok((
                quote! {
                    #ident: #value,
                },
                quote! {
                    settings.add_slider(#id, #name, #value, #min, #max, #inc);
                },
                quote! {
                    settings.add_slider(#id, #name, self.#ident.clone(), #min, #max, #inc);
                },
            ))
        }
        "[f32 ; 3]" => {
            let [x, y, z] = unpack_field!(Vector: map, "value", [FLOAT_DEFAULT, FLOAT_DEFAULT, FLOAT_DEFAULT])?;
            let min = unpack_field!(Float: map, "min", FLOAT_MIN)?;
            let max = unpack_field!(Float: map, "max", FLOAT_MAX)?;
            let inc = unpack_field!(Float: map, "inc", FLOAT_INC)?;

            let value_quote = quote! { [ #x, #y, #z ] };
            Ok((
                quote! {
                    #ident: #value_quote,
                },
                quote! {
                    settings.add_vec3(#id, #name, #value_quote, #min, #max, #inc);
                },
                quote! {
                    settings.add_vec3(#id, #name, self.#ident.clone(), #min, #max, #inc);
                },
            ))
        }
        "String" => {
            let value = unpack_field!(String: map, "value", String::new())?;
            Ok((
                quote! {
                    #ident: #value.to_string(),
                },
                quote! {
                    settings.add_text(#id, #name, #value);
                },
                quote! {
                    settings.add_text(#id, #name, self.#ident.clone());
                },
            ))
        }
        _ => {
            let ty = &field.ty;

            Ok((
                quote! {
                    #ident: <#ty as ::pw_settings::SettingsTrait>::default_settings(),
                },
                quote! {
                    settings.add_settings::<_, _, #ty>(#id, #name);
                },
                quote! {
                    settings.add_settings_with::<_, _, #ty>(#id, #name, self.#ident.to_settings());
                },
            ))
        }
    }
}

#[proc_macro_derive(Settings, attributes(id, name, value, min, max, inc, settings))]
pub fn settings_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut ids = HashSet::new();

    let mut default_settings = Vec::new();
    let mut new_settings = Vec::new();
    let mut to_settings = Vec::new();

    for field in fields {
        match map_field(&field, &mut ids) {
            Ok((default, new_setting, into)) => {
                default_settings.push(default);
                new_settings.push(new_setting);
                to_settings.push(into);
            }
            Err(e) => return e.to_compile_error().into(),
        }
    }
    let default_stream: TokenStream2 = default_settings.into_iter().collect();
    let into_stream: TokenStream2 = to_settings.into_iter().collect();
    let new_stream: TokenStream2 = new_settings.into_iter().collect();

    let generics = input.generics;
    let struct_name = input.ident;
    TokenStream::from(quote! {
        // Preserve the input struct unchanged in the output.
        impl #generics ::pw_settings::SettingsTrait for #struct_name {
            fn default_settings() -> Self {
                Self {
                   #default_stream
                }
            }

            fn to_settings(&self) -> ::pw_settings::Settings {
                let mut settings = ::pw_settings::Settings::new();

                #into_stream

                settings
            }

            fn new_settings() -> ::pw_settings::Settings {
                let mut settings = ::pw_settings::Settings::new();

                #new_stream

                settings
            }
        }
    })
}

#[proc_macro_derive(Test, attributes(attrs))]
pub fn test_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    for field in fields {
        println!("------------------------");
        for attr in &field.attrs {
            if attr.path.is_ident("attrs") {
                match attr.parse_args::<AttrParseMap>() {
                    Ok(map) => {
                        println!("keys: {:?}", map.0.keys());
                        println!("values: {:?}", map.0.values());
                    }
                    Err(e) => println!("Err: {:?}", e),
                }
            }
        }
    }

    TokenStream::from(quote! {})
}
