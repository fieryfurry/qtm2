// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

use bytesize::ByteSize;
use eframe::egui;
use eframe::egui::{
    Align, Context, Frame, Grid, Id, Layout, Margin, Rounding, ScrollArea, show_tooltip, vec2,
    widgets,
};
use eframe::egui::TextStyle::Monospace;
use strum::IntoEnumIterator;
use tracing::{info, warn};

use crate::{
    cache_dir, config_local_dir, data_local_dir, DialogMessage, file_dialog, get_style_by_theme,
    initialise_dirs, selectable_table, set_context,
};
use crate::category::Category;
use crate::file_dialog::select_content;
use crate::image::Image;
use crate::qtm_config::{QtmConfig, QtmTheme};
use crate::selectable_table::{Column, TableBuilder};
use crate::tag::{Tag, TagColor, TagData};
use crate::torrent::create_torrent_file;

pub struct Qtm {
    config: QtmConfig,

    dialog: Option<DialogMessage>,
    dialog_channel: (mpsc::Sender<DialogMessage>, mpsc::Receiver<DialogMessage>),

    is_file: bool,
    content: Option<(PathBuf, String, u64)>,

    categories: [Category; 5],

    images: Vec<Image>,
    selected_index: Option<usize>,

    title: String,
    description: String,

    tags: BTreeMap<TagData, bool>,
    is_tag_menu_open: bool,
}

impl Qtm {
    pub fn new(cc: &eframe::CreationContext<'_>, config: QtmConfig, tags: Vec<TagData>) -> Self {
        info!("Started Main Application");
        set_context(cc, config.theme);

        Self {
            config,
            dialog: None,
            dialog_channel: mpsc::channel(),
            is_file: true,
            content: None,
            categories: [Category::None; 5],
            images: Vec::new(),
            selected_index: None,
            title: "".to_owned(),
            description: "".to_owned(),
            tags: tags.into_iter().map(|tag| (tag, false)).collect(),
            is_tag_menu_open: false,
        }
    }

    fn show_dialog_window(&mut self, context: &Context, message: &str, is_ok_showing: bool) {
        egui::Window::new("dialog")
            .fixed_size(vec2(400., 300.))
            .title_bar(false)
            .frame(Frame::window(&context.style()).rounding(Rounding::same(10.)))
            .show(context, |ui| {
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    ui.label(message);
                    ui.add_space(50.);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if is_ok_showing
                            && ui
                            .add_sized(vec2(200., 25.), widgets::Button::new("OK"))
                            .clicked()
                        {
                            self.dialog = None;
                        }
                    });
                })
            });
    }

    fn is_acceptable(&self) -> bool {
        // TODO: Reject if the content's name contains illegal characters
        if self.content.is_none() {
            return false;
        }
        // rejects if there is no category, image, title or description
        // TODO: Add check for description only whitespace or newline character
        if self.categories[0] == Category::None
            || self.images.is_empty()
            || self.title.trim().is_empty()
            || self.description.trim().is_empty()
        {
            return false;
        }
        true
    }
}

impl eframe::App for Qtm {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if self.dialog.is_none() || !self.dialog.as_ref().unwrap().1 {
            match self.dialog_channel.1.try_recv() {
                Ok(message) => self.dialog = Some(message),
                Err(err) => match err {
                    TryRecvError::Empty => {}
                    TryRecvError::Disconnected => panic!(), // ASSERT: `self` always hold the sender too
                },
            }
        }

        if let Some(DialogMessage(message, is_ok_showing)) = &self.dialog {
            self.show_dialog_window(ctx, &message.clone(), *is_ok_showing);
        }

