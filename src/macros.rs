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
        $crate::__private::std::env!("CARGO_PKG_NAME")
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
        $crate::__private::std::env!("CARGO_PKG_VERSION")
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
/// Joined by a separator string:
/// ```
/// const AUTHORS: &str = pkg::authors!("\n");
/// ```
#[macro_export]
macro_rules! authors {
    ($($sep:literal)?) => {{
        $(const _: &str = $sep;)?
        $crate::__private::macros::authors!($($sep)?)
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
        $crate::__private::std::env!("CARGO_PKG_DESCRIPTION")
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
        $crate::__private::std::env!("CARGO_PKG_HOMEPAGE")
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
        $crate::__private::std::env!("CARGO_PKG_REPOSITORY")
    };
}

/// Expands to the crate major version.
///
/// # Examples
///
/// ```
/// const VERSION_MAJOR: &str = pkg::version_major!();
/// ```
#[macro_export]
macro_rules! version_major {
    () => {
        $crate::__private::std::env!("CARGO_PKG_VERSION_MAJOR")
    };
}

/// Expands to the crate minor version.
///
/// # Examples
///
/// ```
/// const VERSION_MINOR: &str = pkg::version_minor!();
/// ```
#[macro_export]
macro_rules! version_minor {
    () => {
        $crate::__private::std::env!("CARGO_PKG_VERSION_MINOR")
    };
}

/// Expands to the crate patch version.
///
/// # Examples
///
/// ```
/// const VERSION_PATCH: &str = pkg::version_patch!();
/// ```
#[macro_export]
macro_rules! version_patch {
    () => {
        $crate::__private::std::env!("CARGO_PKG_VERSION_PATCH")
    };
}

/// Expands to the crate pre-release version.
///
/// # Examples
///
/// ```
/// const VERSION_PRE: &str = pkg::version_pre!();
/// ```
#[macro_export]
macro_rules! version_pre {
    () => {
        $crate::__private::std::env!("CARGO_PKG_VERSION_PRE")
    };
}
