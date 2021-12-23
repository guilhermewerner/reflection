#![allow(non_snake_case)]
#![allow(unused_variables)]
#![recursion_limit = "256"]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(Reflect, attributes(property))]
pub fn reflect(input: TokenStream) -> TokenStream {
    let class = parse_macro_input!(input as DeriveInput);
    let name = &class.ident;

    let expanded = quote! {
        unsafe impl Reflect for #name {
            fn TypeName(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
        }
    };

    expanded.into()
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;

    let expanded = quote! {
        #func

        paste::paste! {
            #[doc(hidden)]
            pub fn [<__ #name _Client>]() {}

            #[doc(hidden)]
            pub fn [<__ #name _Server>]() {}
        }
    };

    expanded.into()
}
