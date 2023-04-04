// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::cmp::Ordering;
use std::fmt::Formatter;
use std::fs;
use std::ops::Neg;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QtmConfig {
    pub version: QtmVersion,
    pub theme: QtmTheme,
    pub default_directory: Option<PathBuf>,
    pub initial_window_size: (usize, usize),
    pub image_area: usize,
}

impl Default for QtmConfig {
    fn default() -> Self {
        QtmConfig {
            version: QtmVersion::get_current_version(),
            theme: QtmTheme::Light,
            default_directory: None,
            initial_window_size: (800, 700),
            image_area: 120_000,
        }
    }
}

impl QtmConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let file_content = match fs::read_to_string(path.as_ref()) {
            Ok(string) => string,
            Err(err) => {
                warn!(?err, "Unable to load the configuration; IGNORE this warning if initialising");
                info!("Attempt to save the default configuration");
                QtmConfig::default().save(path);
                return QtmConfig::default();
            }
        };

        match toml::from_str::<Self>(&file_content) {
            Ok(config) => {
                info!("Loaded the serialised configuration successfully");
                config
            },
            Err(err) => {
                warn!(?err, "Unable to deserialise the configuration; loading default configuration");
                QtmConfig::default()
            }
        }
    }

    //  TODO: Add auto. log clean-up function
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let Ok(config) = toml::to_string(self) else {
            warn!(?self, "Unable to serialise or hence save the configuration; saving aborted");
            return;
        };

        match fs::write(path, config) {
            Ok(()) => info!("Saved the serialised configuration successfully"),
            Err(err) => warn!(
                ?err,
                "Unable to save the serialised configuration; saving aborted"
            ),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum QtmTheme {
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
pub struct QtmVersion(u8, u8, u8);

impl QtmVersion {
    pub fn get_current_version() -> Self {
        Self(
            env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap(),
        )
    }
}

impl std::fmt::Display for QtmVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
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
