use syn::{Path, Error, FnArg};
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub struct Arguments(pub Vec<Path>);

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let impls = input
            .parse_terminated::<Path, Token![,]>(|path| path.parse())?
            .into_iter()
            .collect::<Vec<Path>>();
        Ok(Arguments(impls))
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