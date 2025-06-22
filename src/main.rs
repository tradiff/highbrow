mod config;
mod launch_with_url;
mod setup_ui;

use config::load_config_or_default;
use gtk4::Application;
use gtk4::CssProvider;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use setup_ui::SetupUI;

fn main() {
    env_logger::init();
    log::info!("Starting Highbrow browser chooser");

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
        SetupUI::show(app);
    });

    // When started with a URI
    app.connect_open(move |app, files, _| {
        launch_with_url::run(app, files, &config);
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
        font-weight: bold;
    }
    
    .dim-label {
        opacity: 0.7;
    }
    
    .heading {
        font-weight: bold;
        font-size: 1.1em;
    }
    
    button {
        border-radius: 8px;
        padding: 12px 16px;
    }
    
    button:hover {
        background: alpha(@accent_color, 0.1);
    }
    
    .browser-button {
        padding: 16px;
        margin: 4px;
        border-radius: 12px;
        transition: all 200ms ease;
    }
    
    .browser-button:hover {
        background: alpha(@accent_color, 0.15);
        box-shadow: 0 4px 12px alpha(@accent_color, 0.2);
    }
    
    .url-label {
        font-family: monospace;
        font-size: 0.9em;
        background: alpha(@window_bg_color, 0.5);
        padding: 8px 12px;
        border-radius: 6px;
        border: 1px solid alpha(@borders, 0.3);
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
