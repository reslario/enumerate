extern crate proc_macro;

mod parse;
mod generate;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemTrait;
use crate::parse::Arguments;
use crate::generate::ToSnakeCase;

#[proc_macro_attribute]
pub fn enumerate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let Arguments { enum_name, impls } = syn::parse_macro_input!(attr as Arguments);
    let trait_def = syn::parse_macro_input!(item as ItemTrait);
    let enum_name = enum_name.unwrap_or(trait_def.ident.clone());
    let enum_def = generate::generate_enum(&enum_name, &trait_def.ident, &impls);
    let impl_def = generate::generate_impl(&trait_def, &enum_name, &impls);
    let froms_def = generate::generate_froms(&enum_name, &impls, &trait_def.ident);
    let vis = &trait_def.vis;
    let body = quote! {
        #enum_def
        #impl_def
        #froms_def
    };
    if enum_name == trait_def.ident {
        let module = Ident::new(
            &*format!("{}_enm", enum_name.to_string().to_snake_case()),
            Span::call_site()
        );
        (quote! {
            #trait_def
            #vis mod #module {
                pub #body
            }
        }).into()
    } else {
        (quote! {
            #trait_def
            #vis #body
        }).into()
    }
}

