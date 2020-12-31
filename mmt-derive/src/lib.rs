extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn DB(attr: TokenStream, item: TokenStream) -> TokenStream {
    let collection = attr.to_string();

    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident.clone();

    let tokens = quote! {
        #input

        impl mmt::db::Db<'_> for #name {
            const COLLECTION: &'static str = #collection;
        }

        impl mmt::db::Update for #name {
            const COLLECTION: &'static str = #collection;
        }

        impl mmt::db::Create for #name {
            const COLLECTION: &'static str = #collection;
        }

        impl mmt::db::Delete for #name {
            const COLLECTION: &'static str = #collection;
        }
    };

    tokens.into()
}
