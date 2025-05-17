use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, Label,
    MessageDialog, MessageType, Orientation,
};
use std::{env, fs, path::PathBuf, process::Command};

pub struct SetupUI;

impl SetupUI {
    pub fn show(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Highbrow Setup")
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
        let _ = Self::create_desktop_file_if_needed();
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

    fn create_desktop_file_if_needed() -> Result<(), ()> {
        let home = env::var("HOME").map_err(|_| ())?;
        Self::install_icon_if_needed(&home);

        let desktop_dir = PathBuf::from(&home).join(".local/share/applications");
        fs::create_dir_all(&desktop_dir).map_err(|_| ())?;

        let desktop_file = desktop_dir.join("highbrow.desktop");
        if desktop_file.exists() {
            return Ok(());
        }

        let exe_cmd = env::current_exe()
            .ok()
            .and_then(|p| p.to_str().map(String::from))
            .unwrap_or_else(|| "highbrow".into());
        let content = format!("\
[Desktop Entry]
Version=1.0
Name=Highbrow
GenericName=Web Browser
Comment=Browse the Web
Exec={} %u
Icon=highbrow
Terminal=false
Type=Application
MimeType=text/html;text/xml;application/xhtml+xml;application/vnd.mozilla.xul+xml;text/mml;x-scheme-handler/http;x-scheme-handler/https;
StartupNotify=true
Categories=Network;WebBrowser;
Keywords=web;browser;internet;
X-Desktop-File-Install-Version=0.27
", exe_cmd);

        fs::write(&desktop_file, content).map_err(|_| ())?;
        let _ = Command::new("update-desktop-database")
            .arg(desktop_dir)
            .status();
        Ok(())
    }

    fn install_icon_if_needed(home: &str) {
        let target_icon_dir = PathBuf::from(&home).join(".local/share/icons/hicolor/symbolic/apps");
        let target_icon_file = target_icon_dir.join("highbrow.svg");
        if target_icon_file.exists() {
            return;
        }

        if let Err(e) = fs::create_dir_all(&target_icon_dir) {
            Self::show_dialog(
                &format!(
                    "Could not create icon directory {}: {}",
                    target_icon_dir.display(),
                    e
                ),
                MessageType::Error,
                "Error",
            );
            return;
        }

        let exe_dir = match env::current_exe() {
            Ok(mut path) => {
                path.pop();
                path
            }
            Err(e) => {
                Self::show_dialog(
                    &format!("Error getting executable directory: {}", e),
                    MessageType::Error,
                    "Error",
                );
                return;
            }
        };

        let source_icon_path = exe_dir.join("highbrow.svg");
        match fs::copy(&source_icon_path, &target_icon_file) {
            Ok(_) => {}
            Err(e) => {
                Self::show_dialog(
                    &format!("Error copying icon file: {}", e),
                    MessageType::Error,
                    "Error",
                );
            }
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
