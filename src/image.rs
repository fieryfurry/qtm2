// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::path::{Path, PathBuf};

use eframe::egui;
use eframe::egui::TextureHandle;
use vcsr::args::Args;
use vcsr::models::{Grid, MetadataPosition, TimestampPosition};
use walkdir::WalkDir;

#[derive(Clone)]
pub(crate) struct Image {
    pub(crate) path: PathBuf,
    pub(crate) filename: String,
    pub(crate) size: u64,
    // TODO: Add GIF animation
    // see https://github.com/Gui-Yom/vibin/blob/26e1a89a193d16754a1e33bd495aa51cd9b886a1/src/main.rs
    pub(crate) texture_handle: Option<TextureHandle>,
}

impl Image {
    // ASSERT: ONLY CALLABLE WHEN TEXTURE_HANDLE IS SOME
    pub(crate) fn calculate_image_dimension(&self, image_area: usize) -> egui::Vec2 {
        let texture_handle = self.texture_handle.as_ref().unwrap();
        let scaling_factor = ((image_area as f32 / texture_handle.size()[0] as f32)
            / texture_handle.size()[1] as f32)
            .sqrt();
        texture_handle.size_vec2() * scaling_factor
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Image {}

pub(crate) fn generate_video_thumbnail_image<P: AsRef<Path>>(video_path: P) -> anyhow::Result<()> {
    todo!()
}
