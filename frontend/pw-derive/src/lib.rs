extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, LitStr, Token};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use quote::ToTokens;

#[proc_macro_derive(Settings)]
pub fn settings_derive(input: TokenStream) -> TokenStream {
    // let args = parse_macro_input!(args as RouteArgs);
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    for field in fields {
        println!("type: {:?}", field.ty.to_token_stream().to_string());
    }

    // println!("Fields {:?}", fields);


    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);
    let struct_name = &input.ident;

    TokenStream::from(quote! {
        // Preserve the input struct unchanged in the output.
        impl ::pw_settings::SettingsTrait for #struct_name {
            fn into_settings() -> ::pw_settings::Settings {
                ::pw_settings::Settings::new()
            }
        }
    })
}
