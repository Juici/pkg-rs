//! Extra version related macros for finer detail.

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
        $crate::__env!("CARGO_PKG_VERSION_MAJOR")
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
        $crate::__env!("CARGO_PKG_VERSION_MINOR")
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
        $crate::__env!("CARGO_PKG_VERSION_PATCH")
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
        $crate::__env!("CARGO_PKG_VERSION_PRE")
    };
}
