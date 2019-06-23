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
    let Arguments(impls) = syn::parse_macro_input!(attr as Arguments);
    let trait_def = syn::parse_macro_input!(item as ItemTrait);
    let name = &trait_def.ident;
    let enum_def = generate::generate_enum(name, &impls);
    let impl_def = generate::generate_impl(&trait_def, &impls);
    let froms_def = generate::generate_froms(name, &impls);
    let vis = &trait_def.vis;
    let module = Ident::new(
       &*format!("{}_enm", name.to_string().to_snake_case()),
        Span::call_site()
    );
    (quote! {
        #trait_def
        #vis mod #module {
            #enum_def
            #impl_def
            #froms_def
        }
    }).into()
}

