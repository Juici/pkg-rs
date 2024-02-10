//! This library provides utilities for getting build information.

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::module_name_repetitions)]

mod authors;
mod semver;
mod str;

#[cfg(feature = "git")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "git")))]
pub mod git;

pub use crate::semver::Version;

#[doc(hidden)]
pub mod __private {
    //! Private internals used in some macros.

    #[doc(hidden)]
    pub use crate::authors::{authors, count_authors, join_authors, join_authors_buf_len};
}

/// Timestamp of the build as seconds from the Unix epoch.
#[cfg(feature = "proc-macros")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proc-macros")))]
pub const BUILD_TIME: i64 = pkg_macros::build_time!();

/// Expands to the name of the calling package.
///
/// # Example
///
/// ```
/// const NAME: &str = pkg::name!();
/// ```
#[macro_export]
macro_rules! name {
    () => {{
        const NAME: &str = ::core::env!("CARGO_PKG_NAME");
        NAME
    }};
}

/// Expands to the version of the calling package.
///
/// # Example
///
/// ```
/// const VERSION: &str = pkg::version!();
/// ```
#[macro_export]
macro_rules! version {
    () => {{
        const VERSION: &str = ::core::env!("CARGO_PKG_VERSION");
        VERSION
    }};
}

/// Expands to the description of the calling package.
///
/// # Example
///
/// ```
/// const DESCRIPTION: Option<&str> = pkg::description!();
/// ```
#[macro_export]
macro_rules! description {
    () => {{
        const DESCRIPTION: Option<&str> = match ::core::env!("CARGO_PKG_DESCRIPTION") {
            desc if !desc.is_empty() => Some(desc),
            _ => None,
        };
        DESCRIPTION
    }};
}

/// Expands to the authors of the calling package.
///
/// This macro can optionally be called with a string that will be
/// used as a separator to join the authors into a single string.
///
/// # Example
///
/// ```
/// const AUTHORS: &[&str] = pkg::authors!();
/// const AUTHORS_JOINED: &str = pkg::authors!("\n");
///
/// assert_eq!(AUTHORS.join("\n"), AUTHORS_JOINED);
/// ```
#[macro_export]
macro_rules! authors {
    () => {{
        const AUTHORS_STR: &str = ::core::env!("CARGO_PKG_AUTHORS");

        const N_AUTHORS: usize = $crate::__private::count_authors(AUTHORS_STR);
        const AUTHORS: [&str; N_AUTHORS] = $crate::__private::authors(AUTHORS_STR);

        &AUTHORS
    }};
    ($sep:expr) => {{
        const SEP: &str = $sep;
        const AUTHORS: &[&str] = $crate::authors!();

        const BUF_LEN: usize = $crate::__private::join_authors_buf_len(AUTHORS, SEP);
        const BUF: [u8; BUF_LEN] = $crate::__private::join_authors(AUTHORS, SEP);

        const AUTHORS_JOINED: &str = match ::core::str::from_utf8(&BUF) {
            Ok(s) => s,
            Err(_) => ::core::panic!("invalid utf-8"),
        };

        AUTHORS_JOINED
    }};
}

/// Returns the name of the binary executable as determined at runtime.
///
/// # Notes
///
/// Any non-Unicode sequences are replaced with
/// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD] (�).
///
/// [U+FFFD]: char::REPLACEMENT_CHARACTER
///
/// # Example
///
/// ```
/// match pkg::bin_name() {
///     Some(name) => println!("name of this binary is: {name}"),
///     None => println!("failed to get name of binary"),
/// }
/// ```
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
pub fn bin_name() -> Option<&'static str> {
    use std::env;
    use std::path::PathBuf;
    use std::sync::OnceLock;

    static BIN_NAME: OnceLock<Option<Box<str>>> = OnceLock::new();

    let bin_name = BIN_NAME.get_or_init(|| {
        let current_exe = match env::args_os().next() {
            Some(exe) => PathBuf::from(exe),
            None => env::current_exe().ok()?,
        };

        let file = current_exe.file_stem()?;
        let name = file.to_string_lossy();

        Some(name.into_owned().into_boxed_str())
    });

    bin_name.as_deref()
}
