//! A small utility library for binary applications.

#![deny(warnings)]

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
pub fn name() -> &'static str {
    pkg_name!()
}

/// Returns the crate author.
///
/// # Assumption
///
/// The crate only has the one author, otherwise this will return the list of
/// authors separated by `;`.
pub fn author() -> &'static str {
    pkg_authors!()
}

/// Returns a slice of the crate authors.
pub fn authors() -> &'static [&'static str] {
    &*AUTHORS
}

/// Returns the crate version.
pub fn version() -> &'static str {
    pkg_version!()
}

/// Returns the crate description.
pub fn description() -> &'static str {
    pkg_description!()
}

/// Returns the crate homepage.
pub fn homepage() -> &'static str {
    pkg_homepage!()
}

/// Returns the name of the binary file.
pub fn bin_name() -> Option<&'static str> {
    match *BIN_NAME {
        Some(ref name) => Some(name),
        _ => None,
    }
}
