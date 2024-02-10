use std::env;
use std::error::Error;
use std::path::PathBuf;

use gix::Repository;
use proc_macro2::TokenStream;
use quote::quote;
use thiserror::Error;

fn get_manifest_dir() -> Result<PathBuf, GitError> {
    env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).ok_or(GitError::MissingManifestDir)
}

fn get_repo() -> Result<Repository, GitError> {
    let manifest_dir = get_manifest_dir()?;
    let repo = gix::discover(manifest_dir).map_err(GitError::boxed)?;

    Ok(repo)
}

pub fn commit_hash() -> Result<TokenStream, GitError> {
    let repo = get_repo()?;
    let head_id = repo.head_id().map_err(GitError::boxed)?;

    let commit_hash = head_id.to_hex().to_string();

    Ok(quote!(#commit_hash))
}

#[derive(Debug, Error)]
pub enum GitError {
    #[error("missing CARGO_MANIFEST_DIR environment variable")]
    MissingManifestDir,
    #[error(transparent)]
    Boxed(#[from] Box<dyn Error>),
}

impl GitError {
    fn boxed<E: Error + 'static>(err: E) -> Self {
        Self::Boxed(Box::new(err))
    }
}
