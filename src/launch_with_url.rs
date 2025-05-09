use gtk4::gdk::{self};
use gtk4::gio::File;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, MessageDialog, Orientation};
use gtk4::{EventControllerKey, prelude::*};
use regex::Regex;
use std::process::Command;

use crate::config::Config;

pub struct LaunchWithUrl;

impl LaunchWithUrl {
    pub fn run(app: &Application, files: &[File], config: &Config) {
        let compiled_rules: Vec<(Regex, String)> = config
            .rules
            .iter()
            .map(|r| (Regex::new(&r.regex).unwrap(), r.browser_id.clone()))
            .collect();

        let url = files
            .iter()
            .map(|file| file.uri().to_string())
            .collect::<Vec<String>>()
            .get(0)
            .cloned()
            .unwrap_or_default();

        // find the first rule whose regex matches
        let matching_rule = compiled_rules.iter().find(|(re, _)| re.is_match(&url));
        if let Some((_, browser_id)) = matching_rule {
            if let Some(browser) = config.browsers.iter().find(|b| &b.id == browser_id) {
                let result = Self::spawn_browser(app, &browser.command, &url);
                if result {
                    app.quit();
                }
            } else {
                let error_dialog = MessageDialog::builder()
                    .modal(true)
                    .message_type(gtk4::MessageType::Error)
                    .buttons(gtk4::ButtonsType::Close)
                    .text(&format!("No browser with id '{}' in config", browser_id))
                    .build();

                error_dialog.set_transient_for(Some(&ApplicationWindow::new(app)));
                let app_clone = app.clone();
                error_dialog.connect_response(move |dialog, _response| {
                    dialog.close();
                    app_clone.quit();
                });

                error_dialog.show();
            }
        } else {
            Self::build_ui(app, url.clone(), &config);
        }
    }

    fn build_ui(app: &Application, url: String, config: &Config) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Browser Fork")
            .resizable(false)
            .mnemonics_visible(true)
            .build();

        let vbox = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();
        window.set_child(Some(&vbox));

        let url_label = gtk4::Label::new(Some(&format!("Opening {}", url)));
        url_label.set_halign(gtk4::Align::Start);
        vbox.append(&url_label);

        let button_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .build();
        vbox.append(&button_box);

        for browser in &config.browsers {
            let button = Button::builder()
                .label(&browser.label)
                .use_underline(true)
                .build();
            let cmd = browser.command.clone();
            let local_url = url.clone();
            let app_inner = app.clone();

            button.connect_clicked(move |_| {
                let result = Self::spawn_browser(&app_inner, &cmd, &local_url);
                if result {
                    app_inner.quit();
                }
            });
            button_box.append(&button);
        }

        let app_clone = app.clone();
        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(move |_ctrl, keyval, _keycode, _state| {
            if keyval == gdk::Key::Escape {
                app_clone.quit();
                gtk4::Inhibit(true)
            } else {
                gtk4::Inhibit(false)
            }
        });
        window.add_controller(key_controller);

        window.present();
    }

    fn spawn_browser(app: &Application, command: &String, url: &String) -> bool {
        let result = Command::new(command).arg(url).spawn();
        if let Err(e) = result {
            let error_dialog = MessageDialog::builder()
                .modal(true)
                .message_type(gtk4::MessageType::Error)
                .buttons(gtk4::ButtonsType::Close)
                .text(&format!("Failed to launch {}: {}", command, e))
                .build();

            error_dialog.set_transient_for(Some(&ApplicationWindow::new(app)));
            let app_clone = app.clone();
            error_dialog.connect_response(move |dialog, _response| {
                dialog.close();
                app_clone.quit();
            });

            error_dialog.show();
            false
        } else {
            true
        }
    }
}
