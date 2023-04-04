// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::fs;
use std::path::Path;
use eframe::egui::{vec2, Button, Color32, Response, RichText, Rounding, Stroke, Ui, Widget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum TagColor {
    Turquoise,
    Emerland,
    Peterriver,
    Amethyst,
    Wetasphalt,
    Sunflower,
    Carrot,
    Alizarin,
    Clouds,
    Concrete,
}

impl TagColor {
    pub const fn to_color(self) -> Color32 {
        match self {
            TagColor::Turquoise => Color32::from_rgb(26, 188, 156),
            TagColor::Emerland => Color32::from_rgb(46, 204, 113),
            TagColor::Peterriver => Color32::from_rgb(52, 152, 219),
            TagColor::Amethyst => Color32::from_rgb(155, 89, 182),
            TagColor::Wetasphalt => Color32::from_rgb(52, 73, 94),
            TagColor::Sunflower => Color32::from_rgb(241, 196, 15),
            TagColor::Carrot => Color32::from_rgb(230, 126, 34),
            TagColor::Alizarin => Color32::from_rgb(231, 76, 60),
            TagColor::Clouds => Color32::from_rgb(236, 240, 241),
            TagColor::Concrete => Color32::from_rgb(149, 165, 166),
        }
    }
}

fn default_selected() -> Option<bool> {
    Some(true)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagData {
    pub value: String,
    // TODO: Change color to number
    pub color: TagColor,
    #[serde(skip, default = "default_selected")]
    pub is_selected: Option<bool>,
}

impl TagData {
    pub fn init_data<P: AsRef<Path>>(path: P) {
        // ONLY TEMPORARY
        let tags = vec![
        TagData {
            value: "Onlyfans".to_owned(),
            color: TagColor::Peterriver,
            is_selected: Some(false),
        },
        TagData {
            value: "Fansly".to_owned(),
            color: TagColor::Turquoise,
            is_selected: Some(false),
        },
        TagData {
            value: "JustForFans".to_owned(),
            color: TagColor::Alizarin,
            is_selected: Some(false),
        },
        TagData {
            value: "Friends 2 Follow".to_owned(),
            color: TagColor::Amethyst,
            is_selected: Some(false),
        },
        TagData {
            value: "Pornhub".to_owned(),
            color: TagColor::Sunflower,
            is_selected: Some(false),
        },];
        fs::write(path, serde_json::to_string_pretty(&tags).unwrap()).unwrap();
    }


    pub fn fetch_data() -> Vec<TagData> {
        // TODO: Download tags from GT and save it locally
        //       Only re-download if checksum has changed

        todo!()
    }

    pub fn fetch_custom() -> Vec<TagData> {
        todo!()
    }

    pub fn save_custom() {
        todo!()
    }
}

#[derive(Debug)]
pub struct Tag<'a> {
    pub value: &'a str,
    pub color: TagColor,
    pub is_selected: &'a mut Option<bool>,
}

impl<'a> Tag<'a> {
    pub fn new(value: &'a str, color: TagColor, is_selected: &'a mut Option<bool>) -> Self {
        Tag {
            value,
            color,
            is_selected,
        }
    }
}

impl<'a> From<&'a mut TagData> for Tag<'a> {
    fn from(value: &'a mut TagData) -> Self {
        Self {
            value: &value.value,
            color: value.color,
            is_selected: &mut value.is_selected,
        }
    }
}

impl<'a> Widget for Tag<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.style_mut().spacing.button_padding = vec2(10., 4.);

        let text = RichText::new(self.value)
            .small()
            .color(match self.is_selected {
                None | Some(true) => Color32::WHITE,
                Some(false) => self.color.to_color(),
            });

        let response = ui.add(
            Button::new(text)
                .fill(match self.is_selected {
                    None | Some(true) => self.color.to_color(),
                    Some(false) => Color32::WHITE,
                })
                .stroke(match self.is_selected {
                    None => Stroke::NONE,
                    Some(true) => Stroke {
                        width: 1.,
                        color: Color32::WHITE,
                    },
                    Some(false) => Stroke {
                        width: 1.,
                        color: self.color.to_color(),
                    },
                })
                .rounding(Rounding::same(20.)),
        );

        if response.clicked() && self.is_selected.is_some() {
            let is_selected = self.is_selected.as_mut().unwrap();
            *is_selected = !*is_selected;
        }
        response
    }
}
