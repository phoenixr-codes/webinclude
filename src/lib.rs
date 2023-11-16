#![doc = include_str!("../README.md")]

use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
/// Makes a web request and inserts its content where ever this macro is used.
///
/// See [the crate level documentation](self) for more.
pub fn webinclude(url: TokenStream) -> TokenStream {
    for tree in url {
        let url = match tree {
            TokenTree::Literal(literal) => literal.to_string(),
            _ => panic!("expect string literal"),
        };
        let mut chars = url.chars();
        chars.next();
        chars.next_back();
        let url = chars.as_str();
        let res = minreq::get(url).send().expect("reachable URL");
        let body = res.as_str().expect("valid UTF-8");
        return body.parse().expect("valid Rust");
    }
    panic!("expect string literal");
}
