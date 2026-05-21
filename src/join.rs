use std::fmt::Write;

use proc_macro2::TokenStream;
use syn::{
    parse::Parse,
    Token,
    LitStr,
    Expr,
    bracketed,
};
use quote::{
    quote,
    ToTokens,
};

// join!(", ", ["one", "two", "three"])

pub struct JoinInput {
    pub join_str: Expr,
    pub joins: Vec<Expr>,
}

impl Parse for JoinInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let join_str = input.parse::<Expr>()?;
        _ = input.parse::<Token![,]>()?;
        let array;
        bracketed!(array in input);
        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Unexpected tokens past bracketed input."));
        }
        if array.is_empty() {
            return Ok(Self {
                join_str,
                joins: Vec::new(),
            });
        }
        let mut joins = Vec::with_capacity(32);
        joins.push(array.parse()?);
        while !array.is_empty() {
            _ = array.parse::<Token![,]>()?;
            if !array.is_empty() {
                joins.push(array.parse()?);
            }
        }
        Ok(Self {
            join_str,
            joins,
        })
    }
}

impl ToTokens for JoinInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.joins.len() == 0 {
            tokens.extend(quote!(""));
            return;
        }
        let join_str = &self.join_str;
        let mut concat_input = TokenStream::new();
        let mut joins_iter = self.joins.iter();
        let first = joins_iter.next().unwrap();
        concat_input.extend(quote!( #first, ));
        for join in joins_iter {
            concat_input.extend(quote!( #join_str, #join, ));
        }
        tokens.extend(quote!( concat!( #concat_input ) ));
    }
}
