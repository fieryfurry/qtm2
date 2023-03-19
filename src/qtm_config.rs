// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::cmp::Ordering;
use std::fs;
use std::ops::Neg;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use tracing::warn;

use crate::unwrap_trace::UnwrapTrace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QtmConfig {
    pub(crate) version: QtmVersion,
    pub(crate) theme: QtmTheme,
    pub(crate) default_directory: Option<PathBuf>,
}

impl Default for QtmConfig {
    fn default() -> Self {
        QtmConfig {
            version: QtmVersion::get_current_version(),
            theme: QtmTheme::Light,
            default_directory: None,
        }
    }
}

impl QtmConfig {
    pub(crate) fn load<P: AsRef<Path>>(path: P) -> Self {
        let file_content = fs::read_to_string(path).unwrap_or_warn(
            "Unable to load the configuration file; \
                IGNORE this warning if initialising",
            |_| String::default(),
        );

        toml::from_str(&file_content).unwrap_or_warn(
            "Unable to deserialise the configuration file; \
            IGNORE this warning if initialising; loading default configuration",
            |_| QtmConfig::default(),
        )
    }

    //  TODO: Add auto. log clean-up function
    pub(crate) fn save<P: AsRef<Path>>(&self, path: P) {
        let Ok(config) = toml::to_string(self) else {
            warn!(?self, "Unable to serialise or hence save the configuration; saving aborted");
            return;
        };

        fs::write(path, config).unwrap_or_warn(
            "Unable to save the serialised configuration; saving aborted",
            |_| (),
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub(crate) enum QtmTheme {
    Light,
    Dark,
}

impl Neg for QtmTheme {
    type Output = QtmTheme;

    fn neg(self) -> Self::Output {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct QtmVersion(u8, u8, u8);

impl QtmVersion {
    fn get_current_version() -> Self {
        Self(
            env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap(),
            env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap(),
        )
    }
}

impl PartialOrd for QtmVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QtmVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            if self.1 == other.1 {
                self.2.cmp(&other.2)
            } else {
                self.1.cmp(&other.1)
            }
        } else {
            self.0.cmp(&other.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_eq() {
        assert_eq!(QtmVersion(0, 3, 0), QtmVersion(0, 3, 0));
        assert_eq!(QtmVersion(1, 3, 5), QtmVersion(1, 3, 5));
        assert_ne!(QtmVersion(0, 3, 5), QtmVersion(1, 3, 5));
        assert_ne!(QtmVersion(0, 3, 6), QtmVersion(1, 3, 5));
    }

    #[test]
    fn test_version_cmp() {
        assert!(QtmVersion(0, 3, 0) < QtmVersion(1, 3, 0));
        assert!(QtmVersion(0, 3, 2) < QtmVersion(0, 3, 3));
        assert!(QtmVersion(0, 3, 2) > QtmVersion(0, 2, 2));
        assert!(QtmVersion(1, 5, 12) > QtmVersion(0, 12, 7));
    }
}
