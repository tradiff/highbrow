use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, Label, MessageDialog,
    MessageType, Orientation,
};
use std::process::Command;
use std::{env, fs, path::Path};

pub struct SetupUI;

impl SetupUI {
    pub fn show(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Browser Fork Setup")
            .build();
        window.set_default_size(400, 200);

        let vbox = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();
        window.set_child(Some(&vbox));

        let title_label = Label::new(None);
        title_label.set_markup("<span size='xx-large'><b>Browser Fork</b></span>");
        title_label.set_halign(gtk4::Align::Center);
        vbox.append(&title_label);

        let default_browser_button = Button::with_label("Set Browser Fork as default browser");
        default_browser_button.set_halign(gtk4::Align::Center);
        default_browser_button.connect_clicked(move |_| {
            Self::set_as_default_browser();
        });
        vbox.append(&default_browser_button);

        window.present();
    }

    fn set_as_default_browser() {
        Self::create_desktop_file_if_needed();
        // Attempt to set Browser Fork as the default web browser using xdg-settings
        match Command::new("xdg-settings")
            .arg("set")
            .arg("default-web-browser")
            .arg("browser-fork.desktop")
            .spawn()
        {
            Ok(_) => Self::show_info_dialog("Default browser set to Browser Fork."),
            Err(e) => Self::show_error_dialog(&format!("Error setting default browser: {}", e)),
        }
    }

    fn create_desktop_file_if_needed() {
        // Determine the path for the desktop file
        let home = match env::var("HOME") {
            Ok(h) => h,
            Err(e) => {
                Self::show_error_dialog(&format!("Error getting HOME directory: {}", e));
                return;
            }
        };
        let desktop_dir = format!("{}/.local/share/applications", home);
        let desktop_file = format!("{}/browser-fork.desktop", desktop_dir);

        // Create the directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&desktop_dir) {
            Self::show_error_dialog(&format!(
                "Could not create directory {}: {}",
                desktop_dir, e
            ));
            return;
        }

        let exe_cmd = match env::current_exe() {
            Ok(path) => match path.to_str() {
                Some(s) => s.to_string(),
                None => {
                    Self::show_error_dialog("Error converting executable path to string");
                    "browser-fork".to_string()
                }
            },
            Err(e) => {
                Self::show_error_dialog(&format!("Error getting current executable: {}", e));
                "browser-fork".to_string()
            }
        };

        // Desktop file content
        let desktop_content = format!("\
[Desktop Entry]
Version=1.0
Name=Browser Fork
GenericName=Web Browser
Comment=Browse the Web
Exec={} %u
Icon=firefox
Terminal=false
Type=Application
MimeType=text/html;text/xml;application/xhtml+xml;application/vnd.mozilla.xul+xml;text/mml;x-scheme-handler/http;x-scheme-handler/https;
StartupNotify=true
Categories=Network;WebBrowser;
Keywords=web;browser;internet;
X-Desktop-File-Install-Version=0.27
", exe_cmd);

        // Create the desktop file if it doesn't already exist
        if !Path::new(&desktop_file).exists() {
            match fs::write(&desktop_file, desktop_content) {
                Ok(_) => {
                    Self::show_info_dialog(&format!("Created desktop file at {}", desktop_file))
                }
                Err(e) => {
                    Self::show_error_dialog(&format!("Error creating desktop file: {}", e));
                    return;
                }
            }

            // Update the desktop database
            match Command::new("update-desktop-database")
                .arg(&desktop_dir)
                .status()
            {
                Ok(status) if status.success() => {
                    Self::show_info_dialog("Desktop database updated.")
                }
                Ok(status) => Self::show_error_dialog(&format!(
                    "update-desktop-database exited with status: {}",
                    status
                )),
                Err(e) => Self::show_error_dialog(&format!(
                    "Failed to execute update-desktop-database: {}",
                    e
                )),
            }
        }
    }

    fn show_error_dialog(message: &str) {
        let application = Application::default();
        let parent_window = application
            .windows()
            .first()
            .and_then(|w| w.downcast_ref::<ApplicationWindow>().cloned());

        let dialog = MessageDialog::builder()
            .transient_for(&parent_window.unwrap_or_else(|| ApplicationWindow::new(&application)))
            .modal(true)
            .buttons(ButtonsType::Close)
            .message_type(MessageType::Error)
            .text("Error")
            .secondary_text(message)
            .build();

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.present();
    }

    fn show_info_dialog(message: &str) {
        // Similar to error dialog but with MessageType::Info
        let application = Application::default();
        let parent_window = application
            .windows()
            .first()
            .and_then(|w| w.downcast_ref::<ApplicationWindow>().cloned());

        let dialog = MessageDialog::builder()
            .transient_for(&parent_window.unwrap_or_else(|| ApplicationWindow::new(&application)))
            .modal(true)
            .buttons(ButtonsType::Close)
            .message_type(MessageType::Info)
            .text("Information")
            .secondary_text(message)
            .build();

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.present();
    }
}
