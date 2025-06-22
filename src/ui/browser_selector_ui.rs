use crate::config::{BrowserConfig, Config};
use crate::utils;
use gtk4::gdk;
use gtk4::glib::timeout_add_seconds_local;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Image, Label,
    Orientation, glib, prelude::*,
};
use utils::launcher::spawn_browser;

pub struct BrowserSelectorUi;

impl BrowserSelectorUi {
    pub fn show(app: &Application, url: &str, config: &Config) {
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

        let button_container = Self::create_button_container(config, url, app);
        vbox.append(&button_container);

        Self::setup_keyboard_shortcuts(&window, app, url, &url_label);
        window.present();
    }

    fn create_button_container(config: &Config, url: &str, app: &Application) -> GtkBox {
        let container = GtkBox::new(Orientation::Horizontal, 10);

        for browser in &config.browsers {
            let button = Self::create_browser_button(browser, url, app);
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
                    Self::show_status_message(&label_clone, "URL copied to clipboard");
                }
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
        });
        window.add_controller(key_controller);
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
}
