//! A small utility library for binary applications.
//!
//! # Examples
//!
//! ```rust
//! extern crate pkg;
//!
//! fn main() {
//!     println!("{} {}\n{}", pkg_name!(), pkg_version!(), pkg_description!());
//! }
//! ```
//!
//! # Cargo features
//!
//! This crate provides one cargo feature:
//!
//! - `nightly`: This uses language features only available on the nightly
//! release channel for more optimal implementations.

#![deny(missing_docs, warnings)]

#[macro_use]
extern crate lazy_static;

/// Macro for getting the crate `name` from the cargo manifest.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate name is {}", pkg_name!());
/// }
/// ```
#[macro_export]
macro_rules! pkg_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Macro for getting the crate `version` from the cargo manifest.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate version is {}", pkg_version!());
/// }
/// ```
#[macro_export]
macro_rules! pkg_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Macro for getting the crate `authors` from the cargo manifest.
///
/// The resulting `&str` is the join of all the authors by semicolons. If there
/// is only one author the result will be that author.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate authors are {}", pkg_authors!());
/// }
/// ```
#[macro_export]
macro_rules! pkg_authors {
    () => {
        env!("CARGO_PKG_AUTHORS")
    };
}

/// Macro for getting the crate `description` from the cargo manifest.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate name is {}", pkg_name!());
/// }
/// ```
#[macro_export]
macro_rules! pkg_description {
    () => {
        env!("CARGO_PKG_DESCRIPTION")
    };
}

/// Macro for getting the crate `homepage` from the cargo manifest.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate name is {}", pkg_name!());
/// }
/// ```
#[macro_export]
macro_rules! pkg_homepage {
    () => {
        env!("CARGO_PKG_HOMEPAGE")
    };
}

lazy_static! {
    static ref BIN_NAME: Option<String> = {
        use std::env;
        use std::path::PathBuf;

        let arg0 = env::args_os().next()?;
        let path = PathBuf::from(arg0);
        let file_name = path.file_stem()?;
        Some(file_name.to_str()?.to_owned())
    };
}

/// Returns the name of the binary file.
///
/// # Examples
///
/// ```rust
/// extern crate pkg;
///
/// fn main() {
///     println!(
///         "The binary file name is {}",
///         pkg::bin_name().unwrap_or("unknown")
///     );
/// }
/// ```
pub fn bin_name() -> Option<&'static str> {
    match *BIN_NAME {
        Some(ref name) => Some(name),
        _ => None,
    }
}
