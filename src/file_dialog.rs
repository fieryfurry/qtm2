// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::path::{Path, PathBuf};

use eframe::egui::{ColorImage, TextureHandle, TextureOptions, Ui};
use rfd::FileDialog;
use tracing::warn;

use super::Image;

trait Pred {
    fn pred<F1, F2, P, T>(self, predicate: P, true_fn: F1, false_fn: F2) -> T
    where
        P: FnOnce(&Self) -> bool,
        F1: Fn(Self) -> T,
        F2: Fn(Self) -> T,
        Self: Sized,
    {
        if predicate(&self) {
            true_fn(self)
        } else {
            false_fn(self)
        }
    }
}

impl Pred for FileDialog {}

fn create_file_dialog<P: AsRef<Path> + Clone>(default_directory: Option<P>) -> FileDialog {
    FileDialog::new().pred(
        |_| {
            default_directory.is_some()
                && default_directory.clone().unwrap().as_ref().exists()
                && default_directory.clone().unwrap().as_ref().is_dir()
        },
        |dir| dir.set_directory(default_directory.clone().unwrap()),
        |dir| dir,
    )
}

pub(crate) fn select_content<P: AsRef<Path> + Clone>(
    is_file: bool,
    default_directory: Option<P>,
) -> Option<(PathBuf, String, u64)> {
    create_file_dialog(default_directory)
        .pred(|_| is_file, FileDialog::pick_file, FileDialog::pick_folder)
        .map(|c| {
            (
                c.clone(),
                c.to_string_lossy().into_owned(),
                fs_extra::dir::get_size(&c).unwrap(),
            )
        })
}

// TODO: Add all supported file extensions
pub(crate) fn select_image<P: AsRef<Path> + Clone>(default_directory: Option<P>, ui: &mut Ui) -> Option<Image> {
    create_file_dialog(default_directory)
        .add_filter(
            "image",
            &["png", "PNG", "jpg", "JPG", "jpeg", "JPEG", "gif", "GIF"],
        )
        .pick_file()
        .map(|f| Image {
            path: f.clone(),
            filename: f.file_name().unwrap().to_string_lossy().into_owned(),
            size: f.metadata().unwrap().len(),
            texture_handle: {
                match create_image_texture_handle(f, ui) {
                    Ok(th) => Some(th),
                    Err(err) => {
                        warn!(?err, "Unable to load image as texture; image preview unavailable");
                        None
                    }
                }
            }
        })
}

fn create_image_texture_handle<P: AsRef<Path>>(image_path: P, ui: &mut Ui) -> anyhow::Result<TextureHandle> {
    let image = image::io::Reader::open(image_path.as_ref())?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let colour_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    Ok(ui.ctx().load_texture(image_path.as_ref().to_string_lossy(), colour_image, TextureOptions::LINEAR))
}