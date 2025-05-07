mod config;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation};
use std::process::Command;
use config::{load_config, Config};

fn main() {
    let config = load_config();
    let app = Application::builder()
        .application_id("me.travistx.crossroads")
        .flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // When no URIs are provided on startup
    app.connect_activate(|app| {
    });

    // When started with a URI
    app.connect_open(move |app, files, _| {
        let uris: Vec<String> = files.iter()
        .map(|file| file.uri().to_string())
        .collect();
        let url = uris.get(0).cloned().unwrap_or_default();
        build_ui(app, url, &config);
    });

    app.run();
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
            if let Err(e) = Command::new(&cmd).arg(&local_url).spawn() {
                eprintln!("Failed to launch {}: {}", cmd, e);
            }
        });
        button_box.append(&button);
    }

    window.present();
}
