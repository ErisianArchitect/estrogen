use syn::{
    parse::Parse,
    Expr,
    LitInt,
    Token,
};
use quote::{
    quote,
    ToTokens,
};

pub struct RepeatInput {
    pub repeat_expr: Expr,
    pub repeat_count: LitInt,
}

impl Parse for RepeatInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let repeat_count = input.parse::<LitInt>()?;
        _ = input.parse::<Token![,]>()?;
        let lit = input.parse::<Expr>()?;
        Ok(Self {
            repeat_expr: lit,
            repeat_count,
        })
    }
}

impl ToTokens for RepeatInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let repeat_count = match self.repeat_count.base10_parse::<u32>() {
            Ok(count) => count as usize,
            Err(err) => panic!("{err}"),
        };
        if repeat_count == 0 {
            tokens.extend(quote!( "" ));
            return;
        }
        let mut concat_input = proc_macro2::TokenStream::new();
        let repeat_expr = &self.repeat_expr;
        for _ in 0..repeat_count {
            concat_input.extend(quote!( #repeat_expr, ));
        }
        tokens.extend(quote!( concat!( #concat_input ) ));
    }
}
