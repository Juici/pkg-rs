use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::num::ParseIntError;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fmt};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use thiserror::Error;

pub fn build_time() -> syn::Result<TokenStream> {
    static BUILD_TIME: OnceLock<Result<i64, BuildTimeError>> = OnceLock::new();

    let timestamp = BUILD_TIME
        .get_or_init(|| match get_reproducible_time() {
            Ok(Some(timestamp)) => Ok(timestamp),
            Ok(None) => get_system_time(),
            Err(err) => Err(BuildTimeError::SourceDate(err)),
        })
        .as_ref()
        .copied()
        .map_err(|err| syn::Error::new(Span::call_site(), err))?;

    Ok(quote! { #timestamp })
}

// https://reproducible-builds.org/docs/source-date-epoch/
fn get_reproducible_time() -> Result<Option<i64>, SourceDateError> {
    match env::var_os("SOURCE_DATE_EPOCH") {
        Some(timestamp) => Ok(Some(timestamp.into_string()?.parse::<i64>()?)),
        None => Ok(None),
    }
}

fn get_system_time() -> Result<i64, BuildTimeError> {
    let now = SystemTime::now();

    let timestamp = match now.duration_since(UNIX_EPOCH) {
        Ok(since) => i128::from(since.as_secs()),
        Err(err) => -i128::from(err.duration().as_secs()),
    };

    i64::try_from(timestamp).map_err(|_| BuildTimeError::Overflow(timestamp))
}

#[derive(Debug, Error)]
enum BuildTimeError {
    #[error("system clock overflows unix timestamp: {0}")]
    Overflow(i128),
    #[error(transparent)]
    SourceDate(#[from] SourceDateError),
}

#[derive(Debug, Error)]
enum SourceDateError {
    #[error("invalid unicode in $SOURCE_DATE_EPOCH")]
    NotUnicode(OsString),
    #[error("failed to parse $SOURCE_DATE_EPOCH timestamp")]
    Parse(#[from] ParseIntError),
}

impl From<OsString> for SourceDateError {
    fn from(s: OsString) -> Self {
        Self::NotUnicode(s)
    }
}

struct InvalidUtf8<'a>(&'a OsStr);

impl Display for InvalidUtf8<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hack to make use of unstable Utf8Chunks to output lossy string.
        std::path::Path::new(&self.0).display().fmt(f)
    }
}
