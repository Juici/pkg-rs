//! A small utility library for binary applications.

mod macros;

#[doc(hidden)]
pub mod __private {
    pub use pkg_macros as macros;
    pub use std;
}

/// Returns the name of the binary as determined a runtime.
#[cfg(feature = "bin_name")]
pub fn bin_name() -> Option<&'static str> {
    use std::env;
    use std::path::Path;

    use once_cell::sync::Lazy;

    static BIN_NAME: Lazy<Option<String>> = Lazy::new(|| {
        let argv0 = env::args_os().next()?;

        let p = Path::new(&argv0);
        let s = p.file_name()?.to_str()?;

        Some(s.to_owned())
    });
    BIN_NAME.as_deref()
}
