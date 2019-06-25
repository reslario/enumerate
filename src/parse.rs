use syn::{Error, FnArg};
use syn::parse::{Parse, ParseStream};
use syn::Token;
use proc_macro2::{Ident, Span};

pub struct Arguments {
    pub enum_name: Option<Ident>,
    pub impls: Vec<Implementer>
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let enum_name = input.parse::<EnumName>().ok().map(|en| en.0);
        let impls = input
            .parse_terminated::<Implementer, Token![,]>(Implementer::parse)?
            .into_iter()
            .collect::<Vec<Implementer>>();
        Ok(Arguments {
            enum_name,
            impls
        })
    }
}

pub struct Alias(Ident);

impl Parse for Alias {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        input.parse::<Token![as]>()?;
        Ok(Alias(input.parse::<Ident>()?))
    }
}

pub struct EnumName(Ident);

impl Parse for EnumName {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        if !input.peek2(Token![:]) {
            return Err(syn::Error::new(Span::call_site(), ""));
        }
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        Ok(EnumName(ident))
    }
}

pub struct Implementer {
    pub ident: Ident,
    alias: Option<Ident>
}

impl Implementer {
    pub fn alias(&self) -> &Ident {
        if let Some(id) = &self.alias {
            id
        } else {
            &self.ident
        }
    }
}

impl Parse for Implementer {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let ident = input.parse::<Ident>()?;
        let alias = input.parse::<Alias>().ok().map(|a| a.0);
        Ok(Implementer {
            ident,
            alias
        })
    }
}

pub trait IsSelf {
    fn is_self(&self) -> bool;
}

impl IsSelf for FnArg {
    fn is_self(&self) -> bool {
        match self {
            FnArg::SelfRef(_) |
            FnArg::SelfValue(_) => true,
            _ => false
        }
    }
}