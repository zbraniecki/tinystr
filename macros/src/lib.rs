extern crate proc_macro;

use proc_macro::TokenStream;

fn get_value_from_token_stream(input: TokenStream) -> String {
    let val = format!("{}", input);
    if !val.starts_with('"') || !val.ends_with('"') {
        panic!("Argument must be a string literal.");
    }
    let len = val.len();
    (&val[1..len - 1]).to_string()
}

#[proc_macro]
pub fn tinystr4(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);
    let bytes: u32 = tinystr::TinyStr4::from_bytes(val.as_bytes())
        .expect("Failed to construct TinyStr from input")
        .into();

    let formula = format!("unsafe {{ tinystr::TinyStr4::new_unchecked({}) }}", bytes);
    formula.parse().unwrap()
}

#[proc_macro]
pub fn tinystr8(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);
    let bytes: u64 = tinystr::TinyStr8::from_bytes(val.as_bytes())
        .expect("Failed to construct TinyStr from input")
        .into();
    let formula = format!("unsafe {{ tinystr::TinyStr8::new_unchecked({}) }}", bytes);
    formula.parse().unwrap()
}

#[proc_macro]
pub fn tinystr16(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);
    let bytes: u128 = tinystr::TinyStr16::from_bytes(val.as_bytes())
        .expect("Failed to construct TinyStr from input")
        .into();
    let formula = format!("unsafe {{ tinystr::TinyStr16::new_unchecked({}) }}", bytes);
    formula.parse().unwrap()
}
