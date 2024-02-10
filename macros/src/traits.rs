use std::any::Any;
use std::fmt::Display;

use proc_macro2::{Span, TokenStream};

pub trait MacroResult {
    fn macro_result(self) -> proc_macro::TokenStream;
}

impl<E> MacroResult for Result<TokenStream, E>
where
    E: Display + 'static,
{
    fn macro_result(self) -> proc_macro::TokenStream {
        let result = match self {
            Ok(tokens) => tokens,
            Err(err) => match <dyn Any>::downcast_ref::<syn::Error>(&err) {
                Some(err) => err.to_compile_error(),
                None => syn::Error::new(Span::call_site(), err).to_compile_error(),
            },
        };
        result.into()
    }
}

impl MacroResult for TokenStream {
    fn macro_result(self) -> proc_macro::TokenStream {
        self.into()
    }
}

impl MacroResult for proc_macro::TokenStream {
    fn macro_result(self) -> proc_macro::TokenStream {
        self
    }
}
