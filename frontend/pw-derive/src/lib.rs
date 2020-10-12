extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields};
use std::collections::{HashSet, HashMap};

use proc_macro2::TokenStream as TokenStream2;

mod attrs_parse;
use attrs_parse::{AttrParseMap, MapValue};
use syn::spanned::Spanned;

macro_rules! unpack_field {
    (String: $map:ident, $id:expr, $default:expr) => {
        if let Some(v) = $map.get($id) {
            match v {
                attrs_parse::MapValue::Str(s, _) => Ok(s.clone()),
                attrs_parse::MapValue::Float(_, span) => Err(syn::Error::new(span.clone(), "Expected a String value"))
            }
        } else {
            Ok($default)
        }
    };
    (Float: $map:ident, $id:expr, $default:expr) => {
        if let Some(v) = $map.get($id) {
            match v {
                attrs_parse::MapValue::Float(s, _) => Ok(s.clone()),
                attrs_parse::MapValue::Str(_, span) => Err(syn::Error::new(span.clone(), "Expected a Float value"))
            }
        } else {
            Ok($default)
        }
    };
}

#[derive(Debug)]
struct StringDefaults {
    id: TokenStream2,
    name: TokenStream2,
    value: TokenStream2,
}

fn parse_attrs(attrs: &Vec<syn::Attribute>) -> syn::Result<HashMap<String, MapValue>> {
    attrs.iter().find(|i| i.path.is_ident("settings")).map(|attr|  attr.parse_args::<AttrParseMap>().map(|x| x.0)).unwrap_or(Ok(HashMap::new()))
}

fn string_defaults(field: &Field) -> syn::Result<StringDefaults> {
    let id = field.ident.as_ref().unwrap().to_string();
    let name = id.clone();
    let map = parse_attrs(&field.attrs)?;

    let id = unpack_field!(String: map, "id", id)?;
    let name = unpack_field!(String: map, "name", name)?;
    let value = unpack_field!(String: map, "value", String::new())?;

    Ok(StringDefaults {
        id: quote!{ #id },
        name: quote!{ #name },
        value: quote!{ #value },
    })
}

#[derive(Debug)]
struct SliderDefaults {
    id: TokenStream2,
    name: TokenStream2,
    value: TokenStream2,
    min: TokenStream2,
    max: TokenStream2,
    inc: TokenStream2,
}

fn slider_defaults(field: &Field) -> syn::Result<SliderDefaults> {
    let id = field.ident.as_ref().unwrap().to_string();
    let name = id.clone();
    let map = parse_attrs(&field.attrs)?;

    let id = unpack_field!(String: map, "id", id)?;
    let name = unpack_field!(String: map, "name", name)?;
    let value = unpack_field!(Float: map, "value", 0.0)?;

    let min = unpack_field!(Float: map, "min", 0.0)?;
    let max = unpack_field!(Float: map, "max", 1.0)?;
    let inc = unpack_field!(Float: map, "inc", 0.1)?;

    Ok(SliderDefaults {
        id: quote!{ #id },
        name: quote!{ #name },
        value: quote!{ #value },
        min: quote!{ #min },
        max: quote!{ #max },
        inc: quote!{ #inc },
    })
}

fn map_field(field: &Field, ids: &mut HashSet<String>) -> syn::Result<TokenStream2> {
    match &field.ty.to_token_stream().to_string()[..] {
        "f32" => {
            let SliderDefaults { id, name, value, min, max, inc } = slider_defaults(field)?;

            if !ids.insert(id.to_string()) {
                return Err(syn::Error::new(field.span(), format!("Can't add id '{}' twice", id.to_string())));
            }

            Ok(quote!{
                settings.add_slider(#id, #name, #value, #min, #max, #inc);
            })
        },
        "[f32;3]" => Err(syn::Error::new(field.span(), "[f32;3] is not yet implemented")),
        "String" => {
            let StringDefaults { id, name, value } = string_defaults(field)?;

            if !ids.insert(id.to_string()) {
                return Err(syn::Error::new(field.span(), format!("Can't add id '{}' twice", id.to_string())));
            }

            Ok(quote!{
                settings.add_text(#id, #name, #value);
            })
        },
        _ => Err(syn::Error::new(field.span(), "Only supported types are String and f32.")),
    }
}

use quote::ToTokens;

#[proc_macro_derive(Settings, attributes(id, name, value, min, max, inc, settings))]
pub fn settings_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut ids = HashSet::new();
    let mut token_streams = Vec::new();
    for field in fields {
        match map_field(&field, &mut ids) {
            Ok(stream) => token_streams.push(stream),
            Err(e) => return e.to_compile_error().into(),
        }
    }
    let field_stream: TokenStream2 = token_streams.into_iter().collect();

    println!("{}", field_stream.to_string());

    let struct_name = &input.ident;
    TokenStream::from(quote! {
        // Preserve the input struct unchanged in the output.
        impl ::pw_settings::SettingsTrait for #struct_name {
            fn into_settings() -> ::pw_settings::Settings {
                let mut settings = ::pw_settings::Settings::new();

                #field_stream

                settings
            }
        }
    })
}

#[proc_macro_derive(Test, attributes(attrs))]
pub fn test_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
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
                    },
                    Err(e) => println!("Err: {:?}", e)
                }
            }
        }
    }

    TokenStream::from(quote! {
    })
}
