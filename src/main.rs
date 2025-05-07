mod config;
mod launch_with_url;
mod setup_ui;

use config::load_config;
use gtk4::Application;
use gtk4::prelude::*;
use launch_with_url::LaunchWithUrl;
use setup_ui::SetupUI;

fn main() {
    let config = load_config();
    let app = Application::builder()
        .flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // When no URIs are provided on startup
    app.connect_activate(|app| {
        SetupUI::show(app);
    });

    // When started with a URI
    app.connect_open(move |app, files, _| {
        LaunchWithUrl::run(app, files, &config);
    });

    app.run();
}
