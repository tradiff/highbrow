mod config;
mod ui;
mod utils;

use config::{find_browser_for_url, load_config_or_default};
use gtk4::Application;
use gtk4::gio::File;
use gtk4::prelude::*;
use ui::{BrowserSelectorUi, SetupWindowUi, setup_css_overrides};
use utils::is_modifier_pressed;
use utils::launcher::spawn_browser;

fn main() {
    env_logger::init();
    log::info!("Starting Highbrow browser selector");

    let config = load_config_or_default();
    log::info!(
        "Loaded configuration with {} browsers",
        config.browsers.len()
    );

    gtk4::init().unwrap();
    setup_css_overrides();
    let app = Application::builder()
        .flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // When no URIs are provided on startup
    app.connect_activate(|app| {
        SetupWindowUi::show(app);
    });

    // When started with a URI
    app.connect_open(move |app, files, _| {
        handle_url_launch(app, files, &config);
    });

    app.run();
}

fn handle_url_launch(app: &Application, files: &[File], config: &config::Config) {
    let url = files
        .first()
        .map(|f| f.uri().to_string())
        .unwrap_or_default();

    let modifier_pressed = is_modifier_pressed(app);

    if !modifier_pressed {
        if let Some(browser) = find_browser_for_url(&url, config) {
            spawn_browser(&browser.command, &url);
            return;
        }
    }

    // Show the browser selection UI
    BrowserSelectorUi::show(app, &url, config);
}
