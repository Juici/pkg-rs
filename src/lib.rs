//! A small utility library for binary applications.
//!
//! # Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate pkg;
//!
//! fn main() {
//!     println!("{} {}\n{}", pkg_name!(), pkg_version!(), pkg_description!());
//! }
//! ```

#![deny(missing_docs, warnings)]
#![cfg_attr(test, feature(test))]
#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
extern crate pkg_impl;

#[cfg(feature = "build")]
pub mod build;

#[doc(hidden)]
pub use pkg_impl::authors as _authors;

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
/// Returns a `&[&str]` slice of the authors.
///
/// # Examples
///
/// ```rust
/// #![feature(proc_macro_non_items, use_extern_macros)]
///
/// #[macro_use]
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate authors are {:?}", pkg_authors!());
/// }
/// ```
///
/// # Notes
///
/// This macro requires the `proc_macro_non_items` and `use_extern_macros`
/// crate attributes.
#[macro_export]
macro_rules! pkg_authors {
    () => {{
        let authors: &[&str] = $crate::_authors!();
        authors
    }};
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
