// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use eframe::egui::{
    Button, Color32, FontFamily, FontId, Response, RichText, Rounding, Stroke, TextStyle, Ui, vec2,
    Widget,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tracing::{info, warn};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TagColor {
    Cyan,
    Green,
    Blue,
    Purple,
    DarkGray,
    Yellow,
    Orange,
    Red,
    LightGray,
    Gray,
}

impl TagColor {
    pub const fn to_primary_color(self) -> Color32 {
        match self {
            TagColor::Cyan => Color32::from_rgb(26, 188, 156),
            TagColor::Green => Color32::from_rgb(46, 204, 113),
            TagColor::Blue => Color32::from_rgb(52, 152, 219),
            TagColor::Purple => Color32::from_rgb(155, 89, 182),
            TagColor::DarkGray => Color32::from_rgb(52, 73, 94),
            TagColor::Yellow => Color32::from_rgb(241, 196, 15),
            TagColor::Orange => Color32::from_rgb(230, 126, 34),
            TagColor::Red => Color32::from_rgb(231, 76, 60),
            TagColor::LightGray => Color32::from_rgb(236, 240, 241),
            TagColor::Gray => Color32::from_rgb(149, 165, 166),
        }
    }

    pub const fn to_secondary_color(self) -> Color32 {
        match self {
            TagColor::Blue | TagColor::Purple | TagColor::DarkGray => Color32::WHITE,
            _ => Color32::BLACK,
        }
    }
}

fn default_selected() -> Option<bool> {
    Some(false)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TagData {
    pub text: String,
    pub color: TagColor,
}

impl Ord for TagData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.text.cmp(&other.text)
    }
}

impl PartialOrd for TagData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TagData {
    pub fn _init_data<P: AsRef<Path>>(path: P) {
        // ONLY TEMPORARY
        let tags = vec![
            TagData {
                text: "Onlyfans".to_owned(),
                color: TagColor::Blue,
            },
            TagData {
                text: "Fansly".to_owned(),
                color: TagColor::Cyan,
            },
            TagData {
                text: "JustForFans".to_owned(),
                color: TagColor::Red,
            },
            TagData {
                text: "Friends 2 Follow".to_owned(),
                color: TagColor::Purple,
            },
            TagData {
                text: "Pornhub".to_owned(),
                color: TagColor::Orange,
            },
            TagData {
                text: "Chaturbate".to_owned(),
                color: TagColor::Yellow,
            },
            TagData {
                text: "Twitter".to_owned(),
                color: TagColor::Blue,
            },
            TagData {
                text: "Reddit".to_owned(),
                color: TagColor::Orange,
            },
        ];
        fs::write(path, serde_json::to_string(&tags).unwrap()).unwrap();
    }

    pub fn fetch_data<P: AsRef<Path>>(path: P) -> Vec<Self> {
        // TODO: Download tags from GT and save it locally
        //       Only re-download if hash has changed
        let file_content = match fs::read_to_string(path.as_ref()) {
            Ok(string) => string,
            Err(err) => {
                warn!(?err, "Unable to fetch tags locally");
                return Vec::new();
            }
        };

        match serde_json::from_str::<Vec<Self>>(&file_content) {
            Ok(tags) => {
                info!("Fetched tags locally");
                tags
            }
            Err(err) => {
                warn!(?err, "Unable to deserialise tags");
                Vec::new()
            }
        }
    }

    pub fn fetch_custom() -> Vec<Self> {
        todo!()
    }

    pub fn save_custom() {
        todo!()
    }
}

#[derive(Debug)]
pub struct Tag<'a> {
    pub data: &'a TagData,
    pub is_selected: &'a mut bool,
}

impl<'a> Tag<'a> {
    pub fn new(data: &'a TagData, is_selected: &'a mut bool) -> Self {
        Tag { data, is_selected }
    }
}

pub type TagState<'a> = (&'a TagData, &'a mut bool);

impl<'a> From<&'a mut TagState<'_>> for Tag<'a> {
    fn from(value: &'a mut TagState<'_>) -> Self {
        Self {
            data: value.0,
            is_selected: value.1,
        }
    }
}

impl<'a> Widget for Tag<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.style_mut().spacing.button_padding = vec2(10., 4.);

        let text = RichText::new(&self.data.text)
            .small()
            .color(self.data.color.to_secondary_color());

        ui.add(
            Button::new(text)
                .fill(self.data.color.to_primary_color())
                .stroke(if *self.is_selected {
                    Stroke {
                        width: 2.,
                        color: if ui.style().visuals.dark_mode {
                            Color32::WHITE
                        } else {
                            Color32::BLACK
                        },
                    }
                } else {
                    Stroke::NONE
                })
                .rounding(Rounding::same(20.)),
        )
    }
}
