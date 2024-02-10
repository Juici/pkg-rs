//! Macros for obtaining the state of the git repository when building.

/// Expands to the hash of the current commit.
///
/// # Example
///
/// ```
/// use std::process::Command;
///
/// const COMMIT_HASH: &str = pkg::git::commit_hash!();
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # #[cfg(not(miri))]
/// # {
/// let output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
/// let commit_hash = String::from_utf8(output.stdout)?;
///
/// assert_eq!(commit_hash.trim(), COMMIT_HASH);
/// # }
/// # Ok(())
/// # }
/// ```
pub use pkg_macros::git_commit_hash as commit_hash;
