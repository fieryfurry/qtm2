// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::fmt::Debug;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use anyhow::Result;
use bytesize::ByteSize;
use directories::ProjectDirs;
use eframe::egui;
use eframe::egui::TextStyle::*;
use eframe::egui::{
    vec2, widgets, Align, Color32, FontData, FontDefinitions, FontId, Grid, Id, Layout, ScrollArea,
    Style, Visuals,
};
use eframe::egui::{FontFamily, Frame, Margin, Rounding};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tracing::{info, warn, Level};

use crate::file_dialog::select_content;
use crate::qtm_config::{QtmConfig, QtmTheme};
use crate::selectable_table::{Column, TableBuilder};

mod file_dialog;
mod qtm_config;
mod selectable_table;
mod unwrap_trace;

fn proj_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("proton.me", "fieryfurry", "quick-torrent-maker-2").ok_or(
        anyhow::Error::from(Error::new(
            ErrorKind::NotFound,
            "No valid home directory path found",
        )),
    )
}

fn config_dir(filename: &str) -> PathBuf {
    proj_dirs().unwrap().config_dir().join(filename)
}

fn data_local_dir(filename: &str) -> PathBuf {
    proj_dirs().unwrap().data_local_dir().join(filename)
}

fn get_style_by_theme(theme: QtmTheme) -> Style {
    let mut style = Style {
        text_styles: [
            (Heading, FontId::new(30.0, FontFamily::Proportional)),
            (Body, FontId::new(18.0, FontFamily::Proportional)),
            (Monospace, FontId::new(14.0, FontFamily::Monospace)),
            (Button, FontId::new(18.0, FontFamily::Proportional)),
            (Small, FontId::new(14.0, FontFamily::Proportional)),
        ]
        .into(),
        ..Default::default()
    };
    style.spacing.window_margin = Margin::same(20.0);
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

fn main() -> Result<()> {
    // Project directory
    let proj_dirs = proj_dirs()?;
    // Create folders if they do not exist
    if !proj_dirs.config_dir().exists() {
        if let Err(err) = fs::create_dir_all(proj_dirs.config_dir()) {
            warn!(?err, "Unable to create configuration folder; exiting");
            return Err(anyhow::Error::from(err));
        }
    }
    if !proj_dirs.data_local_dir().exists() {
        if let Err(err) = fs::create_dir_all(proj_dirs.data_local_dir()) {
            warn!(?err, "Unable to create data folder; exiting");
            return Err(anyhow::Error::from(err));
        }
    }

    // Tracing init
    let file_appender = tracing_appender::rolling::daily(proj_dirs.data_local_dir(), "qtm2.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Started application");

    // Config init
    let config = QtmConfig::load(config_dir("config.toml"));

    info!("Loaded configuration");

    // Egui init
    eframe::run_native(
        "Quick Torrent Maker 2",
        eframe::NativeOptions {
            initial_window_size: Some(vec2(800., 800.)),
            ..Default::default()
        },
        Box::new(move |cc| Box::new(Qtm::new(cc, config))),
    )
    .map_err(|err| anyhow::Error::msg(err.to_string()))
}

// TODO:
//          Add password prompt
//          Add CLI support
//          Add networking/communication/authentication features
//          Add Bencode encoding/decoding for torrent files [Bendy](https://crates.io/crates/bendy)
//          Add uTorrent/qBittorrent integration

struct Qtm {
    config: QtmConfig,

    is_file: bool,
    content: Option<(PathBuf, String, u64)>,

    categories: [Category; 5],

    images: Vec<Image>,
    selected_index: Option<usize>,

    title: String,
    description: String,
}

#[derive(EnumIter, Display, Debug, Copy, Clone, Eq, PartialEq)]
enum Category {
    None,
    Amateur,
    Clips,
    Images,
}

#[derive(Debug, Clone)]
pub(crate) struct Image {
    path: PathBuf,
    filename: String,
    size: u64,
}

impl Qtm {
    fn new(cc: &eframe::CreationContext<'_>, config: QtmConfig) -> Self {
        info!("Started GUI");

        // Style
        let style = get_style_by_theme(config.theme);

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

        Self {
            config,
            is_file: true,
            content: None,
            categories: [Category::None; 5],
            images: Vec::new(),
            selected_index: None,
            title: "".to_owned(),
            description: "".to_owned(),
        }
    }

    fn is_acceptable(&self) -> bool {
        // TODO: Reject if the content's name contains illegal characters
        if self.content.is_none() {
            return false;
        }
        // rejects if there is no category, image, title or description
        // TODO: Add check for description only whitespace or newline character
        if self.categories[0] == Category::None || self.images.is_empty() || self.title.trim().is_empty() || self.description.trim().is_empty() {
            return false;
        }
        true
    }

    fn generate_torrent(&mut self) {
        // TODO: Reject if images contain duplicates


        todo!()
    }
}

impl eframe::App for Qtm {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .exact_height(25.)
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if ui
                        .add_sized(
                            vec2(ui.available_height(), ui.available_height()),
                            widgets::Button::new(if self.config.theme == QtmTheme::Light {
                                "☼"
                            } else {
                                "☀"
                            }),
                        )
                        .clicked()
                    {
                        self.config.theme = -self.config.theme;
                        self.config
                            .save(config_dir("config.toml"));

                        ctx.set_style(get_style_by_theme(self.config.theme));
                        info!("Theme changed to {}", self.config.theme);
                    }

                    if ui
                        .add_sized(
                            vec2(ui.available_height(), ui.available_height()),
                            widgets::Button::new("☆"),
                        )
                        .clicked()
                    {
                        if let Some((path, _, _)) = select_content(false, self.config.default_directory.as_deref()) {
                            info!("Default directory changed to {}", path.to_string_lossy());
                            self.config.default_directory = Some(path);
                            self.config.save(config_dir("config.toml"));
                        }
                    }
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(40.)
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.set_enabled(self.is_acceptable());

                    if ui
                        .add_sized(vec2(150., 20.), widgets::Button::new("Upload torrent"))
                        .clicked()
                    {
                        self.generate_torrent();
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(Frame::window(&ctx.style()))
            .show(ctx, |ui| {
                // Content type
                ui.horizontal(|ui| {
                    if ui
                        .radio_value(&mut self.is_file, true, "Upload File")
                        .changed()
                    {
                        self.content = None;
                    }
                    ui.add_space(50.);
                    if ui
                        .radio_value(&mut self.is_file, false, "Upload Folder")
                        .changed()
                    {
                        self.content = None;
                    }
                });

                ui.add_space(10.);

                Grid::new("grid")
                    .num_columns(2)
                    .min_col_width(100.)
                    .min_row_height(25.)
                    .spacing([40., 10.])
                    .show(ui, |ui| {
                        // Content
                        ui.horizontal(|ui| {
                            ui.label("Path:");
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui
                                    .add(egui::Button::new("...").min_size(vec2(40., 10.)))
                                    .clicked()
                                {
                                    self.content = select_content(
                                        self.is_file,
                                        self.config.default_directory.as_deref(),
                                    );
                                }
                            });
                        });
                        ui.horizontal(|ui| {
                            if let Some((_, path_str, size)) = &self.content {
                                ui.add(
                                    egui::TextEdit::singleline(&mut path_str.as_str())
                                        .desired_width(ui.available_size().x - 120.)
                                        .font(Monospace),
                                );
                                ui.add(
                                    egui::TextEdit::singleline(
                                        &mut ByteSize(*size).to_string().as_str(),
                                    )
                                        .desired_width(120.)
                                        .horizontal_align(Align::Max)
                                        .font(Monospace),
                                );
                            }
                        });
                        ui.end_row();

                        // Categories
                        // TODO: Add all categories

                        for number in 0..5 {
                            ui.label(if number == 0 {
                                "Category:".to_owned()
                            } else {
                                format!("Sub-category {number}:")
                            });

                            ui.add_enabled_ui(
                                number == 0 || self.categories[number - 1] != Category::None,
                                |ui| {
                                    let changed = egui::ComboBox::new(number, "")
                                        .selected_text(format!("{}", self.categories[number]))
                                        .width(250.)
                                        .show_ui(ui, |ui| {
                                            let mut clicked = false;
                                            let (previous_categories, [current_category, ..]) = self.categories.split_at_mut(number) else {
                                                panic!("impossible");
                                            };
                                            for category in Category::iter()
                                                .filter(|c| *c == Category::None || !previous_categories.contains(c))
                                            {
                                                clicked |= ui
                                                    .selectable_value(
                                                        current_category,
                                                        category,
                                                        category.to_string(),
                                                    )
                                                    .clicked();
                                            }
                                            clicked
                                        })
                                        .inner
                                        .unwrap_or(false);
                                    if changed {
                                        for category in self.categories[number + 1..].iter_mut() {
                                            *category = Category::None;
                                        }
                                    }
                                },
                            );
                            ui.end_row();
                        }

                        // Images
                        // TODO: (1) Add image preview (open in default application/use preview tooltip)
                        //       (2) Add built-in video thumbnail generator

                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Image:");
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui
                                        .add(egui::Button::new("...").min_size(vec2(40., 10.)))
                                        .clicked()
                                    {
                                        if let Some(image) = file_dialog::select_image(
                                            self.config.default_directory.as_deref(),
                                        ) {
                                            self.images.push(image);
                                        }
                                    }
                                });
                            });
                            if let Some(selected_index) = self.selected_index {
                                ui.with_layout(Layout::top_down(Align::Max), |ui| {
                                    ui.add_space(10.);
                                    if ui.add(egui::Button::new("up").min_size(vec2(30., 10.))).clicked() && selected_index != 0 {
                                        self.images.swap(selected_index, selected_index - 1);
                                        self.selected_index = Some(selected_index - 1);
                                    }
                                    if ui.add(egui::Button::new("down").min_size(vec2(30., 10.))).clicked() && selected_index != self.images.len() - 1 {
                                        self.images.swap(selected_index, selected_index + 1);
                                        self.selected_index = Some(selected_index + 1);
                                    }
                                    if ui.add(egui::Button::new("delete").min_size(vec2(30., 10.))).clicked() {
                                        self.images.remove(selected_index);
                                        self.selected_index = None;
                                        ui.data_mut(|d| d.insert_persisted::<Option<usize>>(Id::new("selected_index"), None));
                                    }
                                });
                            }
                        });

                        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_size_before_wrap().x, 150.), selectable_table::SENSE_NONE);
                        {
                            let ui = &mut ui.child_ui(rect, *ui.layout());
                            let table = TableBuilder::new(ui)
                                .striped(true)
                                .vscroll(true)
                                .layout(Layout::left_to_right(Align::Center))
                                .column(Column::fixed(50.))
                                .column(Column::remainder())
                                .column(Column::fixed(100.))
                                .id(Id::new("selected_index"))
                                .build();

                            self.selected_index = table
                                .header(20., |mut row| {
                                    row.col(|ui| {
                                        ui.strong("Index");
                                    });
                                    row.col(|ui| {
                                        ui.strong("Filename");
                                    });
                                    row.col(|ui| {
                                        ui.strong("Size");
                                    });
                                })
                                .body(|mut body| {
                                    for (index, image) in self.images.iter().enumerate() {
                                        let clicked = body.row(20., |mut row| {
                                            row.col(|ui| {
                                                ui.monospace(index.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.monospace(&image.filename);
                                            });
                                            row.col(|ui| {
                                                ui.monospace(ByteSize(image.size).to_string());
                                            });
                                        });
                                        if clicked {
                                            self.selected_index = Some(index);
                                        }
                                    }
                                    self.selected_index
                                });
                        }
                        ui.end_row();

                        // Title and description
                        // TODO: (1) Add invalid character regex and warning
                        // TODO: (2) Add hyperlinks to Gaytor.rent official guides

                        ui.label("Title:");
                        ui.add(widgets::TextEdit::singleline(&mut self.title)
                            .desired_width(ui.available_width())
                            .hint_text("Descriptive title please!")
                        );

                        ui.end_row();
                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            ui.label("Description:");
                        });
                        ui.allocate_ui(vec2(ui.available_size_before_wrap().x, 200.), |ui| {
                            ScrollArea::vertical()
                                .always_show_scroll(true)
                                .stick_to_bottom(true)
                                .show(ui, |ui| {
                                    ui.add_sized(ui.available_size_before_wrap(),
                                                 widgets::TextEdit::multiline(&mut self.description)
                                                     .desired_width(ui.available_width())
                                                     .hint_text("(HTML/BB code not allowed)"),
                                    );
                                });
                        });
                    });
            });
    }
}
