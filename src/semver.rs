use core::fmt::{self, Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: &'static str,
    pub build: &'static str,
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        if !self.pre.is_empty() {
            f.write_str("-")?;
            f.write_str(self.pre)?;
        }

        if !self.build.is_empty() {
            f.write_str("+")?;
            f.write_str(self.build)?;
        }

        Ok(())
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Version");

        debug.field("major", &self.major);
        debug.field("minor", &self.minor);
        debug.field("patch", &self.patch);

        if !self.pre.is_empty() {
            debug.field("pre", &self.pre);
        }

        if !self.build.is_empty() {
            debug.field("build", &self.build);
        }

        debug.finish()
    }
}

/// Expands to the semantic version of the calling package.
///
/// # Example
///
/// ```
/// use pkg::Version;
///
/// const VERSION: Version = pkg::semver!();
///
/// assert_eq!(VERSION.to_string(), pkg::version!());
/// ```
#[macro_export]
macro_rules! semver {
    () => {{
        const fn parse_version(version: &'static str) -> $crate::Version {
            let (major, rest) = $crate::__private::split_on(version, b'.');
            let (minor, rest) = $crate::__private::split_on(rest, b'.');
            let (patch, rest) = $crate::__private::split_on(rest, b'-');

            let (pre, build) = $crate::__private::split_on(rest, b'+');

            let major = parse_number(major);
            let minor = parse_number(minor);
            let patch = parse_number(patch);

            $crate::Version { major, minor, patch, pre, build }
        }

        #[track_caller]
        const fn parse_number(s: &'static str) -> u64 {
            let digits = s.as_bytes();

            // Cannot be an empty string.
            assert!(!digits.is_empty(), "version is empty");

            let mut result = 0;
            let mut i = 0;

            while i < digits.len() {
                result *= 10;

                let digit = digits[i].wrapping_sub(b'0');

                // Check the digit is a valid decimal digit (ie. in range 0..=9).
                assert!(digit <= 10, "invalid digit in version");

                result += digit as u64;

                i += 1;
            }

            result
        }

        const VERSION: $crate::Version = parse_version(::core::env!("CARGO_PKG_VERSION"));

        VERSION
    }};
}
