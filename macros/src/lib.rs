//! `tinystr-macros` exports proc macros to convert byte strings to raw TinyStr data.
//!
//! Not intended for public consumption; use `tinystr` instead.

extern crate proc_macro;

// use proc_macro::bridge::client::Literal as BridgeLiteral;
use proc_macro::{Literal, TokenStream, TokenTree};

fn get_value_from_token_stream(input: TokenStream) -> String {
    let val = input.to_string();
    if !val.starts_with('"') && !val.ends_with('"') {
        panic!("Expected a string literal; found {:?}", input);
    }
    (&val[1..val.len() - 1]).to_string()
}

#[proc_macro]
pub fn u32_from_bytes(input: TokenStream) -> TokenStream {
    let s = get_value_from_token_stream(input);
    let u = tinystr_raw::try_u32_from_bytes(s.as_bytes())
        .expect(&s);
    TokenTree::from(Literal::u32_suffixed(u.into())).into()
}

#[proc_macro]
pub fn u64_from_bytes(input: TokenStream) -> TokenStream {
    let s = get_value_from_token_stream(input);
    let u = tinystr_raw::try_u64_from_bytes(s.as_bytes())
        .expect("Failed to construct TinyStr from input");
    TokenTree::from(Literal::u64_suffixed(u.into())).into()
}

#[proc_macro]
pub fn u128_from_bytes(input: TokenStream) -> TokenStream {
    let s = get_value_from_token_stream(input);
    let u = tinystr_raw::try_u128_from_bytes(s.as_bytes())
        .expect("Failed to construct TinyStr from input");
    TokenTree::from(Literal::u128_suffixed(u.into())).into()
}
