use std::env;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result};

pub fn authors(sep: Option<LitStr>) -> Result<TokenStream> {
    let authors = env::var("CARGO_PKG_AUTHORS").map_err(|err| {
        Error::new(
            Span::call_site(),
            format_args!("failed to get crate authors: {}", err),
        )
    })?;

    let iter = authors.trim().split(':').map(str::trim).peekable();

    Ok(match sep {
        Some(sep) => {
            let sep = sep.value();

            let mut joined = String::with_capacity(authors.len());
            for (i, author) in iter.enumerate() {
                if i > 0 {
                    joined.push_str(&sep);
                }
                joined.push_str(author);
            }

            quote!(#joined)
        }
        None => quote!(&[#(#iter),*]),
    })
}
