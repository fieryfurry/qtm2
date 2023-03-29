// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::path::{Path, PathBuf};

use eframe::egui;
use eframe::egui::TextureHandle;
use vcsr::args::Args;
use vcsr::models::{MetadataPosition, TimestampPosition};
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

// Wrapper for vcsr `process_file` function
pub(crate) fn generate_video_thumbnail_image<P: AsRef<Path>>(video_path: P) -> anyhow::Result<()> {
    let mut args = Args {
        num_groups: None,
        num_selected: None,
        accurate: false,
        accurate_delay_seconds: 0.0,
        actual_size: false,
        background_colour: "".to_string(),
        capture_alpha: 0,
        delay_percent: None,
        end_delay_percent: 0.0,
        exclude_extensions: vec![],
        fast: false,
        frame_type: None,
        filenames: vec![],
        grid: Default::default(),
        grid_spacing: None,
        grid_horizontal_spacing: 0,
        grid_vertical_spacing: 0,
        image_format: "".to_string(),
        ignore_errors: false,
        interval: None,
        manual_timestamps: vec![],
        metadata_background_colour: "".to_string(),
        metadata_font: None,
        metadata_font_colour: "".to_string(),
        metadata_font_size: 0.0,
        metadata_horizontal_margin: 0,
        metadata_margin: 0,
        metadata_position: MetadataPosition::Top,
        metadata_vertical_margin: 0,
        no_overwrite: false,
        output_path: None,
        recursive: false,
        num_samples: None,
        no_shadow: false,
        start_delay_percent: 0.0,
        show_timestamp: false,
        thumbnail_output_path: None,
        timestamp_background_colour: "".to_string(),
        timestamp_border_colour: "".to_string(),
        timestamp_border_mode: false,
        timestamp_border_radius: 0.0,
        timestamp_border_size: 0,
        timestamp_font: None,
        timestamp_font_colour: "".to_string(),
        timestamp_font_size: 0.0,
        timestamp_position: TimestampPosition::North,
        timestamp_horizontal_margin: 0,
        timestamp_horizontal_padding: 0,
        timestamp_vertical_margin: 0,
        timestamp_vertical_padding: 0,
        vcs_width: 0,
        verbose: false,
    };
    match vcsr::process_file(
        &WalkDir::new(video_path.as_ref())
            .into_iter()
            .next()
            .unwrap()
            .unwrap(),
        &mut args,
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow::Error::new(err)),
    }
}
