// SPDX-License-Identifier: BSD-2-Clause-Patent
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Cow;
use std::cell::Cell;
use std::fmt::Debug;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use directories::ProjectDirs;
use eframe::egui::TextStyle::*;
use eframe::egui::{vec2, Color32, FontData, FontDefinitions, FontId, Pos2, Style, Visuals};
use eframe::egui::{FontFamily, Margin, Rounding};
use tracing::{error, info, Level};

use crate::image::Image;
use crate::password_prompt::PasswordPrompt;
use crate::qtm::Qtm;
use crate::qtm_config::{QtmConfig, QtmTheme, QtmVersion};
use crate::qtm_networking::QtmNetworking;
use crate::tag::TagData;

mod category;
mod file_dialog;
mod image;
mod password_prompt;
mod qtm;
mod qtm_config;
mod qtm_networking;
mod selectable_table;
mod torrent;
mod unwrap_trace;
mod tag;

fn proj_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "qtm2").ok_or(anyhow::Error::from(Error::new(
        ErrorKind::NotFound,
        "No valid home directory path found",
    )))
}

fn config_local_dir(filename: &str) -> PathBuf {
    proj_dirs().unwrap().config_local_dir().join(filename)
}

// TODO: Add tracing section in README
fn data_local_dir(filename: &str) -> PathBuf {
    proj_dirs().unwrap().data_local_dir().join(filename)
}

fn cache_dir(filename: &str) -> PathBuf {
    proj_dirs().unwrap().cache_dir().join(filename)
}

fn initialise_dirs() -> Result<()> {
    // Project directory
    let proj_dirs = proj_dirs()?;
    // Create folders if they do not exist
    if !proj_dirs.config_dir().exists() {
        if let Err(err) = fs::create_dir_all(proj_dirs.config_dir()) {
            error!(?err, "Unable to create configuration folder; exiting");
            return Err(anyhow::Error::new(err));
        }
    }
    if !proj_dirs.data_local_dir().exists() {
        if let Err(err) = fs::create_dir_all(proj_dirs.data_local_dir()) {
            error!(?err, "Unable to create data folder; exiting");
            return Err(anyhow::Error::new(err));
        }
    }
    if !proj_dirs.cache_dir().exists() {
        if let Err(err) = fs::create_dir_all(proj_dirs.cache_dir()) {
            error!(?err, "Unable to create data folder; exiting");
            return Err(anyhow::Error::new(err));
        }
    }
    Ok(())
}

fn get_style_by_theme(theme: QtmTheme) -> Style {
    let mut style = Style {
        text_styles: [
            (Heading, FontId::new(30.0, FontFamily::Proportional)),
            (Body, FontId::new(16.0, FontFamily::Proportional)),
            (Monospace, FontId::new(14.0, FontFamily::Monospace)),
            (Button, FontId::new(16.0, FontFamily::Proportional)),
            (Small, FontId::new(14.0, FontFamily::Proportional)),
        ]
        .into(),
        ..Default::default()
    };
    style.spacing.window_margin = Margin {
        left: 20.,
        right: 0.,
        top: 20.,
        bottom: 0.,
    };
    if theme == QtmTheme::Light {
        style.visuals = Visuals::light();
        style.visuals.window_fill = Color32::LIGHT_GRAY;
        style.visuals.widgets.noninteractive.fg_stroke.color = Color32::BLACK;
        style.visuals.widgets.inactive.fg_stroke.color = Color32::BLACK;
    } else {
        style.visuals = Visuals::dark();
        style.visuals.window_fill = Color32::from_rgb(24, 24, 24);
        style.visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        style.visuals.widgets.inactive.fg_stroke.color = Color32::WHITE;
    }
    style.visuals.window_rounding = Rounding::none();
    style.visuals.widgets.inactive.bg_stroke = style.visuals.widgets.noninteractive.bg_stroke;
    style
}

pub fn set_context(cc: &eframe::CreationContext<'_>, theme: QtmTheme) {
    // Style
    let style = get_style_by_theme(theme);

    // Fonts
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "inter".to_owned(),
        FontData::from_static(include_bytes!("../res/Inter-Light.otf")),
    );

    fonts.font_data.insert(
        "source-code-pro".to_owned(),
        FontData::from_static(include_bytes!("../res/SourceCodePro-Regular.otf")),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "inter".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "source-code-pro".to_owned());

    cc.egui_ctx.set_style(style);
    cc.egui_ctx.set_fonts(fonts);
}

// is_open, text, is_ok_showing
#[derive(Debug)]
struct DialogMessage(Cow<'static, str>, bool);

// TODO:
//          Add CLI support
//          Add networking/communication/authentication features
//          Add uTorrent/qBittorrent integration

fn main() -> Result<()> {
    // Initialise directories
    initialise_dirs()?;

    // Tracing init
    let file_appender = tracing_appender::rolling::daily(cache_dir(""), "qtm2.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).map_err(anyhow::Error::new)?;

    info!("Started tracing");

    // Config init
    let config = QtmConfig::load(config_local_dir("config.toml"));
    let icon_data = Some(eframe::IconData {
        rgba: include_bytes!("../res/qtm2.svg").to_vec(),
        width: 512,
        height: 512,
    });
    TagData::init_data(cache_dir("tags.json"));
    let is_authenticated = Rc::new(Cell::new(false));
    let is_authenticated_clone = is_authenticated.clone();

    // Networking init
    let networking = Rc::new(QtmNetworking::try_new()?);
    let networking_clone = networking.clone();
    info!("Started networking");

    #[cfg(not(debug_assertions))]
    {
        // Egui init
        eframe::run_native(
            &format!(
                "Quick Torrent Maker 2 v{}",
                QtmVersion::get_current_version()
            ),
            eframe::NativeOptions {
                initial_window_pos: Some(Pos2::new(400., 400.)),
                initial_window_size: Some(vec2(400., 150.)),
                resizable: false,
                icon_data: icon_data.clone(),
                ..Default::default()
            },
            Box::new(move |cc| {
                Box::new(PasswordPrompt::new(
                    cc,
                    config.theme,
                    is_authenticated_clone,
                    networking_clone,
                ))
            }),
        )
            .map_err(|err| {
                error!(
            ?err,
            "QTM2 failed to set up a graphics context for password prompt"
        );
                anyhow::Error::msg(err.to_string())
            })?;

        if !is_authenticated.get() {
            info!("Not authenticated; exiting");
            return Ok(());
        }
    }

    eframe::run_native(
        &format!(
            "Quick Torrent Maker 2 v{}",
            QtmVersion::get_current_version()
        ),
        eframe::NativeOptions {
            initial_window_size: Some(vec2(
                config.initial_window_size.0 as f32,
                config.initial_window_size.1 as f32,
            )),
            icon_data,
            ..Default::default()
        },
        Box::new(|cc| Box::new(Qtm::new(cc, config))),
    )
    .map_err(|err| {
        error!(?err, "QTM2 failed to set up a graphics context");
        anyhow::Error::msg(err.to_string())
    })
}
