#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![allow(clippy::module_name_repetitions)]

extern crate proc_macro;

use crate::traits::MacroResult;

#[macro_use]
mod macros;
mod build_time;
mod traits;

#[proc_macro]
pub fn build_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_nothing!(input);

    build_time::build_time().macro_result()
}

cfg_git! {
    mod git;

    #[proc_macro]
    pub fn git_commit_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        parse_nothing!(input);

        git::commit_hash().macro_result()
    }
}
