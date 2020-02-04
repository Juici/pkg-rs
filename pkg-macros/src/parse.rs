use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Result};

pub struct AuthorsInput {
    pub join: Option<LitStr>,
}

impl Parse for AuthorsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut join = None;

        if input.peek(LitStr) {
            join = Some(input.parse()?);
        }

        Ok(AuthorsInput { join })
    }
}
