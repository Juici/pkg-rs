//! A small utility library for binary applications.
//!
//! # Examples
//!
//! ```rust
//! extern crate pkg;
//!
//! fn main() {
//!     println!("{} {}\n{}", pkg::name(), pkg::version(), pkg::description());
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

#[macro_use]
mod macros;

lazy_static! {
    static ref AUTHORS: Vec<&'static str> = pkg_authors!().split(';').collect();
    static ref BIN_NAME: Option<String> = {
        use std::env;
        use std::path::PathBuf;

        let arg0 = env::args_os().next()?;
        let path = PathBuf::from(arg0);
        let file_name = path.file_stem()?;
        Some(file_name.to_str()?.to_owned())
    };
}

/// Returns the crate name.
///
/// # Examples
///
/// ```rust
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate name is {}", pkg::name());
/// }
/// ```
pub fn name() -> &'static str {
    pkg_name!()
}

/// Returns the crate author.
///
/// # Assumption
///
/// The crate only has the one author, otherwise this will return the list of
/// authors separated by semicolons.
///
/// # Examples
///
/// ```rust
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate author is {}", pkg::author());
/// }
/// ```
pub fn author() -> &'static str {
    pkg_authors!()
}

/// Returns a slice reference of the crate authors.
///
/// # Examples
///
/// ```rust
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate authors are: {}", pkg::authors().join(", "));
/// }
/// ```
pub fn authors() -> &'static [&'static str] {
    &*AUTHORS
}

/// Returns the crate version.
///
/// # Examples
///
/// ```rust
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate version is {}", pkg::version());
/// }
/// ```
pub fn version() -> &'static str {
    pkg_version!()
}

/// Returns the crate description.
///
/// # Examples
///
/// ```rust
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate description is {}", pkg::description());
/// }
/// ```
pub fn description() -> &'static str {
    pkg_description!()
}

/// Returns the crate homepage.
///
/// # Examples
///
/// ```rust
/// extern crate pkg;
///
/// fn main() {
///     println!("The crate homepage is {}", pkg::homepage());
/// }
/// ```
pub fn homepage() -> &'static str {
    pkg_homepage!()
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
