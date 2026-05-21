#![doc = uranus::readme_text!()]

mod join;
mod repeat;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::{
    quote
};

use join::*;
use repeat::*;

/// Repeat a string literal some number of times.
/// 
/// # Example
/// ```rust
/// use estrogen::repeat;
/// 
/// const SPACES: &'static str = repeat!(4, " ");
/// 
/// assert_eq!(SPACES, "    ");
/// ```
#[proc_macro]
pub fn repeat(input: TokenStream) -> TokenStream {
    let repeat_input = parse_macro_input!(input as RepeatInput);
    quote!( #repeat_input ).into()
}

/// Join string literals together with a join string.
/// 
/// # Example
/// ```rust
/// use estrogen::join;
/// 
/// const JOINED: &'static str = join!(", ", ["foo", "bar", "baz"]);
/// 
/// assert_eq!(JOINED, "foo, bar, baz");
/// ```
#[proc_macro]
pub fn join(input: TokenStream) -> TokenStream {
    let join_input = parse_macro_input!(input as JoinInput);
    quote!( #join_input ).into()
}
