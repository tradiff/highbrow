use gdk4::{Display, ModifierType};
use glib::MainLoop;
use gtk4::glib::timeout_add_seconds_local;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, EventControllerKey, Image,
    Label, MessageDialog, MessageType, Orientation, gdk, gio::File, glib, prelude::*,
};
use regex::RegexBuilder;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::config::{BrowserConfig, Config};

pub fn run(app: &Application, files: &[File], config: &Config) {
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

    build_ui(app, &url, config);
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
        .title("Select Browser - Highbrow")
        .icon_name("highbrow")
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(15)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
    window.set_child(Some(&vbox));

    let truncated_url = if url.len() > 75 {
        format!("{}...", &url[..75])
    } else {
        url.to_string()
    };

    let url_label = Label::new(Some(&truncated_url));
    url_label.set_tooltip_text(Some(url));
    url_label.add_css_class("url-label");
    url_label.set_selectable(true);
    vbox.append(&url_label);

    let instructions = Label::new(Some("Select a browser or press Escape to cancel"));
    instructions.add_css_class("dim-label");
    vbox.append(&instructions);

    let button_container = create_button_container(config, url, app);
    vbox.append(&button_container);

    setup_keyboard_shortcuts(&window, app, url, &url_label);
    window.present();
}

fn create_button_container(config: &Config, url: &str, app: &Application) -> GtkBox {
    let container = GtkBox::new(Orientation::Horizontal, 10);

    for browser in &config.browsers {
        let button = create_browser_button(browser, url, app);
        container.append(&button);
    }

    container
}

fn create_browser_button(browser: &BrowserConfig, url: &str, app: &Application) -> Button {
    let button = Button::builder().build();
    button.add_css_class("browser-button");

    let content = GtkBox::new(Orientation::Vertical, 8);
    let image = Image::from_icon_name(&browser.icon_name);
    image.set_pixel_size(92);
    content.append(&image);
    let label = Label::new(Some(&browser.label));
    label.set_use_underline(true);
    content.append(&label);
    button.set_child(Some(&content));
    let cmd = browser.command.clone();
    let url_clone = url.to_string();
    let app_clone = app.clone();

    button.connect_clicked(move |_| {
        spawn_browser(&cmd, &url_clone);
        app_clone.quit();
    });

    button
}

fn setup_keyboard_shortcuts(
    window: &ApplicationWindow,
    app: &Application,
    url: &str,
    label: &Label,
) {
    let key_controller = EventControllerKey::new();
    let app_clone = app.clone();
    let url_clone = url.to_string();
    let label_clone = label.clone();

    key_controller.connect_key_pressed(move |_, keyval, keymod, _| match keyval {
        gdk::Key::Escape => {
            app_clone.quit();
            glib::Propagation::Stop
        }
        gdk::Key::c if keymod & gdk::ModifierType::CONTROL_MASK.bits() != 0 => {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&url_clone);
                show_status_message(&label_clone, "URL copied to clipboard");
            }
            glib::Propagation::Stop
        }
        _ => glib::Propagation::Proceed,
    });
    window.add_controller(key_controller);
}

fn spawn_browser(cmd: &str, url: &str) {
    let mut parts = cmd.split_whitespace();

    if let Some(program) = parts.next() {
        let mut command = Command::new(program);
        command.args(parts).arg(url);

        if let Err(e) = command.spawn() {
            show_error_dialog(&format!("Failed to launch {}: {}", cmd, e));
        }
    }
}

fn show_status_message(status_label: &Label, message: &str) {
    let original_text = status_label.text().to_string();
    status_label.set_text(message);
    status_label.add_css_class("status-success");

    let status_label_clone = status_label.clone();
    timeout_add_seconds_local(1, move || {
        status_label_clone.set_text(&original_text);
        status_label_clone.remove_css_class("status-success");
        glib::ControlFlow::Break
    });
}

fn show_error_dialog(message: &str) {
    let dialog = MessageDialog::builder()
        .modal(true)
        .buttons(ButtonsType::Close)
        .message_type(MessageType::Error)
        .text("Error")
        .secondary_text(message)
        .build();

    dialog.connect_response(|d, _| d.close());
    dialog.present();
}

fn is_modifier_pressed(app: &Application) -> bool {
    let result_state = Arc::new(Mutex::new(ModifierType::empty()));
    let result_clone = result_state.clone();

    // Create a glib MainLoop to block until focus event
    let main_loop = MainLoop::new(None, false);
    let loop_clone = main_loop.clone();

    // Build a tiny window to trigger focus
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1)
        .default_height(1)
        .decorated(false)
        .build();
    window.show();

    // Listen for the window becoming active
    window.connect_notify(Some("is-active"), move |win, pspec| {
        if pspec.name() == "is-active" && win.is_active() {
            // Capture modifier state from GDK
            if let Some(display) = Display::default() {
                if let Some(seat) = display.default_seat() {
                    if let Some(keyboard) = seat.keyboard() {
                        let state = keyboard.modifier_state();
                        *result_clone.lock().unwrap() = state;
                    }
                }
            }
            // Close the temporary window and quit the blocking MainLoop
            win.close();
            loop_clone.quit();
        }
    });

    // Run until quit() is called
    main_loop.run();

    // Retrieve and test the result
    let state = *result_state.lock().unwrap();
    state.contains(ModifierType::ALT_MASK) || state.contains(ModifierType::CONTROL_MASK)
}
