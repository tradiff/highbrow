use gtk4::gdk::{self, Display};
use gtk4::gio::File;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, Image, Inhibit,
    Label, MessageDialog, MessageType, Orientation,
};
use gtk4::{CssProvider, EventControllerKey, prelude::*};
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
                    .message_type(MessageType::Error)
                    .buttons(ButtonsType::Close)
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
        let css_provider = CssProvider::new();
        css_provider.load_from_data(
            "
        button {
            border: none;
            box-shadow: none;
        }
        ",
        );
        let display = Display::default().expect("Could not get default display");
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Browser Fork")
            .resizable(false)
            .mnemonics_visible(true)
            .modal(true)
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

        let url_label = Label::new(Some(&format!("Opening {}", url)));
        url_label.set_halign(Align::Start);
        vbox.append(&url_label);

        let button_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .build();
        vbox.append(&button_box);

        for browser in &config.browsers {
            let button_content = GtkBox::builder()
                .orientation(Orientation::Vertical)
                .spacing(5)
                .build();

            let image = Image::from_icon_name(&browser.icon_name);
            image.set_pixel_size(92);
            button_content.append(&image);

            let label = Label::new(Some(&browser.label));
            label.set_use_underline(true);
            button_content.append(&label);
            let button = Button::builder().use_underline(true).build();
            button.set_child(Some(&button_content));

            button_box.append(&button);

            let cmd = browser.command.clone();
            let local_url = url.clone();
            let app_inner = app.clone();

            button.connect_clicked(move |_| {
                let result = Self::spawn_browser(&app_inner, &cmd, &local_url);
                if result {
                    app_inner.quit();
                }
            });
        }

        let app_clone = app.clone();
        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(move |_ctrl, keyval, _keycode, _state| {
            if keyval == gdk::Key::Escape {
                app_clone.quit();
                Inhibit(true)
            } else {
                Inhibit(false)
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
                .message_type(MessageType::Error)
                .buttons(ButtonsType::Close)
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
