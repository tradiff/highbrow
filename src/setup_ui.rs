use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, Label,
    MessageDialog, MessageType, Orientation,
};
use std::process::Command;

pub struct SetupUI;

impl SetupUI {
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
                SetupUI::show_dialog("Default browser set.", MessageType::Info, "Info")
            }
            Ok(status) => SetupUI::show_dialog(
                &format!("xdg-settings exited with {}", status),
                MessageType::Error,
                "Error",
            ),
            Err(e) => SetupUI::show_dialog(
                &format!("Failed to run xdg-settings: {}", e),
                MessageType::Error,
                "Error",
            ),
        }
    }

    fn show_dialog(message: &str, msg_type: MessageType, title: &str) {
        let application = Application::default();
        let parent_window = application
            .windows()
            .first()
            .and_then(|w| w.downcast_ref::<ApplicationWindow>().cloned())
            .unwrap_or_else(|| {
                ApplicationWindow::builder()
                    .application(&application)
                    .build()
            });

        let dialog = MessageDialog::builder()
            .transient_for(&parent_window)
            .modal(true)
            .buttons(ButtonsType::Close)
            .message_type(msg_type)
            .text(title)
            .secondary_text(message)
            .build();

        dialog.connect_response(|d, _| d.close());
        dialog.present();
    }
}
