extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields};
use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;

macro_rules! parse_with_name {
    ($attrs:ident, $id:expr, $default:expr) => {
        $attrs.iter().find(|i| i.path.is_ident($id)).and_then(|attr| attr.parse_args::<TokenStream2>().ok()).unwrap_or($default)
    };
    // ($attrs:ident, $id:expr, $default:tt) => {
    //     parse_with_name!($attrs, $id, $default, LitStr)
    // };
}

#[derive(Debug)]
struct StringDefaults {
    id: TokenStream2,
    name: TokenStream2,
    value: TokenStream2,
}

fn string_defaults(field: &Field) -> StringDefaults {
    let id = field.ident.as_ref().unwrap().to_string();
    let name = id.clone();
    let attrs = &field.attrs;

    StringDefaults {
        id: parse_with_name!(attrs, "id", quote!{ #id }),
        name: parse_with_name!(attrs, "name", quote!{ #name }),
        value: parse_with_name!(attrs, "value", quote!{ String::new() }),
    }
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

fn slider_defaults(field: &Field) -> SliderDefaults {
    let id = field.ident.as_ref().unwrap().to_string();
    let name = id.clone();
    let attrs = &field.attrs;

    SliderDefaults {
        id: parse_with_name!(attrs, "id", quote!{ #id }),
        name: parse_with_name!(attrs, "name", quote!{ #name }),
        value: parse_with_name!(attrs, "value", quote!{ 0.0 }),
        min: parse_with_name!(attrs, "value", quote!{ 0.0 }),
        max: parse_with_name!(attrs, "value", quote!{ 0.0 }),
        inc: parse_with_name!(attrs, "value", quote!{ 0.0 }),
    }
}

fn map_field(field: &Field, ids: &mut HashSet<String>) -> Option<TokenStream2> {
    match &field.ty.to_token_stream().to_string()[..] {
        "f32" => {
            let SliderDefaults { id, name, value, min, max, inc } = slider_defaults(field);
            if !ids.insert(id.to_string()) {
                panic!("Can't add id '{}' twice", id.to_string());
            }
            Some(quote!{
                settings.add_slider(#id, #name, #value, #min, #max, #inc);
            })
        },
        "[f32;3]" => None,
        "String" => {
            let StringDefaults { id, name, value } = string_defaults(field);
            if !ids.insert(id.to_string()) {
                panic!("Can't add id '{}' twice", id.to_string());
            }
            Some(quote!{
                settings.add_text(#id, #name, #value);
            })
        },
        _ => None,
    }
}

use quote::ToTokens;

#[proc_macro_derive(Settings, attributes(id, name, value, min, max, inc))]
pub fn settings_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut ids = HashSet::new();
    let field_stream: TokenStream2 = fields.iter().filter_map(|x| map_field(x, &mut ids)).collect();

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
