// SPDX-License-Identifier: BSD-2-Clause-Patent

use eframe::egui::{Button, Color32, Response, RichText, Rounding, Sense, Stroke, Ui, vec2, Widget};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    Concrete
}

impl TagColor {
    pub fn to_color(self) -> Color32 {
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


#[derive(Debug, Clone)]
pub struct Tag {
    pub value: String,
    pub color: TagColor,
    pub is_clickable: bool,
}

impl Tag {
    pub fn new(value: String, color: TagColor, is_clickable: bool) -> Self {
        Tag {
            value,
            color,
            is_clickable
        }
    }
}

impl Widget for Tag {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.style_mut().spacing.button_padding = vec2(10., 4.);
        let text = RichText::new(self.value)
            .small()
            .color(Color32::WHITE);

        ui.add(Button::new(text)
            .fill(self.color.to_color())
            .stroke(Stroke::NONE)
            .sense(Sense::drag())
            .rounding(Rounding::same(20.))
        )
    }
}

