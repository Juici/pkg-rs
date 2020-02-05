extern crate proc_macro;

mod parse;

use std::env;

use proc_macro2::Span;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::Nothing;
use syn::{parse_macro_input, Error};

use crate::parse::AuthorsInput;

#[proc_macro_hack]
pub fn authors(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as AuthorsInput);

    let authors: Vec<String> = match env::var_os("CARGO_PKG_AUTHORS").map_or_else(
        || Err(format!("authors environment variable not found")),
        |s| {
            s.to_str()
                .map(|s| s.trim().split(';').map(ToString::to_string).collect())
                .ok_or_else(|| {
                    format!(
                        "authors environment variable is not valid unicode: {}",
                        s.to_string_lossy()
                    )
                })
        },
    ) {
        Ok(authors) => authors,
        Err(err) => return Error::new(Span::call_site(), err).to_compile_error().into(),
    };

    let output = match input.join {
        Some(join) => {
            let s: String = authors.join(&join.value());
            quote!(#s)
        }
        None => quote!(&[#(#authors),*]),
    };

    output.into()
}

#[cfg(feature = "git")]
#[proc_macro]
pub fn git_commit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ = parse_macro_input!(input as Nothing);

    let _commit = match git_commit_info() {
        Ok(repo) => repo,
        Err(err) => return Error::new(Span::call_site(), err).to_compile_error().into(),
    };

    proc_macro::TokenStream::new()
}

#[cfg(feature = "git")]
fn git_commit_info() -> Result<(), git2::Error> {
    use git2::Repository;

    let repo = Repository::open(".")?;

    let head = repo.head()?;
    let _commit = head.peel_to_commit()?;

    Ok(())
}
