use syn::parse::{Parse, ParseStream};
use syn::LitStr;

pub struct AuthorsInput {
    pub sep: Option<LitStr>,
}

impl Parse for AuthorsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let sep = if input.peek(LitStr) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(AuthorsInput { sep })
    }
}
