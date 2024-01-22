#![cfg_attr(not(feature = "std"), no_std)]

mod internal;

#[cfg(feature = "semver")]
mod semver;

#[cfg(feature = "semver")]
pub use crate::semver::Version;

// Private internals used in some compile-time constant macros.
#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use crate::internal::slice_str;
    #[doc(hidden)]
    pub use crate::internal::split_on;
}

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
        const DESCRIPTION: &str = ::core::env!("CARGO_PKG_DESCRIPTION");

        if DESCRIPTION.is_empty() {
            None
        } else {
            Some(DESCRIPTION)
        }
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
        // Cargo separates the names of authors with the colon ':' character.
        //
        // eg. "author1:author2:author3"
        const AUTHORS_SEP: u8 = b':';
        const AUTHORS_STR: &str = ::core::env!("CARGO_PKG_AUTHORS");

        // Count the number of authors.
        const AUTHORS_LEN: usize = {
            let mut n = 0;

            let mut remaining = AUTHORS_STR;

            while !remaining.is_empty() {
                let (author, rest) = $crate::__private::split_on(remaining, AUTHORS_SEP);

                if !author.is_empty() {
                    n += 1;
                }

                remaining = rest;
            }

            n
        };

        // Slice `AUTHORS_BYTES` up into the names of each author.
        const AUTHORS: [&str; AUTHORS_LEN] = {
            let mut authors = [""; AUTHORS_LEN];

            let mut i = 0;
            let mut remaining = AUTHORS_STR;

            while !remaining.is_empty() {
                let (author, rest) = $crate::__private::split_on(remaining, AUTHORS_SEP);

                if !author.is_empty() {
                    authors[i] = author;
                    i += 1;
                }

                remaining = rest;
            }

            authors
        };

        &AUTHORS
    }};
    ($sep:expr) => {{
        // Assert that `$sep` is a compile-time constant string.
        const SEP: &str = $sep;

        const AUTHORS: &[&str] = $crate::authors!();

        // Calculate the buffer length required to hold the string
        // created by joining `AUTHORS` with the `SEP` string.
        const BUF_LEN: usize = {
            let mut len = 0;
            let mut i = 0;

            while i < AUTHORS.len() {
                if i > 0 {
                    len += SEP.len();
                }
                len += AUTHORS[i].len();
                i += 1;
            }

            len
        };

        // Manually create a buffer the equivalent of `AUTHORS.join(SEP).as_bytes()`.
        const BUF: [u8; BUF_LEN] = {
            let mut buf = [0; BUF_LEN];

            let mut offset = 0;
            let mut i = 0;

            while i < AUTHORS.len() {
                let author = AUTHORS[i].as_bytes();

                // Add a separator if this is not the first author.
                if i > 0 {
                    let mut j = 0;

                    // Copy the bytes of `SEP` into the buffer.
                    while j < SEP.len() {
                        buf[offset] = SEP.as_bytes()[j];

                        offset += 1;
                        j += 1;
                    }
                }

                {
                    let mut j = 0;

                    // Copy the bytes of `author` into the buffer.
                    while j < author.len() {
                        buf[offset] = author[j];

                        offset += 1;
                        j += 1;
                    }
                }

                i += 1;
            }

            buf
        };

        const AUTHORS_STR: &str = match ::core::str::from_utf8(&BUF) {
            Ok(s) => s,
            Err(_) => panic!("invalid utf-8"),
        };

        AUTHORS_STR
    }};
}

/// Expands to the name of the binary that is being compiled.
///
/// If the target being compiled is not a binary, then the result of the
/// expanded expression will be `None`.
///
/// # Note
///
/// If the binary is renamed after being compiled or symlinked to with a
/// different name then this will not reflect that change. For that case use
/// the [`bin_name`] function to get the name of the executable at runtime.
///
/// # Example
///
/// ```
/// const BIN_NAME: Option<&str> = pkg::bin_name!();
/// ```
#[macro_export]
macro_rules! bin_name {
    () => {{
        const BIN_NAME: Option<&str> = ::core::option_env!("CARGO_BIN_NAME");
        BIN_NAME
    }};
}

/// Returns the name of the binary executable as determined at runtime.
///
/// # Example
///
/// ```
/// let bin_name = pkg::bin_name().expect("cannot determine binary name");
///
/// println!("the name of the current binary is {bin_name}");
/// ```
#[cfg(feature = "rt_bin_name")]
pub fn bin_name() -> Option<&'static str> {
    use std::env;
    use std::path::PathBuf;

    use once_cell::sync::OnceCell;

    static BIN_NAME: OnceCell<Option<Box<str>>> = OnceCell::new();

    let bin_name = BIN_NAME.get_or_init(|| {
        let current_exe = match env::args_os().next() {
            Some(exe) => PathBuf::from(exe),
            None => env::current_exe().ok()?,
        };

        let file = current_exe.file_name()?;
        let name = file.to_string_lossy();

        Some(name.into_owned().into_boxed_str())
    });

    bin_name.as_deref()
}
