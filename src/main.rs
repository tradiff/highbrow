mod config;
mod launch_with_url;
mod setup_ui;

use config::load_config;
use gtk4::Application;
use gtk4::CssProvider;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use launch_with_url::LaunchWithUrl;
use setup_ui::SetupUI;

fn main() {
    let config = load_config();
    gtk4::init().unwrap();
    setup_css_overrides();
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

fn setup_css_overrides() {
    let css = "
    .background {
        border-top-left-radius: 0px;
        border-top-right-radius: 0px;
    }
    .status-success {
        color: @success_color;
    }
    ";
    let provider = CssProvider::new();
    provider.load_from_data(css);

    gtk4::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );
}
