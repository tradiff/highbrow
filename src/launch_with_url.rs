use gtk4::gdk;
use gtk4::gio::File;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, EventControllerKey, Image,
    Label, MessageDialog, MessageType, Orientation,
};
use gtk4::{Inhibit, prelude::*};
use regex::Regex;
use std::process::Command;

use crate::config::Config;

pub struct LaunchWithUrl;

impl LaunchWithUrl {
    pub fn run(app: &Application, files: &[File], config: &Config) {
        let url = files
            .get(0)
            .map(|f| f.uri().to_string())
            .unwrap_or_default();

        // find the first rule whose regex matches
        if let Some(browser) = config
            .rules
            .iter()
            .filter_map(|r| {
                Regex::new(&r.regex)
                    .ok()
                    .filter(|re| re.is_match(&url))
                    .map(|_| r.browser_id.clone())
            })
            .filter_map(|id| config.browsers.iter().find(|b| b.id == id))
            .next()
        {
            Self::spawn_browser(&browser.command, &url);
            app.quit();
        } else {
            Self::build_ui(app, &url, &config);
        }
    }

    fn build_ui(app: &Application, url: &str, config: &Config) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Browser Fork")
            .resizable(false)
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

        vbox.append(&Label::new(Some(&format!("Opening {}", url))));
        let hbox = GtkBox::new(Orientation::Horizontal, 10);
        vbox.append(&hbox);

        for browser in &config.browsers {
            let btn = Button::builder().build();
            let content = GtkBox::new(Orientation::Vertical, 5);
            let image = Image::from_icon_name(&browser.icon_name);
            image.set_pixel_size(92);
            content.append(&image);
            let label = Label::new(Some(&browser.label));
            label.set_use_underline(true);
            content.append(&label);
            btn.set_child(Some(&content));

            let cmd = browser.command.clone();
            let url = url.to_string();
            let app_clone = app.clone();
            btn.connect_clicked(move |_| {
                Self::spawn_browser(&cmd, &url);
                app_clone.quit();
            });
            hbox.append(&btn);
        }

        let app_clone = app.clone();
        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
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

    fn spawn_browser(cmd: &str, url: &str) {
        if Command::new(cmd).arg(url).spawn().is_err() {
            show_dialog(
                &format!("Failed to launch {} for {}", cmd, url),
                MessageType::Error,
                "Error",
            );
        }
    }
}

fn show_dialog(message: &str, msg_type: MessageType, title: &str) {
    let application = Application::default();
    let parent_window = application
        .windows()
        .first()
        .and_then(|w| w.downcast_ref::<ApplicationWindow>().cloned())
        .unwrap_or_else(|| {
            ApplicationWindow::builder()
                .application(&application)
                .build()
        });

    let dialog = MessageDialog::builder()
        .transient_for(&parent_window)
        .modal(true)
        .buttons(ButtonsType::Close)
        .message_type(msg_type)
        .text(title)
        .secondary_text(message)
        .build();

    dialog.connect_response(|d, _| d.close());
    dialog.present();
}
