use syn::{ItemTrait, TraitItem, MethodSig, FnArg};
use proc_macro2::{TokenStream, Ident};
use quote::quote;
use crate::parse::{IsSelf, Implementer};
use syn::punctuated::Punctuated;
use syn::Token;
use std::ops::Deref;

pub fn generate_enum(enum_name: &Ident, trait_name: &Ident, impls: &Vec<Implementer>) -> TokenStream {
    let variants = impls
        .iter()
        .map(|im| {
            let alias = im.alias();
            let ty = &im.ident;
            let path = get_path(ty, enum_name, trait_name);
            quote! { #alias(#path), }
        })
        .collect::<TokenStream>();
    quote! {
        enum #enum_name {
            #variants
        }
    }
}

pub fn generate_impl(trait_def: &ItemTrait, enum_name: &Ident, impls: &Vec<Implementer>) -> TokenStream {
    let trait_name = &trait_def.ident;
    let path = get_path(trait_name, enum_name, trait_name);
    let methods = trait_def.items
        .iter()
        .filter_map(|i| match i {
                TraitItem::Method(m) => Some(m),
                _ => None
            }
        )
        .map(|m| generate_method(&enum_name, &m.sig, &impls))
        .collect::<TokenStream>();
    quote! {
        impl #path for #enum_name {
            #methods
        }
    }
}

fn generate_method(enum_name: &Ident, sig: &MethodSig, impls: &Vec<Implementer>) -> TokenStream {
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
        .map(|im| {
            let alias = im.alias();
            quote! {
                #enum_name::#alias(inner) => inner.#method_name(#args),
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

pub fn generate_froms(to: &Ident, froms: &Vec<Implementer>, trait_name: &Ident) -> TokenStream {
    froms.iter()
        .map(|f| generate_from(to, f, trait_name))
        .collect()
}

fn generate_from(to: &Ident, from: &Implementer, trait_name: &Ident) -> TokenStream {
    let ty = &from.ident;
    let alias = from.alias();
    let path = get_path(ty, to, trait_name);
    quote! {
        impl From<#path> for #to {
            fn from(from: #path) -> #to {
                #to::#alias(from)
            }
        }
    }
}

fn get_path(ty: &Ident, enum_name: &Ident, trait_name: &Ident) -> TokenStream {
    if enum_name == trait_name {
        quote! { super::#ty }
    } else {
        quote! { #ty }
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