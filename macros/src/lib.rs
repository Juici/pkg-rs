extern crate proc_macro;

use crate::parse::AuthorsInput;

mod expand;
mod parse;

#[proc_macro]
pub fn authors(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as AuthorsInput);

    expand::authors(input.sep)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
