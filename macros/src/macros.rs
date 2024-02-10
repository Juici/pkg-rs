#![allow(unused_macros)]

macro_rules! cfg_git {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "git")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "git")))]
            $item
        )*
    };
}

macro_rules! parse_nothing {
    ($input:expr) => {{
        let input = ::proc_macro2::TokenStream::from($input);

        if let Some(token) = input.into_iter().next() {
            let err = ::syn::Error::new(token.span(), "unexpected token");

            return err.into_compile_error().into();
        }
    }};
}
