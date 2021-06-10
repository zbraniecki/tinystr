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
    if val.contains('\\') {
        // Note: If this proc macro has been called then rustc parsed the string as valid
        // including escapes. Jut try to call this macro with a invalid string
        let mut val: &str = &val[1..val.len() - 1];
        let mut buf = String::with_capacity(val.len());
        while !val.is_empty() {
            // 01234
            // \xXX
            // \n
            match val.find('\\') {
                Some(mut index) => {
                    buf.push_str(&val[0..index]);
                    match val.as_bytes()[index + 1] {
                        b'n' => buf.push('\n'),
                        b'r' => buf.push('\r'),
                        b't' => buf.push('\t'),
                        b'\\' => buf.push('\\'),
                        b'0' => buf.push('\0'),
                        b'\'' => buf.push('\''),
                        b'"' => buf.push('"'),
                        b'u' => panic!("Unicode Escapes not supported with this macro"),
                        b'x' => {
                            buf.push(char::from(
                                u8::from_str_radix(&val[index + 2..index + 4], 16).unwrap(),
                            ));
                            // Hex escapes are longer so bump the pointer
                            index += 2;
                        }
                        _ => unreachable!(),
                    }
                    val = &val[index + 2..];
                }
                None => break,
            }
        }
        buf.push_str(val);
        buf
    } else {
        (&val[1..val.len() - 1]).to_string()
    }
}

#[proc_macro]
pub fn u32_from_bytes(input: TokenStream) -> TokenStream {
    let s = get_value_from_token_stream(input);
    let u = tinystr_raw::try_u32_from_bytes(s.as_bytes()).expect(&s);
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
