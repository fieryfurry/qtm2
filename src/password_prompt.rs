// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::cell::Cell;
use std::rc::Rc;

use eframe::egui;
use eframe::egui::{Context, Frame, vec2, widgets};
use tracing::info;

use crate::qtm_config::QtmTheme;
use crate::qtm_networking::QtmNetworking;
use crate::set_context;

#[derive(Debug)]
pub(crate) struct PasswordPrompt {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) is_authenticated: Rc<Cell<bool>>,
    pub(crate) networking: Rc<QtmNetworking>,
}

impl PasswordPrompt {
    pub(crate) fn new(
        cc: &eframe::CreationContext<'_>,
        theme: QtmTheme,
        is_authenticated: Rc<Cell<bool>>,
        networking: Rc<QtmNetworking>,
    ) -> Self {
        info!("Started Password Prompt");
        set_context(cc, theme);
        Self {
            username: String::new(),
            password: String::new(),
            is_authenticated,
            networking,
        }
    }

    fn is_valid(&self) -> bool {
        !self.username.is_empty()
            && self.username.is_ascii()
            && !self.password.is_empty()
            && self.password.is_ascii()
    }

    fn authenticate(&mut self, frame: &mut eframe::Frame) {
        info!("Attempted to log in");
        if self.networking.login(self.username.clone(), self.password.clone()) {
            self.is_authenticated.set(true);
            frame.close();
        } else {
            // TODO: add 'incorrect login credential' message
            self.username.clear();
            self.password.clear();
        }
    }
}

impl eframe::App for PasswordPrompt {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::window(&ctx.style()))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.);
                    ui.horizontal(|ui| {
                        ui.add_space(20.);
                        ui.add_sized(vec2(100., 20.), widgets::Label::new("Username:"));
                        ui.add_sized(
                            vec2(200., 20.),
                            widgets::text_edit::TextEdit::singleline(&mut self.username),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(20.);
                        ui.add_sized(vec2(100., 20.), widgets::Label::new("Password:"));
                        if ui.add_sized(
                            vec2(200., 20.),
                            widgets::text_edit::TextEdit::singleline(&mut self.password)
                                .password(true),
                        ).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.authenticate(frame);
                        }
                    });
                    ui.add_space(10.);
                    ui.set_enabled(self.is_valid());
                    if ui
                        .add_sized(vec2(200., 40.), widgets::Button::new("LOG IN"))
                        .clicked()
                    {
                        self.authenticate(frame);
                    }
                });
            });
    }
}
