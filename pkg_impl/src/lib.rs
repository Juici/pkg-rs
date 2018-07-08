//! An internal crate with procedural macros for compile-time processing.

#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn authors(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("takes no arguments");
    }

    let authors_string = std::env::var("CARGO_PKG_AUTHORS")
        .expect("missing cargo environment variable: CARGO_PKG_AUTHORS");

    generate_authors_slice_code(&authors_string)
        .parse()
        .unwrap()
}

fn generate_authors_slice_code(string: &str) -> String {
    let mut n = 3; // `&[]`

    let authors: Vec<&str> = string
        .split(';')
        .map(|s| {
            let s = s.trim();
            n += s.len() + 2; // Add string length + 2 for quotes.
            s
        })
        .collect();

    let len = authors.len();
    n += len - 1; // Comma separators.

    let mut result = String::with_capacity(n);

    result.push_str("&[");

    let mut i = 0;
    while i < len {
        if i > 0 {
            result.push(',');
        }

        result.push('"');
        result.push_str(authors[i]);
        result.push('"');

        i += 1;
    }

    result.push(']');

    result
}
