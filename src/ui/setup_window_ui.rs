use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::process::Command;

use crate::utils::{show_error_dialog, show_info_dialog};

pub struct SetupWindowUi;

impl SetupWindowUi {
    pub fn show(app: &Application) {
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

        let title_label = Label::builder().halign(Align::Center).build();
        title_label.set_markup("<span size='xx-large'><b>Highbrow</b></span>");
        vbox.append(&title_label);

        let default_browser_button = Button::builder()
            .label("Set Highbrow as default browser")
            .halign(Align::Center)
            .build();
        default_browser_button.connect_clicked(|_| Self::set_as_default_browser());
        vbox.append(&default_browser_button);

        window.present();
    }

    fn set_as_default_browser() {
        let result = Command::new("xdg-settings")
            .args(["set", "default-web-browser", "highbrow.desktop"])
            .status();

        match result {
            Ok(status) if status.success() => {
                show_info_dialog("Default browser set.");
            }
            Ok(status) => {
                show_error_dialog(&format!("xdg-settings exited with {}", status));
            }
            Err(e) => {
                show_error_dialog(&format!("Failed to run xdg-settings: {}", e));
            }
        }
    }
}
