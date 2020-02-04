//! A small utility library for binary applications.

#![no_std]
#![deny(missing_docs, warnings)]

extern crate core;

mod version;

#[cfg(feature = "build")]
pub mod build;

#[doc(hidden)]
pub use core::env as __env;

#[doc(hidden)]
#[proc_macro_hack::proc_macro_hack]
pub use pkg_macros::authors as __authors;

/// Expands to the crate name.
///
/// # Examples
///
/// ```
/// const NAME: &str = pkg::name!();
/// ```
#[macro_export]
macro_rules! name {
    () => {
        $crate::__env!("CARGO_PKG_NAME")
    };
}

/// Expands to the full crate version.
///
/// # Examples
///
/// ```
/// const VERSION: &str = pkg::version!();
/// ```
#[macro_export]
macro_rules! version {
    () => {
        $crate::__env!("CARGO_PKG_VERSION")
    };
}

/// Expands to the crate authors.
///
/// # Examples
///
/// Basic usage:
/// ```
/// const AUTHORS: &[&str] = pkg::authors!();
/// ```
///
/// Joined string:
/// ```
/// const AUTHORS: &str = pkg::authors!(", ");
/// ```
#[macro_export]
macro_rules! authors {
    ($join:literal) => {{
        let authors: &'static str = $crate::__authors!($join);
        authors
    }};
    () => {{
        let authors: &'static [&'static str] = $crate::__authors!();
        authors
    }};
}

/// Expands to the crate description.
///
/// # Examples
///
/// ```
/// const DESCRIPTION: &str = pkg::description!();
/// ```
#[macro_export]
macro_rules! description {
    () => {
        $crate::__env!("CARGO_PKG_DESCRIPTION")
    };
}

/// Expands to the crate homepage URL.
///
/// # Examples
///
/// ```
/// const HOMEPAGE: &str = pkg::homepage!();
/// ```
#[macro_export]
macro_rules! homepage {
    () => {
        $crate::__env!("CARGO_PKG_HOMEPAGE")
    };
}

/// Expands to the crate homepage URL.
///
/// # Examples
///
/// ```
/// const REPOSITORY: &str = pkg::repository!();
/// ```
#[macro_export]
macro_rules! repository {
    () => {
        $crate::__env!("CARGO_PKG_REPOSITORY")
    };
}
