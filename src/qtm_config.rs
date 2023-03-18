// SPDX-License-Identifier: BSD-2-Clause-Patent

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum QtmTheme {
    Light,
    Dark,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct QtmConfig {
    version: u32,
    theme: QtmTheme,
}

fn get_version_u32() -> u32 {
    env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap()
    + 100 * env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap()
    + 10000 * env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap()
}

impl Default for QtmConfig {
    fn default() -> Self {
        QtmConfig {
            version: get_version_u32(),
            theme: QtmTheme::Light,
        }
    }
}