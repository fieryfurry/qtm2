// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;
use eframe::egui::{vec2, widgets, Context, Frame};
use tracing::info;

use crate::qtm_config::QtmTheme;
use crate::set_context;

#[derive(Debug)]
pub(crate) struct PasswordPrompt {
    pub(crate) cache_path: PathBuf,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) is_authenticated: Rc<Cell<bool>>,
}

impl PasswordPrompt {
    pub(crate) fn new(
        cc: &eframe::CreationContext<'_>,
        theme: QtmTheme,
        cache_path: PathBuf,
        is_authenticated: Rc<Cell<bool>>,
    ) -> Self {
        info!("Started Password Prompt");
        set_context(cc, theme);
        Self {
            cache_path,
            is_authenticated,
            username: String::new(),
            password: String::new(),
        }
    }

    fn is_valid(&self) -> bool {
        !self.username.is_empty()
            && self.username.is_ascii()
            && !self.password.is_empty()
            && self.password.is_ascii()
    }

    fn authenticate(&self) -> bool {
        // TODO: Authenticate and store cookie in `cache_path`
        self.is_authenticated.set(true);
        true
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
                        ui.add_sized(
                            vec2(200., 20.),
                            widgets::text_edit::TextEdit::singleline(&mut self.password)
                                .password(true),
                        );
                    });
                    ui.add_space(10.);
                    ui.set_enabled(self.is_valid());
                    if ui
                        .add_sized(vec2(200., 40.), widgets::Button::new("LOG IN"))
                        .clicked()
                    {
                        info!("Attempt to log in");
                        // TODO: authenticate username/password
                        if self.authenticate() {
                            info!("Authenticated");
                            frame.close();
                        }
                    }
                });
            });
    }
}
