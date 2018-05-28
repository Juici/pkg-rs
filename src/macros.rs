/// Macro for getting the crate `name` from the cargo manifest.
#[macro_export]
macro_rules! pkg_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Macro for getting the crate `version` from the cargo manifest.
#[macro_export]
macro_rules! pkg_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Macro for getting the crate `authors` from the cargo manifest.
#[macro_export]
macro_rules! pkg_authors {
    () => {
        env!("CARGO_PKG_AUTHORS")
    };
}

/// Macro for getting the crate `description` from the cargo manifest.
#[macro_export]
macro_rules! pkg_description {
    () => {
        env!("CARGO_PKG_DESCRIPTION")
    };
}

/// Macro for getting the crate `homepage` from the cargo manifest.
#[macro_export]
macro_rules! pkg_homepage {
    () => {
        env!("CARGO_PKG_HOMEPAGE")
    };
}