        egui::TopBottomPanel::top("top_panel")
            .exact_height(25.)
            .show(ctx, |ui| {
                ui.set_enabled(self.dialog.is_none() && !self.is_tag_menu_open);
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
                        .on_hover_text("Toggle light/dark theme")
                        .clicked()
                    {
                        self.config.theme = -self.config.theme;

                        ctx.set_style(get_style_by_theme(self.config.theme));
                        info!("Theme changed to {}", self.config.theme);
                        self.config.save(config_local_dir("config.toml"));
                    }

                    if ui
                        .add_sized(
                            vec2(ui.available_height(), ui.available_height()),
                            widgets::Button::new("☆"),
                        )
                        .on_hover_text("Set default directory")
                        .clicked()
                    {
                        if let Some((path, _, _)) =
                            select_content(false, self.config.default_directory.as_deref())
                        {
                            info!("Default directory set to {}", path.to_string_lossy());
                            self.config.default_directory = Some(path);
                            self.config.save(config_local_dir("config.toml"));
                        }
                    }
                    ui.add_space(110.);
                    if ui
                        .add_sized(
                            vec2(ui.available_height(), ui.available_height()),
                            widgets::Button::new("⚠"),
                        )
                        .on_hover_text("CAUTION: clear all cache")
                        .clicked()
                    {
                        for dir in [cache_dir(""), config_local_dir(""), data_local_dir("")] {
                            fs::remove_dir_all(dir).unwrap();
                        }
                        initialise_dirs().unwrap();
                        frame.close();
                    }
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(40.)
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_enabled_ui(self.dialog.is_none() && !self.is_tag_menu_open && self.is_acceptable(), |ui| {
                        if ui
                            .add_sized(vec2(150., 20.), widgets::Button::new("Upload torrent"))
                            .clicked()
                        {
                            info!("Begin torrent upload");
                            self.dialog_channel
                                .0
                                .send(DialogMessage(
                                    Cow::Borrowed("Creating torrent...\n\nThis may take a while..."),
                                    false,
                                ))
                                .unwrap();

                            let content_path = self.content.clone().unwrap().0;
                            let sender = self.dialog_channel.0.clone();
                            std::thread::spawn(|| {
                                create_torrent_file(content_path, sender);
                            });
                        }
                    });

                    // Uploading rules
                    ui.add_space(10.);
                    let rule_url = "https://www.gaytor.rent/rules.php#102";
                    if ui.hyperlink_to("Uploading Rules", rule_url).clicked() {
                        if let Err(err) = open::that(rule_url) {
                            warn!(?err, "Failed to open uploading rules link: {rule_url}");
                        }
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(Frame::window(&ctx.style()))
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.set_enabled(self.dialog.is_none() && !self.is_tag_menu_open);
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

                        ui.add_space(8.);

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
                                // TODO: Add built-in video thumbnail generator

                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Image:");
                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            if ui
                                                .add(egui::Button::new("...").min_size(vec2(40., 10.)))
                                                .clicked()
                                            {
                                                if let Some(mut images) = file_dialog::select_images(
                                                    self.config.default_directory.as_deref(),
                                                    &self.images,
                                                    &self.dialog_channel.0,
                                                    ui,
                                                ) {
                                                    self.images.append(&mut images);
                                                }
                                            }
                                        });
                                    });
                                    if let Some(selected_index) = self.selected_index {
                                        ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                            ui.add_space(10.);
                                            if ui.add(egui::Button::new("↑").min_size(vec2(50., 10.))).clicked() && selected_index != 0 {
                                                self.images.swap(selected_index, selected_index - 1);
                                                self.selected_index = Some(selected_index - 1);
                                            }
                                            if ui.add(egui::Button::new("↓").min_size(vec2(50., 10.))).clicked() && selected_index != self.images.len() - 1 {
                                                self.images.swap(selected_index, selected_index + 1);
                                                self.selected_index = Some(selected_index + 1);
                                            }
                                            if ui.add(egui::Button::new("✗").min_size(vec2(50., 10.))).clicked() {
                                                self.images.remove(selected_index);
                                                self.selected_index = None;
                                                ui.data_mut(|d| d.insert_persisted::<Option<usize>>(Id::new("selected_index"), None));
                                            }
                                        });
                                    }
                                });

                                let (rect, _) = ui.allocate_exact_size(vec2(ui.available_size_before_wrap().x, 200.), selectable_table::SENSE_NONE);
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
                                                let response = body.row(20., |mut row| {
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
                                                if response.clicked() {
                                                    self.selected_index = Some(index);
                                                }
                                                if response.hovered() && image.texture_handle.is_some() {
                                                    show_tooltip(ui.ctx(), Id::new("image preview"), |ui| {
                                                        ui.image(image.texture_handle.as_ref().unwrap(),
                                                                 image.calculate_image_dimension(self.config.image_area),
                                                        )
                                                    });
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

                                ui.end_row();

                                // Tags
                                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                    ui.label("Tags:");
                                });

                                ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true),
                                               |ui| {
                                                   ui.style_mut().spacing.item_spacing = vec2(10., 10.);
                                                   for mut tag in self.tags
                                                       .iter_mut()
                                                       .filter(|t| *t.1) {
                                                       if ui.add(Tag::from(&mut tag)).secondary_clicked() {
                                                           *tag.1 = false;
                                                       }
                                                   }

                                                   if ui.add(Tag::new(&TagData {
                                                       text: "➕".to_owned(),
                                                       color: TagColor::BlueGrey,
                                                   }, &mut self.is_tag_menu_open)).clicked() {
                                                       self.is_tag_menu_open = !self.is_tag_menu_open;
                                                   }
                                               });
                            });
                        ui.add_space(20.);
                    });
            });
        if self.is_tag_menu_open {
            egui::Window::new("tags")
                .frame(Frame::window(&ctx.style())
                    .rounding(Rounding::same(10.))
                    .inner_margin(Margin::same(10.))
                )
                .fixed_size(vec2(400., 200.))
                .default_pos(ctx.pointer_latest_pos().unwrap_or_default())
                .title_bar(false)
                .drag_bounds(ctx.screen_rect())
                .show(ctx, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
                        ui.style_mut().spacing.item_spacing = vec2(10., 10.);
                        for mut tag_state in self.tags.iter_mut() {
                            if ui.add(Tag::from(&mut tag_state)).clicked() {
                                *tag_state.1 = !*tag_state.1;
                            }
                        }
                    });
                    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                        if ui
                            .add_sized(vec2(100., 20.), widgets::Button::new("OK").rounding(Rounding::same(10.)))
                            .clicked() {
                            self.is_tag_menu_open = false;
                        }
                    });
                    ui.allocate_space(ui.available_size());
                });
        }
    }
}
