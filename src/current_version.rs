use thiserror::Error;

use crate::config::{self, FnmConfig};
use crate::system_version;
use crate::version::Version;

pub fn current_version(config: &FnmConfig) -> Result<Option<Version>, Error> {
    let multishell_path = config.multishell_path().ok_or(Error::EnvNotApplied)?;

    if multishell_path.read_link().ok() == Some(system_version::path()) {
        Ok(Some(Version::Bypassed))
    }

    if let Ok(resolved_path) = std::fs::canonicalize(multishell_path) {
        let installation_path = resolved_path.parent().expect("multishell path can't be in the root")
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("`fnm env` was not applied in this context.\nCan't find fnm's environment variables")]
    EnvNotApplied,
    #[error("Can't read the version as a valid semver")]
    VersionError {
        source: node_semver::SemverError,
        version: String,
    },
}
