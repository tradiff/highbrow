use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation};
use std::process::Command;

fn main() {
    let app = Application::builder()
        .application_id("me.travistx.crossroads")
        .flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // When no URIs are provided on startup
    app.connect_activate(|app| {
    });

    // When started with a URI
    app.connect_open(|app, files, _| {
        let uris: Vec<String> = files.iter()
        .map(|file| file.uri().to_string())
        .collect();
        build_ui(app, uris);
    });

    app.run();
}

fn build_ui(app: &Application, args: Vec<String>) {
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

    // (Display label, command)
    let apps = vec![
        ("Firefox", "firefox"),
        ("Chromium", "chromium-browser"),
    ];

    for (label, cmd) in apps {
        let button = Button::builder().label(label).build();
        let cmd = cmd.to_string();

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
