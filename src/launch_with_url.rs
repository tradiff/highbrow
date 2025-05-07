use gtk4::gio::File;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation};
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
                Self::spawn_browser(&browser.command, &url);
                app.quit();
            } else {
                eprintln!("⚠ no browser with id `{}` in config", browser_id);
                app.quit();
            }
        } else {
            eprintln!("Loading default ui");
            Self::build_ui(app, url.clone(), &config);
        }
    }

    fn build_ui(app: &Application, url: String, config: &Config) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Crossroads")
            .build();
        window.set_resizable(false);

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
            let button = Button::builder().label(&browser.label).build();
            let cmd = browser.command.clone();
            let local_url = url.clone();

            button.connect_clicked(move |_| {
                Self::spawn_browser(&cmd, &local_url);
            });
            button_box.append(&button);
        }

        window.present();
    }

    fn spawn_browser(command: &String, url: &String) {
        let result = Command::new(command).arg(url).spawn();
        if let Err(e) = result {
            eprintln!("Failed to launch {}: {}", command, e);
        }
    }
}
