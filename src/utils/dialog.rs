use gtk4::{ApplicationWindow, ButtonsType, MessageDialog, MessageType, prelude::*};

// Show a simple error dialog
pub fn show_error_dialog(message: &str) {
    show_message_dialog(message, MessageType::Error, "Error", None);
}

// Show an info dialog
pub fn show_info_dialog(message: &str) {
    show_message_dialog(message, MessageType::Info, "Info", None);
}

// Show a message dialog with custom parameters
pub fn show_message_dialog(
    message: &str,
    msg_type: MessageType,
    title: &str,
    parent: Option<&ApplicationWindow>,
) {
    let mut builder = MessageDialog::builder()
        .modal(true)
        .buttons(ButtonsType::Close)
        .message_type(msg_type)
        .text(title)
        .secondary_text(message);

    if let Some(parent_window) = parent {
        builder = builder.transient_for(parent_window);
    }

    let dialog = builder.build();
    dialog.connect_response(|d, _| d.close());
    dialog.present();
}
