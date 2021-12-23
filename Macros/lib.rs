#![allow(non_snake_case)]
#![allow(unused_variables)]
#![recursion_limit = "256"]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Reflect, attributes(property))]
pub fn reflect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl Reflect for #name {}
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
