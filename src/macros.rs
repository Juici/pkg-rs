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
/// To get a slice of the authors look at the [authors](fn.authors.html)
/// function.
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
