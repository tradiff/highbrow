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
        build_ui(app, uris, &config);
    });

    app.run();
}

fn build_ui(app: &Application, args: Vec<String>, config: &Config) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Crossroads")
        .default_width(300)
        .default_height(100)
        .build();

    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    window.set_child(Some(&button_box));

    for browser in &config.browsers {
        let button = Button::builder().label(&browser.label).build();
        let cmd = browser.command.clone();
        let args_clone = args.clone();

        button.connect_clicked(move |_| {
            if let Err(e) = Command::new(&cmd).args(&args_clone).spawn() {
                eprintln!("Failed to launch {}: {}", cmd, e);
            }
        });
        button_box.append(&button);
    }

    window.present();
}
