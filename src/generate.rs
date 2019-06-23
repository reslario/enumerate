use syn::{Path, ItemTrait, TraitItem, MethodSig, FnArg};
use proc_macro2::{TokenStream, Ident};
use quote::quote;
use crate::parse::IsSelf;
use syn::punctuated::Punctuated;
use syn::Token;
use std::ops::Deref;

pub fn generate_enum(name: &Ident, impls: &Vec<Path>) -> TokenStream {
    let variants = impls
        .iter()
        .map(|path| {
            let short = path.segments.last().unwrap();
            quote!{ #short(super::#path), }
        })
        .collect::<TokenStream>();
    quote! {
        pub enum #name {
            #variants
        }
    }
}

pub fn generate_impl(trait_def: &ItemTrait, impls: &Vec<Path>) -> TokenStream {
    let name = &trait_def.ident;
    let methods = trait_def.items
        .iter()
        .map(|i| match i {
                TraitItem::Method(m) => Some(m),
                _ => None
            }
        )
        .filter(Option::is_some)
        .map(|m| generate_method(&name, &m.unwrap().sig, &impls))
        .collect::<TokenStream>();
    quote! {
        impl super::#name for #name {
            #methods
        }
    }
}

fn generate_method(enum_name: &Ident, sig: &MethodSig, impls: &Vec<Path>) -> TokenStream {
    let method_name = &sig.ident;
    let mut args: Punctuated<FnArg, Token![,]> = Punctuated::new();
    sig.decl.inputs
        .iter()
        .filter(|arg| !arg.is_self())
        .cloned()
        .for_each(|arg| args.push(arg));
    if sig.decl.inputs.len() == args.len() {
        return quote! {
            #sig {
                panic!("illegal associated function call on generated enum {}", stringify!(#enum_name));
            }
        }
    }
    let branches = impls
        .iter()
        .map(|i| {
            let short = i.segments.last().unwrap();
            quote! {
                #enum_name::#short(inner) => inner.#method_name(#args),
            }
        })
        .collect::<TokenStream>();
    quote! {
        #sig {
            match self {
                #branches
            }
        }
    }
}

pub fn generate_froms(to: &Ident, froms: &Vec<Path>) -> TokenStream {
    froms.iter()
        .map(|f| generate_from(to, f))
        .collect()
}

fn generate_from(to: &Ident, from: &Path) -> TokenStream {
    let short = from.segments.last().unwrap();
    quote! {
        impl From<super::#from> for #to {
            fn from(from: super::#from) -> #to {
                #to::#short(from)
            }
        }
    }
}

pub trait ToSnakeCase {
    fn to_snake_case(&self) -> String;
}

impl <T: Deref<Target = str>> ToSnakeCase for T {
    fn to_snake_case(&self) -> String {
        let mut res = String::with_capacity(self.len());
        let mut first = true;
        self.chars()
            .for_each(|c| {
                if !first && c.is_uppercase() {
                    res.push('_');
                }
                res.push(c.to_lowercase().take(1).last().unwrap());
                first = false;
            });
        res
    }
}