//! `build.rs` script utilities.

use std::process::Command;

/// A git commit.
pub struct GitCommit {
    inner: String,
    indices: [usize; 5],
}

/// Attempts to get the most recent git commit.
pub fn git_commit() -> Option<GitCommit> {
    let output = Command::new("git")
        .args(&[
            "--no-pager",
            "log",
            "-1",
            "--date=short",
            "--pretty=format:%H%n%cn%n%ce%n%cd%n%s%n%b",
        ])
        .output()
        .ok()?;

    let output = String::from_utf8(output.stdout).ok()?;

    let mut inner = String::with_capacity(output.len());
    let mut indices = [0; 5];
    let mut i = 0;

    output
        .splitn(5, '\n')
        .map(|s: &str| s.trim())
        .for_each(|s: &str| {
            inner.push_str(s);

            indices[i] = inner.len();
            i += 1;
        });

    inner.shrink_to_fit();

    Some(GitCommit { inner, indices })
}

impl GitCommit {
    /// Returns the commit hash.
    pub fn hash(&self) -> &str {
        &self.inner[..self.indices[0]]
    }

    /// Returns the commit author name.
    pub fn author_name(&self) -> &str {
        &self.inner[self.indices[0]..self.indices[1]]
    }

    /// Returns the commit author email.
    pub fn author_email(&self) -> &str {
        &self.inner[self.indices[1]..self.indices[2]]
    }

    /// Returns the commit date in the format `yyyy-mm-dd`.
    pub fn date(&self) -> &str {
        &self.inner[self.indices[2]..self.indices[3]]
    }

    /// Returns the commit subject.
    pub fn subject(&self) -> &str {
        &self.inner[self.indices[3]..self.indices[4]]
    }

    /// Returns the commit body.
    pub fn body(&self) -> &str {
        &self.inner[self.indices[4]..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    // Note: whilst this is not the same method to obtain the hash, the resulting
    // value is the same and individually this is faster.
    fn git_hash() -> Option<String> {
        let mut vec = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .ok()?
            .stdout;
        vec.pop();
        String::from_utf8(vec).ok()
    }

    fn git_author_name() -> Option<String> {
        let vec = Command::new("git")
            .args(&["--no-pager", "log", "-1", "--pretty=format:%cn"])
            .output()
            .ok()?
            .stdout;
        String::from_utf8(vec).ok()
    }

    fn git_author_email() -> Option<String> {
        let vec = Command::new("git")
            .args(&["--no-pager", "log", "-1", "--pretty=format:%ce"])
            .output()
            .ok()?
            .stdout;
        String::from_utf8(vec).ok()
    }

    fn git_date() -> Option<String> {
        let vec = Command::new("git")
            .args(&[
                "--no-pager",
                "log",
                "-1",
                "--date=short",
                "--pretty=format:%cd",
            ])
            .output()
            .ok()?
            .stdout;
        String::from_utf8(vec).ok()
    }

    fn git_subject() -> Option<String> {
        let vec = Command::new("git")
            .args(&["--no-pager", "log", "-1", "--pretty=format:%s"])
            .output()
            .ok()?
            .stdout;
        String::from_utf8(vec).ok()
    }

    fn git_body() -> Option<String> {
        let vec = Command::new("git")
            .args(&["--no-pager", "log", "-1", "--pretty=format:%b"])
            .output()
            .ok()?
            .stdout;
        String::from_utf8(vec).ok()
    }

    #[test]
    fn test_hash() {
        assert_eq!(git_hash(), git_commit().map(|c| c.hash().to_owned()));
    }

    #[test]
    fn test_author_name() {
        assert_eq!(
            git_author_name(),
            git_commit().map(|c| c.author_name().to_owned())
        );
    }

    #[test]
    fn test_author_email() {
        assert_eq!(
            git_author_email(),
            git_commit().map(|c| c.author_email().to_owned())
        );
    }

    #[test]
    fn test_date() {
        assert_eq!(git_date(), git_commit().map(|c| c.date().to_owned()));
    }

    #[test]
    fn test_subject() {
        assert_eq!(git_subject(), git_commit().map(|c| c.subject().to_owned()));
    }

    #[test]
    fn test_body() {
        assert_eq!(git_body(), git_commit().map(|c| c.body().to_owned()));
    }
}
