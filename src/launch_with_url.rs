use gtk4::gdk;
use gtk4::gio::File;
use gtk4::glib::timeout_add_seconds_local;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, EventControllerKey, Image,
    Label, MessageDialog, MessageType, Orientation,
};
use gtk4::{Inhibit, prelude::*};
use regex::RegexBuilder;
use std::process::Command;

use crate::config::{BrowserConfig, Config};

pub struct LaunchWithUrl;

impl LaunchWithUrl {
    pub fn run(app: &Application, files: &[File], config: &Config) {
        let url = files
            .get(0)
            .map(|f| f.uri().to_string())
            .unwrap_or_default();

        if let Some(browser) = Self::find_browser_for_url(&url, config) {
            Self::spawn_browser(&browser.command, &url);
            app.quit();
        } else {
            Self::build_ui(app, &url, &config);
        }
    }

    fn find_browser_for_url(url: &str, config: &Config) -> Option<BrowserConfig> {
        config
            .browsers
            .iter()
            .find(|b| {
                b.patterns.as_ref().map_or(false, |pats| {
                    pats.iter().any(|pat| {
                        RegexBuilder::new(pat)
                            .case_insensitive(true)
                            .build()
                            .map_or(false, |re| re.is_match(url))
                    })
                })
            })
            .cloned()
    }

    fn build_ui(app: &Application, url: &str, config: &Config) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Highbrow")
            .icon_name("highbrow")
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

        let truncated_url = if url.len() > 75 {
            format!("{}...", &url[..75])
        } else {
            url.to_string()
        };

        let label = Label::new(Some(&truncated_url));
        label.set_tooltip_text(Some(url));
        vbox.append(&label);
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
        let url_clone = url.to_string();
        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, keymod, _| {
            if keyval == gdk::Key::Escape {
                app_clone.quit();
                Inhibit(true)
            } else if keyval == gdk::Key::c
                && (keymod & gdk::ModifierType::CONTROL_MASK.bits()) != 0
            {
                // Copy URL to clipboard
                if let Some(display) = gdk::Display::default() {
                    display.clipboard().set_text(&url_clone);

                    // Show temporary status message
                    Self::show_status_message(&label, "URL copied to clipboard");
                }
                Inhibit(true)
            } else {
                Inhibit(false)
            }
        });
        window.add_controller(key_controller);

        window.present();
    }

    fn spawn_browser(cmd: &str, url: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        let mut command = Command::new(parts[0]);
        for arg in parts.iter().skip(1) {
            command.arg(arg);
        }
        // Add the URL as the final argument
        command.arg(url);

        if command.spawn().is_err() {
            show_dialog(
                &format!("Failed to launch {} for {}", cmd, url),
                MessageType::Error,
                "Error",
            );
        }
    }

    fn show_status_message(status_label: &Label, message: &str) {
        let old_message = status_label.text().to_string();
        status_label.set_text(message);
        status_label.add_css_class("status-success");

        let status_label_clone = status_label.clone();
        timeout_add_seconds_local(1, move || {
            status_label_clone.set_text(&old_message);
            status_label_clone.remove_css_class("status-success");
            Continue(false)
        });
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
